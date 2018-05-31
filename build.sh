#!/usr/bin/sh

# Debug
#echo "Building speaker...."
#cargo build --bin speaker --features="speaker"
#echo "Building mute...."
#cargo build --bin mute --features="microphone"

# Release
echo "Building speaker...."
cargo build --bin speaker --features="speaker" --release
echo "Building mute for pi...."
cargo build --bin mute --features="microphone" --target=arm-unknown-linux-gnueabihf --release

# Deploy
# scp target/arm-unknown-linux-gnueabihf/release/mute pi@raspberrypi.local://home/pi/mute
