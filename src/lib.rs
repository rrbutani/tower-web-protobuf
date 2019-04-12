#![deny(missing_docs, missing_debug_implementations)]

//! Middleware and friends that help deal with protobuf messages in tower-web.
//!
//! <TODO>

#[macro_use(try_ready)]
extern crate futures;

pub(crate) mod common {
    pub use futures::future::{Err as FutErr, Future, FutureResult, Ok as FutOk};
    pub use futures::Poll;
    pub use http::{header::HeaderName, Request as HttpRequest, Response as HttpResponse};
    pub use tower_service::Service;

    #[derive(Debug)]
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

pub(crate) mod errors;
pub(crate) mod extensions;
pub(crate) mod extractors;
pub(crate) mod middleware;
pub(crate) mod response;
pub(crate) mod types;

pub use middleware::ProtobufMiddleware;
pub use types::{MessagePlus, Proto};

// TODO: deny missing docs
// TODO: check protobuf message name with type_info (feature gated, perhaps)
// TODO: with fork
// TODO: fix Errors in Tower Web
