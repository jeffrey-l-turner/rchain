extern crate prost_build;

fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.message_attribute(".", "#[derive(Hash)]");
    prost_build
        .compile_protos(&["src/rtypes.proto"], &["src/"])
        .unwrap();
}
