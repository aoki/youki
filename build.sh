#!/bin/bash

TARGET=${TARGET-x86_64-unknown-linux-gnu}
if [ "$TARGET" != "" ]; then
    rustup target add ${TARGET}
    TGT="--target $TARGET"
fi
VERSION=debug
if [[ "$1" == "--release" ]]; then
    VERSION=release
fi
cargo install cargo-when
cargo when --channel=stable build --verbose $TGT $1 && \
cargo when --channel=beta build --verbose $TGT $1 && \
cargo when --channel=nightly build --verbose --features nightly $TGT $1 && \
rm -f youki
cp target/$TARGET/$VERSION/youki .
