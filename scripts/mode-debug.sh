#!/bin/sh

# this script should be ran from the root dir, i.e. `./scripts/mode-debug.sh`

set -e

cp ./scripts/llvm-sys.rs/build-dbg.rs ./scripts/llvm-sys.rs/build.rs
