#!/bin/bash
DIR="$(dirname "$0")"

if cargo "$@"; then
    [ -d "$DIR/target/arm-unknown-linux-gnueabihf/debug" ] && cp -r "$DIR/shaders" "$DIR/target/arm-unknown-linux-gnueabihf/debug/shaders"
    # [ -d "$DIR/target/arm-unknown-linux-gnueabihf/release" ] && cp -r "$DIR/shaders" "$DIR/target/arm-unknown-linux-gnueabihf/release/shaders"
fi