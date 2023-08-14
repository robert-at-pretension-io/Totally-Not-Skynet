#!/bin/bash

# Path to your .proto files
PROTO_DIR="../common/protobuf"

# Output directory for generated TypeScript files
OUT_DIR="./src/generated"

# Path to protoc-gen-ts
PROTOC_GEN_TS="./node_modules/.bin/protoc-gen-ts"

# Compile .proto files to TypeScript
protoc \
    --proto_path=${PROTO_DIR} \
    --plugin="protoc-gen-ts=${PROTOC_GEN_TS}" \
    --ts_out="${OUT_DIR}" \
    ${PROTO_DIR}/*.proto
