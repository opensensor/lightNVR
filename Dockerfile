# Stage 1: Build image
FROM debian:trixie-slim AS builder

# Set non-interactive mode
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies including Node.js for web assets
RUN apt-get update && apt-get install -y \
    git cmake build-essential pkg-config file \
    libavcodec-dev libavformat-dev libavutil-dev libswscale-dev \
    libcurl4-openssl-dev sqlite3 libsqlite3-dev \
    libmbedtls-dev curl wget ca-certificates gnupg libcjson-dev \
    libmosquitto-dev && \
    # Try to install Node.js from NodeSource (for amd64/arm64)
    # For armv7/armhf, NodeSource may not have packages, so we fall back to Debian's nodejs
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_20.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list && \
    apt-get update && \
    apt-get install -y nodejs && \
    # Install npm separately only if not bundled with nodejs (Debian armhf case)
    # NodeSource bundles npm, but Debian's package doesn't
    (npm --version 2>/dev/null || apt-get install -y npm) && \
    # Verify installation
    node --version && \
    npm --version && \
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

# Create pkg-config files for MbedTLS with architecture-specific paths
RUN mkdir -p /usr/lib/pkgconfig && \
    ARCH=$(uname -m) && \
    case $ARCH in \
        x86_64) LIB_DIR="/usr/lib/x86_64-linux-gnu" ;; \
        aarch64) LIB_DIR="/usr/lib/aarch64-linux-gnu" ;; \
        armv7l) LIB_DIR="/usr/lib/arm-linux-gnueabihf" ;; \
        *) echo "Unsupported architecture: $ARCH"; exit 1 ;; \
    esac && \
    echo "prefix=/usr\nexec_prefix=\${prefix}\nlibdir=$LIB_DIR\nincludedir=\${prefix}/include\n\nName: mbedtls\nDescription: MbedTLS Library\nVersion: 2.28.0\nLibs: -L\${libdir} -lmbedtls\nCflags: -I\${includedir}" > /usr/lib/pkgconfig/mbedtls.pc && \
    echo "prefix=/usr\nexec_prefix=\${prefix}\nlibdir=$LIB_DIR\nincludedir=\${prefix}/include\n\nName: mbedcrypto\nDescription: MbedTLS Crypto Library\nVersion: 2.28.0\nLibs: -L\${libdir} -lmbedcrypto\nCflags: -I\${includedir}" > /usr/lib/pkgconfig/mbedcrypto.pc && \
    echo "prefix=/usr\nexec_prefix=\${prefix}\nlibdir=$LIB_DIR\nincludedir=\${prefix}/include\n\nName: mbedx509\nDescription: MbedTLS X509 Library\nVersion: 2.28.0\nLibs: -L\${libdir} -lmbedx509\nCflags: -I\${includedir}" > /usr/lib/pkgconfig/mbedx509.pc && \
    chmod 644 /usr/lib/pkgconfig/mbedtls.pc /usr/lib/pkgconfig/mbedcrypto.pc /usr/lib/pkgconfig/mbedx509.pc

# Build go2rtc from opensensor fork (includes memory leak fixes)
# Install Go for building go2rtc
# Map dpkg architecture to Go architecture (armhf -> armv6l)
RUN DPKG_ARCH=$(dpkg --print-architecture) && \
    case "$DPKG_ARCH" in \
        amd64) GO_ARCH="amd64" ;; \
        arm64) GO_ARCH="arm64" ;; \
        armhf) GO_ARCH="armv6l" ;; \
        *) echo "Unsupported architecture: $DPKG_ARCH" && exit 1 ;; \
    esac && \
    curl -L "https://go.dev/dl/go1.23.4.linux-${GO_ARCH}.tar.gz" | tar -C /usr/local -xzf - && \
    export PATH=$PATH:/usr/local/go/bin && \
    mkdir -p /bin /etc/lightnvr/go2rtc && \
    # Clone and build go2rtc from opensensor fork
    cd /tmp && \
    git clone --depth 1 --branch feature/native-jpeg-transcode https://github.com/opensensor/go2rtc.git && \
    cd go2rtc && \
    CGO_ENABLED=0 /usr/local/go/bin/go build -ldflags "-s -w" -trimpath -o /bin/go2rtc . && \
    cd / && rm -rf /tmp/go2rtc /usr/local/go && \
    chmod +x /bin/go2rtc && \
    # Create basic configuration file
    echo "# go2rtc configuration file" > /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "api:" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "  listen: :1984" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "webrtc:" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "  ice_servers:" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "    - urls: [stun:stun.l.google.com:19302]" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "log:" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "  level: info" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "streams:" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "  # Streams will be added dynamically by LightNVR" >> /etc/lightnvr/go2rtc/go2rtc.yaml

# Make a slight modification to the install script to skip systemd
RUN if grep -q "systemctl" scripts/install.sh; then \
        sed -i 's/systemctl/#systemctl/g' scripts/install.sh; \
    fi

# Build web assets using Vite
RUN echo "Building web assets..." && \
    # Verify Node.js and npm are available
    node --version && \
    npm --version && \
    cd /opt/web && \
    # Install npm dependencies (use --ignore-scripts to skip chromedriver install which doesn't support ARM)
    npm ci --ignore-scripts && \
    # Build web assets
    npm run build && \
    # Verify build output exists
    ls -la dist/ && \
    echo "Web assets built successfully"

# Clean any existing build files and build the application with go2rtc support
RUN mkdir -p /etc/lightnvr /var/lib/lightnvr/data /var/log/lightnvr /var/run/lightnvr && \
    chmod -R 777 /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr && \
    # Clean any existing build files
    rm -rf build/ && \
    # Determine architecture-specific pkgconfig path
    ARCH=$(uname -m) && \
    case $ARCH in \
        x86_64) PKG_CONFIG_ARCH_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig" ;; \
        aarch64) PKG_CONFIG_ARCH_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig" ;; \
        armv7l) PKG_CONFIG_ARCH_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig" ;; \
        *) echo "Unsupported architecture: $ARCH"; exit 1 ;; \
    esac && \
    # Build the application with go2rtc and SOD dynamic linking
    PKG_CONFIG_PATH=/usr/lib/pkgconfig:$PKG_CONFIG_ARCH_PATH:$PKG_CONFIG_PATH \
    ./scripts/build.sh --release --with-sod --sod-dynamic --with-go2rtc --go2rtc-binary=/bin/go2rtc --go2rtc-config-dir=/etc/lightnvr/go2rtc --go2rtc-api-port=1984 && \
    ./scripts/install.sh --prefix=/ --with-go2rtc --go2rtc-config-dir=/etc/lightnvr/go2rtc --without-systemd

# Stage 2: Minimal runtime image
FROM debian:trixie-slim AS runtime

ENV DEBIAN_FRONTEND=noninteractive

# Install only necessary runtime dependencies
# Trixie has FFmpeg 7.x: libavcodec61, libavformat61, libavutil59, libswscale8
RUN apt-get update && apt-get install -y \
    libavcodec61 libavformat61 libavutil59 libswscale8 \
    libcurl4t64 libmbedtls21 libmbedcrypto16 sqlite3 procps curl \
    libmosquitto1 && \
    rm -rf /var/lib/apt/lists/*

# Create directory structure
RUN mkdir -p \
    /usr/share/lightnvr/models \
    /etc/lightnvr \
    /etc/lightnvr/go2rtc \
    /var/lib/lightnvr \
    /var/lib/lightnvr/www \
    /var/log/lightnvr \
    /var/run/lightnvr && \
    chmod -R 755 /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr

# Copy binaries from builder
COPY --from=builder /bin/lightnvr /bin/lightnvr
COPY --from=builder /bin/go2rtc /bin/go2rtc

# Copy SOD libraries
COPY --from=builder /lib/libsod.so.1.1.9 /lib/libsod.so.1.1.9
COPY --from=builder /lib/libsod.so.1 /lib/libsod.so.1
COPY --from=builder /lib/libsod.so /lib/libsod.so

# Copy web assets (copy CONTENTS of dist into /var/lib/lightnvr/www)
COPY --from=builder /opt/web/dist/ /var/lib/lightnvr/www/

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Create a startup script to launch both services
RUN echo '#!/bin/bash' > /bin/start.sh && \
    echo '# Start go2rtc in the background' >> /bin/start.sh && \
    echo '/bin/go2rtc --config /etc/lightnvr/go2rtc/go2rtc.yaml &' >> /bin/start.sh && \
    echo 'GO2RTC_PID=$!' >> /bin/start.sh && \
    echo '' >> /bin/start.sh && \
    echo '# Wait a moment for go2rtc to start' >> /bin/start.sh && \
    echo 'sleep 2' >> /bin/start.sh && \
    echo '' >> /bin/start.sh && \
    echo '# Start lightnvr in the foreground' >> /bin/start.sh && \
    echo 'exec /bin/lightnvr -c /etc/lightnvr/lightnvr.ini' >> /bin/start.sh && \
    echo '' >> /bin/start.sh && \
    echo '# If lightnvr exits, kill go2rtc' >> /bin/start.sh && \
    echo 'kill $GO2RTC_PID 2>/dev/null || true' >> /bin/start.sh && \
    chmod +x /bin/start.sh

# Define volumes for persistent data only
# Note: Do NOT mount /var/lib/lightnvr directly as it will overwrite web assets
VOLUME ["/etc/lightnvr", "/var/lib/lightnvr/data"]

# Expose ports
EXPOSE 8080 8554 8555 8555/udp 1984

# Environment variables for configuration
ENV GO2RTC_CONFIG_PERSIST=true \
    LIGHTNVR_AUTO_INIT=true \
    LIGHTNVR_WEB_ROOT=/var/lib/lightnvr/www

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/ || exit 1

# Use entrypoint script for initialization
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

# Command to start the services
CMD ["/bin/start.sh"]
