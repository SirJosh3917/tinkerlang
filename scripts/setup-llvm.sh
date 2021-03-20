#!/bin/bash

# make sure to set
# CMAKE_BUILD_PARALLEL_LEVEL=16
# to the number of cores you have on your machine (as done so above)
#
# LLVM is a HEFTY boi to compile, so you may run out of ram in the later stages
# once the script fails from RAM overuse, turn down the number of parallel
# units so you have some free ram you can use to compile it. in my experience,
# it used up to 7.232 GB (not GiB) of ram per LLVM process, so set your parallel
# units to however much RAM you have (the intense ram usage happens at ~95% compiled)

set -e

LLVM_SRC_ARCHIVE="llvm-src.tar.xz"
LLVM_ARCHIVE_OUT="llvm-project-llvmorg-11.0.1"

if ! [[ -f $LLVM_SRC_ARCHIVE ]]
then
    curl https://codeload.github.com/llvm/llvm-project/tar.gz/refs/tags/llvmorg-11.0.1 \
        --output $LLVM_SRC_ARCHIVE
fi

if ! [[ -d $LLVM_ARCHIVE_OUT ]]
then
    tar -xzf $LLVM_SRC_ARCHIVE
fi

pushd $LLVM_ARCHIVE_OUT/llvm

if ! [[ -d build ]]
then
    cmake -B build \
        -DLLVM_STATIC_LINK_CXX_STDLIB=ON \
        -DBUILD_SHARED_LIBS=OFF \
        -DLLVM_ENABLE_Z3_SOLVER=ON
fi

cmake --build build --config Release

popd
