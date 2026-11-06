# Contract and explorer performance evidence

Measured on 2026-11-05. The final 33-operation workload and focused repeat passed.
Snapshot SQL counts remain constant as nonempty fleet size increases; operation
expansion increases their fixed cost, response size, latency and validator bytes.
Normal pages load no explorer vendor assets. These workstation results do not
qualify an embedded device or active-media workload. The original T20/T20a pilot
measurements below remain dated comparison evidence.

## Final T29 expanded-contract measurements

Release binary `build/eng-api-T24/bin/lightnvr` has SHA-256
`fadd75c4694e09a321a6c21f37305370779b05ca80050e82b9f5e769deb7ecba`;
its source manifest is `d4ad3880e4f95bb1f726eb2682e067db70efb9711987d030d1a95d0a250fb0f8`
in `build/eng-api-T24/evidence.json`. Canonical revision:
`c7f5f796e564d551e9d56515ba56f72615d1be0148e7c0bd22fa1689ae6b0b58`.
The coherent-FFmpeg Release has SSL enabled and Go2RTC/MQTT/SOD/LiteRT disabled.

The same declared fixture/sample method now checks all 33 covered operation IDs.
Full sizes are 0/16/128/512/1024; the repeat uses 16/1024, with 20 serial samples and
five four-request concurrent batches per principal after three warmups. Actual
SQL tracing runs separately. Reports are retained in
`build/eng-api-T29-projection-{full,repeat}/result.json`.

At 1,024 cameras:

| Principal | JSON bytes | Full serial median ms | Full serial p95 ms | Full concurrent p95 ms | Repeat serial median ms | Repeat concurrent p95 ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| admin | 131,314 | 16.11 | 17.55 | 33.14 | 15.69 | 33.78 |
| empty policy | 68,006 | 34.63 | 49.12 | 70.86 | 25.46 | 61.44 |
| split | 84,781 | 40.07 | 49.16 | 71.17 | 33.66 | 68.10 |
| mixed | 87,474 | 25.03 | 29.08 | 55.41 | 23.64 | 51.07 |
| admin-owned read token | 53,587 | 30.60 | 33.91 | 77.14 | 26.56 | 66.85 |

For every nonempty size and both runs, snapshot statement counts are 19/19/25/25/24
in that principal order, versus the pilot's 11/11/17/17/16. Empty fleets retain 6/7.
This is bounded fixed growth with broader eligibility checks, not unchanged cost
or constant total database work: returned rows and VM steps grow with fleet size.
Whole-process high water at 1,024 peaks at 267.90 MiB, repeat 267.98 MiB, versus
the accepted pilot 264.22/264.39 MiB. The compact camera storage remains 22.09 MiB
and the configured-max runtime metrics buffer remains 11.91 MiB. Negative/split
policy cases are slower with the expanded contract; no target-device SLO is
inferred from these numbers.

Final installed web bytes are **14,946,302 default** and **10,925,296 modern**.
The modern validator chunk grows from 136,439 to 358,973 raw bytes
(30,565 stored gzip); the default legacy counterpart is 358,251 raw/28,940 gzip.
The 47 exported guards currently share a module. This measured growth motivates
the follow-on task to evaluate excluding/splitting unused guards before further
coverage expansion; it is not attributed to Swagger.

| Entry | Default closure bytes / gzip | Modern closure bytes / gzip |
| --- | ---: | ---: |
| index | 3,502,510 / 925,006 | 1,519,723 / 441,587 |
| streams | 2,744,836 / 509,585 | 1,148,377 / 232,792 |
| recordings | 1,790,937 / 411,547 | 669,727 / 183,092 |
| api-docs | 2,745,173 / 592,481 | 2,250,992 / 515,432 |

The separately packaged Swagger assets remain 2,365,964 installed bytes. Static
closures and fresh Chromium capture against an owned, authenticated, empty real
server find zero vendor/external requests and zero page errors on normal pages
in both profiles. The explorer loads its two local vendor files. This final
network fixture differs from T20's mocked empty APIs; both qualify asset isolation,
not every interaction. Default was rebuilt after modern and all 261 files match
the immutable default snapshot used by the 13 browser contract tests.

Reproduce with the named fresh binary/build manifest and pinned dependencies:

```sh
node tests/contracts/projection-benchmark.mjs --binary build/eng-api-T24/bin/lightnvr --build-manifest build/eng-api-T24/evidence.json
node tests/contracts/projection-benchmark.mjs --binary build/eng-api-T24/bin/lightnvr --build-manifest build/eng-api-T24/evidence.json --sizes 16,1024
node scripts/measure-contract-assets.mjs --dist web/dist --profile default --out /tmp/new-assets.json
node tests/contracts/asset-isolation.mjs --binary build/eng-api-T24/bin/lightnvr --web-root web/dist --profile default --output /tmp/new-network.json
```

Build modern with `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`, capture
it with `--profile modern`, then restore the default build. Reports require new
output paths. Final local receipt: `build/eng-api-T29/acceptance.json`.

## Build and measurement identity

- Git HEAD: `dea6094a731c86debb03cc1c75d0251ab3b96cec` with the in-progress plan changes. HEAD alone does not identify this build.
- Final post-repair Release: `build/eng-api-T20a/bin/lightnvr`, SHA-256 `08323d6a472f34bbcbdfcdf397d2ab0438378a1f4437e3c42c7fdb384f53e02b`; source manifest digest `68a80fb0bc4578c86d12e08ae399c3805e27c1f2aa6c4e8ca590e787a3f9ed68`. The comparison section below reports its results separately.
- Pre-repair native source manifest digest: `67636faf3d8bf0e66169724c80a914c1c313da99213cdaa8ddcfda349474645b`; file hashes and configuration are in `build/eng-api-T20/evidence.json` and copied into both benchmark reports.
- Pre-repair Release binary: `build/eng-api-T20/bin/lightnvr`, SHA-256 `900023126c7d252c6977dac76c0ecc488cccac505cd1bc855af72ae48e39261e`.
- Host: AMD Ryzen 9 7950X, 32 logical CPUs, Linux `7.0.0-38-generic`, Node `v24.22.1`. Full-run load averages start `[0.69,1.18,1.19]`, end `[1.69,1.45,1.30]`; repeat start `[0.96,1.15,1.19]`, end `[2.68,1.56,1.33]`. Other agents paused heavy tests during timing; this is a shared workstation, not an isolated performance lab.
- Native configuration: Release, system FFmpeg headers/libraries, SSL on; SOD, LiteRT, managed go2rtc and MQTT off. The baseline used `BUILD_TESTS=OFF`; the post-repair Release uses `BUILD_TESTS=ON` so its compiled unit targets can be reused for integration. The manifest helper records an installed go2rtc hash for provenance, but this benchmark does not run it.
- Web manifests include per-file hashes, source digest, environment, profile and measurement definitions. Default and modern builds were both regenerated after the explorer's final cache option and T14a's row-permission repair; default was restored and every installed file hash matched the first default build.

## Reproduction

Run from the repository root with existing native dependencies and installed npm dependencies. Serialize builds with other workers because version generation and `web/dist` are shared. Never point the fixture at production data.

```sh
PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig PKG_CONFIG_PATH= cmake --fresh -S . -B build/eng-api-T20a -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTS=ON -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=OFF -DENABLE_MQTT=OFF -DENABLE_SSL=ON '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu' '-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
cmake --build build/eng-api-T20a --target lightnvr ci-unit-tests -j6
python3 - <<'MANIFEST'
import importlib.util, json
from pathlib import Path
spec = importlib.util.spec_from_file_location('reliability', 'tests/reliability/interrupted-recording.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
evidence = module.source_evidence(Path('build/eng-api-T20a/bin/lightnvr'), Path('go2rtc/go2rtc'))
Path('build/eng-api-T20a/evidence.json').write_text(json.dumps(evidence, indent=2) + '\n')
MANIFEST
node tests/contracts/projection-benchmark.mjs --binary build/eng-api-T20a/bin/lightnvr
node tests/contracts/projection-benchmark.mjs --binary build/eng-api-T20a/bin/lightnvr --sizes 16,1024 --timing-only
npm --prefix web run build
node scripts/measure-contract-assets.mjs --dist web/dist --profile default --out /tmp/lightnvr-eng-current-default.json
LIGHTNVR_WEB_LEGACY=false npm --prefix web run build
node scripts/measure-contract-assets.mjs --dist web/dist --profile modern --out /tmp/lightnvr-eng-current-modern.json
npm --prefix web run build
node scripts/measure-contract-assets.mjs --dist web/dist --profile default --out /tmp/lightnvr-T20-restored-default.json
```

These commands build and measure the current post-repair source. The immutable pre-repair binary and exact original harness remain in the baseline artifacts below; the baseline was measured before T20a. The build-manifest helper's go2rtc argument should identify the installed development binary (`go2rtc/go2rtc` here); it is provenance only. Capture the manifest immediately after building while native source is frozen. The benchmark rejects a binary hash mismatch or a non-Release manifest. `--output` accepts a new directory only; the default creates a fresh retained directory. Linux `/proc`, a C compiler, SQLite headers and a dynamically linked SQLite library are required for the memory/trace measurements. `--instrument-only` separates SQL evidence from `--timing-only` timing runs.

## Web cost

Final web-only source digest: `22fb7b73ff93c01c66967dcf410d0e2d72b7d847b7168fbd1a4fb1f4a1b23147` in `/tmp/lightnvr-T20-final-web-source.json`, including generated `web/js/version.js`. Every recorded file hash was verified unchanged through the final default/modern/default builds. The broader asset-manifest digest includes native/test files that other workers were editing; use this web-specific manifest to identify the final frontend sources.

Installed totals include all files and precompressed `.gz` copies. Entry closures include discoverable lazy imports, HTML, CSS and both legacy/modern scripts in the default profile; they are not initial browser transfers. Gzip is the sum of independent level-9 compressed closure files. Deltas below use T2's retained baseline manifests, whose source digest is `748f831a592a58d9611e8856920d3633f326e30d014f9ecc2887206650032e67`.

| Profile | Installed bytes | Delta from T2 baseline |
|---|---:|---:|
| default | 14,467,958 | +2,740,946 |
| modern | 10,685,341 | +2,564,955 |

| Profile / entry | Closure bytes | Gzip bytes | Raw delta from baseline | Gzip delta |
|---|---:|---:|---:|---:|
| default / index.html | 3,502,510 | 925,006 | +4,847 | +3,128 |
| default / streams.html | 2,300,167 | 475,912 | +284,631 | +32,701 |
| default / recordings.html | 1,790,937 | 411,547 | +4,605 | +2,671 |
| default / api-docs.html | 2,300,504 | 558,807 | new entry | new entry |
| modern / index.html | 1,519,723 | 441,587 | +2,396 | +1,573 |
| modern / streams.html | 925,843 | 215,373 | +142,218 | +16,606 |
| modern / recordings.html | 669,727 | 183,092 | +2,271 | +1,343 |
| modern / api-docs.html | 2,028,458 | 498,010 | new entry | new entry |

The pinned Swagger distribution occupies **2,365,964 installed bytes**, including its compressed copies and licenses. Its JS is 1,685,697 raw / 449,711 gzip bytes, CSS 187,576 / 27,016. These vendor files occur only in the docs closure. Actual Chromium production-page request capture also found no vendor requests, no third-party requests and no page errors on index, streams or recordings in either profile. The browser run used empty synthetic API responses and a one-second settled sampling window; it verifies asset isolation, not authorization or every possible user interaction. T17 covers real authorization/proxy behavior.

Generated declarations (`schema.d.ts` 44,017 source bytes and validator declarations 1,458) contribute zero runtime bytes. Generated validators are 236,317 source bytes; production chunks are 136,439 raw / 13,145 stored gzip bytes modern, plus 136,116 / 12,688 legacy in the default profile. A standalone esbuild-minified streams adapter including its guards and transport is 141,287 / 15,639 gzip bytes; this diagnostic is not an additional production-page delta. `streams.ts` itself is 7,675 source bytes. Streams' larger baseline delta is therefore substantially the runtime contract guards, with transport/UI/rechunk effects also present.

The retained T14 pre-explorer assets compared with the pre-T14a explorer build isolate a smaller docs-related change on default normal-page closures:

| Entry | Raw increase | Gzip increase | Explorer vendor files |
|---|---:|---:|---|
| index | 1,450 | 1,709 | none |
| streams | 1,467 | 1,552 | none |
| recordings | 1,456 | 1,724 | none |

T15's grouped chunk comparison attributes 862 raw bytes to extracted auth helpers, 175 to extracted fetch helpers, 107 to validator extraction, with the remainder affected by HTML/import/export/minifier changes. These are grouped differences, not a proof of individual function attribution; separate chunk compression can increase gzip more than raw bytes. Locale installed bytes also increased 14,807. The T14 manifest's built file hashes are valid pre-explorer evidence, but its source digest was captured after unbuilt T15 edits and must not be represented as a T14 source/build pairing.

T14a subsequently adds 207 installed bytes to default and 105 to modern; the streams entry closure adds 149 raw / 58 gzip bytes default and 73 / 32 modern. Final totals above include this permission repair. The pre-T14a manifests remain under `/tmp/lightnvr-T20-pre-T14a-{default,modern}.json` so this repair is not misattributed to explorer loading.

**Accepted criterion clarification (integration owner):** require zero explorer-only renderer/vendor assets in normal-page closures and observed requests; permit the explicitly measured shared utility/rechunk increase above. The literal “normal-page bytes attributable to docs are zero” criterion is not met. The plan must retain this clarification rather than report a zero total delta. Future shared-cost growth should trigger a fresh attribution review; these measured bytes are a baseline, not an invented universal size budget.

## Pre-repair discovery fixture and latency

Each fleet size uses a fresh owned database and real HTTP server with `max_streams=1024`, four libuv worker threads and disabled synthetic cameras. Configuration is fixed at the supported ceiling to expose capacity-dependent allocations; a 16-camera case here is not a max-streams-16 device profile. Cameras have no tags/location trees or active media. Three static shared collections contain alternating left/right halves and the full fleet. There are three added policy users, two custom roles, four grants across split/mixed users, and one scoped token (two grants for split, two for mixed, zero for empty). Total principal count is fixed, not a many-user scaling experiment.

Principals are admin; empty policy; split (`live.view` left, `camera.configure` right); mixed (`live.view` all, configure right); and an admin-owned token limited to `live.view` left. This exercises empty, disjoint and overlapping scopes without fetching camera IDs from discovery. Each response must be 200, `private, no-store`, and have the expected visible operation count. Response sizes at 1,024 are 41,618 / 22,150 / 38,925 / 41,618 / 24,623 bytes respectively.

After three warmups per principal, the uninstrumented run records 20 sequential full-response-body samples, then five batches of four concurrent requests (20 samples). JSON parsing/assertions follow the timed interval. Serial medians and nearest-rank p95s below include HTTP/auth/snapshot/projection/serialization work; they are not pure projector CPU timings. SQL instrumentation runs in separate processes. Startup is bounded at 60 seconds, individual requests at 10 seconds and fleet seeding at 180 seconds. Raw samples and batch durations are retained.

| Cameras | Principal | Serial median ms | Serial p95 ms | Four-concurrent p95 ms |
|---:|---|---:|---:|---:|
| 0 | admin | 8.81 | 9.11 | 18.88 |
| 0 | empty | 8.74 | 9.35 | 22.82 |
| 0 | split | 8.73 | 9.24 | 18.32 |
| 0 | mixed | 8.68 | 9.18 | 18.65 |
| 0 | adminReadToken | 3.69 | 3.83 | 18.32 |
| 16 | admin | 9.89 | 10.51 | 26.62 |
| 16 | empty | 9.40 | 9.80 | 20.81 |
| 16 | split | 9.87 | 10.79 | 20.69 |
| 16 | mixed | 9.99 | 11.08 | 21.70 |
| 16 | adminReadToken | 4.40 | 4.91 | 22.08 |
| 128 | admin | 10.48 | 12.58 | 22.88 |
| 128 | empty | 10.04 | 10.52 | 21.96 |
| 128 | split | 10.62 | 11.10 | 26.75 |
| 128 | mixed | 10.65 | 11.09 | 27.97 |
| 128 | adminReadToken | 5.32 | 5.65 | 22.75 |
| 512 | admin | 13.47 | 16.72 | 30.79 |
| 512 | empty | 12.82 | 13.26 | 30.27 |
| 512 | split | 15.25 | 17.50 | 30.88 |
| 512 | mixed | 13.55 | 14.33 | 29.26 |
| 512 | adminReadToken | 9.38 | 10.39 | 30.90 |
| 1024 | admin | 16.39 | 26.87 | 48.48 |
| 1024 | empty | 16.68 | 25.15 | 49.13 |
| 1024 | split | 31.05 | 38.33 | 59.46 |
| 1024 | mixed | 23.88 | 31.71 | 50.41 |
| 1024 | adminReadToken | 22.56 | 30.83 | 54.55 |

The documented focused repeat passed with the same binary, fixture and sample counts. At 16 cameras, repeat serial medians were 4.57–10.17 ms and concurrent p95s 21.67–28.49 ms. At 1,024, serial medians were 18.15–30.55 ms and concurrent p95s 52.22–67.97 ms. The admin median changed from 16.39 to 24.86 ms; allocator/scheduling and host variation are visible, so these small samples do not establish a stable latency SLO or sub-millisecond comparison. All requests passed; no timeout/retry was used to obtain a passing run.

## Pre-repair actual SQL execution growth

A task-local generated `LD_PRELOAD` helper observes `sqlite3_trace_v2` on read-only connections, recognizes the request snapshot's principal query, and records counts at close. Every measured request must yield exactly one snapshot record. Five samples per principal/size had identical counts. No SQL text, bind values or credentials are emitted. Counts exclude normal authentication/writer-connection work; this is a direct measurement of snapshot SQL, not a claim that the entire endpoint uses only these queries. Row counts include SQLite's internal schema/catalog loading (a large constant baseline); VM steps are SQLite statement-profile counters.

| Cameras | Admin / empty statements | Split / mixed statements | Token statements | Admin rows / VM steps | Mixed rows / VM steps |
|---:|---:|---:|---:|---:|---:|
| 0 | 6 / 6 | 6 / 6 | 7 | 265 / 42 | 265 / 42 |
| 16 | 11 / 11 | 17 / 17 | 16 | 285 / 1,799 | 313 / 2,215 |
| 128 | 11 / 11 | 17 / 17 | 16 | 397 / 12,663 | 593 / 14,757 |
| 512 | 11 / 11 | 17 / 17 | 16 | 781 / 49,911 | 1,553 / 57,765 |
| 1024 | 11 / 11 | 17 / 17 | 16 | 1,293 / 99,575 | 2,833 / 115,109 |

For these scopes, statement counts are flat for nonempty fleets. The empty/nonempty boundary adds batched inventory/authorization work; yielded rows and VM work grow with camera/collection membership count. This rules out per-camera query growth in this matrix, not every future selector shape. Preserve batched loading and repeat this matrix when adding selectors or contract operations; static collection cardinality, dynamic selector complexity, tag/location depth and many distinct grants need separate scaling coverage before expansion.

## Pre-repair memory and deployment decision

Memory is whole-process Linux RSS/VmHWM, not allocation accounting. Baseline is after API seeding and before discovery warmups; peak RSS is the maximum of five-millisecond concurrent samples and per-principal pre-sample RSS. VmHWM includes all preparation and measured phases. Sampling can miss peaks; allocator retention and server state mean differences cannot be attributed exclusively to currently live snapshot allocations. The table uses uninstrumented processes only.

| Cameras | Baseline RSS MiB | Sampled peak RSS MiB | Observed VmHWM MiB | Minimum three camera arrays per request MiB |
|---:|---:|---:|---:|---:|
| 0 | 94.03 | 97.77 | 97.77 | 0.00 |
| 16 | 131.57 | 149.25 | 149.25 | 1.03 |
| 128 | 141.00 | 181.95 | 181.95 | 8.25 |
| 512 | 178.12 | 314.70 | 314.70 | 32.99 |
| 1024 | 201.74 | 467.53 | 476.72 | 65.98 |

The compiler measured `sizeof(fleet_camera_t)=22,520`: `authorization_snapshot_create` holds raw, runtime and live arrays, requiring `3*N*22,520` bytes. At 1,024 this is 65.98 MiB per request and 263.91 MiB for four simultaneously live snapshots, before context/JSON/SQLite allocations. `fleet_camera_capture_runtime_health` also allocates a configured-maximum-sized metrics buffer for a nonempty fleet. Here `sizeof(stream_metrics_t)=12,192`, so that transient allocation is 11.91 MiB even at 16 disabled cameras with max-streams 1,024. It includes ring-buffer data although discovery needs health observations. The existing global metrics array and seeded server state add baseline cost. No heap profile was taken, so these source-derived quantities explain mechanisms without claiming an exact RSS decomposition.

The 1,024 repeat reached 475.00 MiB observed high water, versus 476.72 MiB in the full run. Empty-policy responses still construct the fleet snapshot and therefore do not avoid its memory cost. This is a concrete limitation for memory-constrained deployment, even though no allocation failures occurred on this host.

**Baseline limitation and follow-up decision:** the integration owner assigned T20a to replace three whole-camera copies with one inventory and compact raw/runtime/live facts. This report preserves the pre-repair baseline. The post-repair build and comparison below address this decision; this section retains the original measurements. Do not claim embedded acceptance from this report. T20a now captures compact health observations and avoids three copies of large fixed-capacity tag/location fields while preserving one-request observation consistency and permission semantics. Remaining follow-up is a compact runtime metrics buffer instead of copying ring-buffer slots, plus target-specific sizing. Establish a target-specific memory/concurrency envelope using the actual `max_streams`, active-media workload and device RAM; no arbitrary MiB or latency ceiling is introduced here. The current architectural limits measured are 1,024 inventory entries and four workers, not a recommendation to run that combination on small hardware.

## Post-repair T20a comparison

The integration owner chose a focused repair before acceptance: one full fleet inventory plus two compact observation records per camera, preserving all seven mutable health fields. The fresh Release build compiled `lightnvr` and `ci-unit-tests` with `BUILD_TESTS=ON`; other feature/compiler flags match the baseline. Native source digest is `68a80fb0bc4578c86d12e08ae399c3805e27c1f2aa6c4e8ca590e787a3f9ed68`, binary SHA-256 `08323d6a472f34bbcbdfcdf397d2ab0438378a1f4437e3c42c7fdb384f53e02b`, manifest `build/eng-api-T20a/evidence.json`.

The current benchmark verifies native file hashes against the build manifest before running. Its compiler size probe includes the snapshot implementation for private type sizes and discards all product function sections; it does not execute a second server. Sizes are 22,520 bytes for inventory, 96 bytes for the two compact observations, and 12,192 bytes for a metrics slot. Camera heap storage is therefore `N*(22,520+96)`: **22.09 MiB per request at 1,024 cameras**, down from 65.98 MiB (66.52% less). A single full-camera stack copy is used during evaluation. The configured-maximum metrics allocation is unchanged.

The full matrix passed with identical sample counts and setup. All 25 principal/fleet combinations preserved operation sets and response sizes. SQL statement and row-count distributions were identical to the baseline; VM-step differences were at most two steps on the regenerated UUID fixtures (all 1,024-camera counts identical). The repair did not add per-camera SQL. Full-run load averages start `[1.58,2.25,1.99]`, end `[1.42,1.93,1.90]`; heavy parallel work was paused again.

| Cameras | Principal | Serial median ms | Serial p95 ms | Four-concurrent p95 ms |
|---:|---|---:|---:|---:|
| 0 | admin | 8.89 | 9.37 | 24.66 |
| 0 | empty | 8.93 | 9.75 | 23.65 |
| 0 | split | 8.89 | 9.42 | 18.49 |
| 0 | mixed | 8.82 | 9.25 | 23.93 |
| 0 | adminReadToken | 3.73 | 4.27 | 17.68 |
| 16 | admin | 10.42 | 10.98 | 31.13 |
| 16 | empty | 9.58 | 10.08 | 23.10 |
| 16 | split | 10.07 | 10.44 | 22.72 |
| 16 | mixed | 10.44 | 11.29 | 21.42 |
| 16 | adminReadToken | 4.43 | 4.98 | 32.80 |
| 128 | admin | 10.27 | 11.48 | 23.81 |
| 128 | empty | 10.29 | 11.24 | 26.78 |
| 128 | split | 10.98 | 12.72 | 23.34 |
| 128 | mixed | 10.56 | 11.94 | 21.16 |
| 128 | adminReadToken | 5.34 | 5.74 | 30.00 |
| 512 | admin | 11.36 | 11.95 | 23.81 |
| 512 | empty | 12.57 | 12.99 | 27.13 |
| 512 | split | 14.84 | 15.40 | 29.25 |
| 512 | mixed | 11.66 | 12.45 | 28.83 |
| 512 | adminReadToken | 9.15 | 10.16 | 28.42 |
| 1024 | admin | 13.07 | 14.21 | 32.96 |
| 1024 | empty | 16.98 | 19.46 | 35.55 |
| 1024 | split | 24.65 | 26.61 | 42.06 |
| 1024 | mixed | 12.89 | 13.30 | 31.97 |
| 1024 | adminReadToken | 18.75 | 20.56 | 41.82 |

| Cameras | Baseline RSS MiB | Sampled peak RSS MiB | Observed VmHWM MiB | Camera storage per request MiB |
|---:|---:|---:|---:|---:|
| 0 | 94.13 | 97.87 | 97.88 | 0.00 |
| 16 | 131.61 | 146.57 | 146.57 | 0.35 |
| 128 | 140.93 | 159.99 | 159.99 | 2.76 |
| 512 | 170.89 | 205.26 | 205.26 | 11.04 |
| 1024 | 201.88 | 264.22 | 264.22 | 22.09 |

At 1,024 cameras, observed high water fell from **476.72 to 264.22 MiB** (44.57% lower), with similar pre-discovery baselines (201.74 vs 201.88 MiB). This whole-process result supports the source-derived copying reduction; it is not an exact allocator attribution. The unchanged metrics-buffer allocation and actual device/media concurrency remain deployment considerations. No unmeasured memory or latency SLO is implied.

The documented 16/1,024 focused repeat also passed. At 1,024 its serial medians were 12.74–24.15 ms, serial p95s 13.38–25.02 ms, concurrent p95s 27.26–46.74 ms, and observed high water 264.39 MiB. At 16 its serial medians were 4.62–10.34 ms and concurrent p95s 21.10–35.75 ms. Repeat load averages start `[1.42,1.88,1.88]`, end `[1.39,1.83,1.87]`. Small-sample scheduling variation remains visible; these figures support the bounded comparison rather than a production SLO.

Proposed pilot disposition: accept the measured workstation costs with the agreed explorer-asset criterion and retain target-device sizing as a separate qualification. The material copying cost is measured and reduced; no disproportionate query growth or unexplained asset increase remains in this declared matrix. This does not replace T21's fresh security, browser and regression integration checks.

## Retained evidence and checks

- Pre-repair full run: `/tmp/lightnvr-projection-benchmark-rfRCU9/result.json`; focused repeat: `/tmp/lightnvr-projection-benchmark-j1ewed/result.json`. Both `status=passed`; each retains fixture paths, server logs, harness/build hashes and raw samples. Generated trace source, shared library, compiler size probe and per-size JSONL execution counts remain in the full-run directory.
- Post-repair full run: `/tmp/lightnvr-projection-benchmark-X0XdYp/result.json`; focused repeat: `/tmp/lightnvr-projection-benchmark-YuKeSb/result.json`, both passed. Harness SHA-256 `4e840ceeda0702358c993541bce9205da79e4ce9830e6c748518c927c5d00112`, fixture `31c2aa2409e2af57f5ca9a1f84f92b435eed1aed8319e35ae5935083cedeb6ea`; exact harness and fixture copies are retained alongside reports. Source-derived memory accounting/provenance changed; request/sampling methodology matches the original harness (`4e8bc1a8581dce7e0ae72f790b11d9b78f4ca87cdbb362c5335d05df64f5f9cf`).
- Trace smoke: `/tmp/lightnvr-projection-benchmark-s5jx9N/result.json` (sizes 0/16, instrumented only).
- Native configure/build logs: `/tmp/lightnvr-T20-configure.log`, `/tmp/lightnvr-T20-build.log`; post-repair `/tmp/lightnvr-T20a-configure.log`, `/tmp/lightnvr-T20a-build.log`. Full/repeat output logs use `/tmp/lightnvr-T20a-benchmark{,-repeat}.log`.
- Baseline: `/tmp/lightnvr-eng-baseline-{default,modern}.json`; final: `/tmp/lightnvr-eng-current-{default,modern}.json`; restored: `/tmp/lightnvr-T20-restored-default.json`.
- Chromium captures: `/tmp/lightnvr-T20-final-network-{default,modern}.json`, produced by the retained T15 script `/tmp/lightnvr-T15-network.cjs` against each final build (`node /tmp/lightnvr-T15-network.cjs default /tmp/lightnvr-T20-final-network-default.json`, likewise modern). The script predates T20; source and captures are local review artifacts, not a new committed browser harness.
- Attribution: `/tmp/lightnvr-T20-generated-attribution.json`, `/tmp/lightnvr-T15-attribution.json`, `/tmp/lightnvr-T14-before-explorer-default.json`; the T14 source-digest caveat above applies.
- Cross-run/source/asset audit: `/tmp/lightnvr-T20-final-audit.json` passed; 25 operation/response-size comparisons, identical statement/row counts, at most two VM-step difference, matching harness/fixture hashes and restored/default asset isolation.
- `npm run contracts:typecheck`, benchmark syntax check and fixture `maxStreams` bounds checks pass. The fixture extension accepts integer 1..1024, defaults to 8 and preserves T17's isolated web-root support.

These `/tmp` and ignored build artifacts are local evidence, not permanent repository attachments. Archive them with an integration/CI run before relying on them for a release. The commands and harness remain in the repository for fresh reproduction.
