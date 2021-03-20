#!/bin/bash

echo "llvm3: compile llvm (shared)"
echo ""
echo "   !!! [OPTIONAL] -- only necessary for development"
echo ""
echo "       compiles LLVM from source, dynamically linked"
echo "       shared libs are built to speed up building a debug binary"
echo ""
echo "   !!! [WARNING] -- building LLVM takes a long time"
echo "       set the CMAKE_BUILD_PARALLEL_LEVEL option before running this script"
echo ""
echo "           $ CMAKE_BUILD_PARALLEL_LEVEL=16 ./llvm1-compile-llvm-static.sh"

set -e

LLVM_ARCHIVE_OUT="llvm-project-llvmorg-11.0.1"

pushd $LLVM_ARCHIVE_OUT/llvm

cmake -B build_shared \
    `# configure for dynamic` \
    -DLLVM_STATIC_LINK_CXX_STDLIB=OFF \
    -DLLVM_BUILD_LLVM_DYLIB=ON \
    -DLLVM_LINK_LLVM_DYLIB=ON \
    `# enable libs (Z3 for fast math, don't need RTTI anymore` \
    -DLLVM_ENABLE_Z3_SOLVER=ON \
    `# disable crud we don't need` \
    -DLLVM_INCLUDE_EXAMPLES=OFF \
    -DLLVM_ENABLE_BINDINGS=ON \
    `# don't build the tools, we just want the shared libraries` \
    -DLLVM_BUILD_TOOLS=OFF

cmake --build build_shared --config Release

strip ./build_shared/bin/* || true

sudo cmake --install build_shared --config Release

popd
