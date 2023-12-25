#!/bin/bash

# Path to protoc

# Path to your .proto files
PROTO_SPEC_DIR="../common/protobuf"

ABSOLUTE_SPEC_DIR=$(realpath "$PROTO_SPEC_DIR")
# ABSOLUTE_SPEC_DIR="/home/robert/Projects/totally_not_skynet/common/protobuf"

# Path to protoc executable
# PROTOC_EXECUTABLE="/usr/local/bin/protoc"

# Output directory for generated TypeScript files
OUT_DIR="./src/generated"

# Path to protoc-gen-ts
PROTOC_GEN_TS="./node_modules/.bin/protoc-gen-ts"

# Delete and recreate the output directory
rm -rf $OUT_DIR
mkdir -p $OUT_DIR


# Compile .proto files to TypeScript
protoc \
    --proto_path=${ABSOLUTE_SPEC_DIR} \
    --plugin="protoc-gen-ts=${PROTOC_GEN_TS}" \
    --ts_out="${OUT_DIR}" \
    --ts_opt="no_namespace=true" \
    --ts_opt="no_grpc=true" \
    --ts_opt="explicit_override=false" \
    ${ABSOLUTE_SPEC_DIR}/*.proto
