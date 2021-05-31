#!/bin/bash
set -ex

TF_TAG="v2.5.0"
TF_VERSION="2.5.0"
TF_SRC_URL="https://github.com/tensorflow/tensorflow/archive/refs/tags/$TF_TAG.zip"
TF_SRC_DIR="tensorflow_src"

wget $TF_SRC_URL
unzip $TF_TAG.zip
mv tensorflow-$TF_VERSION tensorflow_src
rm $TF_TAG.zip
