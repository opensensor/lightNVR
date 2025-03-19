FROM debian:bookworm-slim

# Set non-interactive mode to avoid prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && apt-get install -y \
    git cmake build-essential pkg-config \
    libavcodec-dev libavformat-dev libavutil-dev libswscale-dev \
    sqlite3 libsqlite3-dev nano systemctl && \
    rm -rf /var/lib/apt/lists/*

# Copy current directory contents into container
WORKDIR /opt
COPY . .

RUN ./scripts/build.sh --release --with-sod && \
    ./scripts/install.sh

# Expose required ports
EXPOSE 8080

# Volume for configuration and recordings persistence
VOLUME /etc/lightnvr
VOLUME /var/lib/lightnvr/recordings

# Command to start the service
CMD ["/bin/bash", "-c", "/usr/local/bin/lightnvr -c /etc/lightnvr/lightnvr.ini"]
