#[macro_use] extern crate tower_web;
// #[macro_use] extern crate log;
// #[macro_use(try_ready)] extern crate futures;
extern crate tokio;
extern crate tower_service;
extern crate http;

extern crate prost;

use tower_web::ServiceBuilder;
use tower_web_protobuf::{Proto, ProtobufMiddleware};

pub mod telemetry {
    include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
}

pub mod interop {
    include!(concat!(env!("OUT_DIR"), "/interop.rs"));
}


/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
struct HelloWorld;

impl_web! {
    impl HelloWorld {
        #[get("/return_pos/")]
        fn name(&self, name: Proto<telemetry::Position>) -> Result<String, ()> {
            Ok(format!("{}, {}", name.lat, name.lon))
            // Ok(String::from("jk"))
        }
    }
}


pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .middleware(ProtobufMiddleware::new(true, true))
        .run(&addr)
        .unwrap();
}