#!/bin/bash
set -ex
export TFLITE_RS_LINK=static
export TFLITE_RS_LIB_DIR=$PWD/build/tflite_c-static-x86_64-unknown-linux-gnu/static_libs
cargo build -v

export TFLITE_RS_LINK=dylib
export TFLITE_RS_LIB_DIR=$PWD/build/tflite_c-dylib-x86_64-unknown-linux-gnu
cargo build -v
