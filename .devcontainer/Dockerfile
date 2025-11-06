FROM ubuntu:22.04

# Avoid interactive prompts during package install
ENV DEBIAN_FRONTEND=noninteractive

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libsqlite3-dev \
    nodejs \
    npm \
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
    && apt-get clean

# Create a non-root user for VS Code
RUN useradd -m vscode
USER vscode
WORKDIR /home/vscode