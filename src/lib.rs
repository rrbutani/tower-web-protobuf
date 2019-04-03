#[deny(missing_docs)]
#[macro_use(try_ready)]
extern crate futures;

pub(crate) mod common {
    pub use futures::future::{Err as FutErr, Future, FutureResult, Ok as FutOk};
    pub use futures::Poll;
    pub use http::{header::HeaderName, Request as HttpRequest, Response as HttpResponse};
    pub use tower_service::Service;

    pub struct ResponseFuture<T> {
        pub response: T,
    }

    impl<F, RespBody> Future for ResponseFuture<F>
    where
        F: Future<Item = HttpResponse<RespBody>>,
    {
        type Item = F::Item;
        type Error = F::Error;

        // Just pass the response through unmodified:
        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            self.response.poll()
        }
    }
}

pub mod errors;
pub mod extensions;
pub mod extractors;
pub mod middleware;
pub mod response;
pub mod types;

pub use middleware::ProtobufMiddleware;
pub use types::Proto;

// TODO: deny missing docs
// TODO: check protobuf message name with type_info (feature gated, perhaps)
// TODO: with fork
// TODO: fix Errors in Tower Web
