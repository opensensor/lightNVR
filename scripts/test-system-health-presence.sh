#!/usr/bin/env bash
# Real-broker system-health presence contract test.
#
# Exit 0: all broker-visible contracts passed.
# Exit 1: a presence/lifecycle contract failed.
# Exit 77: an optional integration-test prerequisite is unavailable.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_DIR
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
readonly REPO_ROOT
readonly PRESENCE_INTERVAL_SECONDS="${LIGHTNVR_PRESENCE_TEST_INTERVAL_SECONDS:-15}"
readonly START_TIMEOUT_SECONDS="${LIGHTNVR_PRESENCE_START_TIMEOUT_SECONDS:-25}"
readonly STOP_TIMEOUT_SECONDS="${LIGHTNVR_PRESENCE_STOP_TIMEOUT_SECONDS:-15}"
readonly MQTT_PREFIX="lightnvr-presence-test-$$"
readonly STATUS_PREFIX="$MQTT_PREFIX/v1/status/"

BROKER_PID=""
PROXY_PID=""
SUBSCRIBER_PID=""
OBSERVER_PID=""
STALE_WATCH_PID=""
NVR_PID=""
TEST_ROOT=""

skip() {
    printf 'SKIP: %s\n' "$*" >&2
    exit 77
}

fail() {
    printf 'FAIL: %s\n' "$*" >&2
    exit 1
}

pid_is_alive() {
    local pid="${1:-}"
    [[ "$pid" =~ ^[0-9]+$ ]] && kill -0 "$pid" 2>/dev/null
}

bounded_stop_pid() {
    local pid="${1:-}"
    local signal="${2:-TERM}"
    local ticks=0
    if ! pid_is_alive "$pid"; then
        return 0
    fi
    kill -s "$signal" "$pid" 2>/dev/null || true
    while pid_is_alive "$pid" && (( ticks < 30 )); do
        sleep 0.1
        ticks=$((ticks + 1))
    done
    if pid_is_alive "$pid"; then
        kill -KILL "$pid" 2>/dev/null || true
    fi
    wait "$pid" 2>/dev/null || true
}

cleanup() {
    set +e
    bounded_stop_pid "$NVR_PID" TERM
    bounded_stop_pid "$STALE_WATCH_PID" TERM
    bounded_stop_pid "$SUBSCRIBER_PID" TERM
    bounded_stop_pid "$OBSERVER_PID" TERM
    bounded_stop_pid "$PROXY_PID" TERM
    bounded_stop_pid "$BROKER_PID" TERM
    if [[ -n "$TEST_ROOT" && -d "$TEST_ROOT" ]]; then
        case "$TEST_ROOT" in
            /tmp/lightnvr-presence.*|"${TMPDIR:-/tmp}"/lightnvr-presence.*)
                if [[ "${LIGHTNVR_PRESENCE_KEEP_TEMP:-0}" == "1" ]]; then
                    printf 'Presence test artifacts retained at %s\n' "$TEST_ROOT" >&2
                else
                    rm -rf -- "$TEST_ROOT"
                fi
                ;;
        esac
    fi
}
trap cleanup EXIT
trap 'exit 130' HUP INT TERM

for tool in mosquitto mosquitto_sub mosquitto_pub jq python3 awk grep mkfifo; do
    command -v "$tool" >/dev/null 2>&1 || skip "optional tool '$tool' is not installed"
done
mosquitto_version="$(mosquitto -h 2>&1 | awk '
    tolower($0) ~ /mosquitto.*version/ {
        for (field = 1; field <= NF; field++) {
            if ($field ~ /^[0-9]+([.][0-9]+)+/) {
                print $field
                exit
            }
        }
    }
')"
mosquitto_major="${mosquitto_version%%.*}"
if [[ ! "$mosquitto_major" =~ ^[0-9]+$ ]] || (( mosquitto_major < 2 )); then
    skip "Mosquitto 2.x local-only config-free mode is required (found ${mosquitto_version:-unknown})"
fi

if [[ ! "$PRESENCE_INTERVAL_SECONDS" =~ ^[0-9]+$ ]] ||
   (( PRESENCE_INTERVAL_SECONDS < 15 || PRESENCE_INTERVAL_SECONDS > 60 )); then
    fail "LIGHTNVR_PRESENCE_TEST_INTERVAL_SECONDS must be a whole number from 15 to 60"
fi

TEST_BIN="${LIGHTNVR_TEST_BIN:-$REPO_ROOT/build/swarm-t24/bin/lightnvr}"
if [[ ! -x "$TEST_BIN" ]]; then
    skip "LightNVR test binary is unavailable at $TEST_BIN (set LIGHTNVR_TEST_BIN)"
fi

SOURCE_CONFIG="${LIGHTNVR_TEST_CONFIG:-$REPO_ROOT/config/lightnvr-health-test.ini}"
if [[ ! -r "$SOURCE_CONFIG" ]]; then
    if [[ -z "${LIGHTNVR_TEST_CONFIG:-}" && -r "$REPO_ROOT/config/lightnvr.ini" ]]; then
        SOURCE_CONFIG="$REPO_ROOT/config/lightnvr.ini"
    else
        skip "readable test config is unavailable at $SOURCE_CONFIG (set LIGHTNVR_TEST_CONFIG)"
    fi
fi

umask 077
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/lightnvr-presence.XXXXXXXX")"
readonly TEST_ROOT
readonly BROKER_CONFIG="$TEST_ROOT/mosquitto.conf"
readonly BROKER_LOG="$TEST_ROOT/mosquitto.log"
readonly PROXY_SCRIPT="$TEST_ROOT/mqtt_filter_proxy.py"
readonly PROXY_READY="$TEST_ROOT/proxy.ready"
readonly SUPPRESS_STATUS="$TEST_ROOT/suppress-status"
readonly SUPPRESSED_COUNT="$TEST_ROOT/suppressed-count"
readonly SUBSCRIBER_FIFO="$TEST_ROOT/subscriber.fifo"
readonly OBSERVED_STATUS="$TEST_ROOT/observed-status.log"
readonly RAW_STATUS="$TEST_ROOT/raw-status.log"
readonly OTHER_MESSAGES="$TEST_ROOT/other-messages.log"
readonly LAST_STATUS_AT="$TEST_ROOT/last-status-at"
readonly STALE_EVENT="$TEST_ROOT/stale-event"
readonly NVR_CONFIG="$TEST_ROOT/lightnvr.ini"
readonly NVR_LOG="$TEST_ROOT/lightnvr.log"
readonly NVR_STDIO="$TEST_ROOT/lightnvr-stdio.log"
readonly NVR_PID_FILE="$TEST_ROOT/lightnvr.pid"
readonly DATABASE_PATH="$TEST_ROOT/lightnvr.db"
readonly RECORDING_PATH="$TEST_ROOT/recordings"
readonly WEB_ROOT="$TEST_ROOT/www"

mkdir -p -- "$RECORDING_PATH" "$TEST_ROOT/hls" "$TEST_ROOT/mp4" \
    "$TEST_ROOT/models" "$WEB_ROOT"
: > "$OBSERVED_STATUS"
: > "$RAW_STATUS"
: > "$OTHER_MESSAGES"
printf '0\n' > "$SUPPRESSED_COUNT"
mkfifo "$SUBSCRIBER_FIFO"

allocate_ports() {
    python3 - <<'PY'
import socket
sockets = []
try:
    for _ in range(3):
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.bind(("127.0.0.1", 0))
        sockets.append(sock)
    for sock in sockets:
        print(sock.getsockname()[1])
finally:
    for sock in sockets:
        sock.close()
PY
}

port_attempt=0
BROKER_PORT=""
PROXY_PORT=""
WEB_PORT=""
while (( port_attempt < 10 )); do
    mapfile -t allocated_ports < <(allocate_ports)
    if [[ "${#allocated_ports[@]}" -eq 3 &&
          "${allocated_ports[0]}" != "1883" &&
          "${allocated_ports[1]}" != "1883" &&
          "${allocated_ports[2]}" != "1883" ]]; then
        BROKER_PORT="${allocated_ports[0]}"
        PROXY_PORT="${allocated_ports[1]}"
        WEB_PORT="${allocated_ports[2]}"
        break
    fi
    port_attempt=$((port_attempt + 1))
done
[[ -n "$BROKER_PORT" && -n "$PROXY_PORT" && -n "$WEB_PORT" ]] ||
    fail "could not allocate isolated localhost ports after 10 attempts"
readonly BROKER_PORT PROXY_PORT WEB_PORT

printf '%s\n' \
    "listener $BROKER_PORT 127.0.0.1" \
    'allow_anonymous true' \
    'persistence false' \
    'connection_messages true' \
    'log_type warning' \
    'log_type error' \
    "log_dest file $BROKER_LOG" > "$BROKER_CONFIG"

# Derive only bounded, non-secret sampler cadences from the supplied test
# config. Every path, listener, integration, credential, and executable-facing
# value is rebuilt below. The derived file is mode 0600 and is never printed.
awk '
    BEGIN { in_health = 0; wrote_header = 0 }
    /^\[/ {
        in_health = (tolower($0) == "[health]")
        next
    }
    in_health && /^[[:space:]]*[A-Za-z_]+[[:space:]]*=/ {
        key = tolower($0)
        sub(/[[:space:]]*=.*/, "", key)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", key)
        if (key == "fast_interval_seconds" ||
            key == "normal_interval_seconds" ||
            key == "slow_interval_seconds" ||
            key == "device_interval_seconds") {
            if (!wrote_header) { print "[health]"; wrote_header = 1 }
            print
        }
    }
    END { if (!wrote_header) print "[health]" }
' "$SOURCE_CONFIG" > "$NVR_CONFIG"

printf '%s\n' \
    '' '[general]' \
    "pid_file = $NVR_PID_FILE" \
    "log_file = $NVR_LOG" \
    'log_level = 1' \
    'syslog_enabled = false' \
    '' '[health]' \
    'enabled = true' \
    'profile = balanced' \
    'write_probe_enabled = false' \
    'hardware_provider = disabled' \
    "presence_interval_seconds = $PRESENCE_INTERVAL_SECONDS" \
    '' '[storage]' \
    "path = $RECORDING_PATH" \
    "path_hls = $TEST_ROOT/hls" \
    'max_size = 0' \
    'retention_days = 1' \
    'auto_delete_oldest = false' \
    'record_mp4_directly = false' \
    "mp4_path = $TEST_ROOT/mp4" \
    '' '[database]' \
    "path = $DATABASE_PATH" \
    'backup_interval_minutes = 0' \
    'backup_retention_count = 0' \
    'post_backup_script =' \
    '' '[web]' \
    "port = $WEB_PORT" \
    "root = $WEB_ROOT" \
    'auth_enabled = false' \
    '' '[models]' \
    "path = $TEST_ROOT/models" \
    '' '[memory]' \
    'buffer_size = 256' \
    'use_swap = false' \
    "swap_file = $TEST_ROOT/swap" \
    'swap_size = 16777216' \
    '' '[detection_engine]' \
    'enabled = false' \
    '' '[go2rtc]' \
    'enabled = false' \
    '' '[mqtt]' \
    'enabled = true' \
    'broker_host = 127.0.0.1' \
    "broker_port = $PROXY_PORT" \
    "client_id = lightnvr-presence-$$" \
    "topic_prefix = $MQTT_PREFIX" \
    'tls_enabled = false' \
    'keepalive = 5' \
    'qos = 1' \
    'retain = false' \
    'ha_discovery = false' \
    '' '[onvif]' \
    'discovery_enabled = false' >> "$NVR_CONFIG"
chmod 600 "$NVR_CONFIG"

# This bounded MQTT 3.1.1 proxy forwards every packet except status PUBLISH
# packets while SUPPRESS_STATUS exists. Dropped QoS 1 publishes receive PUBACK,
# keeping LightNVR connected so stale detection is tested independently of LWT.
printf '%s\n' \
'import os' \
'import select' \
'import socket' \
'import sys' \
'import time' \
'' \
'listen_port = int(sys.argv[1])' \
'broker_port = int(sys.argv[2])' \
'status_prefix = sys.argv[3].encode("utf-8")' \
'suppress_flag = sys.argv[4]' \
'count_file = sys.argv[5]' \
'ready_file = sys.argv[6]' \
'suppressed = 0' \
'' \
'def packet_size(buffer):' \
'    if len(buffer) < 2:' \
'        return None' \
'    multiplier = 1' \
'    remaining = 0' \
'    offset = 1' \
'    for _ in range(4):' \
'        if len(buffer) <= offset:' \
'            return None' \
'        byte = buffer[offset]' \
'        remaining += (byte & 127) * multiplier' \
'        offset += 1' \
'        if not byte & 128:' \
'            total = offset + remaining' \
'            if total > 1048576:' \
'                raise ValueError("MQTT packet exceeds test proxy bound")' \
'            return (total, offset)' \
'        multiplier *= 128' \
'    raise ValueError("invalid MQTT remaining length")' \
'' \
'def write_count():' \
'    temporary = count_file + ".tmp"' \
'    with open(temporary, "w", encoding="ascii") as output:' \
'        output.write(str(suppressed) + "\n")' \
'    os.replace(temporary, count_file)' \
'' \
'def maybe_suppress(packet, header_size, client):' \
'    global suppressed' \
'    if packet[0] >> 4 != 3 or not os.path.exists(suppress_flag):' \
'        return False' \
'    if len(packet) < header_size + 2:' \
'        return False' \
'    topic_length = int.from_bytes(packet[header_size:header_size + 2], "big")' \
'    topic_start = header_size + 2' \
'    topic_end = topic_start + topic_length' \
'    if topic_end > len(packet) or not packet[topic_start:topic_end].startswith(status_prefix):' \
'        return False' \
'    qos = (packet[0] >> 1) & 3' \
'    if qos in (1, 2) and topic_end + 2 <= len(packet):' \
'        packet_id = packet[topic_end:topic_end + 2]' \
'        client.sendall(bytes([0x40 if qos == 1 else 0x50, 0x02]) + packet_id)' \
'    suppressed += 1' \
'    write_count()' \
'    return True' \
'' \
'listener = socket.socket(socket.AF_INET, socket.SOCK_STREAM)' \
'listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)' \
'listener.bind(("127.0.0.1", listen_port))' \
'listener.listen(4)' \
'listener.settimeout(0.25)' \
'with open(ready_file, "w", encoding="ascii") as output:' \
'    output.write("ready\n")' \
'' \
'while True:' \
'    try:' \
'        client, _ = listener.accept()' \
'    except socket.timeout:' \
'        continue' \
'    upstream = socket.create_connection(("127.0.0.1", broker_port), timeout=5)' \
'    client.setblocking(False)' \
'    upstream.setblocking(False)' \
'    buffers = {client: bytearray(), upstream: bytearray()}' \
'    peers = {client: upstream, upstream: client}' \
'    active = True' \
'    while active:' \
'        readable, _, _ = select.select([client, upstream], [], [], 0.25)' \
'        for source in readable:' \
'            try:' \
'                data = source.recv(65536)' \
'            except BlockingIOError:' \
'                continue' \
'            if not data:' \
'                active = False' \
'                break' \
'            buffer = buffers[source]' \
'            buffer.extend(data)' \
'            while active:' \
'                details = packet_size(buffer)' \
'                if details is None or len(buffer) < details[0]:' \
'                    break' \
'                total, header_size = details' \
'                packet = bytes(buffer[:total])' \
'                del buffer[:total]' \
'                if source is client and maybe_suppress(packet, header_size, client):' \
'                    continue' \
'                try:' \
'                    peers[source].sendall(packet)' \
'                except (BrokenPipeError, ConnectionResetError):' \
'                    active = False' \
'                    break' \
'    client.close()' \
'    upstream.close()' > "$PROXY_SCRIPT"
chmod 700 "$PROXY_SCRIPT"

line_count() {
    local file="$1"
    wc -l < "$file" | tr -d '[:space:]'
}

wait_for_file() {
    local file="$1"
    local seconds="$2"
    local deadline=$((SECONDS + seconds))
    while (( SECONDS < deadline )); do
        [[ -s "$file" ]] && return 0
        sleep 0.1
    done
    return 1
}

wait_for_process_exit() {
    local pid="$1"
    local seconds="$2"
    local deadline=$((SECONDS + seconds))
    while (( SECONDS < deadline )); do
        pid_is_alive "$pid" || return 0
        sleep 0.1
    done
    return 1
}

state_count_after() {
    local state="$1"
    local first_line="$2"
    awk -v first="$first_line" -v needle="\"state\":\"$state\"" \
        'NR > first && index($0, needle) { count++ } END { print count + 0 }' \
        "$OBSERVED_STATUS"
}

wait_for_state_count() {
    local state="$1"
    local first_line="$2"
    local wanted="$3"
    local seconds="$4"
    local deadline=$((SECONDS + seconds))
    while (( SECONDS < deadline )); do
        if [[ "$(state_count_after "$state" "$first_line")" -ge "$wanted" ]]; then
            return 0
        fi
        if [[ -n "$NVR_PID" ]] && ! pid_is_alive "$NVR_PID"; then
            return 1
        fi
        sleep 0.1
    done
    return 1
}

last_state_payload_after() {
    local state="$1"
    local first_line="$2"
    awk -F '\t' -v first="$first_line" -v needle="\"state\":\"$state\"" \
        'NR > first && index($0, needle) { payload=$3 } END { print payload }' \
        "$OBSERVED_STATUS"
}

assert_presence_payload() {
    local payload="$1"
    local state="$2"
    printf '%s' "$payload" | jq -e --arg state "$state" '
        .schema_version == 1 and .state == $state and
        (.installation_uuid | test("^[0-9a-fA-F-]{36}$")) and
        (.run_id | test("^[0-9a-fA-F-]{36}$")) and
        (.sequence | type == "number") and
        (.timestamp_ms | type == "number") and
        (.overall_state | IN("unknown", "healthy", "warning", "error", "critical"))
    ' >/dev/null || fail "invalid retained $state presence payload"
}

retained_payload() {
    local expected_state="$1"
    local payload
    if ! payload="$(mosquitto_sub -h 127.0.0.1 -p "$BROKER_PORT" \
            -t "$MQTT_PREFIX/v1/status/#" -C 1 -W 5 2>/dev/null)"; then
        fail "retained $expected_state presence document was not available"
    fi
    assert_presence_payload "$payload" "$expected_state"
}

observer_loop() {
    local line topic payload now
    while IFS= read -r line; do
        topic="${line%%$'\t'*}"
        payload="${line#*$'\t'}"
        now="$(date +%s)"
        if [[ "$topic" == "$STATUS_PREFIX"* ]]; then
            printf '%s\t%s\t%s\n' "$now" "$topic" "$payload" >> "$RAW_STATUS"
            if [[ ! -e "$SUPPRESS_STATUS" ]]; then
                printf '%s\t%s\t%s\n' "$now" "$topic" "$payload" >> "$OBSERVED_STATUS"
                printf '%s\n' "$now" > "$LAST_STATUS_AT.tmp"
                mv -f -- "$LAST_STATUS_AT.tmp" "$LAST_STATUS_AT"
            fi
        else
            printf '%s\t%s\t%s\n' "$now" "$topic" "$payload" >> "$OTHER_MESSAGES"
        fi
    done
}

stale_watch_loop() {
    local last now
    while true; do
        if [[ -e "$SUPPRESS_STATUS" && -s "$LAST_STATUS_AT" ]]; then
            last="$(<"$LAST_STATUS_AT")"
            now="$(date +%s)"
            if [[ "$last" =~ ^[0-9]+$ ]] &&
               (( now - last >= 2 * PRESENCE_INTERVAL_SECONDS )); then
                printf 'stale_after_seconds=%s last_status_at=%s observed_at=%s\n' \
                    "$((2 * PRESENCE_INTERVAL_SECONDS))" "$last" "$now" > "$STALE_EVENT"
            fi
        fi
        sleep 0.2
    done
}

start_nvr() {
    : > "$NVR_STDIO"
    "$TEST_BIN" -c "$NVR_CONFIG" > "$NVR_STDIO" 2>&1 &
    NVR_PID=$!
}

stop_nvr_and_reap() {
    local signal="$1"
    local pid="$NVR_PID"
    kill -s "$signal" "$pid" 2>/dev/null || fail "could not send SIG$signal to LightNVR"
    wait_for_process_exit "$pid" "$STOP_TIMEOUT_SECONDS" ||
        fail "LightNVR did not exit within ${STOP_TIMEOUT_SECONDS}s after SIG$signal"
    wait "$pid" 2>/dev/null || true
    NVR_PID=""
}

broker_startup_failure() {
    local summary
    summary="$(awk 'NR <= 12 { gsub(/[[:space:]]+/, " "); printf "%s%s", separator, $0; separator=" | " }' "$BROKER_LOG" 2>/dev/null)"
    [[ -n "$summary" ]] || summary="no broker diagnostic output"
    if grep -Eqi 'permission denied|operation not permitted|address family not supported|unable to drop privileges|error while loading shared libraries' "$BROKER_LOG" 2>/dev/null; then
        skip "Mosquitto cannot run in this environment: $summary"
    fi
    fail "isolated Mosquitto broker exited during startup: $summary"
}

# Debian/Ubuntu Mosquitto packages may confine /usr/sbin/mosquitto from reading
# arbitrary mktemp config files even when the invoking user owns them. Config-
# free -p mode is local-only in Mosquitto 2.x, needs no privileged path, and
# retains the same isolated random non-1883 listener and non-persistent broker.
mosquitto -p "$BROKER_PORT" -v > "$BROKER_LOG" 2>&1 &
BROKER_PID=$!
broker_deadline=$((SECONDS + 10))
while (( SECONDS < broker_deadline )); do
    if ! pid_is_alive "$BROKER_PID"; then
        broker_startup_failure
    fi
    if mosquitto_pub -h 127.0.0.1 -p "$BROKER_PORT" \
            -t "$MQTT_PREFIX/harness-ready" -m ready >/dev/null 2>&1; then
        break
    fi
    sleep 0.1
done
mosquitto_pub -h 127.0.0.1 -p "$BROKER_PORT" \
    -t "$MQTT_PREFIX/harness-ready" -m ready >/dev/null 2>&1 ||
    fail "isolated Mosquitto broker did not listen within 10s; diagnostic: $(awk 'NR <= 12 { gsub(/[[:space:]]+/, " "); printf "%s%s", separator, $0; separator=" | " }' "$BROKER_LOG")"

python3 "$PROXY_SCRIPT" "$PROXY_PORT" "$BROKER_PORT" "$STATUS_PREFIX" \
    "$SUPPRESS_STATUS" "$SUPPRESSED_COUNT" "$PROXY_READY" \
    > "$TEST_ROOT/proxy.log" 2>&1 &
PROXY_PID=$!
wait_for_file "$PROXY_READY" 10 || fail "MQTT filter proxy did not start within 10s"
pid_is_alive "$PROXY_PID" || fail "MQTT filter proxy exited during startup"

mosquitto_sub -h 127.0.0.1 -p "$BROKER_PORT" -t "$MQTT_PREFIX/#" \
    -F '%t\t%p' > "$SUBSCRIBER_FIFO" 2> "$TEST_ROOT/subscriber.log" &
SUBSCRIBER_PID=$!
observer_loop < "$SUBSCRIBER_FIFO" &
OBSERVER_PID=$!
stale_watch_loop &
STALE_WATCH_PID=$!
sleep 0.2
pid_is_alive "$SUBSCRIBER_PID" || fail "external MQTT subscriber exited during startup"
pid_is_alive "$OBSERVER_PID" || fail "external presence observer exited during startup"

printf 'Presence broker listening on isolated 127.0.0.1:%s (proxy %s)\n' \
    "$BROKER_PORT" "$PROXY_PORT"

# 1. Initial retained online plus a later heartbeat, followed by an unclean
# process death that lets Mosquitto publish the retained LWT offline document.
marker="$(line_count "$OBSERVED_STATUS")"
start_nvr
wait_for_state_count online "$marker" 1 "$START_TIMEOUT_SECONDS" ||
    fail "LightNVR did not publish initial online presence"
initial_payload="$(last_state_payload_after online "$marker")"
assert_presence_payload "$initial_payload" online
initial_sequence="$(printf '%s' "$initial_payload" | jq -r '.sequence')"
wait_for_state_count online "$marker" 2 "$((PRESENCE_INTERVAL_SECONDS + 10))" ||
    fail "LightNVR did not publish an online heartbeat within one interval"
heartbeat_payload="$(last_state_payload_after online "$marker")"
heartbeat_sequence="$(printf '%s' "$heartbeat_payload" | jq -r '.sequence')"
(( heartbeat_sequence > initial_sequence )) || fail "heartbeat sequence did not advance"
kill_marker="$(line_count "$OBSERVED_STATUS")"
stop_nvr_and_reap KILL
wait_for_state_count offline "$kill_marker" 1 10 ||
    fail "SIGKILL did not produce the retained broker LWT offline document"
assert_presence_payload "$(last_state_payload_after offline "$kill_marker")" offline
retained_payload offline
printf 'PASS: retained online heartbeat and SIGKILL LWT offline\n'

# 2. A fresh run replaces offline with online; graceful termination publishes
# retained stopping before disconnecting.
marker="$(line_count "$OBSERVED_STATUS")"
start_nvr
wait_for_state_count online "$marker" 1 "$START_TIMEOUT_SECONDS" ||
    fail "LightNVR did not replace offline with online on restart"
stop_marker="$(line_count "$OBSERVED_STATUS")"
stop_nvr_and_reap TERM
wait_for_state_count stopping "$stop_marker" 1 10 ||
    fail "SIGTERM did not publish retained stopping presence"
assert_presence_payload "$(last_state_payload_after stopping "$stop_marker")" stopping
retained_payload stopping
printf 'PASS: SIGTERM retained stopping\n'

# 3. Keep LightNVR connected while the proxy drops only status PUBLISH packets.
# A direct control publish proves the broker/subscriber path remains live. The
# external observer must declare stale only after two configured intervals.
marker="$(line_count "$OBSERVED_STATUS")"
start_nvr
wait_for_state_count online "$marker" 1 "$START_TIMEOUT_SECONDS" ||
    fail "LightNVR did not publish online before stale-heartbeat test"
rm -f -- "$STALE_EVENT"
suppressed_before="$(<"$SUPPRESSED_COUNT")"
other_before="$(line_count "$OTHER_MESSAGES")"
touch "$SUPPRESS_STATUS"
control_value="control-$$-$(date +%s)"
mosquitto_pub -h 127.0.0.1 -p "$BROKER_PORT" \
    -t "$MQTT_PREFIX/control" -m "$control_value" ||
    fail "control publish failed during status-only suppression"

control_deadline=$((SECONDS + 5))
while (( SECONDS < control_deadline )); do
    if (( $(line_count "$OTHER_MESSAGES") > other_before )) &&
       grep -Fq -- "$control_value" "$OTHER_MESSAGES"; then
        break
    fi
    sleep 0.1
done
grep -Fq -- "$control_value" "$OTHER_MESSAGES" ||
    fail "non-status MQTT traffic did not reach the external observer"

suppress_deadline=$((SECONDS + PRESENCE_INTERVAL_SECONDS + 10))
while (( SECONDS < suppress_deadline )); do
    suppressed_now="$(<"$SUPPRESSED_COUNT")"
    (( suppressed_now > suppressed_before )) && break
    if ! pid_is_alive "$NVR_PID"; then
        fail "LightNVR exited during status-only suppression"
    fi
    sleep 0.1
done
suppressed_now="$(<"$SUPPRESSED_COUNT")"
(( suppressed_now > suppressed_before )) ||
    fail "proxy did not suppress a scheduled status heartbeat"
wait_for_file "$STALE_EVENT" "$((2 * PRESENCE_INTERVAL_SECONDS + 10))" ||
    fail "external observer did not declare stale after two presence intervals"
pid_is_alive "$NVR_PID" || fail "LightNVR was not alive when observer declared stale"
(( $(line_count "$OBSERVED_STATUS") == marker + 1 )) ||
    fail "external observer accepted a status update during suppression"
printf 'PASS: status-only suppression declared stale after %ss while control traffic remained live\n' \
    "$((2 * PRESENCE_INTERVAL_SECONDS))"

rm -f -- "$SUPPRESS_STATUS"
stop_nvr_and_reap TERM
printf 'PASS: real-broker system-health presence contract\n'
