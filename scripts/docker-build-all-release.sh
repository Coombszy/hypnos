#!/bin/bash

cd $(git rev-parse --show-toplevel)
# Clean up
rm -rf ./target

# Ensure armv7 support is updated
rustup target add armv7-unknown-linux-gnueabihf
rustup target add aarch64-apple-ios-sim

# Create builder
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx rm builder || true
docker buildx create --name builder --driver docker-container --use
docker buildx use builder

# Run builds
docker buildx build --push \
-f docker/Dockerfile-server --platform linux/amd64,linux/arm64 --tag coombszy/hypnos-server:release --tag coombszy/hypnos-server:latest \
--tag coombszy/hypnos-server:1.0.0 .

docker buildx build --push \
-f docker/Dockerfile-ipmi-bridge --platform linux/amd64,linux/arm64 --tag coombszy/hypnos-ipmi-bridge:release --tag coombszy/hypnos-ipmi-bridge:latest \
--tag coombszy/hypnos-ipmi-bridge:1.0.0 .

docker buildx build --push \
-f docker/Dockerfile-ssh-bridge --platform linux/amd64,linux/arm64 --tag coombszy/hypnos-ssh-bridge:release --tag coombszy/hypnos-ssh-bridge:latest \
--tag coombszy/hypnos-ssh-bridge:1.0.0 .