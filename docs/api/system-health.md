# System health API and metrics

System health is an administrative, read-only view of the latest completed
sampler generation and the durable incident lifecycle. Reading either endpoint
or `/api/metrics` never starts a probe. These endpoints require an authenticated
administrator and return `401` when credentials are missing or invalid.

Operational health does not change the semantics of `GET /api/health`.
`/api/health` remains the process liveness endpoint suitable for a service
manager; a warning or critical operational incident does not make it fail.

## Current state

`GET /api/system/health` returns schema version 1:

```json
{
  "schema_version": 1,
  "overall_state": "warning",
  "visibility": {
    "effective_scope": "container",
    "host_hardware_visible": false,
    "coverage_boundary": "container_and_visible_mounts"
  },
  "snapshot": {
    "available": true,
    "sequence": 42,
    "completed_at_ms": 1788432000000,
    "age_ms": 812,
    "freshness": "fresh"
  },
  "coverage": {},
  "observations": [],
  "thresholds": [],
  "active_incidents": [],
  "recent_samples": [],
  "evaluator": {},
  "self_observability": {}
}
```

Important contracts:

- `overall_state` is `unknown`, `healthy`, `warning`, `error`, or `critical`.
- Scope is explicit: `process`, `container`, `host`, `filesystem`, or `device`.
- Capability is `available`, `unsupported`, `permission_denied`, `stale`, or
  `error`. An unavailable value is JSON `null`, never a fabricated zero.
- `snapshot.sequence`, timestamps, and age are nullable until the first
  generation completes. Freshness is `unknown`, `fresh`, or `stale`.
- Observations carry only bounded logical resource IDs. The response excludes
  raw paths, device serial/WWN values, addresses, credentials, command lines,
  and command output.
- `coverage.complete` is false when visibility is unavailable, stale, dropped,
  timed out, or capped. Inspect the counters rather than assuming absent data
  means healthy.
- Arrays are bounded: 256 observations, 64 active incidents, 64 policy rules,
  120 recent generation summaries, 33 collector rows, and 65 delivery rows.

Each threshold row identifies its condition, direction, unit, warning/critical/
recovery values, and dwell periods. Each active incident includes one stable
incident ID, condition and logical subject, scope, state, severity, safe latest
observation, applicable thresholds, and whether durable persistence is pending.

## Incident history

`GET /api/system/health/incidents` reads durable episodes in reverse
`last_seen_at_ms` order. Query parameters are:

| Parameter | Default | Contract |
| --- | --- | --- |
| `limit` | server default | Integer from 1 through 100 |
| `include_closed` | `true` | `true`, `false`, `1`, or `0` |
| `cursor` | none | Opaque `next_cursor` from the preceding response |

The response contains `schema_version`, `count`, `incidents`, and a nullable
`next_cursor`. Clients must treat the cursor as opaque and restart pagination
after a validation error or policy/database change.

## Prometheus

`GET /api/metrics` exports bounded health series alongside existing LightNVR
metrics:

| Family | Meaning |
| --- | --- |
| `lightnvr_health_snapshot_sequence` | Latest immutable generation |
| `lightnvr_health_observations{scope,capability}` | Observation coverage |
| `lightnvr_health_incidents{severity}` | Active incident counts |
| `lightnvr_health_collector_up{collector,scope}` | Fresh available collector result |
| `lightnvr_health_collector_duration_seconds{collector,scope,kind}` | Last/maximum collector duration |
| `lightnvr_health_collector_events_total{collector,scope,event}` | Collector calls, failures, timeouts, and skips |
| `lightnvr_health_collector_stale{collector,scope}` | Staleness indicator |
| `lightnvr_health_collector_busy{collector,scope}` | In-progress collection indicator |
| `lightnvr_health_sampler_events_total{event}` | Sampler lifecycle counters |
| `lightnvr_health_evaluator_events_total{event}` | Transitions and persistence failures/retries |
| `lightnvr_health_persistence_pending` | Conditions waiting for durable storage |
| `lightnvr_health_persistence_retry_age_seconds` | Oldest pending persistence age |
| `lightnvr_health_abandoned_helpers` | Timed-out helpers being reaped |
| `lightnvr_health_coverage_overflows_total` | Bounded resources omitted at a cap |
| `lightnvr_health_snapshot_observations_dropped` | Observations omitted from the current snapshot |

Current-value families are grouped by bounded dimensions:

- `lightnvr_system_cpu_usage_ratio`, `lightnvr_system_cpu_throttled_ratio`,
  `lightnvr_system_load_average`, `lightnvr_system_load_ratio`,
  `lightnvr_system_pressure_stall_ratio`, `lightnvr_system_memory_bytes`,
  `lightnvr_system_swap_bytes`, `lightnvr_system_vm_events_delta`,
  `lightnvr_system_uptime_seconds`, and `lightnvr_system_boot_info`;
- `lightnvr_process_open_fds`, `lightnvr_process_max_fds`,
  `lightnvr_process_threads`, `lightnvr_process_max_pids`, and
  `lightnvr_process_start_time_seconds`;
- `lightnvr_filesystem_bytes`, `lightnvr_filesystem_inodes`,
  `lightnvr_filesystem_read_only`, and
  `lightnvr_filesystem_probe_duration_seconds`;
- `lightnvr_network_bytes_total`, `lightnvr_network_packets_total`,
  `lightnvr_network_errors_total`, `lightnvr_thermal_celsius`,
  `lightnvr_clock_synchronized`, and `lightnvr_clock_jump_seconds`;
- `lightnvr_device_health_flag`, `lightnvr_device_wear_ratio`,
  `lightnvr_device_available_spare_ratio`, `lightnvr_device_events_delta`,
  `lightnvr_device_smart_attribute`, `lightnvr_hardware_ecc_errors_delta`,
  `lightnvr_hardware_fan_flag`, `lightnvr_hardware_fan_rpm`, and
  `lightnvr_hardware_power_flag`.

Labels use registry names and logical IDs only. Incident IDs, event IDs,
process run IDs, raw paths, device identifiers, and IP addresses are never
Prometheus labels. This prevents both privacy leakage and unbounded cardinality.
