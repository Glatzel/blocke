FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge

# Install extra dependencies or tools
RUN apt-get update && apt-get install -y \
    vim \
    net-tools \
    libudev-dev:arm64 \
    && apt-get clean

# Optional: set environment variables or tweak linker flags
ENV RUST_BACKTRACE=1
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
