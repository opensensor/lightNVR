# Stage 1: Build image
ARG DEBIAN_SUITE=sid
ARG SQLITE_YEAR=2026
ARG SQLITE_AUTOCONF_VERSION=3530400
ARG LIBUV_VERSION=1.52.1
ARG LLHTTP_VERSION=9.3.1
ARG NODE_MAJOR=24
ARG DEB_BUILD=false

FROM --platform=$BUILDPLATFORM debian:${DEBIAN_SUITE}-slim AS builder

ARG DEBIAN_SUITE
ARG SQLITE_YEAR
ARG SQLITE_AUTOCONF_VERSION
ARG LIBUV_VERSION
ARG LLHTTP_VERSION
ARG NODE_MAJOR
ARG DEB_BUILD
ARG BUILDARCH
ARG TARGETARCH
ARG TARGETVARIANT

# Set non-interactive mode
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies including Node.js and target FFmpeg dev libraries.
# Node.js comes from NodeSource so every Debian suite uses the Node 24 LTS
# baseline required by the Babel 8 web test toolchain.
# sid ships Go 1.26+/FFmpeg 8.x; trixie ships Go 1.24+/FFmpeg 7.x.
#
# ARMv7 is cross-compiled on the x86_64 runner. Compiling LiteRT/XNNPACK under
# QEMU accounted for nearly three hours of each release; Debian multiarch gives
# the native compiler the same armhf headers and libraries without emulation.
#
# Pre-install systemd-standalone-sysusers to satisfy the sysusers virtual
# dependency without pulling in the full systemd package.  The full systemd
# postinst crashes under QEMU ARM emulation (SIGSEGV in systemd 260.x),
# breaking all cross-architecture builds.
RUN if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      test "$BUILDARCH" = "amd64" || { \
        echo "linux/arm/v7 cross-compilation requires an amd64 builder"; exit 1; \
      }; \
      dpkg --add-architecture armhf; \
    elif [ "$TARGETARCH" != "$BUILDARCH" ]; then \
      echo "Unsupported cross-build: $BUILDARCH -> $TARGETARCH/$TARGETVARIANT"; \
      exit 1; \
    fi && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
      systemd-standalone-sysusers curl ca-certificates gpg && \
    curl -fsSL "https://deb.nodesource.com/setup_${NODE_MAJOR}.x" \
      -o /tmp/nodesource_setup.sh && \
    bash /tmp/nodesource_setup.sh && \
    rm /tmp/nodesource_setup.sh && \
    apt-get install -y --no-install-recommends \
      git cmake build-essential pkg-config file wget nodejs golang-go && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      GCC_MAJOR="$(gcc -dumpfullversion -dumpversion | cut -d. -f1)" && \
      apt-get install -y --no-install-recommends \
        clang:amd64 binutils-arm-linux-gnueabihf:amd64 \
        "libstdc++-${GCC_MAJOR}-dev:armhf" \
        libavcodec-dev:armhf libavformat-dev:armhf \
        libavutil-dev:armhf libswscale-dev:armhf \
        libcurl4-openssl-dev:armhf libmbedtls-dev:armhf \
        libcjson-dev:armhf libmosquitto-dev:armhf libyaml-dev:armhf && \
      # Debian sid may satisfy the GCC cross-compiler meta-package with an
      # armhf compiler binary when the native and cross package versions are
      # temporarily out of sync.  Native Clang infers its target from these
      # GNU-compatible command names and uses Debian's armhf multiarch sysroot.
      ln -sf /usr/bin/clang /usr/local/bin/arm-linux-gnueabihf-gcc && \
      ln -sf /usr/bin/clang++ /usr/local/bin/arm-linux-gnueabihf-g++ && \
      file -L /usr/local/bin/arm-linux-gnueabihf-gcc | grep -q 'x86-64' && \
      test "$(arm-linux-gnueabihf-gcc -print-target-triple)" = \
        arm-unknown-linux-gnueabihf && \
      printf '#include <iostream>\nint main() { std::cout << 42; }\n' | \
        arm-linux-gnueabihf-g++ -x c++ -o /tmp/armv7-toolchain-check - && \
      file /tmp/armv7-toolchain-check | grep -q 'ELF 32-bit.*ARM' && \
      rm /tmp/armv7-toolchain-check; \
    else \
      apt-get install -y --no-install-recommends \
        libavcodec-dev libavformat-dev libavutil-dev libswscale-dev \
        libcurl4-openssl-dev libmbedtls-dev libcjson-dev \
        libmosquitto-dev libyaml-dev; \
    fi && \
    # Verify installation
    node --version && \
    npm --version && \
    go version && \
    rm -rf /var/lib/apt/lists/*

# For .deb builds: use system dev packages instead of building from source.
# This ensures the binary links against system SONAMEs so that libuv, libsqlite3,
# and libllhttp can be proper package dependencies instead of bundled libraries.
RUN if [ "$DEB_BUILD" = "true" ]; then \
      if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
        TARGET_DEB_ARCH=armhf; \
        LIBDIR=/usr/lib/arm-linux-gnueabihf; \
      else \
        TARGET_DEB_ARCH=""; \
        case "$TARGETARCH" in \
          amd64) LIBDIR=/usr/lib/x86_64-linux-gnu ;; \
          arm64) LIBDIR=/usr/lib/aarch64-linux-gnu ;; \
          *) echo "Unsupported target architecture: $TARGETARCH/$TARGETVARIANT"; exit 1 ;; \
        esac; \
      fi && \
      apt-get update && apt-get install -y --no-install-recommends \
        "libuv1-dev${TARGET_DEB_ARCH:+:$TARGET_DEB_ARCH}" \
        "libsqlite3-dev${TARGET_DEB_ARCH:+:$TARGET_DEB_ARCH}" \
        "libllhttp-dev${TARGET_DEB_ARCH:+:$TARGET_DEB_ARCH}" \
        "sqlite3${TARGET_DEB_ARCH:+:$TARGET_DEB_ARCH}" && \
      rm -rf /var/lib/apt/lists/* && \
      cp -a ${LIBDIR}/libuv.so* /usr/lib/ && \
      cp -a ${LIBDIR}/libsqlite3.so* /usr/lib/ && \
      cp -a ${LIBDIR}/libllhttp.so* /usr/lib/; \
    fi

# Build upstream SQLite (skipped for .deb builds which use system libsqlite3)
RUN if [ "$DEB_BUILD" != "true" ]; then \
    cd /tmp && \
    wget -q "https://www.sqlite.org/${SQLITE_YEAR}/sqlite-autoconf-${SQLITE_AUTOCONF_VERSION}.tar.gz" && \
    tar -xzf "sqlite-autoconf-${SQLITE_AUTOCONF_VERSION}.tar.gz" && \
    cd "sqlite-autoconf-${SQLITE_AUTOCONF_VERSION}" && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      CONFIGURE_TARGET="--host=arm-linux-gnueabihf"; \
    else \
      CONFIGURE_TARGET=""; \
    fi && \
    ./configure $CONFIGURE_TARGET --prefix=/usr --libdir=/usr/lib --disable-static && \
    make -j"$(nproc)" && \
    make install && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      file /usr/bin/sqlite3 | grep -q "ARM"; \
    else \
      sqlite3 --version; \
    fi; \
    fi

# Build upstream libuv (skipped for .deb builds which use system libuv)
RUN if [ "$DEB_BUILD" != "true" ]; then \
    cd /tmp && \
    wget -q "https://github.com/libuv/libuv/archive/refs/tags/v${LIBUV_VERSION}.tar.gz" -O libuv.tar.gz && \
    tar -xzf libuv.tar.gz && \
    cd "libuv-${LIBUV_VERSION}" && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      UV_CROSS_ARGS="-DCMAKE_SYSTEM_NAME=Linux -DCMAKE_SYSTEM_PROCESSOR=armv7 -DCMAKE_C_COMPILER=arm-linux-gnueabihf-gcc -DCMAKE_CXX_COMPILER=arm-linux-gnueabihf-g++"; \
    else \
      UV_CROSS_ARGS=""; \
    fi && \
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTING=OFF \
      -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=lib $UV_CROSS_ARGS && \
    cmake --build build -j"$(nproc)" && \
    cmake --install build && \
    pkg-config --modversion libuv; \
    fi

# Build upstream llhttp (skipped for .deb builds which use system libllhttp)
RUN if [ "$DEB_BUILD" != "true" ]; then \
    mkdir -p /tmp/llhttp/include /tmp/llhttp/src /usr/include && \
    wget -q "https://raw.githubusercontent.com/nodejs/llhttp/release/include/llhttp.h" -O /tmp/llhttp/include/llhttp.h && \
    wget -q "https://raw.githubusercontent.com/nodejs/llhttp/release/src/llhttp.c" -O /tmp/llhttp/src/llhttp.c && \
    wget -q "https://raw.githubusercontent.com/nodejs/llhttp/release/src/api.c" -O /tmp/llhttp/src/api.c && \
    wget -q "https://raw.githubusercontent.com/nodejs/llhttp/release/src/http.c" -O /tmp/llhttp/src/http.c && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      TARGET_CC=arm-linux-gnueabihf-gcc; \
    else \
      TARGET_CC=cc; \
    fi && \
    "$TARGET_CC" -fPIC -I/tmp/llhttp/include -c /tmp/llhttp/src/llhttp.c -o /tmp/llhttp/llhttp.o && \
    "$TARGET_CC" -fPIC -I/tmp/llhttp/include -c /tmp/llhttp/src/api.c -o /tmp/llhttp/api.o && \
    "$TARGET_CC" -fPIC -I/tmp/llhttp/include -c /tmp/llhttp/src/http.c -o /tmp/llhttp/http.o && \
    "$TARGET_CC" -shared -Wl,-soname,libllhttp.so.9 -o /usr/lib/libllhttp.so.${LLHTTP_VERSION} /tmp/llhttp/llhttp.o /tmp/llhttp/api.o /tmp/llhttp/http.o && \
    ln -sf /usr/lib/libllhttp.so.${LLHTTP_VERSION} /usr/lib/libllhttp.so.9 && \
    ln -sf /usr/lib/libllhttp.so.${LLHTTP_VERSION} /usr/lib/libllhttp.so && \
    install -m 644 /tmp/llhttp/include/llhttp.h /usr/include/llhttp.h && \
    printf 'prefix=/usr\nexec_prefix=${prefix}\nlibdir=${prefix}/lib\nincludedir=${prefix}/include\n\nName: libllhttp\nDescription: llhttp parser\nVersion: %s\nLibs: -L${libdir} -lllhttp\nCflags: -I${includedir}\n' "$LLHTTP_VERSION" > /usr/lib/pkgconfig/libllhttp.pc && \
    pkg-config --modversion libllhttp; \
    fi

# Fetch external dependencies
RUN mkdir -p /opt/external && \
    # ezxml
    cd /opt/external && \
    git clone https://github.com/lxfontes/ezxml.git && \
    # inih
    cd /opt/external && \
    git clone https://github.com/benhoyt/inih.git

# LiteRT requires a build-host flatc binary while cross-compiling. Build the
# exact FlatBuffers revision selected by LiteRT, in a layer that changes only
# when its CMake dependency definitions change.
COPY third_party/litert/tflite/tools/cmake /tmp/tflite-cmake
RUN if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      cmake -S /tmp/tflite-cmake/native_tools/flatbuffers \
        -B /tmp/flatc-build \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX=/opt/host-tools && \
      cmake --build /tmp/flatc-build -j"$(nproc)" && \
      test -x /opt/host-tools/bin/flatc && \
      /opt/host-tools/bin/flatc --version; \
    fi

# Copy current directory contents into container
WORKDIR /opt
COPY . .
ARG GIT_COMMIT

# Create pkg-config files for MbedTLS with architecture-specific paths
RUN mkdir -p /usr/lib/pkgconfig && \
    case "$TARGETARCH/$TARGETVARIANT" in \
        amd64/) LIB_DIR="/usr/lib/x86_64-linux-gnu"; MBEDTLS_PACKAGE=libmbedtls-dev ;; \
        arm64/) LIB_DIR="/usr/lib/aarch64-linux-gnu"; MBEDTLS_PACKAGE=libmbedtls-dev ;; \
        arm/v7) LIB_DIR="/usr/lib/arm-linux-gnueabihf"; MBEDTLS_PACKAGE=libmbedtls-dev:armhf ;; \
        *) echo "Unsupported target architecture: $TARGETARCH/$TARGETVARIANT"; exit 1 ;; \
    esac && \
    MBEDTLS_VERSION=$(dpkg-query -W -f='${Version}' "$MBEDTLS_PACKAGE" | cut -d- -f1) && \
    echo "prefix=/usr\nexec_prefix=\${prefix}\nlibdir=$LIB_DIR\nincludedir=\${prefix}/include\n\nName: mbedtls\nDescription: MbedTLS Library\nVersion: $MBEDTLS_VERSION\nLibs: -L\${libdir} -lmbedtls\nCflags: -I\${includedir}" > /usr/lib/pkgconfig/mbedtls.pc && \
    echo "prefix=/usr\nexec_prefix=\${prefix}\nlibdir=$LIB_DIR\nincludedir=\${prefix}/include\n\nName: mbedcrypto\nDescription: MbedTLS Crypto Library\nVersion: $MBEDTLS_VERSION\nLibs: -L\${libdir} -lmbedcrypto\nCflags: -I\${includedir}" > /usr/lib/pkgconfig/mbedcrypto.pc && \
    echo "prefix=/usr\nexec_prefix=\${prefix}\nlibdir=$LIB_DIR\nincludedir=\${prefix}/include\n\nName: mbedx509\nDescription: MbedTLS X509 Library\nVersion: $MBEDTLS_VERSION\nLibs: -L\${libdir} -lmbedx509\nCflags: -I\${includedir}" > /usr/lib/pkgconfig/mbedx509.pc && \
    chmod 644 /usr/lib/pkgconfig/mbedtls.pc /usr/lib/pkgconfig/mbedcrypto.pc /usr/lib/pkgconfig/mbedx509.pc

# Build go2rtc from the opensensor/go2rtc dev submodule, including AlexxIT/dev.
# Go 1.26 is installed from Debian sid packages
RUN mkdir -p /bin /etc/lightnvr/go2rtc && \
    # Build go2rtc from local submodule (already copied by COPY . .)
    cd /opt/go2rtc && \
    GOTOOLCHAIN=auto go mod tidy && \
    case "$TARGETARCH/$TARGETVARIANT" in \
      amd64/) GOARCH=amd64; GOARM= ;; \
      arm64/) GOARCH=arm64; GOARM= ;; \
      arm/v7) GOARCH=arm; GOARM=7 ;; \
      *) echo "Unsupported target architecture: $TARGETARCH/$TARGETVARIANT"; exit 1 ;; \
    esac && \
    GOTOOLCHAIN=auto CGO_ENABLED=0 GOOS=linux GOARCH="$GOARCH" GOARM="$GOARM" \
      go build -ldflags "-s -w" -trimpath -o /bin/go2rtc . && \
    chmod +x /bin/go2rtc && \
    # Create basic configuration file
    echo "# go2rtc configuration file" > /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "api:" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "  listen: :1984" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo "  base_path: /go2rtc" >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
    echo '  origin: "*"' >> /etc/lightnvr/go2rtc/go2rtc.yaml && \
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

# Generate version.js before building web assets (it is not checked into git)
RUN LIGHTNVR_GIT_COMMIT="$GIT_COMMIT" ./scripts/extract_version.sh

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
    case "$TARGETARCH/$TARGETVARIANT" in \
        amd64/) PKG_CONFIG_ARCH_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig"; TOOLCHAIN_FILE="" ;; \
        arm64/) PKG_CONFIG_ARCH_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig"; TOOLCHAIN_FILE="" ;; \
        arm/v7) PKG_CONFIG_ARCH_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"; \
                TOOLCHAIN_FILE="/opt/cmake/toolchains/armv7-linux-gnueabihf.cmake" ;; \
        *) echo "Unsupported target architecture: $TARGETARCH/$TARGETVARIANT"; exit 1 ;; \
    esac && \
    # Cross-installing armhf packages does not run the target ldconfig, so
    # recreate the SONAME link that libmosquitto needs for the final link.
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      PICOHTTP_LIB=$(find /usr/lib/arm-linux-gnueabihf -maxdepth 1 \
        -name 'libpicohttpparser.so.1.*' -print -quit) && \
      if [ -n "$PICOHTTP_LIB" ]; then \
        ln -sf "$(basename "$PICOHTTP_LIB")" \
          /usr/lib/arm-linux-gnueabihf/libpicohttpparser.so.1; \
      fi; \
    fi && \
    # Build the application with go2rtc and SOD dynamic linking
    LIGHTNVR_GIT_COMMIT="$GIT_COMMIT" \
    CMAKE_TOOLCHAIN_FILE="$TOOLCHAIN_FILE" \
    PKG_CONFIG_PATH=/usr/lib/pkgconfig:$PKG_CONFIG_ARCH_PATH:$PKG_CONFIG_PATH \
    PKG_CONFIG_LIBDIR=/usr/lib/pkgconfig:$PKG_CONFIG_ARCH_PATH:/usr/share/pkgconfig \
    ./scripts/build.sh --release --without-tests --with-sod --sod-dynamic --with-go2rtc --go2rtc-binary=/bin/go2rtc --go2rtc-config-dir=/etc/lightnvr/go2rtc --go2rtc-api-port=1984 && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      ./scripts/install.sh --prefix=/ --with-go2rtc --go2rtc-config-dir=/etc/lightnvr/go2rtc --without-systemd --without-ldconfig; \
    else \
      ./scripts/install.sh --prefix=/ --with-go2rtc --go2rtc-config-dir=/etc/lightnvr/go2rtc --without-systemd; \
    fi

# Fail the build if a cross-compiler silently produced host binaries.
RUN file /bin/lightnvr /bin/go2rtc && \
    if [ "$TARGETARCH/$TARGETVARIANT" = "arm/v7" ]; then \
      file /bin/lightnvr | grep -q "ELF 32-bit.*ARM" && \
      file /bin/go2rtc | grep -q "ELF 32-bit.*ARM"; \
    fi

# Stage 2: Minimal runtime image
FROM debian:${DEBIAN_SUITE}-slim AS runtime

ARG DEBIAN_SUITE
ARG SQLITE_YEAR
ARG SQLITE_AUTOCONF_VERSION

ENV DEBIAN_FRONTEND=noninteractive

# Install only necessary runtime dependencies
# ffmpeg pulls the correct versioned libavcodec/libavformat/libavutil/libswscale
# for the target suite (e.g. libavcodec62 on sid, libavcodec61 on trixie)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ffmpeg \
    libcurl4t64 libmbedtls21 libmbedcrypto16 procps curl ca-certificates \
    libmosquitto1 \
    libyaml-0-2 && \
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
COPY --from=builder /usr/bin/sqlite3 /usr/bin/sqlite3
COPY --from=builder /usr/lib/libuv.so* /usr/lib/
COPY --from=builder /usr/lib/libllhttp.so* /usr/lib/

# Copy latest upstream SQLite shared library built in the builder stage
COPY --from=builder /usr/lib/libsqlite3.so* /usr/lib/

# Copy SOD libraries (use /usr/lib/ consistently; on usrmerge systems /lib → /usr/lib)
COPY --from=builder /usr/lib/libsod.so* /usr/lib/

# Copy web assets (copy CONTENTS of dist into /var/lib/lightnvr/www)
COPY --from=builder /opt/web/dist/ /var/lib/lightnvr/www/

# Copy database migrations
COPY --from=builder /opt/db/migrations/ /usr/share/lightnvr/migrations/

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Start LightNVR only. LightNVR owns the go2rtc process lifecycle so stream
# source overrides and config override changes can restart go2rtc in-place.
COPY docker-start.sh /bin/start.sh
RUN chmod +x /bin/start.sh

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
