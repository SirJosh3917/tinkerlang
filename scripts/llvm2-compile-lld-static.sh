#!/bin/bash

echo "llvm2: compile lld (static)"
echo ""
echo "       compiles LLD from source, producing some static libraries"
echo "       this is used when building the project, as it generates static libraries"
echo "       that are referenced in debug and release mode, providing the linking"
echo "       functionality of tinkerlang"

set -e

LLVM_ARCHIVE_OUT="llvm-project-llvmorg-11.0.1"
LLVM_CONFIG_PATH="$(realpath "$LLVM_ARCHIVE_OUT/llvm/build_static/bin/llvm-config")"

pushd $LLVM_ARCHIVE_OUT/lld

cmake -B build_static -DLLVM_CONFIG_PATH="$LLVM_CONFIG_PATH"
cmake --build build_static --config Release
strip ./build_static/bin/* || true
sudo cmake --install build_static --config Release

popd

