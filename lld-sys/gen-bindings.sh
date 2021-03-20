#!/bin/sh

# we can't generate bindings pragmatically because we segfault trying to generate
# bindings for the C++ stdlib. this command runs the binding generation manually
bindgen \
    headers.h -o src/bindings.rs \
    --whitelist-function '.*lld.*link.*' \
    --whitelist-function '.*outs.*' \
    --whitelist-function '.*errs.*' \
    --whitelist-function '.*nulls.*' \
    -- \
    -xc++ \
    -I"$(realpath ../scripts/llvm-project-llvmorg-11.0.1/lld/include)" \
    $(../scripts/llvm-project-llvmorg-11.0.1/llvm/build/bin/llvm-config --ldflags --cxxflags)

