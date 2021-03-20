#!/bin/bash

shopt -s extglob
set -e

LLVM_ARCHIVE_OUT="llvm-project-llvmorg-11.0.1"

pushd $LLVM_ARCHIVE_OUT
rm -rf -- !(llvm)

pushd llvm
rm -rf -- !("build"|"include")

pushd build/bin
strip ./*
popd
