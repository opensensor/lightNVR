FROM ubuntu:24.04

ARG NODE_MAJOR=24

# Avoid interactive prompts during package install
ENV DEBIAN_FRONTEND=noninteractive

# Install the Node 24 LTS baseline required by Babel 8 and the web toolchain.
RUN apt-get update && apt-get install -y curl ca-certificates gnupg \
    && curl -fsSL "https://deb.nodesource.com/setup_${NODE_MAJOR}.x" | bash - \
    && apt-get install -y nodejs

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libsqlite3-dev \
    pkg-config \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libcurl4-openssl-dev \
    libmbedtls-dev \
    libcjson-dev \
    libwolfssl-dev \
    libssl-dev \
    libuv1-dev \
    libllhttp-dev \
    file \
    systemd \
    && apt-get clean

# Create a non-root user for VS Code
RUN useradd -m vscode
USER vscode
WORKDIR /home/vscode
