# Pinned environment that defines the bytes in rust/.
#
# System headers (glibc, FFmpeg, SQLite, libuv, ...) are inlined into the
# translated Rust, so the base image, apt snapshot and every tool below are
# pinned. llhttp is deliberately not installed: the bundled copy is built
# and linked statically, matching toolchain.json. Changing anything here
# regenerates all of rust/; do it in its own PR.
#
#   docker build -t lightnvr-rs-transpile -f tools/rust-transpile/Dockerfile tools/rust-transpile
FROM debian:sid-slim@sha256:07a3205e8583fdff12d2b05bd030f50437e9fd340ee616df0c05dc77af75767b

ARG DEBIAN_SNAPSHOT=20261105T000000Z
ARG CLANG_MAJOR=21
ARG C2RUST_REVISION=10c776738db3f6b202dd1cade85fcdd3c1ada6f8
ARG RUST_TOOLCHAIN=nightly-2026-03-03
ARG C2RUST_BUILD_TOOLCHAIN=1.95.0
ARG NODE_VERSION=24.22.1

ENV DEBIAN_FRONTEND=noninteractive LC_ALL=C.UTF-8 \
    RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:/opt/c2rust/bin:/opt/node/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN rm -f /etc/apt/sources.list.d/* \
 && echo "deb [check-valid-until=no] http://snapshot.debian.org/archive/debian/${DEBIAN_SNAPSHOT} sid main" \
      > /etc/apt/sources.list \
 && apt-get -o Acquire::Retries=5 update \
 && apt-get install -y --no-install-recommends --allow-downgrades \
      ca-certificates curl git xz-utils python3 build-essential cmake pkg-config ninja-build \
      clang-${CLANG_MAJOR} libclang-${CLANG_MAJOR}-dev llvm-${CLANG_MAJOR}-dev libssl-dev zlib1g-dev \
      libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libswresample-dev libavfilter-dev \
      libsqlite3-dev libuv1-dev libcurl4-openssl-dev libcjson-dev \
      libmbedtls-dev libmosquitto-dev ffmpeg \
 && rm -rf /var/lib/apt/lists/* \
 && ln -s /usr/bin/clang-${CLANG_MAJOR} /usr/local/bin/clang \
 && ln -s /usr/bin/clang++-${CLANG_MAJOR} /usr/local/bin/clang++

RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none \
 && rustup toolchain install --profile minimal "${C2RUST_BUILD_TOOLCHAIN}" \
 && rustup toolchain install --profile minimal --component rustfmt "${RUST_TOOLCHAIN}" \
 && rustup default "${RUST_TOOLCHAIN}"

RUN LLVM_CONFIG_PATH=/usr/bin/llvm-config-${CLANG_MAJOR} \
    LIBCLANG_PATH=/usr/lib/llvm-${CLANG_MAJOR}/lib \
    CLANG_PATH=/usr/bin/clang-${CLANG_MAJOR} \
    cargo "+${C2RUST_BUILD_TOOLCHAIN}" install --locked --root /opt/c2rust \
      --git https://github.com/immunant/c2rust.git --rev "${C2RUST_REVISION}" c2rust \
 && rm -rf /opt/cargo/registry /opt/cargo/git \
 && c2rust --version

RUN curl -fsSLO "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz" \
 && curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/SHASUMS256.txt" | grep " node-v${NODE_VERSION}-linux-x64.tar.xz\$" | sha256sum -c - \
 && mkdir -p /opt/node && tar -xJf "node-v${NODE_VERSION}-linux-x64.tar.xz" -C /opt/node --strip-components=1 \
 && rm "node-v${NODE_VERSION}-linux-x64.tar.xz"

ENV C2RUST=/opt/c2rust/bin/c2rust LIGHTNVR_RS_TRANSPILE_IMAGE=1
RUN git config --system --add safe.directory '*'
WORKDIR /work
