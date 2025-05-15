#!/bin/bash

set -xeu

choco install -y llvm
choco install -y opencv --version=4.11.0

export PATH="/C/tools/opencv/build/x64/vc16/bin:/C/tools/opencv/build/x64/vc15/bin:$PATH"
export OPENCV_LINK_PATHS="C:/tools/opencv/build/x64/vc16/lib,C:/tools/opencv/build/x64/vc15/lib"
export OPENCV_LINK_LIBS="opencv_world${OPENCV_VERSION//./}"
export OPENCV_INCLUDE_PATHS="C:/tools/opencv/build/include"

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --version
rustc --print=cfg

export RUST_BACKTRACE=full

export CXX=clang++

pnpm i
pnpm tauri build