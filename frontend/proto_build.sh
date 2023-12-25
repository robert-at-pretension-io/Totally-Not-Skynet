#!/bin/bash

echo "Starting the protobuf compilation process..."

# Path to your .proto files
PROTO_SPEC_DIR="../common/protobuf"

ABSOLUTE_SPEC_DIR=$(realpath "$PROTO_SPEC_DIR")
echo "Absolute path to .proto files: $ABSOLUTE_SPEC_DIR"

# Output directory for generated TypeScript files
OUT_DIR="./src/generated"

echo "Deleting the existing output directory..."
rm -rf $OUT_DIR

echo "Creating a new output directory..."
mkdir -p $OUT_DIR

# Path to protoc-gen-ts
PROTOC_GEN_TS="./node_modules/.bin/protoc-gen-ts"

echo "Compiling .proto files to TypeScript..."
protoc \
    --proto_path=${ABSOLUTE_SPEC_DIR} \
    --plugin="protoc-gen-ts=${PROTOC_GEN_TS}" \
    --ts_out="${OUT_DIR}" \
    --ts_opt="no_namespace=true" \
    --ts_opt="no_grpc=true" \
    --ts_opt="explicit_override=false" \
    ${ABSOLUTE_SPEC_DIR}/*.proto

echo "Compilation completed. TypeScript files are generated in $OUT_DIR."
