use super::{Config, ProtobufService};
use crate::common::*;

use tower_web::middleware::Middleware;

/// Decorate a service by converting incoming and outgoing Protobuf messages.
pub struct ProtobufMiddleware {
    config: Config,
}

impl ProtobufMiddleware {
    /// Create a new `ProtobufMiddleware` instance with options.
    pub fn new(send_json: bool, receive_json: bool) -> ProtobufMiddleware {
        ProtobufMiddleware {
            config: Config {
                send_json,
                receive_json,
            },
        }
    }
}

impl Default for ProtobufMiddleware {
    fn default() -> Self {
        ProtobufMiddleware::new(true, true)
    }
}

impl<S, ReqBody, RespBody> Middleware<S> for ProtobufMiddleware
where
    S: Service<Request = HttpRequest<ReqBody>, Response = HttpResponse<RespBody>>,
    S::Future: Future<Item = HttpResponse<RespBody>>,
    S::Error: ::std::error::Error,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Service = ProtobufService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        ProtobufService::new(service, self.config)
    }
}
