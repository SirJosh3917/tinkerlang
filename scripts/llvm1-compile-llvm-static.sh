#!/bin/bash

echo "llvm1: compile llvm (static)"
echo ""
echo "       compiles LLVM from source, statically linked"
echo "       this is used when building in release mode"
echo ""
echo "   !!! [WARNING] -- building LLVM takes a long time"
echo "       set the CMAKE_BUILD_PARALLEL_LEVEL option before running this script"
echo ""
echo "           $ CMAKE_BUILD_PARALLEL_LEVEL=16 ./llvm1-compile-llvm-static.sh"

set -e

LLVM_ARCHIVE_OUT="llvm-project-llvmorg-11.0.1"

pushd $LLVM_ARCHIVE_OUT/llvm

# https://llvm.org/docs/Packaging.html#c-features
# we need RTTI because otherwise when we try to statically link we can't
# and the error message looks similar to this one https://issues.apache.org/jira/browse/ARROW-5148
export REQUIRES_RTTI=1

cmake -B build_static \
    `# configure for static` \
    -DLLVM_STATIC_LINK_CXX_STDLIB=ON \
    -DBUILD_SHARED_LIBS=OFF \
    `# enable libs (Z3 for fast math, RTTI for using lld` \
    -DLLVM_ENABLE_Z3_SOLVER=ON \
    -DLLVM_ENABLE_RTTI=ON \
    `# disable examples` \
    -DLLVM_INCLUDE_EXAMPLES=OFF

cmake --build build_static --config Release

strip ./build_static/bin/* || true

sudo cmake --install build_static --config Release

popd
