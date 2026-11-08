# Observation contract fixtures (INT_01 C3)

Canonical examples of the [observations contract](../../../../docs/integrations/OBSERVATIONS_CONTRACT.md)
shared by lightNVR (L5, L7, L8, L11, L14) and YA-WAMF (W4, W5, W6, W10). Every
file is listed in `manifest.json` with its kind, the OpenAPI schema it
validates against, and the HTTP request, status and headers or the MQTT
topic it belongs to.

Do not edit these files by hand. They are generated from a single scenario
by `scripts/contracts/observation-fixtures.mjs` and checked by
`tests/contracts/tooling/observation-fixtures.test.mjs`.

## Scenario

There is one installation (`urn:lightnvr:11111111-…`), one camera
(`22222222-…`, `feeder-east`) and DB epoch `66666666-…`. `manifest.changes` is
the complete change log in persistence (`seq`) order:

| seq | Observation | Rev | Cause | Evidence |
| --- | --- | --- | --- | --- |
| 1001 | A `5555…` single bird | 1 | created | pending |
| 1002 | B `bbbb…` single bird | 1 | created | unavailable (`queue_full`) |
| 1003 | A | 2 | evidence | ready (contract §3.6 example; §5.4 test vector) |
| 1004–1007 | C0/C1 `cccc…00/01`, two birds sharing asset `9999…` | 1→2 | created, evidence | pending → ready |
| 1008–1009 | D `dddd…`, HTTP detector, letterboxed | 1→2 | created, evidence | pending → ready |
| 1010 | A | 3 | classification | ready (automated `yawamf/species`) |
| 1011 | E `e0e0…`, ONVIF, no box | 1 | created | unavailable (`no_matching_media`) |
| 1012 | A | 4 | classification | ready (owner correction) |
| 1013 | A | 5 | evidence | expired (`retention_expired`) |

- `observations/`: the Observation at a given revision.
- `events/`: complete MQTT payloads (envelopes) for
  `io.lightnvr.detection.observation_changed.v1`. The topic is in the
  manifest.
- `message-ids.json`: the UUIDv5 namespace derivation and every envelope ID.
- `pages/`: three chained history pages (`limit=2`, watermark seq 1010) and a
  `cursor=head` page. Items hold the current state at the watermark, so A
  appears at revision 3 on pages 1 and 3.
- `errors/`: coded error bodies, including `cursor_expired`,
  `evidence_expired` and the `202 evidence_pending` body with `Retry-After`.
- `capabilities/`: P1 (annotate not available) and P2 documents.
- `classifications/`: PUT request and response pairs. `observation_file` is
  the observation state before the request. The writer namespace is
  user-derived (D3). The example user is `yawamf`, but consumers MUST read it
  from `capabilities.principal.writer_namespace`.
- `media/two-birds-160x80.jpg`: the evidence image of C0/C1, with box pixel
  coordinates and colors in `media/two-birds-160x80.json`.

Two conventions are fixture choices that the contract does not state outright:

- **Pending evidence** keeps `origin`, `media_type`, `captured_at` and
  `transform`, and leaves `width`, `height`, `bytes` and `expires_at` null
  until the asset is ready.
- **Evidence that never had media** (`queue_full`, `no_matching_media`) has
  every media field null.

## Commands

```sh
node scripts/contracts/observation-fixtures.mjs            # regenerate JSON
node scripts/contracts/observation-fixtures.mjs --check    # fail if stale
node scripts/contracts/observation-fixtures.mjs --jpeg     # also re-encode the JPEG (needs sharp)
node --test tests/contracts/tooling/observation-fixtures.test.mjs

# Vendor into YA-WAMF (backend/tests/fixtures/lightnvr/) after committing here:
node scripts/contracts/observation-fixtures.mjs --vendor ../YetAnother-WhosAtMyFeeder
node scripts/contracts/observation-fixtures.mjs --check-vendor ../YetAnother-WhosAtMyFeeder
```

When the YA-WAMF checkout is a sibling directory, or `LIGHTNVR_YAWAMF_DIR`
points at it, the tooling test also checks that the vendored copy is
byte-identical.
