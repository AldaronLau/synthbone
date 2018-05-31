#!/usr/bin/sh

cargo build --bin mute --features="microphone"
cargo build --bin speaker --features="speaker"
