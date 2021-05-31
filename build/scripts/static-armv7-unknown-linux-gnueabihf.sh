#!/bin/bash
set -ex

BUILD_DIR=tflite_c-static-armv7-unknown-linux-gnueabihf
mkdir -p $BUILD_DIR
cd $BUILD_DIR

TF_SRC_DIR=../tensorflow_src

ARMCC_FLAGS="-march=armv7-a -mfpu=neon-vfpv4 -funsafe-math-optimizations"
ARMCC_PREFIX=$PWD/../toolchains/gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf/bin/arm-linux-gnueabihf-
cmake -DCMAKE_C_COMPILER=${ARMCC_PREFIX}gcc \
  -DCMAKE_CXX_COMPILER=${ARMCC_PREFIX}g++ \
  -DCMAKE_C_FLAGS="${ARMCC_FLAGS}" \
  -DCMAKE_CXX_FLAGS="${ARMCC_FLAGS}" \
  -DCMAKE_VERBOSE_MAKEFILE:BOOL=ON \
  -DCMAKE_SYSTEM_NAME=Linux \
  -DCMAKE_SYSTEM_PROCESSOR=armv7 \
  -DTFLITE_C_BUILD_SHARED_LIBS=OFF \
  $TF_SRC_DIR/tensorflow/lite/c

cmake --build . -j6

mkdir static_libs
fd --glob "*.a" | xargs -I {} cp {} static_libs/

ln -s $PWD/../toolchains/gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf/arm-linux-gnueabihf/lib/libstdc++.a static_libs/
