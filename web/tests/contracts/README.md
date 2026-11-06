# Streams adapter handoff (T8 → T14)

Import runtime functions and type-only DTO aliases from `web/js/api/streams.ts`.
Generated types live in `web/generated/api/schema.d.ts`; runtime guards in
`validators.js` are standalone and contain no schema compiler. Regenerate from
the canonical document, never edit generated files.

| Function | Return value |
| --- | --- |
| `listStreamSummaries(query?, options?)` | Admin/default `StreamSummaryPage`; literal `surface: 'live'` returns `LiveStreamSummaryPage`; literal `surface: 'lpr'` returns `LprStreamSummaryPage` |
| `listStreams(options?)` | `StreamCompatibilityList`, including an empty array |
| `getStreamFull(name, options?)` | `StreamFullResponse`, retaining its `{stream: ...}` wrapper and permission-redacted fields |
| `updateStream(name, body, options?)` | `{status: 202, data: StreamUpdateAccepted}` or `{status: 200, data: StreamPrivacyUpdated}` |

`StreamSummaryQuery` uses the generated query names (`page`, `page_size`,
`sort_by`, `sort_order`, `search`, `availability`, `include_admin_url`, `surface`).
The adapter supplies `summary=true` and selects the exact surface guard. Do not
reuse a permissive operation-union guard for administration rows.

`StreamRequestOptions` preserves transport `signal`, `timeout`, `retries`,
`retryDelay`, headers, credentials and auth redirect behavior. Method, body and
sensitive logging mode are owned by the adapter. Feed the query library's signal
directly to the options object. Reads keep existing transport retry defaults;
PUT defaults to zero retries, with explicit caller overrides respected. A write
timeout is never retried by transport, even with a retry override.

`StreamUpdateRequest` is the generated wire DTO, separate from editable form
state. Validate before sending, and keep existing field mappings and omission
semantics. Ordinary updates return 202 queued acknowledgements; a valid response
does not prove durable application. Privacy-only updates return 200. A privacy
502 can occur after saving the flag; do not unconditionally roll back local
state or blindly retry. Refresh current state after ambiguous write failures.

`ContractValidationError` identifies operation, request/response phase, schema,
HTTP status when available, and shape/JSON/status failure. `StreamRequestError`
retains HTTP `status` and `kind` (`http`, `network`, `timeout`). Both supply
`mayHaveApplied` for ambiguous writes. Native `AbortError` identity is retained;
pre-aborted requests also reject with that name. Cancellation of a write does
not prove rollback. Error messages omit URLs, server messages, payloads and
credentials. Adapters always opt into `EnhancedFetchOptions.sensitive`, which is
consumed before calling fetch and suppresses transport/JSON diagnostic logging.

`streams.spec.ts` exercises real captured fixtures, source-traced live/LPR
variants, empty/redacted/additive fields, malformed required fields, forbidden
redacted credentials, update status/body distinctions, invalid requests,
privacy partial-failure semantics, HTTP/network/timeout/abort behavior and log
suppression. `streams.type-fixture.ts` is checked by web typecheck and proves
unknown input and malformed DTOs cannot silently become typed data.

```sh
npm run contracts:generate -- --target=web
npm run contracts:check -- --target=web
npm --prefix web run typecheck
npm --prefix web test -- --runInBand --testPathPatterns='contracts/'
```

## Consumer regressions (T14)

`streamsConsumer.spec.ts` exercises the real Streams page, cards, modal, query
observers and generated adapters in Chromium. It runs under the regular Jest
contract-test path, so every CI gate that discovers it must first install root
dependencies, web dependencies, and the Chromium version pinned by the root
Playwright lockfile. Web test typechecking also needs those root dependencies.
T19 must arrange these prerequisites before frontend tests, rather than silently
skipping the browser assertions:

```sh
npm ci
npm --prefix web ci
npx playwright install --with-deps chromium
```

The fixture uses local browser routing, captured synthetic API payloads and
in-memory compilation; it needs no running NVR or external service. It proves
loading/empty/error states, safe malformed-response retry, per-camera controls,
fresh full-detail authorization, zero-preserving form/DTO mapping, queued-update
feedback and cancellation. Existing create-after-edit and pending-state browser
suites retain their behavioral assertions with canonical-shaped fixture data.
