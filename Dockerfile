# Stage 1: Build image
FROM debian:bookworm-slim AS builder

# Set non-interactive mode
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
RUN apt-get update && apt-get install -y \
    git cmake build-essential pkg-config \
    libavcodec-dev libavformat-dev libavutil-dev libswscale-dev \
    libcurl4-openssl-dev sqlite3 libsqlite3-dev && \
    rm -rf /var/lib/apt/lists/*

# Build MbedTLS from source with tests disabled
WORKDIR /opt
RUN git clone --branch v3.4.0 --depth 1 https://github.com/Mbed-TLS/mbedtls.git && \
    cd mbedtls && mkdir build && cd build && \
    cmake -DCMAKE_INSTALL_PREFIX=/usr/local -DENABLE_TESTING=OFF -DENABLE_PROGRAMS=OFF .. && \
    make -j$(nproc) && make install && \
    ldconfig

# Create pkg-config files for MbedTLS libraries
RUN mkdir -p /usr/local/lib/pkgconfig && \
    echo "prefix=/usr/local\n\
exec_prefix=\${prefix}\n\
libdir=\${exec_prefix}/lib\n\
includedir=\${prefix}/include\n\
\n\
Name: mbedtls\n\
Description: MbedTLS Library\n\
Version: 3.4.0\n\
Libs: -L\${libdir} -lmbedtls\n\
Cflags: -I\${includedir}" > /usr/local/lib/pkgconfig/mbedtls.pc && \
    echo "prefix=/usr/local\n\
exec_prefix=\${prefix}\n\
libdir=\${exec_prefix}/lib\n\
includedir=\${prefix}/include\n\
\n\
Name: mbedcrypto\n\
Description: MbedTLS Crypto Library\n\
Version: 3.4.0\n\
Libs: -L\${libdir} -lmbedcrypto\n\
Cflags: -I\${includedir}" > /usr/local/lib/pkgconfig/mbedcrypto.pc && \
    echo "prefix=/usr/local\n\
exec_prefix=\${prefix}\n\
libdir=\${exec_prefix}/lib\n\
includedir=\${prefix}/include\n\
\n\
Name: mbedx509\n\
Description: MbedTLS X509 Library\n\
Version: 3.4.0\n\
Libs: -L\${libdir} -lmbedx509\n\
Cflags: -I\${includedir}" > /usr/local/lib/pkgconfig/mbedx509.pc

# Fetch external dependencies
RUN mkdir -p /opt/external && \
    # ezxml
    cd /opt/external && \
    git clone https://github.com/lumberjaph/ezxml.git && \
    # inih
    cd /opt/external && \
    git clone https://github.com/benhoyt/inih.git

# Copy current directory contents into container
WORKDIR /opt
COPY . .

# Prepare and build the application
RUN mkdir -p /etc/lightnvr /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr /var/lib/lightnvr/recordings && \
    chmod -R 777 /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr && \
    PKG_CONFIG_PATH=/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH ./scripts/build.sh --release --with-sod && \
    ./scripts/install.sh --prefix=/

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim AS runtime

ENV DEBIAN_FRONTEND=noninteractive

# Install only necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libavcodec59 libavformat59 libavutil57 libswscale6 \
    libcurl4 sqlite3 && \
    rm -rf /var/lib/apt/lists/*

# Create necessary directories in runtime
RUN mkdir -p /etc/lightnvr /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr /var/lib/lightnvr/recordings && \
    chmod -R 777 /var/lib/lightnvr /var/log/lightnvr /var/run/lightnvr

# Copy MbedTLS libraries from builder
COPY --from=builder /usr/local/lib/libmbed*.so* /usr/local/lib/

# Update library cache
RUN ldconfig

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