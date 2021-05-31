#!/bin/bash
set -ex

TF_TAG="v2.5.0"
TF_SRC_DIR="build/tensorflow_src"

TFLITE_C_URL="https://github.com/tensorflow/tensorflow/tree/$TF_TAG/tensorflow/lite/c"
TFLITE_RS_BINDINGS="src/bindings.rs"

cat > tflite_c.h << EOF
#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/c/builtin_op_data.h"
#include "tensorflow/lite/c/c_api.h"
#include "tensorflow/lite/c/c_api_experimental.h"
EOF

bindgen tflite_c.h \
    --allowlist-type 'TfLite.*' \
    --allowlist-function 'TfLite.*' \
    --allowlist-var 'TfLite.*' \
    --default-enum-style moduleconsts \
    --size_t-is-usize \
    -o tmp1 \
    -- \
    -I$TF_SRC_DIR

rm tflite_c.h

cat > tmp2 << EOF
/* tensorflow-lite $TF_TAG C API */
/* $TFLITE_C_URL */
EOF

cat tmp2 tmp1 > $TFLITE_RS_BINDINGS
rm tmp1 tmp2
