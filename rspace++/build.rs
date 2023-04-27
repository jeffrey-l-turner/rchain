extern crate prost_build;

// https://docs.rs/prost-build/latest/prost_build/struct.Config.html

fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.message_attribute(".", "#[derive(Hash)]");
    prost_build.message_attribute(".", "#[repr(C)]");
    prost_build
        .compile_protos(&["src/main/protobuf/rtypes.proto"], &["src/"])
        .unwrap();
}
