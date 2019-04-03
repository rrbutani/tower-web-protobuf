use crate::common::*;
use crate::extensions::{MessageEncodeStrategy, MessageParseStrategy, MessageStrategy};

#[derive(Debug, Copy, Clone)]
/// Configuration options for a [`ProtobufService`](struct.ProtobufService.html).
pub struct Config {
    /// Allow incoming Protobuf messages to be sent as [protobuf-compliant JSON](https://developers.google.com/protocol-buffers/docs/proto3#json).
    pub receive_json: bool,
    /// Allow outgoing Protobuf messages to be sent as [protobuf-compliant JSON](https://developers.google.com/protocol-buffers/docs/proto3#json).
    pub send_json: bool, // TODO: use
}

/// Decorates another Service by parsing incoming Protobuf messages and
/// serializing outgoing Protobuf messages.
///
/// Incoming and outgoing messages can be sent as [protobuf-compliant JSON](https://developers.google.com/protocol-buffers/docs/proto3#json), if
/// enabled in the given configuration. Headers are used to figure out whether
/// a message is Protobuf or JSON encoded (Content-Type and Accept headers for
/// receiving and sending, respectively).
///
/// If both JSON (`application/json`) and Protobuf (`application/x-protobuf`)
/// Accept headers are set, Protobuf will be preferred.
pub struct ProtobufService<S> {
    inner: S,
    config: Config,
}

impl<S> ProtobufService<S> {
    pub(super) fn new(inner: S, config: Config) -> ProtobufService<S> {
        ProtobufService { inner, config }
    }
}

fn parse_headers<T>(request: &HttpRequest<T>, header: HeaderName, allow_json: bool) -> MessageStrategy {
    let content_type_headers = request.headers().get_all(header);

    // We're going to be strict about having the right header for JSON:
    let json = content_type_headers.iter()
        .any(|h| h == &"application/json");

    // But somewhat lenient for Protobufs since there isn't an official
    // thing. We'll take: "application/[x-]protobuf[; <message type>]".
    let (proto, name) = content_type_headers.iter().filter_map(|h| {
        match h.to_str() {
            Ok(x) => {
                let p = x.starts_with("application/protobuf") ||
                    x.starts_with("application/x-protobuf");

                let pair = (p, if p {
                    x.split(";").next()
                } else { None });

                Some(pair)
            },
            _ => None
        }
    }).next().unwrap_or((false, None));

    // Note: for now we disregard message types in the content-type header, but
    // in the future we should ensure that this matches the type for the
    // endpoint we're hitting (TODO). This is a little tricky since we don't
    // have access to that information here.

    use self::MessageStrategy::*;
    match (json && allow_json, proto, name) {
        (_,     true, Some(name))  => { NamedProto(String::from(name)) },
        (_,     true, None)        => { Proto },
        (true,  false, _)          => { Json },
        (false, false, _)          => { Plaintext },
    }
}

impl<S, ReqBody, RespBody> Service for ProtobufService<S>
where
    S: Service<Request = HttpRequest<ReqBody>,
               Response = HttpResponse<RespBody>>,
    S::Future: Future<Item = HttpResponse<RespBody>>,
    S::Error: ::std::error::Error,
{
    type Request = S::Request; // HttpRequest<ReqBody>
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    /// Modifies the request's headers to signal to the Extractor what to do.
    fn call(&mut self, mut request: HttpRequest<ReqBody>) -> Self::Future {
        use http::header::{CONTENT_TYPE, ACCEPT};

        // Set MessageParseStrategy:
        let mps = parse_headers(&request, CONTENT_TYPE, self.config.receive_json).into();
        request.extensions_mut()
            .insert::<MessageParseStrategy>(mps)
            .map(|prev| {
            // We're using extensions to record what the Extractor should do; this
            // is a little janky but it should be fine. This check should warn us
            // if somehow we overwrite a value already in extensions.
            //
            // TODO: this should use log as be marked as a warning
            println!("eek! we've been made!! {:?}", prev)
        });

        // And likewise for MessageEncodeStrategy:
        let mes = parse_headers(&request, ACCEPT, self.config.send_json).into();
        request.extensions_mut()
            .insert::<MessageEncodeStrategy>(mes)
            .map(|prev| println!("TODO: Re-inserting!! {:?}", prev));

        // TODO:
        Self::Future {
            response: self.inner.call(request),
        }
    }
}
