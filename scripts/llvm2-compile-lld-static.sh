#!/bin/bash

echo "llvm2: compile lld (static)"
echo ""
echo "       compiles LLD from source, producing some static libraries"
echo "       this is used when building the project, as it generates static libraries"
echo "       that are referenced in debug and release mode, providing the linking"
echo "       functionality of tinkerlang"

if [ -z ${CMAKE_BUILD_PARALLEL_LEVEL+x} ]
then

echo ""
echo "   !!! [WARNING] -- CMAKE_BUILD_PARALLEL_LEVEL not set"
echo "       building LLD with no parallelism seems like a bad idea. if you intend to"
echo "       not set CMAKE_BUILD_PARALLEL_LEVEL, the build will commence in 5 seconds."
echo "       if you want parallelism, please quit the script (CTRL + C) now."

    sleep 5

fi

set -e

LLVM_ARCHIVE_OUT="llvm-project-llvmorg-11.0.1"
LLVM_CONFIG_PATH="$(realpath "$LLVM_ARCHIVE_OUT/llvm/build_static/bin/llvm-config")"

pushd $LLVM_ARCHIVE_OUT/lld

cmake -B build_static \
    `# idk why but it gets picky if we don't put the path manually` \
    -DLLVM_CONFIG_PATH="$LLVM_CONFIG_PATH" \
    `# install into place where libs can be found` \
    -DCMAKE_INSTALL_PREFIX:PATH=/usr

cmake --build build_static --config Release

strip ./build_static/bin/* || true

sudo cmake --install build_static --config Release

popd

