#!/bin/bash

# so for the LIFE of me i CANNOT get LLVM_SYS_110_PREFIX to propagate down our
# dependencies, so i'm just cloning the sources manually and hacking at it to
# forcibly set it in the build.rs script... not pretty, but PLEASE submit a PR
# that fixes it. like, this doesn't work <https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts>
# it just says unused manifest key or some BS
#
# to make matters worse, i'm actually switching on which llvm-sys we build if
# we're in release versus debug mode :)))
#   ^ making a single line change takes ~1.5 MINUTES... hence, dynamic linking in debug

set -e

INKWELL_SRC="inkwell"
LLVM_SYS_SRC="llvm-sys.rs"

if ! [[ -d $INKWELL_SRC ]]
then
    git clone --depth=1 https://github.com/TheDan64/inkwell.git

    pushd $INKWELL_SRC

    sed -i 's/llvm11-0 = \["llvm-sys-110"\]/llvm11-0 = \[\]/' Cargo.toml
    sed -i 's/llvm-sys-110 =/# llvm-sys-110 =/' Cargo.toml

    cat <<'EOF' >> Cargo.toml

[dependencies.llvm-sys-110]
path = "../llvm-sys.rs"
package = "llvm-sys"
EOF

    popd
fi

if ! [[ -d $LLVM_SYS_SRC ]]
then
    git clone --depth=1 https://gitlab.com/taricorp/llvm-sys.rs.git

    pushd $LLVM_SYS_SRC

    cp build.rs build-rel.rs
    sed -i 's/env::var_os(\&\*ENV_LLVM_PREFIX)/Some("\.\.\/llvm-project-llvmorg-11\.0\.1\/llvm\/build\/")/' build-rel.rs

    # we expect the system host to have a dynamically linked version of LLVM
    curl https://gitlab.com/benjaminrsherman/llvm-sys.rs/-/raw/dynlib/build.rs \
        -o build-dbg.rs

    popd
fi
