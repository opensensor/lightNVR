# System health configuration

Host health is enabled by default. The shipped values are conservative
engineering defaults, but their numeric thresholds remain **provisional** until
the documented physical hardware matrix is calibrated. Review them against the
recording workload and alert history before relying on them for paging.

## INI settings

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

| Key | Allowed values or range | Notes |
| --- | --- | --- |
| `enabled` | `true`, `false` | Master switch |
| `profile` | `balanced`, `conservative`, `disabled` | Base policy for every condition |
| `fast_interval_seconds` | 5–60 | CPU, memory, pressure, process limits |
| `normal_interval_seconds` | 15–600 | Filesystems, network, clock, restart evidence |
| `slow_interval_seconds` | 60–3600 | Write probe and slower hardware state |
| `device_interval_seconds` | 300–86400 | Optional device-provider cadence |
| `write_probe_enabled` | `true`, `false` | Bounded create/write/fsync/unlink probe |
| `hardware_provider` | `auto`, `smartctl`, `disabled` | See below |
| `presence_interval_seconds` | 15–3600 | Default-broker status heartbeat |
| `incident_retention_days` | 7–3650 | Durable closed-incident retention |

Intervals must also satisfy `fast <= normal <= slow <= device`. Invalid or
unknown health keys reject the complete configuration instead of being
silently clamped.

`balanced` uses the provisional defaults shown in the System page.
`conservative` opens earlier and shortens applicable dwell periods. `disabled`
turns conditions off while retaining an explicit configured state.

The principal balanced numeric defaults are:

| Condition | Warning | Critical | Recovery | Warning/critical/recovery dwell |
| --- | ---: | ---: | ---: | --- |
| `memory.available_low` | `< 0.15` | `< 0.08` | `> 0.20` | 120 / 30 / 300 s |
| `cpu.saturation` | `> 0.90` | `> 0.95` | `< 0.75` | 300 / 300 / 300 s |
| `cpu.throttled` | `> 0.10` | `> 0.30` | `< 0.05` | 300 / 300 / 300 s |
| `io.pressure` | `> 0.50 s` | `> 2.00 s` | `< 0.20 s` | 300 / immediate / 600 s |
| `filesystem.bytes_low` | `< 0.15` | `< 0.05` | `> 0.20` | 300 / 60 / 300 s |
| `filesystem.inodes_low` | `< 0.10` | `< 0.05` | `> 0.15` | 300 / 60 / 300 s |
| `network.error_rate` | `> 0.01` | `> 0.05` | `< 0.005` | 300 / 300 / 600 s |
| `process.fd_exhaustion` | `> 0.80` | `> 0.90` | `< 0.70` | 300 / 60 / 300 s |
| `process.pid_exhaustion` | `> 0.80` | `> 0.90` | `< 0.70` | 300 / 60 / 300 s |

Link-down uses 30/300/120-second warning/critical/recovery dwell; clock
unsynchronized uses a 600-second startup/warning dwell and 300-second recovery.
Read-only and write-failed recovery requires 300 seconds. OOM kills,
allocation failures, clock jumps, critical device/ECC evidence, and unexpected
restart are evidence-driven or one-shot rules rather than arbitrary numeric
thresholds. `GET /api/system/health` is the authoritative effective registry
after profile and per-condition overrides.

## Runtime policy changes

Administrators can change Health settings in the web UI. The settings API uses
the `health_`-prefixed scalar names and accepts a complete structured override
object in `health_condition_overrides`:

```json
{
  "health_profile": "balanced",
  "health_condition_overrides": {
    "version": 1,
    "conditions": [
      { "code": "memory.available_low", "profile": "conservative" },
      { "code": "network.error_rate", "profile": "disabled" },
      {
        "code": "filesystem.inodes_low",
        "unit": "ratio",
        "warning": 0.12,
        "critical": 0.06,
        "recovery": 0.18,
        "warning_for_seconds": 300,
        "critical_for_seconds": 60,
        "recovery_for_seconds": 300
      }
    ]
  }
}
```

Profile overrides contain only `code` and `profile`. Numeric overrides must be
complete, preserve the registry unit and threshold ordering, use whole-second
dwell values, and name each condition once. The server validates and replaces
the full immutable policy atomically; a rejected save leaves the previous
generation active. Reload after a generation conflict.

## Hardware providers

`auto` uses portable Linux `/proc` and `/sys` visibility and does not invoke an
external device command. `disabled` suppresses optional device providers.
`smartctl` explicitly enables the bounded `/usr/sbin/smartctl` JSON provider for
eligible storage devices.

For SMART collection, install smartmontools and give the LightNVR service only
the device-read capabilities required by the selected devices. Test as the
service account with `smartctl -j -n standby <device>`. Avoid running LightNVR
as root or granting a container `--privileged` solely for telemetry. Missing
binaries, sleeping devices, unsupported JSON, permission denial, and helper
timeouts are reported as capability states; raw output and serial numbers are
not retained or exposed.

## Containers and network storage

A normal container sees cgroup limits, its network namespace, and mounted
filesystems. It usually cannot see host block devices, full host networking,
EDAC, fans, voltage flags, or all thermal sensors. The API labels this boundary;
do not read container health as complete host health. Add an independent host
monitor when host-level failure detection is required.

Slow or unavailable network mounts are isolated behind bounded helpers, but a
kernel filesystem call can still be uninterruptible. Shutdown is protected by
the process cleanup watchdog. Monitor the recorder and mount from outside the
container for terminal NAS or host loss.

## Upgrade and rollback

Upgrade applies additive incident/run and destination-status migrations. Back
up the SQLite database and INI file before changing versions. On startup,
LightNVR reconciles an unclosed process run and preserves active incident IDs;
raw periodic samples are not stored.

Before rolling back to a binary that predates system health, disable health
event routes and remove managed destination status-topic configuration through
the current UI/API. Older binaries ignore the new tables, but cannot interpret
new policy settings or presence semantics. Keep the database backup so a later
upgrade can resume incident history. Rollback does not prove that an open
incident recovered; external alerting should expire or reconcile it explicitly.
