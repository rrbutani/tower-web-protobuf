#[macro_use]
extern crate tower_web;
extern crate http;
extern crate tokio;
extern crate tower_service;

extern crate prost;

use tower_web::ServiceBuilder;
use tower_web_protobuf::{Proto, ProtobufMiddleware};

pub mod telemetry {
    include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
}

use telemetry::Position;

#[derive(Clone, Debug)]
struct HelloWorld;

type In<M> = Proto<M>;
type Out<M> = Result<Proto<M>, ()>;

impl_web! {
    impl HelloWorld {
        #[get("/identity/pos/")]
        fn pos_ident(&self, pos: In<Position>) -> Out<Position> {
            Ok(pos)
        }

        #[get("/return_pos/")]
        fn name(&self, pos: In<Position>) -> Result<String, ()> {
            Ok(format!("{}, {}", pos.lat, pos.lon))
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
