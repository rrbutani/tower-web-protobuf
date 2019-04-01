use crate::common::*;
use crate::extensions::MessageParseStrategy;

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

fn parse_headers<T>(request: &HttpRequest<T>, header: HeaderName) -> (bool, bool) {
    let content_type_headers = request.headers().get_all(header);

    // We're going to be strict about having the right header for JSON:
    let json = content_type_headers.iter()
        .any(|h| h == &"application/json");

    // But somewhat lenient for Protobufs since there isn't an official
    // thing. We'll take: "application/[x-]protobuf[; <message type>]".
    let proto = content_type_headers.iter().any(|h| {
        match h.to_str() {
            Ok(x) => x.starts_with("application/protobuf") ||
                x.starts_with("application/x-protobuf"),
            _ => false
        }
    });

    // Note: for now we disregard message tpes in the content-type header, but
    // in the future we should ensure that this matches the type for the
    // endpoint we're hitting (TODO). This is a little tricky since we don't
    // have access to that information here.

    (json, proto)
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
        use http::header::CONTENT_TYPE;


        let (json, proto) = parse_headers(&request, CONTENT_TYPE);
        let json = json && self.config.receive_json;

        let extensions = request.extensions_mut();

        use self::MessageParseStrategy::*;
        let exists = extensions.insert(match (json, proto) {
            (_,     true)  => {Proto},
            (true,  false) => {Json},
            (false, false) => {None},
        }).is_some();

        // We're using extensions to record what the Extractor should do; this
        // is a little janky but it should be fine. This check should warn us
        // if somehow we overwrite a value already in extensions.
        //
        // TODO: this should use log as be marked as a warning
        if exists { println!("eek! we've been made!!"); }

        // TODO:
        Self::Future {
            response: self.inner.call(request),
        }
    }
}
