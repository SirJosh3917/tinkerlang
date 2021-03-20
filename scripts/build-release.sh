#!/bin/sh

# this expects to be called in the same way mode-debug/mode-release expect to be called
# read those shell scripts for info

set -e

./scripts/mode-release.sh
cargo build --release
strip ./target/release/tinkerlang
