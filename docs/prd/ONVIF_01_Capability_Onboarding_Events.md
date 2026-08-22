# PRD — ONVIF Capability Onboarding & Events

**Status**: Draft
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 6 — camera interoperability
**Scope**: Capability-driven discovery and onboarding, modern media/profile
selection, reliable device events, diagnostics, and a tested compatibility matrix.

---

## 1. Problem

lightNVR already has ONVIF discovery, profile enumeration, add/test flows, PTZ,
presets, home position, imaging settings, and motion-oriented event support. The
product and UI still treat ONVIF as a set of optional operations rather than a
device capability contract.

At fleet scale, operators need to claim many cameras, choose valid main/sub/
detection profiles, understand what each device actually supports, and diagnose
vendor-specific failures. Event support also needs to discover and normalize more
than a few assumed motion topics.

## 2. Goals

- Build and persist a capability matrix for every ONVIF camera.
- Make discovery a staged, repeatable inventory and claim workflow.
- Account for device clock offset and authentication behavior.
- Enumerate and validate modern media profiles, including H.265 and audio where
  devices expose them.
- Discover event topics and maintain robust subscriptions.
- Normalize supported device events into Fleet 03 contracts.
- Publish an evidence-backed tested-camera compatibility matrix.

## 3. Non-goals

- Claiming ONVIF conformance or certification without passing official tests.
- Supporting deprecated Profile Q anonymous setup behavior.
- Profile M analytics metadata and Profile G recordings; covered by ONVIF 02.
- Implementing every vendor extension.
- Firmware update or vendor device-management portals.

## 4. Capability model

For each camera UUID, store a timestamped capability snapshot containing:

- Device identity: endpoint, manufacturer, model, firmware, serial, hardware ID,
  IP and MAC where available.
- Services and versions: Device, Media/Media2, Events, PTZ, Imaging, Analytics,
  Recording/Search/Replay, DeviceIO.
- Media: profiles, tokens, stream/snapshot URI support, codec, resolution, FPS,
  bitrate, audio source/encoder, and backchannel.
- PTZ and imaging operations already supported by lightNVR.
- Event service mode and discovered topic set.
- Security/authentication and observed clock offset.
- Last probe result, changed fields, raw redacted diagnostic reference.

Capability changes emit an event and may create configuration drift; they do not
silently rewrite a working stream configuration.

## 5. Requirements

### 5.1 Discovery inventory

- Run bounded WS-Discovery scans on selected interfaces/subnets.
- Deduplicate devices by endpoint plus serial/MAC evidence, not IP alone.
- Persist first seen, last seen, addresses, identity, claim state, and duplicate
  suspicion.
- Never auto-claim a discovered device or persist supplied credentials until an
  authorized operator confirms.
- Feed the Fleet 04 discovery staging inbox.

### 5.2 Authentication and time

- Query device time and calculate clock offset before authenticated calls where
  supported.
- Distinguish network, TLS, authentication, time-skew, SOAP-fault, unsupported,
  and malformed-response errors.
- Redact credentials and security headers from logs and diagnostics.
- Credential profiles are reusable secret references, not passwords copied into
  templates or job records.

### 5.3 Capability probe

- Use GetServices/GetCapabilities and service-specific probes rather than vendor
  assumptions.
- Cache successful results with manual refresh and bounded periodic refresh.
- Treat partial capability discovery as a usable state with visible warnings.
- Preserve the raw operation name and SOAP fault for diagnostics while presenting
  a plain-language operator result.

### 5.4 Media/profile pairing

- Enumerate Media and Media2 profiles where supported.
- Present codec, resolution, FPS, bitrate, audio, snapshot, and transport facts.
- Let an operator pair main, sub, and detection uses, preventing accidental use of
  the same unsuitable high-resolution profile for every role.
- Test selected URIs before committing configuration.
- Prefer direct camera-reported URIs, then apply explicit address rewriting only
  when the camera advertises an unreachable host.
- Support H.264 and H.265 discovery; actual browser playback remains subject to
  existing lightNVR transport/codec support.

### 5.5 Event discovery and subscription

- Query event properties/topics instead of hard-coding a fixed vendor topic list.
- Create PullPoint subscriptions, renew before expiry, recreate after reboot or
  invalid subscription, and back off on repeated failure.
- Normalize initial types: motion state, tamper, digital input, line crossing,
  simple analytic alarm, and device fault where semantics are sufficiently known.
- Preserve vendor topic and raw key/value metadata in a bounded diagnostic field.
- Map asserted/cleared state consistently and deduplicate repeated device messages.
- Publish normalized events through Fleet 03; event delivery never runs in the
  ONVIF polling/subscription thread.

### 5.6 Diagnostics and compatibility

- Generate a redacted per-camera ONVIF report: identity, services, profiles,
  capability matrix, clock offset, event topics, selected configuration, last
  faults, and software version.
- Maintain response fixtures for tested vendors and regress them in CI.
- Publish a tested-camera matrix distinguishing discovered, streaming, snapshot,
  events, PTZ, imaging, audio, and backchannel support.
- “Unknown/not tested” is distinct from “unsupported.”

### 5.7 Fleet UI integration

- Discovery staging supports batch credential test, capability probe, profile
  pairing, organization, template assignment, and commit.
- Camera detail shows capability badges and diagnostic actions.
- Bulk probe jobs use Fleet 04 bounded concurrency and per-camera results.

## 6. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Persistent discovery inventory, identity/deduplication, time/auth diagnostics |
| P1 | Capability snapshot and Media/Media2 profile pairing |
| P2 | Event topic discovery, resilient PullPoint manager, normalized events |
| P3 | Fleet staging UI, bulk probes, fixtures, published compatibility matrix |
| P4 | Additional Profile T controls selected from real camera/customer demand |

## 7. Acceptance criteria

- Repeated discovery of a DHCP-renumbered camera updates one inventory record.
- A clock-skewed device can be diagnosed without reporting a generic bad-password
  error.
- Batch onboarding can claim, organize, profile-pair, test, and commit at least 50
  discovered cameras with individual results.
- Subscription renewal and lightNVR restart recover event delivery without manual
  camera reconfiguration.
- A normalized motion/tamper/input event contains stable camera UUID and original
  vendor topic diagnostics.
- Compatibility claims are backed by stored fixtures or recorded hardware test
  results and never imply ONVIF certification.

## 8. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Vendor SOAP behavior varies | Capability probing, fixtures, redacted raw faults, compatibility matrix |
| Discovery floods a large subnet | Interface/subnet scope, bounded concurrency, rate limits |
| Subscription loops hammer cameras | Lease-aware renewals and exponential backoff |
| Capability refresh breaks working setup | Snapshot changes are advisory until explicitly reconciled |

## 9. Dependencies

- Fleet 01 for stable identity and organization.
- Fleet 03 for normalized event publication.
- Fleet 04 for batch discovery/claim UX; core probes can land first.
- Fleet 02 controls who may discover, claim, configure, and operate cameras.
