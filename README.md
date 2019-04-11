# Tower Web Protobuf

Protobuf middleware and friends for [tower-web](https://github.com/carllerche/tower-web).

[![Build Status](https://travis-ci.com/rrbutani/tower-web-protobuf.svg?branch=master)](https://travis-ci.com/rrbutani/tower-web-protobuf)
[![Docs](https://img.shields.io/badge/docs-v0.1.0-blue.svg)](https://rrbutani.github.io/tower-web-protobuf/tower-web-protobuf/)
[![Coverage Status](https://coveralls.io/repos/github/rrbutani/tower-web-protobuf/badge.svg?branch=master)](https://coveralls.io/github/rrbutani/tower-web-protobuf?branch=master)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## [A Quick Example](examples/identity.rs)
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

## How does it work?
`tower-web-protobuf` provides a generic `Proto` type that can wrap any Protobuf message struct (really anything that implements prost's `Message` type - unfortunately this does mean that you're forced to use prost with this crate).

`Proto` types have a custom `Extract` and `Response` implementation that handle serializing and deserializing protobuf messages. By default these implementations will try to parse all incoming messages as protobufs and will encode all responses as protobufs, regardless of the content-type and accept headers that were sent with the corresponding request.

In order to actually abide by the headers, we need to introduce [some middleware](src/middlware/protobuf/middleware.rs). Currently, the middleware allows you to enable/disable JSON support for encoding/decoding (shown above).

To be clear, it's possible to just use the `Proto` type and to ignore the middlware entirely. If your use case only involves sending and receiving protobufs (no JSON), this may even be ideal.

There's also a plaintext serialization/deserialization option based on [serde-plain](https://github.com/mitsuhiko/serde-plain).

## Usage
First, add `tower-web-protobuf` to your Cargo.toml:
```toml
[dependencies]
tower-web-protobuf = { git = "https://github.com/rrbutani/tower-web-protobuf" }
```

In order to use the `Proto` type, your message structs must implement the traits in `MessagePlus`. This means implementing prost's `Message` and serde's `DeserializeOwned` and `Serialize`. For most use cases, this means using prost and adding `#[derive]` attributes for serde's traits.

With prost, there are two main ways to do this: add a build.rs file ([like this one](build.rs)) that uses prost_build or add `#[derive]`s on your existing structs to turn them into protobuf messages.

If you go the first route, make sure you add something like the following to derive serde's traits for your messages:
```rust
    prost_build::Config::new()
        .type_attribute(".", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"snake_case\")]")
        .type_attribute(".", "#[serde(deny_unknown_fields)]")
        .compile_protos(
            proto_files_in_dir(MESSAGE_DIR).as_slice(),
            &[MESSAGE_DIR.into()],
        )
        .unwrap();
```

[build.rs](build.rs) has the above code in context and the [endpoints example](examples/endpoints.rs) has a sample usage.

An example of the other way is shown in [example above](#A-Quick-Example) which is identical to the [identity example](examples/identity.rs). The key bit is the [`#[derive]` on the message](examples/identity.rs#L11).

Finally, wrap your message types with `Proto` (or use the `In` and `Out` type aliases shown above) and [add the `ProtobufMiddleware`](examples/identity.rs#L42) if you so desire. It _might_ just work.

Error handling isn't great and logging for serialization/deserialization errors isn't quite there yet either. As is probably obvious, if you're planning to use this for important things expect trouble.

<todo: replace docs with docs.rs link, add license from crates.io, add crates.io link)>
<todo: link all structs to the official docs.rs doc pages>