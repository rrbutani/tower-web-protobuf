use middleware::*;

use extensions::MessageParseStrategy;

/// Configuration options for a [`ProtobufService`](struct.ProtobufService.html).
pub struct Config {
    /// Allow incoming Protobuf messages to be sent as [protobuf-compliant JSON](https://developers.google.com/protocol-buffers/docs/proto3#json).
    pub receiveJson: bool,
    /// Allow outgoing Protobuf messages to be sent as [protobuf-compliant JSON](https://developers.google.com/protocol-buffers/docs/proto3#json).
    pub sendJson: bool,
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

pub struct ResponseFuture<T>
{
    response: T,
}

impl<S> ProtobufService<S> {
    pub(super) fn new(inner: S, config: Config) -> ProtobufService<S> {
        ProtobufService { inner, config }
    }
}

fn parse_headers<T>(request: HttpRequest<T>, header: HeaderName) -> (bool, bool) {
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

    (json, proto)
}

impl<S, ReqBody, RespBody> Service for ProtobufService<S>
where
    S: Service<Request = HttpRequest<ReqBody>,
               Response = HttpResponse<RespBody>>,
    S::Future: Future<Item = HttpResponse<RespBody>>,
    S::Error: ::std::error::Error,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    /// Modifies the request's headers to signal to the Extractor what to do
    fn call(&mut self, request: Self::Request) -> Self::Future {
        use http::header::CONTENT_TYPE;

        let (json, proto) = parse_headers(request, CONTENT_TYPE);
        let json = json && self.config.receiveJson;

        let extensions = request.extensions_mut();

        use self::MessageParseStrategy::*;
        let exists = extensions.insert(match (json, proto) {
            (_,     true)  => {Proto},
            (true,  false) => {Json},
            (false, false) => {None},
        }).is_some();

        if exists { println!("eek! we've been made!!"); }

        // TODO:
        Self::Future {
            response: self.inner.call(request),
        }
    }
}

impl<F, RespBody> Future for ResponseFuture<F>
where
    F: Future<Item = HttpResponse<RespBody>>
{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use Async::*;

        let response = try_ready!(self.response.poll());


        Ok(Ready(response))
    }
}

