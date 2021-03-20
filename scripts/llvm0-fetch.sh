#!/bin/bash

echo "llvm0: fetch"
echo ""
echo "       fetches LLVM and extracts it"

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
