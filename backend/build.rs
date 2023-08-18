fn main() {
    println!("Change in build !!!!!!!!");

    prost_build
        ::compile_protos(&["../common/protobuf/system_types.proto"], &["../common/protobuf"])
        .unwrap();

    println!("Should compile types to: {:?}", std::env::var("OUT_DIR"));
}
