#!/bin/bash

echo "TARGETPLATFORM: $1"
echo "BINARY: $2"

apt-get update && apt-get upgrade -qq
apt-get install --install-recommends -y perl openssl pkg-config libssl-dev librust-openssl-dev \
	build-essential ca-certificates

# Raspi 1/2/3 (armv7)
if [ "$1" = "linux/arm/v7" ] 
then
    rm -f target/armv7-unknown-linux-gnueabihf/release/deps/$2
    apt-get install --install-recommends -y gcc-arm-linux-gnueabihf
    rustup target add armv7-unknown-linux-gnueabihf
    cargo build --release --bin $2 --target armv7-unknown-linux-gnueabihf ; code=$?
    mkdir -p /target/release
    ls -ltra target/armv7-unknown-linux-gnueabihf/release/
    cp target/armv7-unknown-linux-gnueabihf/release/$2 target/release/
    exit $code
fi

# Raspi 4 (arm64)
if [ "$1" = "linux/arm64" ] 
then
    rm -f target/aarch64-unknown-linux-gnu/release/deps/$2
    apt-get install --install-recommends -y gcc-aarch64-linux-gnu
    rustup target add aarch64-unknown-linux-gnu
    cargo build --release --bin $2 --target aarch64-unknown-linux-gnu ; code=$?
    mkdir -p /target/release
    ls -ltra target/aarch64-unknown-linux-gnu/release/
    cp target/aarch64-unknown-linux-gnu/release/$2 target/release/
    exit $code
fi

if [ "$1" = "linux/amd64" ]
then
    # Assuming you are running this on a amd64 machine
    rm -f target/release/deps/$2
    cargo build --release --bin $2
    exit $?
fi

echo "Not supported cross-compile! ($1) Add support in cross-compile.sh and update .cargo/config"
exit 1
