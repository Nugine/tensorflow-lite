#!/bin/bash
set -ex
curl -LO https://storage.googleapis.com/mirror.tensorflow.org/developer.arm.com/media/Files/downloads/gnu-a/8.3-2019.03/binrel/gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf.tar.xz
mkdir -p toolchains
FILE=gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf.tar.xz
tar xvf $FILE -C toolchains
rm $FILE
