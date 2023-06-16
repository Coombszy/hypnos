#/bin/bash

cd $(git rev-parse --show-toplevel)

# Create builder
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx rm builder || true
docker buildx create --name builder --driver docker-container --use
docker buildx use builder

# Run build
docker buildx build -f docker/Dockerfile-server --platform linux/amd64,linux/arm64 --tag registry.coombszy.com/hypnos:latest . --push
docker buildx build -f docker/Dockerfile-ipmi-bridge --platform linux/amd64,linux/arm64 --tag registry.coombszy.com/hypnos-ipmi:latest . --push
docker buildx build -f docker/Dockerfile-ssh-bridge --platform linux/amd64,linux/arm64 --tag registry.coombszy.com/hypnos-ssh:latest . --push
