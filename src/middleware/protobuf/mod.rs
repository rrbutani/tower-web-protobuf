//! Protobuf middleware.
//!
//! Parses protobuf messages coming in and encodes protobuf messages going out.
//! Note that this piece of middleware only reads and sets headers; it should
//! be used with the protobuf Extractor and Serializer.

mod middleware;
mod service;

// pub use self::middleware::ProtobufMiddleware;
pub use self::middleware::ProtobufMiddleware;
pub use self::service::{Config, ProtobufService};
