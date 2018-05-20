//extern crate protoc;
//extern crate protoc_rust;
extern crate protoc_rust_grpc;

//use protoc_rust::Customize;

fn main() {
    println!("Starting build");

//    protoc_rust::run(protoc_rust::Args {
//        out_dir: "src/proto",
//        input: &["proto/shorty.proto"],
//        includes: &["proto"],
//        customize: Customize {
//            ..Default::default()
//        },
//    }).expect("protoc");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["proto"],
        input: &["proto/shorty.proto"],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");
}