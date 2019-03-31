#![feature(trait_alias)]
#![feature(optin_builtin_traits)]

#[macro_use] extern crate tower_web;
#[macro_use] extern crate log;
#[macro_use(try_ready)] extern crate futures;
extern crate tokio;
extern crate tower_service;
extern crate pretty_env_logger;
extern crate http;

extern crate prost;
#[macro_use] extern crate prost_derive;

pub mod telemetry {
    include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
}

pub mod interop {
    include!(concat!(env!("OUT_DIR"), "/interop.rs"));
}

// use futures::Future;
// use futures::future;
// use futures::{Async, Future, Poll};
// use futures::Future;
// use tower_web::codegen::bytes::buf::IntoBuf;
// use prost::decode_length_delimiter;
use tower_web::codegen::CallSite;
use tower_web::ServiceBuilder;

// use tower_service::Service;
// use tower_web::middleware::Middleware;
// use futures::Poll;

mod middleware;
use middleware::logger::LogMiddleware;

use prost::Message;
use tower_web::extract::Immediate;
// use tokio::util::buf_stream::BufStream;
use tower_web::util::BufStream;
use tower_web::extract::{Extract};
use tower_web::extract::Context;
use tower_web::extract::Error;

// use std::marker::PhantomData;

mod extractors;
use extractors::Proto as OP;

mod extensions;

#[derive(Extract)]
pub struct Screamer {
    int: u32,
    juk: u64,
    lulz: String,
    wjuiw: Result<String, String>
}


pub struct Proto<M: Message + Default> {
    message: M
}

impl<M: Message + Default> Proto<M> {
    pub fn get(&self) -> &M {
        &self.message
    }
}

// use std::fmt::Debug;

// auto trait NotDebug {}
// impl<T: Debug> !NotDebug for T {}

// pub struct BufWrapper<B: BufStream> {
//     buf: B
// }

// extern crate bytes;
// use bytes::buf::Buf;

// impl<B: BufStream> IntoBuf for BufWrapper<B> {
//     type Buf = Buf;

//     fn into_buf(self) -> Self::Buf {

//     }
// }

// struct Element<M> {}

// trait Screamer<M> {
//     // type Ret: Element<Self>;

//     fn get(&self) -> Ret;
// }

// impl<M> Screamer for M
// where
//     M: Message {

//     fn get(&self) -> &Self {
//         self
//     }
// }

trait MessageWrapper<M> { }

// impl MessageWrapper for telemetry::RawMission {}

// impl<M> Screamer for M
// where
//     M: 'static + Message + MessageWrapper {

//     fn get(&self) -> &Self {
//         self
//     }
// }

// use std::fmt::Debug;

// trait Printer<K: Debug> {
//     fn pout(&self, uno: K) -> String;
// }

// impl<M, K: Debug> Printer<K> for M
// where
//     M: 'static + Message + MessageWrapper
// {
//     fn pout(&self, uno: K) -> String {
//         format!("{:?}", uno)
//     }
// }

// struct ProtoWrapper<M: Message> {}

// impl<M, B: BufStream> Extract<B> for M
// where
//     M: 'static + Message + MessageWrapper<M>
// {
//     type Future = Immediate<Proto<M>>;
// }

// impl<B: BufStream, M> Extract<B> for M
// where
//     M: 'static + Message + MessageWrapper
// {
//     type Future = Immediate<Self>;
// }
// impl<B: BufStream> Extract<B> for Message {
//     type Future = Immediate<B>;
// }

// use std::io::Read;
use futures::Async;

impl<B: BufStream, M: 'static +  Message + Default> Extract<B> for Proto<M> {
    type Future = Immediate<Proto<M>>;

    fn extract(_ctx: &Context) -> Self::Future {
        Immediate::err(Error::missing_argument())
    }


    fn extract_body(_ctx: &Context, mut body: B) -> Self::Future {

        let b;

        println!("We're extract!!");

        // let body = body.clone();

        loop {
            match body.poll() {
                Ok(Async::Ready(Some(buf))) => {
                    b = buf;
                    break;
                }
                _ => { println!("hup"); return Immediate::err(Error::missing_argument()); }
            }
        }


        match M::decode(b){
            Ok(message) => Immediate::ok(Proto{message}),
            Err(_) => Immediate::err(Error::missing_argument())
        }
    }

    fn requires_body(_callsite: &CallSite) -> bool {
        // drop(callsite);
        true
    }

}



// use std::fmt;

// pub struct SerializationMiddleware {
//     message: &'static str,
// }

// impl SerializationMiddleware {
//     fn new(message: &'static str) -> SerializationMiddleware {
//         SerializationMiddleware { message }
//     }
// }

// impl<S, RequestBody, ResponseBody> Middleware<S> for SerializationMiddleware
// where S: Service<Request = http::Request<RequestBody>,
//         Response = http::Response<ResponseBody>> {

//     type Request = http::Request<RequestBody>;
//     type Response = http::Response<ResponseBody>;

// }

// pub struct LogMiddleware {
//     target: &'static str,
// }

// impl LogMiddleware {
//     fn new(target: &'static str) -> LogMiddleware {
//         LogMiddleware { target }
//     }
// }

// impl<S> Middleware<S> for LogMiddleware
// where
//     S: Service,
//     S::Request: fmt::Debug,
//     // S::Response: Future,
//     S::Future: 'static,
// {
//     type Request = S::Request;
//     type Response = S::Response;
//     type Error = S::Error;
//     type Service = LogService<S>;

//     fn wrap(&self, service: S) -> LogService<S> {
//         LogService {
//             target: self.target,
//             service
//         }
//     }
// }

// // This service implements the Log behavior
// pub struct LogService<S> {
//     target: &'static str,
//     service: S,
// }

// impl<S> Service for LogService<S>
// where
//     S: Service,
//     S::Request: fmt::Debug,
// {
//     type Request = S::Request;
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = EncodedFuture<S::Future>;
//     // type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
//     // type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

//     fn poll_ready(&mut self) -> Poll<(), Self::Error> {
//         self.service.poll_ready()
//     }

//     fn call(&mut self, request: Self::Request) -> Self::Future {
//         // println!("target: {}, request: {:?}", self.target, request);
//         println!("on the way in");
//         // let response = self.service.call(request);

//         // Box::new(self.service.call(request)
//             // .map(|v| v)
//             // .map_err(|e| e))


//         // self.service.call(request).and_then(|response| response).into()
//         EncodedFuture(self.service.call(request))
//     }
// }

// pub struct EncodedFuture<F>(F);

// impl<F> Future for EncodedFuture<F>
// where
//     F: Future
// {
//     type Item = F::Item;
//     type Error = F::Error;

//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         // self.0.poll()

//         let response = try_ready!(self.0.poll());
//         println!("OUT!");
//         Ok(Async::Ready(response))
//     }
// }

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
struct HelloWorld;

/// This will be the JSON response
#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

impl_web! {
    impl HelloWorld {
        #[get("/name/:name")]
        fn name(&self, name: String) -> Result<String, ()> {

            let num = 7;
            // let above_five: String;
            // let how_much_above: u8;

            let above_five = String::from(if num > 5 {
                "yes"
            } else {
                "no"
            });

            // if num > 5 {
            //     let above_five = String::from("yes");
            // } else {
            //     let above_five = String::from("yes");
            // }

            println!("{:?}", above_five);

            Ok(format!("Hello, {}!", name))
        }


        #[get("/")]
        #[content_type("json")]
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                message: "hello world",
            })
        }

        #[get("/yo")]
        fn yo(&self, _message: Proto<telemetry::RawMission>) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                message: "yay!",
            })
        }
    }
}


pub fn main() {
    pretty_env_logger::init();

    // let t: Message = telemetry::Position {
    //     lat: 78f64,
    //     lon: 23f64,
    // };

    let message = telemetry::Position {
        lat: 90f64,
        lon: 78f64,
    };

    let wrapped: OP<telemetry::Position> = (&message).into();
    let _jo = wrapped.lat;
    // let ref _mu = egg.0;
    let message2: &telemetry::Position = wrapped.into();

    let wrapped2: OP<telemetry::Position> = message2.into();
    let _message3: &telemetry::Position = wrapped2.into();


    // drop(message);

    let _ji = message.lat;
    // raw.get();

    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .middleware(LogMiddleware::new("loggy"))
        .run(&addr)
        .unwrap();
}