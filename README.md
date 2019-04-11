# Tower Web Protobuf

Protobuf middleware and friends for [tower-web](https://github.com/carllerche/tower-web).

[![Build Status](https://travis-ci.com/rrbutani/tower-web-protobuf.svg?branch=master)](https://travis-ci.com/rrbutani/tower-web-protobuf)
[![Docs](https://img.shields.io/badge/docs-v0.1.0-blue.svg)](https://rrbutani.github.io/tower-web-protobuf/tower-web-protobuf/)
[![Coverage Status](https://coveralls.io/repos/github/rrbutani/tower-web-protobuf/badge.svg?branch=master)](https://coveralls.io/github/rrbutani/tower-web-protobuf?branch=master)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## [A quick example](examples/identity.rs)
```rust
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
```
## Usage

## How does it work?

<TODO: replace docs with docs.rs link, add license from crates.io, add crates.io link)