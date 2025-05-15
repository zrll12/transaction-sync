#!/bin/bash

set -xeu

ci_dir="$(dirname "$0")"

export CHOCO_LLVM_VERSION=20.1.0

if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
	"$ci_dir/install-windows-vcpkg.sh"
else # chocolatey build
	"$ci_dir/install-windows-chocolatey.sh"
fi