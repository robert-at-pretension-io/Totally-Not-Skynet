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

# Path to protoc-gen-js
PROTOC_GEN_JS="./node_modules/.bin/protoc-gen-js"

# Path to grpc_tools_node_protoc_plugin
# GRPC_TOOLS_NODE_PROTOC_PLUGIN="./node_modules/.bin/grpc_tools_node_protoc_plugin"

# Compile .proto files to TypeScript
protoc \
    --proto_path=${ABSOLUTE_SPEC_DIR} \
    --plugin="protoc-gen-ts=${PROTOC_GEN_TS}" \
    --js_out="import_style=commonjs,binary:${OUT_DIR}" \
    --ts_out="${OUT_DIR}" \
    ${ABSOLUTE_SPEC_DIR}/*.proto
