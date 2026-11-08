# syntax=docker/dockerfile:1.7
#
# INT_01 X1: one image that holds every process the E2E suite owns (Mosquitto,
# lightNVR built from this checkout, the YA-WAMF backend from its branch, ffmpeg
# for the feeder fixture). The suite starts, stops, SIGKILLs and SIGSTOPs those
# processes and reads both SQLite databases, so they share one container.
#
# NOT EXECUTED IN X1: the committed results come from the host-process mode
# (see ../README.md). This is the documented CI/nightly path.
#
# Build contexts (see docker-compose.yml):
#   lightnvr  this repository
#   yawamf    a YA-WAMF checkout on feature/lightnvr-provider

ARG DEBIAN_SUITE=sid

FROM debian:${DEBIAN_SUITE}-slim AS lightnvr-build
# Same package set as .github/workflows/sanitizer.yml, plus MQTT on.
RUN apt-get update && apt-get install -y --no-install-recommends \
      git ca-certificates build-essential cmake pkg-config python3 \
      libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libavfilter-dev \
      libsqlite3-dev libuv1-dev libllhttp-dev libcurl4-openssl-dev \
      libcjson-dev libmbedtls-dev libmosquitto-dev \
 && rm -rf /var/lib/apt/lists/*
COPY --from=lightnvr . /src
RUN git config --global --add safe.directory /src \
 && (cd /src && git submodule update --init --depth 1 third_party/faac || true) \
 && cmake -S /src -B /build -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTS=OFF \
      -DENABLE_MQTT=ON -DENABLE_SOD=OFF -DENABLE_GO2RTC=OFF -DENABLE_LITERT=OFF -DENABLE_SSL=ON \
 && cmake --build /build -j"$(nproc)" --target lightnvr

# The build stage already carries every runtime library lightNVR links against.
FROM lightnvr-build
RUN apt-get update && apt-get install -y --no-install-recommends \
      curl sqlite3 ffmpeg mosquitto mosquitto-clients libgl1 libglib2.0-0t64 \
 && rm -rf /var/lib/apt/lists/*
COPY --from=ghcr.io/astral-sh/uv:0.11.21 /uv /usr/local/bin/uv
COPY --from=lightnvr-build /build/bin/lightnvr /opt/lightnvr/bin/lightnvr
# The suite only uses the API; an empty web root is enough.
RUN mkdir -p /opt/lightnvr/www
COPY --from=yawamf . /opt/yawamf
# YA-WAMF's own pins (python 3.12, base + CPU provider requirements), in the
# venv path the suite expects (<yawamf>/backend/.venv).
RUN uv python install 3.12 \
 && uv venv --python 3.12 /opt/yawamf/backend/.venv \
 && VIRTUAL_ENV=/opt/yawamf/backend/.venv uv pip install \
      -r /opt/yawamf/backend/requirements-base.txt -r /opt/yawamf/backend/requirements-provider-cpu.txt
COPY --from=lightnvr tests/integration/yawamf_e2e /opt/x1
WORKDIR /opt/x1
ENTRYPOINT ["/opt/yawamf/backend/.venv/bin/python", "/opt/x1/run_e2e.py", \
            "--lightnvr-bin", "/opt/lightnvr/bin/lightnvr", "--web-root", "/opt/lightnvr/www", \
            "--yawamf", "/opt/yawamf", "--results-dir", "/results"]
