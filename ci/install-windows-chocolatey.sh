#!/bin/bash

set -vex

choco install -y llvm
choco install -y opencv --version "$CHOCO_OPENCV_VERSION"
