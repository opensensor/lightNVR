# PRD — Host & Hardware Health Observability

**Status**: Implemented — automated acceptance complete; numeric thresholds remain provisional pending field calibration
**Created**: 2026-09-03
**Owner**: TBD
**Priority**: Operations resilience
**Scope**: Low-overhead Linux, container, filesystem, and hardware health
collection; operational alert state; Prometheus exposure; normalized MQTT events;
and loss-of-contact signaling.

---

## 1. Problem

LightNVR already reports stream health, recording gaps, recording-volume
pressure, and storage-target availability. The System page also displays a
point-in-time CPU, memory, disk, process, and network summary. These signals do
not cover several conditions that can stop recording, corrupt data, or precede
hardware failure:

- memory pressure, swap thrashing, cgroup OOM kills, and PID/FD exhaustion;
- sustained CPU or I/O contention that starves recording work;
- inode exhaustion, read-only remounts, write latency, and media I/O errors;
- SMART, NVMe, eMMC, ECC, thermal, fan, throttling, and undervoltage warnings;
- network-link degradation and an unsynchronized or jumping system clock;
- an unexpected process restart, host reboot, power loss, or total loss of
  contact.

The present UI values are observations, not a sampled health model: they have no
common freshness contract, capability state, duration, hysteresis, incident
identity, or recovery transition. Publishing every raw sample to MQTT would
also create noise and unnecessary load on small systems.

## 2. Product position

LightNVR collects facts, evaluates local operational state, and emits durable
state transitions. It does not send email, SMS, or push notifications. Existing
event routes carry health transitions to MQTT, LightNVR Cloud, Home Assistant,
Node-RED, or another system that decides how to notify a person.

```text
/proc, /sys, cgroups, statvfs, optional HW provider
                         |
                  bounded samplers
                         |
             latest snapshot + alert state
              /          |              \
     Prometheus/API   System UI     normalized events -> MQTT
                                            |
                              broker LWT/stale-heartbeat check
```

Liveness and operational health are separate:

- **Liveness** asks whether the LightNVR process can serve a request. It is safe
  for a service manager or container healthcheck to act on it.
- **Operational health** asks whether the system is at risk of degraded or lost
  recording. A warning must not cause the existing web-server self-healer,
  Docker, or Home Assistant Supervisor to restart the NVR.
- **Presence** asks whether an outside observer still hears from the NVR. Only
  the broker, a supervisor, or another machine can detect complete loss of the
  host or network.

## 3. Current baseline and gaps

| Area | Present behavior | Remaining gap |
| --- | --- | --- |
| Stream/camera | FPS, bitrate, frame age, reconnects, errors, degraded/down and recovery events | Retain; correlate but do not duplicate in this project |
| Recording | Active state, byte/segment counters, gaps and gap events | Actual write errors and resource-pressure correlation are not surfaced as host incidents |
| Recording volume | 60-second `statvfs()` pressure levels, cleanup, normalized pressure/recovery events, legacy MQTT topics | No inode, read-only, write-latency, or underlying device health; event severity is currently static even when the data says `warning` |
| Storage targets | Capacity, mount/writeability checks, optional manual write probe, unavailable/recovered events | Scheduled refresh does not run the write/fsync probe; healthy-to-degraded target transitions and slow-write precursors are not emitted |
| System API | Cgroup-aware CPU/memory totals, process RSS, root/recording disk, interfaces, uptime | Point-in-time UI data has no alert lifecycle; bare-metal CPU `usage` is a cumulative since-boot ratio and system memory uses free rather than `MemAvailable`, so neither is suitable as an alert input |
| Prometheus | Stream metrics, recording counters, recording storage bytes, LightNVR/go2rtc RSS, LightNVR process CPU | No host/container pressure, swap, filesystem inodes, thermal, network, clock, collector freshness, event-delivery, or hardware metrics |
| MQTT presence | HA availability LWT only when HA discovery is enabled | No general installation presence contract; managed MQTT destinations have no LWT/status heartbeat |
| Restart evidence | Process uptime is displayed | No host uptime, durable clean-shutdown/run marker, boot identity, or unexpected-restart event |

The existing event bus, durable outbox, route suppression, MQTT destinations,
installation UUID, storage heartbeat, and Prometheus endpoint should be extended
rather than replaced.

### 3.1 Target failure-mode coverage

This matrix defines the intended boundary. “External” means LightNVR can publish
supporting state while alive, but another process or machine must declare the
terminal loss.

| Failure mode | Useful precursor | Failure evidence | Primary observer | Phase |
| --- | --- | --- | --- | --- |
| LightNVR crash or forced kill | memory/FD/PID growth, internal watchdog state | stale heartbeat, MQTT LWT, unclosed run marker | External broker/supervisor; marker on next start | P2 |
| Host power loss, lockup, kernel panic, or watchdog reset | voltage/thermal/ECC warning when exposed | stale heartbeat; changed boot ID plus unclean marker | External observer; LightNVR after restart | P2/P3 |
| Memory exhaustion | low available headroom, memory PSI, major faults, swap churn | cgroup OOM/OOM-kill increment or allocation failure | LightNVR | P0/P1 |
| CPU starvation | sustained usage, normalized load, CPU PSI, cgroup throttling | sustained full stall with recorder/stream degradation | LightNVR | P0/P1 |
| Root or recording filesystem full | falling byte and inode headroom, growth forecast from existing storage data | `ENOSPC`/`EDQUOT` or threshold exhausted | LightNVR | P0/P1 |
| Filesystem becomes read-only/corrupt | write/fsync latency and errors, kernel warning if available | read-only flag, `EROFS`, `EIO`, failed write probe | LightNVR | P0/P1 |
| Mount/NAS disappears or stalls | carrier/errors, rising probe latency | mount guard absent, target unavailable, timed-out probe | LightNVR; external monitor for total network loss | Existing/P1 |
| HDD/SSD/NVMe failure | SMART prefail/wear, temperature, media/error counter deltas | SMART/NVMe critical health or actual I/O failure | Optional device provider | P3 |
| eMMC/flash wear-out | lifetime and pre-EOL indicators | critical pre-EOL/read-only/I/O state | Optional device provider | P3 |
| Thermal throttling/shutdown | temperature relative to kernel trips, throttling counter | critical trip, active throttling plus service loss | LightNVR while alive; external after shutdown | P0/P3 |
| Undervoltage/power instability | current/historical voltage flags where exposed | repeated undervoltage with resets or I/O errors | Optional board provider | P3 |
| Fan failure | falling RPM while temperature rises | stopped/below-min fan while above safe temperature | Optional hwmon provider | P3 |
| RAM/ECC degradation | increasing corrected ECC count | uncorrectable ECC or machine-check evidence | Optional EDAC/kernel provider | P3 |
| NIC/cable/switch degradation | carrier flaps, packet error/drop rate | selected link down or loss of broker/cameras | LightNVR plus external broker | P0/P2 |
| Bad system clock/RTC/NTP | unsynchronized kernel state, realtime/monotonic drift | backward jump or sustained unsynchronized state | LightNVR | P0/P1 |
| Process resource leak | RSS/headroom trend, FD/thread/PID utilization | effective limit reached or allocation/open failure | LightNVR | P0/P1 |
| MQTT/outbox failure hides other alerts | disconnects, retry age, queue/dead-letter growth | only route unavailable or outbox full | Local API/Prometheus or independent destination | P2 |

Filesystem repair, root-cause diagnosis of kernel panic, and guaranteed power-loss
classification remain outside LightNVR. The product reports the strongest
available evidence and clearly labels inference; it does not claim certainty
from a reboot marker alone.

## 4. Goals

- Detect the highest-value OS and hardware failure precursors without a large
  agent, time-series database, or privileged daemon in the common case.
- Distinguish `healthy`, `warning`, `error`, `critical`, and `unknown`, with a
  separate capability state for unsupported observations.
- Emit only incident open, escalation, material change, and recovery events to
  the durable event pipeline.
- Export raw current values and monotonic counters through Prometheus for users
  who want centralized rules and history.
- Work on old Linux kernels and low-memory MIPS/ARM systems through graceful
  capability detection.
- Report whether a value describes the LightNVR process, its container/cgroup,
  a mounted storage target, or the bare-metal host.
- Detect total LightNVR loss externally through a general MQTT LWT and heartbeat
  contract.
- Keep all sampling and hardware probes off recording, detection, web request,
  and event-delivery threads.

## 5. Non-goals

- A replacement for Prometheus, Grafana, Home Assistant, or a fleet monitoring
  service.
- Long-term raw metric storage in the LightNVR SQLite database.
- Native email, SMS, push, paging, or escalation policies.
- Kernel remediation, automatic reboot, fan control, SMART self-test scheduling,
  filesystem repair, or automatic drive replacement.
- Claiming host visibility from a normal container. LightNVR will not recommend
  `--privileged` merely to collect more metrics.
- Parsing arbitrary shell-command output on the sampler thread.
- Treating a high CPU value by itself as an outage.
- Folding application concerns such as database integrity, backup freshness,
  license state, or camera authentication into the OS/HW collector. They can use
  the same incident/event model in later work.

## 6. Health model

### 6.1 Scope and capability

Every collector reports both a scope and a capability state:

| Scope | Meaning |
| --- | --- |
| `process` | The LightNVR process, such as RSS or open file descriptors |
| `container` | Effective cgroup/namespace limits and usage visible to LightNVR |
| `host` | Bare-metal kernel and hardware values |
| `filesystem` | A stable logical filesystem: `root`, `recording`, or storage-target UUID |
| `device` | A stable, privacy-safe storage or hardware sensor ID |

Capability is `available`, `unsupported`, `permission_denied`, `stale`, or
`error`. An unavailable metric is never encoded as zero and never silently
treated as healthy. Collector capability and freshness are themselves exposed
to the API and Prometheus; repeated loss of a previously available critical
collector may open `health.collector_stale`.

In auto mode:

- cgroup limits and counters take precedence for memory, CPU, PIDs, and pressure;
- bare-metal `/proc` values are used when the process is unconstrained;
- mounted filesystem values describe the mount visible in the current namespace;
- hardware sensors are used only when present and readable;
- container rootfs, container networking, and host hardware are labeled as such
  and never presented as full host coverage.

### 6.2 Sampling tiers

One coordinator owns immutable latest snapshots. Collectors have independent
cadences and never perform work in an HTTP or MQTT call:

| Tier | Default | Signals |
| --- | ---: | --- |
| Fast | 10 seconds | CPU deltas, cgroup throttling/OOM counters, `MemAvailable`, swap deltas, PSI, process FD/PID use |
| Normal | 60 seconds | filesystems, network counters/carrier, thermals, clock state, boot/process markers |
| Slow | 5 minutes | bounded write/fsync probe, fan/power/ECC status |
| Device | 15 minutes | optional SMART/NVMe/eMMC health, with immediate refresh after a real recording I/O error |

Cadences are configurable within safe bounds. A slow or stuck collector cannot
delay fast sampling; overlapping executions of the same collector are skipped
and counted. Potentially blocking filesystem and external-device probes use a
separate worker and deadline. Shutdown never waits indefinitely for a probe.

The subsystem keeps only the latest sample, previous counter sample, evaluator
windows, and a small in-memory UI ring. It does not write periodic samples to
SQLite.

### 6.3 Alert lifecycle

Each condition uses a stable code, for example `memory.available_low` or
`storage.smart_prefail`, and follows this state machine:

```text
unknown -> healthy -> pending -> warning -> error -> critical
               ^                   |         |          |
               +---- recovery dwell+---------+----------+
```

- Thresholds have a `for` duration so one spike does not alert.
- Recovery has a different threshold and duration to provide hysteresis.
- Immediate faults such as an OOM kill, read-only filesystem, uncorrectable
  media error, or SMART health failure bypass the pending duration.
- Escalation emits an update for the same incident. A condition may open
  directly at any severity; it does not have to pass through every level.
- Repeated samples do not emit repeated MQTT events. Optional reminders are a
  downstream notification concern.
- An `unknown` or stale value is explicit. It only becomes an incident when a
  previously available required collector remains stale beyond its own limit.
- Active incident identity and state are persisted, not raw samples, so restart
  does not create a duplicate incident or a false recovery.
- One-shot episode conditions such as `memory.oom_kill` and
  `system.unexpected_restart` create a closed incident/event without pretending
  that a measurable recovery threshold exists.

The persisted incident record contains a UUID, condition code, logical subject,
scope, current severity/state, timestamps, last safe observation, and event
delivery references. It contains no raw path, device serial number, IP address,
or command output.

Severity has an operator-facing meaning: `warning` is a precursor without
observed data loss, `error` is a current component failure with service possibly
preserved by redundancy, and `critical` means evidence loss is occurring or
imminent, no viable fallback remains, or the whole instance is at risk. Context
may therefore escalate the same condition—for example, one failed replicated
target can be an error while the only writable target failing is critical.

### 6.4 Default policy

Defaults are a conservative starting profile, not universal hardware truth.
Operators may select `conservative`, `balanced`, or `disabled` per condition and
override thresholds. A bad override is rejected atomically. Initial proposed
balanced rules are:

| Condition | Warning | Critical | Recovery |
| --- | --- | --- | --- |
| Available memory/headroom | `<15%` for 2 min | `<8%` for 30 sec, or any OOM-kill increment | `>20%` for 5 min |
| CPU saturation | `>90%` plus CPU PSI/load evidence for 5 min | `>95%` plus sustained stall or recording degradation for 5 min | `<75%` for 5 min |
| Cgroup CPU throttling | throttled time `>10%` for 5 min | `>30%` for 5 min with service degradation | `<5%` for 5 min |
| I/O pressure | I/O PSI or probe latency above calibrated limit for 5 min | full-stall evidence or fsync `>2 sec` in 3 probes | below warning limit for 10 min |
| Filesystem bytes | Reuse configured recording watermarks; root defaults to `<15%` free | recording emergency watermark; root `<5%` free | existing low watermark / root `>20%` |
| Filesystem inodes | `<10%` available for 5 min | `<5%` for 1 min | `>15%` for 5 min |
| Filesystem write state | write probe failure or read-only state is immediate | actual `EROFS`, `EIO`, or repeated fsync failure is immediate | successful probes for 5 min |
| Temperature | within 10 C of a kernel critical trip for 5 min | at/above critical trip or active shutdown trip | 15 C below critical trip for 5 min |
| Network link | selected primary interface down for 30 sec | down for 5 min while recording is expected | carrier up and stable 2 min |
| Network errors | error/drop delta `>1%` of packets for 5 min, with a minimum sample | `>5%` for 5 min | `<0.5%` for 10 min |
| Clock sync | kernel reports unsynchronized after a 10 min startup grace | backward jump `>2 sec` or TLS/event timing is affected | synchronized and stable 5 min |
| Process FDs/PIDs | `>80%` of effective limit for 5 min | `>90%` for 1 min or allocation failure | `<70%` for 5 min |
| Device health | wear/prefail threshold, new pending sector, ECC correction surge, fan/power warning | SMART/NVMe critical status, uncorrectable ECC/media error, fan stopped while hot | provider explicitly clears a recoverable condition |

Rules based on PSI, kernel trips, hardware vendor limits, and effective cgroup
limits are preferred over arbitrary global percentages. Swap occupancy alone is
not an alert: sustained page-in/page-out rate and memory pressure are.

## 7. Collector coverage

### 7.1 P0 portable Linux and cgroup collectors

| Failure or precursor | Primary source | Required output |
| --- | --- | --- |
| CPU saturation/load | delta of `/proc/stat`, `/proc/loadavg`; effective cgroup CPU limit | usage ratio, normalized load, sample duration |
| CPU throttling | cgroup v1/v2 CPU statistics | throttled periods/time deltas |
| Memory exhaustion | `/proc/meminfo` using `MemAvailable`; cgroup current/max | total, available/headroom, cache-aware used ratio |
| OOM and allocation pressure | cgroup `memory.events`, PSI, `/proc/vmstat` where supported | high/max/OOM/OOM-kill and major-fault deltas |
| Swap thrashing | `/proc/meminfo`, `/proc/vmstat` | swap total/free and page-in/page-out byte rates |
| Process exhaustion | `getrlimit()`, `/proc/self/fd`, `/proc/self/status`, cgroup PIDs | open FDs, thread/PID count, effective limits |
| Capacity/inodes | `statvfs()` for root, recording filesystem, and each target | total/available bytes and inodes, stable logical ID |
| Read-only/missing mount | mount metadata, filesystem flags, existing target mount guard | mounted/read-only/writeable state |
| Write path | small create/write/fsync/unlink probe and actual recorder errno counters | latency and normalized failure reason |
| I/O contention | PSI where available; bounded write-probe latency | stall ratios and probe duration |
| Temperature | `/sys/class/thermal`, readable `hwmon` channels and trip points | temperature, limit, sensor capability |
| Link quality | `/sys/class/net` counters and carrier in current namespace | carrier, errors, drops, packet/byte deltas |
| Clock | `adjtimex()`, realtime-vs-monotonic delta | synchronized state and detected jumps |
| Restart/reboot | `/proc/sys/kernel/random/boot_id` plus durable run/clean marker | process restart versus host reboot and whether previous exit was clean |

All virtual filesystem paths are injectable in unit tests. Parsers are bounded,
reject partial/overflow values, tolerate missing fields on old kernels, and use
monotonic time for durations and rates.

### 7.2 P1 storage-device and hardware providers

Hardware support is capability-based because device type, kernel exposure,
permissions, and container access vary widely:

- ATA/SAS SMART overall health, temperature, reallocated/pending/uncorrectable
  sectors, interface CRC errors, and wear attributes;
- NVMe critical-warning bits, available spare, percentage used, media/data
  integrity errors, unsafe shutdown count, and temperature;
- eMMC/UFS life-time and pre-EOL indicators when exported by sysfs;
- EDAC corrected/uncorrected memory errors;
- thermal throttling, undervoltage/power warnings, and fan tachometer/minimum
  where a kernel or board provider exposes them.

One physical device is probed once even when several storage targets share it.
A configured target uses its target UUID as the public identity. Other devices
use an installation-scoped hash of provider identity material so the label is
stable locally without exposing a serial number or WWN.

The core first uses stable sysfs/ioctl sources. An optional `smartctl -j`
provider may be compiled/packaged for bare-metal and explicitly device-enabled
deployments. It is executed directly with a fixed argv, sanitized environment,
output cap, concurrency of one, and hard deadline—never through a shell. Missing
tools or permissions produce `unsupported`/`permission_denied`, not an alert
storm. Device serial numbers and raw paths are not exported.

Normal Docker and Home Assistant add-on installations will usually have
container/cgroup and mounted-volume coverage but not host SMART, power, EDAC, or
fan coverage. Full host hardware monitoring requires a separately authorized
host integration in a later phase; it is preferable to a privileged NVR
container.

### 7.3 Kernel log signals

Kernel messages can reveal filesystem remounts, block I/O errors, machine
checks, thermal shutdowns, and the system OOM killer, but `/dev/kmsg` access is
not portable and is commonly prohibited in containers. Kernel-log ingestion is
an optional provider, never a P0 dependency. Where it is unavailable, LightNVR
uses direct syscall/cgroup counters and actual recorder error instrumentation.

## 8. API, metrics, and UI

### 8.1 API boundaries

- Keep `GET /api/health` as a lightweight liveness/application endpoint. Do not
  return a failing HTTP status merely because an operational warning is active.
- Add an admin-authorized `GET /api/system/health` containing overall
  operational state, visibility scope, collector capabilities/freshness,
  bounded current observations, and active incidents.
- Add an admin-authorized incident history endpoint with bounded pagination and
  retention. This stores transitions only, not raw samples.
- Fold the System page's CPU/memory/filesystem display onto the shared snapshot
  so the UI and alert evaluator cannot disagree.
- Return `null` plus capability metadata for unavailable observations; never
  substitute zero.

The overall state is the maximum active incident severity. It is `unknown` when
required baseline collection has not completed. No opaque health score is used.

### 8.2 Prometheus

Extend the existing authenticated endpoint with stable, bounded-cardinality
families. Proposed names include:

- `lightnvr_system_cpu_usage_ratio`
- `lightnvr_system_load_ratio{period="1m|5m|15m"}`
- `lightnvr_system_pressure_stall_ratio{resource="cpu|memory|io",kind="some|full",window="10s|60s|300s"}`
- `lightnvr_system_cpu_throttled_seconds_total`
- `lightnvr_system_memory_bytes{state="total|available|used"}`
- `lightnvr_system_swap_bytes{state="total|free"}`
- `lightnvr_system_vm_events_total{event="major_fault|swap_in|swap_out|oom|oom_kill"}`
- `lightnvr_process_open_fds` and `lightnvr_process_max_fds`
- `lightnvr_filesystem_bytes{filesystem="root|recording|<target-uuid>",state="total|available"}`
- `lightnvr_filesystem_inodes{filesystem="...",state="total|available"}`
- `lightnvr_filesystem_read_only{filesystem="..."}`
- `lightnvr_filesystem_probe_duration_seconds{filesystem="..."}`
- `lightnvr_thermal_celsius{sensor="<stable-id>"}`
- `lightnvr_network_packets_total{interface="<bounded-name>",direction="rx|tx"}`
- `lightnvr_network_errors_total{interface="...",direction="rx|tx",kind="error|drop"}`
- `lightnvr_clock_synchronized`
- `lightnvr_health_collector_up{collector="...",scope="process|container|host|filesystem|device"}`
- `lightnvr_health_incidents{severity="warning|error|critical"}`

Prometheus counters never go backward within a process run; counter resets are
identifiable through process-start and boot metrics. Labels exclude paths,
addresses, serial numbers, model-specific SMART attribute names, and unbounded
error text. Cardinality is hard-capped using existing target limits plus
deterministic caps for network interfaces, thermal sensors, and physical
devices; overflow is counted and shown as incomplete coverage. Existing process
metrics remain for compatibility and are corrected or deprecated only with
release notes.

### 8.3 System UI

Add a Health section to the System page:

- overall operational state and active-incident count;
- explicit coverage banner such as “container + mounted volumes; host hardware
  unavailable”;
- resource, storage, thermal/hardware, network/clock, and component groups;
- observation, threshold, duration, freshness, and plain-language next action;
- incident history with open/escalated/recovered transitions;
- threshold profile/configuration for administrators;
- no chart longer than the bounded in-memory ring unless an external metrics
  service is configured.

## 9. Event and MQTT contract

### 9.1 Normalized incident events

Add two low-rate normalized types:

- `io.lightnvr.system.health_alert.v1`
- `io.lightnvr.system.health_recovered.v1`

Subjects use `system/host`, `system/container`, `system/process`, or
`system/storage`. The event contract therefore needs new system subject kinds;
system events must work with route scope `all`, while camera selectors do not
match them.

Example alert data:

```json
{
  "incident_id": "44444444-4444-4444-8444-444444444444",
  "code": "memory.available_low",
  "scope": "container",
  "resource": "memory",
  "state": "open",
  "severity": "warning",
  "observed": { "value": 0.12, "unit": "ratio" },
  "threshold": { "operator": "lt", "value": 0.15, "for_ms": 120000 },
  "first_observed_at": "2026-09-03T14:20:00Z"
}
```

Recovery includes the same incident ID and condition code, the prior severity,
incident duration, and safe observation. Payloads contain normalized values and
logical identifiers only. Condition codes come from a versioned registry rather
than collector-provided strings, keeping schemas, route predicates, and metric
labels bounded.

The current event registry assigns one static severity to each event type. Host
incidents need warning-to-critical escalation without multiplying schemas. Add
a backward-compatible producer-selected severity constrained by the registry's
allowed range; existing producers continue using their registry default. Add
route predicates for health condition code and severity. If dynamic envelope
severity is rejected during implementation review, use separate warning and
critical event types instead—the alert must not say `warning` in data while the
envelope says `critical`, as the current storage pressure type can.

### 9.2 Presence topic

Presence is not a durable recurring event. The default MQTT connection publishes
a retained status document to:

```text
{topic_prefix}/v1/status/{installation_uuid}
```

Managed destination profiles gain an optional explicit status-topic template;
when configured, their persistent connection publishes the same contract. A
status topic is not inferred from an arbitrary event topic template.

- Configure an `offline` Last Will independently of Home Assistant discovery.
- Publish `online` at connection and every 60 seconds with installation UUID,
  process-run ID, boot ID, visibility scope, version, and sequence/timestamp.
- Publish `stopping` during a clean shutdown when possible.
- Consumers declare the installation stale after more than two heartbeat
  intervals even if a retained `online` message survived a broker restart.
- Keep the existing Home Assistant `availability` topic as a compatibility
  adapter.

The durable outbox cannot report the death of the process that owns it. LWT and
stale-heartbeat evaluation must therefore happen at the broker/subscriber. A
simultaneous host and broker outage still requires an independent fleet or
network monitor; the UI must state this limitation.

### 9.3 Self-observability

The same metrics/API expose event-bus drops, outbox pending/dead rows and age,
MQTT destination connection state/reconnects, and collector failures. Alerting
on the only MQTT destination's outage through that destination is circular, so
it appears locally and in Prometheus and may be routed only to an independent
destination.

## 10. Configuration

Add a `[health]` configuration domain persisted through the same settings
boundary as other administrative configuration:

```ini
[health]
enabled = true
profile = balanced
fast_interval_seconds = 10
normal_interval_seconds = 60
slow_interval_seconds = 300
device_interval_seconds = 900
write_probe_enabled = true
hardware_provider = auto
presence_interval_seconds = 60
incident_retention_days = 90
```

Per-condition overrides belong in validated structured settings rather than an
ever-growing flat INI list. Defaults are visible through the API. Secret or
filesystem-path fields are not accepted. A configuration reload builds and
validates a complete policy before atomically replacing the active one.

## 11. Delivery plan

| Phase | Scope | Dependencies |
| --- | --- | --- |
| P0 — model and portable sampling | Shared snapshot/capability model; injected `/proc`/`sysfs`/cgroup parsers; CPU, memory, swap, PSI, limits, filesystems/inodes/read-only, network, clock, thermal, boot/process markers; corrected System page inputs | None |
| P1 — operational alerts | Incident state machine and persistence, balanced defaults, `/api/system/health`, Prometheus families, root/recording/target filesystem integration, actual recorder errno counters | P0; existing storage manager |
| P2 — events and presence | System subject kinds, alert/recovery producers, dynamic severity or split-type decision, route predicates, general LWT/status heartbeat, self-observability metrics | P1; Fleet 03 event pipeline |
| P3 — hardware providers | SMART/NVMe/eMMC/EDAC/fan/power providers, device mapping to logical storage targets, bounded optional helper execution | P0/P1; packaging per platform |
| P4 — operator experience | System Health UI, incident history, profiles/overrides, capability guidance, Home Assistant discovery entities where useful | P1/P2 |
| P5 — host integration | Optional least-privilege host service/protocol for container and HA deployments that require real host hardware visibility | Concrete deployment demand |

P0 and P1 should land together for the first user-visible release; raw metrics
without a coherent state model would repeat the current gap. P3 is additive and
must not delay portable memory, filesystem, thermal, and presence coverage.

Suggested implementation boundaries:

- `include/telemetry/system_health.h`, `src/telemetry/system_health.c`: snapshot,
  capability, sampling coordinator, public read API;
- collector-specific translation units under `src/telemetry/collectors/`;
- `system_health_policy` and `system_health_incidents`: evaluator and transition
  persistence, independent of MQTT;
- additions to `event_producers`, `event_envelope`, and `event_router` only at
  the normalized transition boundary;
- API and Prometheus handlers read a completed snapshot and never sample live.

## 12. Acceptance criteria

- Fixture-driven tests cover cgroup v1/v2 and bare-metal inputs, old-kernel
  missing files, malformed/overflow values, counter reset/wrap, permission
  denial, clock jumps, and disappearing sensors.
- Memory alerts use cgroup headroom when constrained and `MemAvailable` on the
  host; page cache alone cannot trigger low-memory state.
- A 30-second CPU spike emits no alert; a replayed sustained pressure fixture
  opens one incident, escalates it once, and emits one recovery only after the
  recovery dwell.
- An OOM-kill counter increment, read-only filesystem, actual `EIO`, and SMART
  critical status create immediate critical incidents.
- Byte capacity, inode capacity, mounted/read-only state, and write-probe latency
  are independently visible for root, recording storage, and each configured
  target without publishing a raw path.
- A collector changing from available to permission-denied is shown as unknown;
  it is never reported as a zero-valued healthy sample.
- MQTT receives one alert event per open/escalation and one recovery per
  incident; a broker outage does not block sampling or recording, and queued
  transition events retain their incident ID.
- Killing LightNVR without a clean disconnect causes the broker to publish its
  retained offline LWT. Suspending heartbeat publication causes an external
  test subscriber to mark it stale within two configured intervals.
- A Docker fixture reports container/cgroup scope and mounted-volume coverage,
  not bare-metal host coverage. No privileged mode is required.
- HTTP polling and multiple Prometheus scrapes do not alter CPU-rate baselines or
  alert state.
- On the smallest supported 256 MB target, portable sampling uses bounded memory
  (target: no more than 512 KiB incremental steady-state heap), no unbounded
  labels/history, and less than 0.5% average of one CPU outside active probes.
- Sampling, a timed-out hardware helper, a missing network filesystem, and an
  unavailable MQTT broker cannot block or restart recording threads.

## 13. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| False alerts across diverse hardware | Prefer kernel/vendor limits; use dwell, hysteresis, profiles, capability state, and field calibration |
| Container values mistaken for host values | Scope every collector and show a prominent coverage summary |
| Health work harms recording | Bounded data, tiered cadence, isolated probe workers, no live sampling in APIs, concurrency one for device helpers |
| Event storm during flapping | Persistent incident identity, transition-only events, recovery dwell, existing route suppression |
| NVR cannot report its own death | Broker LWT plus stale heartbeat; document need for independent monitoring when broker shares the host |
| SMART access expands privileges | Optional least-privilege provider; never require a privileged NVR container |
| Hardware output leaks identity | Logical IDs and normalized codes only; omit serials, paths, IPs, and raw logs |
| Database/disk failure prevents durable incident storage | Keep current state in memory, log locally, attempt direct presence/status update; accept that an independent observer is the final safety net |

## 14. Decisions and follow-ups

The implementation adopted these decisions:

1. Enable portable sampling by default with the `balanced` profile; unsupported
   collectors remain quiet and explicit.
2. Keep raw periodic values in Prometheus/API only. MQTT carries transitions and
   one retained presence heartbeat, not a metric firehose.
3. Extend event severity and route health predicates rather than creating a new
   event type for every threshold and severity.
4. Treat full host hardware visibility from containers as a later least-privilege
   integration, not a reason to grant the NVR broad host access.
5. Add actual recorder/write errno instrumentation in P1 because it is stronger
   evidence than a synthetic probe and closes the path from hardware symptoms
   to possible evidence loss.

Before freezing default thresholds, replay them against at least one 256 MB
embedded device, Raspberry Pi/ARM SBC, x86 host, cgroup-limited Docker instance,
rotating disk, SSD/NVMe target, and unavailable/slow network mount. The replay
must record false-positive rate and time-to-detection; defaults remain
provisional until that matrix passes.

### 14.1 Implementation evidence

Portable and cgroup collectors, immutable snapshots, durable incident episodes,
dynamic-severity events, route predicates, the administrative API, Prometheus,
the System/Settings UI, retained MQTT presence, bounded Linux hardware
collection, and the opt-in smartctl provider are implemented. API and scrape
reads remain detached from collection, and liveness remains independent of
operational severity.

Automated release evidence includes parser/unit coverage, 10 deterministic
failure/recovery replays, process-vs-host restart classification, a real
SQLite-full persistence/backoff/reconciliation test, concurrent sampler and
recording-sentinel stress, a live cgroup-limited Docker probe, the 360-sample
512 KiB/0.5% resource budget, a real Mosquitto LWT/staleness lifecycle, and 15
API/MQTT/UI integration cases. MQTT-on and MQTT-off builds are release gates;
optional hardware support degrades to explicit capability states.

The physical calibration matrix remains open. No 256 MB embedded, ARM SBC,
rotating-disk, SSD/NVMe, or slow/unavailable network-mount measurements were
available during this implementation, so no field claims or threshold changes
were inferred from synthetic tests. P5 least-privilege host integration also
remains demand-driven follow-up work.
