# Streams/discovery canonical schema evidence

T4 authors `docs/api/openapi.json` from current C handlers at execution baseline
`dea6094a` and the independent T2 captured payloads. It generates only the embedded
native header; T8 owns production frontend generated artifacts. No product code,
API behavior, package dependencies or web build outputs were changed by T4.

Run from the repository root:

```sh
npm run contracts:lint
npm run contracts:generate -- --target=native
npm run contracts:check -- --target=native
node --test tests/contracts/fixtures/schema/validate.mjs
```

The validation script generates frontend validators into a private temporary
folder, checks the captured payloads and authored malformed cases, then removes
that folder. It does not update `web/generated/api`. Five test groups pass,
including all twelve actual T2 responses. Source-traced synthetic live/LPR
summary variants are clearly labeled; actual-server coverage of those variants
belongs to T16. Negative checks cover missing/wrong required fields, redacted
credentials, schedule length, privacy/config mixing, contradictory enabling,
nullable clears, zero retention/threshold values and future additive fields.

## Consumer interfaces

T8 can import the following named generated guards (each name prefixed by
`validate`) and corresponding `components["schemas"]` types:

- `StreamSummaryPage`: `summary=true`, default/admin surface.
- `LiveStreamSummaryPage`: live surface.
- `LprStreamSummaryPage`: identity-only LPR surface.
- `StreamCompatibilityList`: no summary flag, or a value other than `true`/`1`.
- `StreamFullResponse`: `{stream: ...}` with redacted/configurable variants.
- `StreamUpdateRequest`: ordinary partial update or privacy-only update.
- `StreamUpdateAccepted`: HTTP 202 queued acknowledgment.
- `StreamPrivacyUpdated`: HTTP 200 synchronous privacy result.
- `ApiError`: string errors and the existing nested YAML-validation error.
- `OpenApiDocument`: discovery envelope, independently linted as OpenAPI by
  the projection/API test phases.

Use the requested surface-specific summary guard. The operation-level
`StreamListResponse` union intentionally includes minimal LPR rows; it cannot
prove that a malformed administrative row contains administrative fields.
Generated declaration types are DTOs; editable form strings and defaults stay
separate. Validator diagnostics are sanitized by T3 and do not print payloads.

## Source reconciliation and limits

- `src/web/api_handlers_streams_get.c`: `handle_get_stream_summaries` selects
  admin/live/LPR shapes, performs filtering before totals, and bounds page size
  at 100. Only `summary=true` or `1` selects summary mode. Compatibility list
  and full serialization share fields but full uses the `{stream: ...}` wrapper.
- Full-read visibility uses raw `live.view`, then runtime `camera.configure`;
  compatibility/summary privilege checks use runtime plus go2rtc health. These
  are different existing semantics. Each privileged `anyOf` alternative carries
  the appropriate same-camera conjunction. Redacted alternatives remain
  available alongside privileged ones to describe mixed-camera results.
- Redacted sensitive strings remain present and empty. URL userinfo stripping
  differs from returning the raw configured URL, especially for ONVIF/separate
  credentials and custom URL formats. No real or synthetic credential values,
  camera names, instance UUIDs or input URLs appear in canonical examples.
- Read strings are empty rather than null. `retention_days` and detection
  retention use -1 for global, 0 for unlimited. Zero detection thresholds and
  tier multipliers are preserved; no schema default rewrites them. The existing
  handler does not clamp detection threshold to 0–100.
- `src/web/api_handlers_streams_modify.c`: `handle_put_stream` keeps ordinary
  updates partial and returns 202 after queueing. This is not durable success.
  Boolean `set_privacy_mode` selects the separate synchronous form; only `name`
  may accompany it. `enable_disabled=true` with `enabled=false` is rejected.
- Privacy removal failure has a distinct **502**: the privacy flag was saved,
  but go2rtc could still expose the stream. The request must be retried; this is
  not a no-side-effect error. The contract documents it without changing C.
- Null explicitly clears tags, admin URL, motion source, go2rtc source override,
  substream URL, detection URL and publish URL. Null is ignored for many other
  typed fields; null playback transport/ePTZ is rejected. Retention input is
  numerically truncated toward zero before its -1..365 check, so the schema
  preserves that behavior. Other integer-backed update fields likewise accept
  numeric input. Known typed values are documented without promising C rejects
  every wrong type: existing handlers sometimes ignore malformed input or
  wrong-length schedules. Additive ordinary-update fields remain allowed.
- Width/height/FPS are detected and ignored as ordinary update fields. Codec is
  accepted as a hint despite an older nearby comment saying detected codec is
  not accepted; the actual later `codec_put` branch is authoritative.
- Names are path-segment encoded; the normal creation API restricts names, while
  older configured names may need encoding. The full-read handler trims its
  suffix at the first decoded slash, so embedded slashes are not advertised as
  supported. A missing runtime stream can produce 404 before authentication.
- Override size constraints are UTF-8 **byte** limits in C. String schemas and
  descriptions do not pretend JSON Schema `maxLength` counts bytes. ePTZ JSON
  embedded in a string and go2rtc YAML validation remain server-side semantics.

## Projection handoff

Stable operation IDs are `listStreams`, `getStreamFull`, `updateStream` and
`getOpenApi`. Every operation uses version-1 authorization metadata from T1.
Conditional alternatives use `x-lightnvr-visibility` directly on `anyOf` `$ref`
branches, both at full-response level and list-item level. T6 must prune those
branches and unreachable components while retaining additive JSON schemas.

Remove root `x-lightnvr-validators` and actual permission annotations. Preserve
named schema property-map keys: `OpenApiDocument` has a property literally named
`x-lightnvr-coverage`. Do not remove all `x-*` keys indiscriminately.

Top-level coverage has `partial`, `canonical_revision`, `covered_operations` and
`description`. The covered IDs represent the whole canonical pilot independently
of current visibility; T7 replaces the source marker with the generated document
SHA-256. The relative server `../` resolves against the served spec URL. Discovery
responses and errors declare `Cache-Control: private, no-store`; ordinary existing
routes do not claim new cache headers. Synthetic demo/auth-disabled discovery is
excluded, while real authenticated demo users retain actual action permissions.

Remaining public/bootstrap APIs and unrelated routes are explicitly outside the
pilot. Their absence is not a permission decision. HTTP principal behavior,
independent C projection validity, actual live/LPR results, malformed server
payload handling and browser prefix behavior remain T6/T7/T16/T17 validation.
