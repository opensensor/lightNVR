# Stage 1: Build image
FROM debian:bookworm-slim AS builder

# Set non-interactive mode
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
RUN apt-get update && apt-get install -y \
    git cmake build-essential pkg-config \
    libavcodec-dev libavformat-dev libavutil-dev libswscale-dev \
    libcurl4-openssl-dev sqlite3 libsqlite3-dev libmbedtls-dev && \
    rm -rf /var/lib/apt/lists/*

# Fetch external dependencies
RUN mkdir -p /opt/external && \
    # ezxml
    cd /opt/external && \
    git clone https://github.com/lxfontes/ezxml.git && \
    # inih
    cd /opt/external && \
    git clone https://github.com/benhoyt/inih.git

# Copy current directory contents into container
WORKDIR /opt
COPY . .

# Build the application
RUN mkdir -p /etc/lightnvr /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr /var/lib/lightnvr/recordings && \
    chmod -R 777 /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr && \
    ./scripts/build.sh --release --with-sod && \
    ./scripts/install.sh --prefix=/

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim AS runtime

ENV DEBIAN_FRONTEND=noninteractive

# Install only necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libavcodec59 libavformat59 libavutil57 libswscale6 \
    libcurl4 libmbedtls14 sqlite3 && \
    rm -rf /var/lib/apt/lists/*

# Create necessary directories in runtime
RUN mkdir -p /etc/lightnvr /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr /var/lib/lightnvr/recordings && \
    chmod -R 777 /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr

# Copy compiled binary and config files from builder stage
COPY --from=builder /bin/lightnvr /bin/lightnvr
COPY --from=builder /etc/lightnvr /etc/lightnvr
COPY --from=builder /var/lib/lightnvr /var/lib/lightnvr
COPY --from=builder /var/log/lightnvr /var/log/lightnvr
COPY --from=builder /var/run/lightnvr /var/run/lightnvr
COPY --from=builder /lib/libsod.so.1.1.9 /lib/libsod.so.1.1.9
COPY --from=builder /lib/libsod.so.1.1.9 /lib/libsod.so.1
COPY --from=builder /lib/libsod.so.1 /lib/libsod.so

# Expose required ports
EXPOSE 8080

# Volume for configuration and recordings persistence
VOLUME /etc/lightnvr
VOLUME /var/lib/lightnvr/recordings

# Command to start the service
CMD ["/bin/lightnvr", "-c", "/etc/lightnvr/lightnvr.ini"]