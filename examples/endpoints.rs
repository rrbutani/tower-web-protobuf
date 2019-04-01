#[macro_use] extern crate tower_web;

extern crate prost;
#[macro_use] extern crate prost_derive;


pub mod telemetry {
    include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
}

pub mod interop {
    include!(concat!(env!("OUT_DIR"), "/interop.rs"));
}
