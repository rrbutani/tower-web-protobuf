extern crate prost_build;

use std::fs;
use std::path::{Path, PathBuf};

const MESSAGE_DIR: &'static str = "examples/messages";

fn proto_files_in_dir(dir: &'static str) -> Vec<PathBuf> {
    fs::read_dir(Path::new(dir)).unwrap()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().extension().is_some())
        .filter(|f| f.path().extension().unwrap() == "proto")
        .map(|f| f.path())
        .collect()
}


fn main() {
    let mut prost_build = prost_build::Config::new();

    prost_build.compile_protos(proto_files_in_dir(MESSAGE_DIR).as_slice(),
        &[MESSAGE_DIR.into()]).unwrap();
}
