#!/bin/bash
set -ex

BUILD_DIR=tflite_c-static-x86_64-unknown-linux-gnu
mkdir -p $BUILD_DIR
cd $BUILD_DIR

TF_SRC_DIR=../tensorflow_src
cmake $TF_SRC_DIR/tensorflow/lite/c -DTFLITE_C_BUILD_SHARED_LIBS=OFF
cmake --build . -j6

mkdir static_libs
fd --glob "*.a" | xargs -I {} cp {} static_libs/
