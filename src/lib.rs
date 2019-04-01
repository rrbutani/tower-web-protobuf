// TODO: deny missing docs
// TODO: stable
// TODO: with fork

#[macro_use(try_ready)] extern crate futures;

pub(crate) mod common {
    pub use futures::Poll;
    pub use futures::future::{FutureResult, Future, Ok as FutOk, Err as FutErr};
    pub use http::{Request as HttpRequest, Response as HttpResponse, header::HeaderName};
    pub use tower_service::Service;

    pub struct ResponseFuture<T> {
        pub response: T,
    }

    impl<F, RespBody> Future for ResponseFuture<F>
    where
        F: Future<Item = HttpResponse<RespBody>>
    {
        type Item = F::Item;
        type Error = F::Error;

        // Just pass the response through unmodified:
        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            self.response.poll()
        }
    }
}

pub mod config;
pub mod errors;
pub mod extensions;
pub mod extractors;
pub mod middleware;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
