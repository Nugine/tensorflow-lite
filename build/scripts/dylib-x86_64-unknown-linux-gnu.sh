#!/bin/bash
set -ex

BUILD_DIR=tflite_c-dylib-x86_64-unknown-linux-gnu
mkdir -p $BUILD_DIR
cd $BUILD_DIR

TF_SRC_DIR=../tensorflow_src
cmake $TF_SRC_DIR/tensorflow/lite/c -DTFLITE_C_BUILD_SHARED_LIBS=ON
cmake --build . -j6
