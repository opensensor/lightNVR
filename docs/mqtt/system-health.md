# System health events and presence

LightNVR emits health state changes through the durable event pipeline. It does
not publish every raw sample and does not page a person itself. Route these
events to MQTT, Home Assistant, Node-RED, or a fleet service that owns
notification and escalation policy.

## Health events

The versioned registry contains:

- `io.lightnvr.system.health_alert.v1` for incident open, escalation, material
  change, and one-shot evidence. Severity is `warning`, `error`, or `critical`.
- `io.lightnvr.system.health_recovered.v1` for recovery. Severity is `info`.

Both use `subject_kind: system` and the normalized envelope documented in the
[event contract](../EVENT_CONTRACT.md). Health routes may filter with
`predicate.health.condition_codes_any` and `severities_any`. Camera selectors
are rejected for system events. Unknown condition codes and severities are
rejected instead of becoming never-matching rules.

Condition codes are stable and bounded:

```text
memory.available_low       memory.oom_kill             memory.swap_thrash
cpu.saturation             cpu.throttled               io.pressure
filesystem.bytes_low       filesystem.inodes_low       filesystem.read_only
filesystem.write_failed    thermal.high                network.link_down
network.error_rate         clock.unsynchronized        clock.jump
process.fd_exhaustion      process.pid_exhaustion      process.allocation_failed
storage.device_prefail     storage.device_critical     hardware.ecc_corrected
hardware.ecc_uncorrectable hardware.fan_failed         hardware.power_unstable
health.collector_stale     system.unexpected_restart   event.delivery_degraded
```

## Default-broker presence

When the default MQTT integration is enabled, LightNVR publishes retained
schema-v1 presence at:

```text
<topic_prefix>/v1/status/<installation_uuid>
```

The retained document has installation, process-run, and boot identities;
visibility scope; version; monotonically increasing sequence; timestamp;
`state`; `overall_state`; and active incident count. Normal values of `state`
are `online`, `stopping`, and `offline`. `offline` is the broker-published Last
Will after an unclean disconnect; a graceful shutdown publishes `stopping`.
Online heartbeats use `[health] presence_interval_seconds`.

## Managed-destination presence

An MQTT event destination can set `broker.status_topic_template`. The template
must contain `{installation_uuid}` and may contain `{destination_uuid}`. It may
not contain MQTT wildcards or unknown placeholders. For example:

```text
lightnvr/v1/status/{installation_uuid}/{destination_uuid}
```

Managed status documents use the same schema and are retained at QoS configured
for that destination. The managed heartbeat is currently 60 seconds.

## Detecting loss correctly

LWT is broker behavior, while staleness is observer behavior. An external
subscriber should retain the last document and declare the installation stale
after two expected heartbeat intervals without a newer sequence. Do not ask
LightNVR to alert on its own absence: it cannot run after a process crash, host
lockup, power loss, or total network failure.

If the broker and LightNVR host fail simultaneously, that broker cannot publish
the Will or evaluate heartbeat age. Use a broker on an independent failure
domain or a second supervisor/monitor. Presence proves loss of contact, not the
root cause; changed boot identity and an unclosed run marker add evidence only
after LightNVR starts again.

Operational incidents and presence are intentionally separate. A critical
incident does not mark the process offline, and an `online` document may carry
`overall_state: critical`.
