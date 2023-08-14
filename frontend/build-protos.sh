#!/bin/bash

# Path to your .proto files
PROTO_DIR="../common/protobuf"

# Output directory for generated TypeScript files
OUT_DIR="./src/generated"

# Compile .proto files to TypeScript
protoc \
    --proto_path=${PROTO_DIR} \
    --plugin="protoc-gen-ts=$(npm bin)/protoc-gen-ts" \
    --js_out="import_style=commonjs,binary:${OUT_DIR}" \
    --ts_out="${OUT_DIR}" \
    ${PROTO_DIR}/*.proto
