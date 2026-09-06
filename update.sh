#!/bin/bash
set -euxo pipefail

if ! command -v chiptool &> /dev/null; then
    echo "chiptool not found. Install with:"
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    exit 1
fi

if ! command -v form &> /dev/null; then
    echo "form not found. Install with:"
    echo "    cargo install form"
    exit 1
fi

# Shared common module (Reg wrapper, access traits)
chiptool gen-common --output src/common.rs

rm -rf src/efm32*

for chip in efm32pg1b efm32gg11; do
    chiptool generate --svd svd/$chip.svd --transform svd/$chip.yaml --common-module crate::common
    rustfmt lib.rs
    sed -i '/#!\[no_std\]/d' lib.rs
    form -i lib.rs -o src/$chip
    mv src/$chip/lib.rs src/$chip/mod.rs
    rm lib.rs
    rm -f device.x
done

cargo fmt
cargo check --features efm32pg1b
cargo check --no-default-features --features efm32gg11
