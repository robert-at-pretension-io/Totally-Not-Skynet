fn main() {
    prost_build::compile_protos(&["system_types.proto"], &["../common/protobuf"]).unwrap();
}
