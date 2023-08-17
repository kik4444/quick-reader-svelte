#!/usr/bin/env bash

TARGET=/build/target
BUNDLE=$TARGET/release/bundle
CARGO_CACHE=/usr/local/cargo/registry

cleanup() {
    # Stop the container
    docker rm -f builder > /dev/null
}

# Abort script on error
set -euo pipefail

# Run cleanup on error
trap cleanup ERR

# Build image
docker build -t builder -f linux-Dockerfile .

# Create a volume for the crates and compilation cache
docker volume create builder_cache > /dev/null

# Create a container with an infinite command in the background so it can accept other commands
docker run -d --rm --name builder -v builder_cache:$CARGO_CACHE -v builder_cache:$TARGET builder tail -f /dev/null > /dev/null 

# Copy source code into container
docker cp "$PWD/../." builder:/build/

# Compile the project
docker exec -it builder sh -c "npm install && npm run tauri build"

# Move the output to a different directory so that docker cp can target only them
docker exec -it builder sh -c "mkdir /build/output && mv $BUNDLE/deb/*.deb $BUNDLE/appimage/*.AppImage /build/output/"

# Extract the output out of the container
mkdir linux-output || true
docker cp builder:/build/output/. linux-output/

# All finished
cleanup
