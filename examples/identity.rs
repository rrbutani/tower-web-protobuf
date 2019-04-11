extern crate prost;
#[macro_use]
extern crate tower_web;
extern crate tower_web_protobuf;

use prost::Message;
use tower_web::ServiceBuilder;
use tower_web_protobuf::{Proto, ProtobufMiddleware};

// Messages:
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Hello {
    #[prost(string, tag = "1")]
    pub name: String,
}

#[derive(Clone, Debug)]
struct HelloWorld;

type In<M> = Proto<M>;
type Out<M> = Result<Proto<M>, ()>;

impl_web! {
    impl HelloWorld {
        // You can provide this endpoint with either a Protobuf or JSON
        // encoded body. The endpoint will respond with a Protobuf or JSON
        // encoded `Hello` message depending on the Accept header.
        #[get("/hello/")]
        fn hello(&self, hello: In<Hello>) -> Out<Hello> {
            Ok(hello)
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");

    ServiceBuilder::new()
        .resource(HelloWorld)
        // (true, true) permits JSON decoding (requests) and encoding
        // (responses)
        .middleware(ProtobufMiddleware::new(true, true))
        .run(&addr)
        .unwrap();
}
