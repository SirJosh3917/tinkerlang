#!/bin/bash

echo "llvm3: compile llvm (shared)"
echo ""
echo "     ! [OPTIONAL] -- only necessary for development"
echo ""
echo "       compiles LLVM from source, dynamically linked"
echo "       shared libs are built to speed up building a debug binary"

if [ -z ${CMAKE_BUILD_PARALLEL_LEVEL+x} ]
then

echo ""
echo "   !!! [WARNING] -- CMAKE_BUILD_PARALLEL_LEVEL not set"
echo "       building LLVM with no parallelism seems like a bad idea. if you intend to"
echo "       not set CMAKE_BUILD_PARALLEL_LEVEL, the build will commence in 5 seconds."
echo "       if you want parallelism, please quit the script (CTRL + C) now."
echo ""
echo "       recommended to set the CMAKE_BUILD_PARALLEL_LEVEL option"
echo ""
echo "           $ CMAKE_BUILD_PARALLEL_LEVEL=16 ./llvm3-compile-llvm-shared.sh"

    sleep 5

fi

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
