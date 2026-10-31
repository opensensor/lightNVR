# Running tests

Use Node 24 from [.nvmrc](../.nvmrc), install the root and web lockfiles, and
follow [BUILD.md](BUILD.md) for native dependencies. Build the web assets before
server-backed tests. Playwright UI runs also require Chromium:

```sh
npm ci
npm ci --prefix web
npm --prefix web run build
npx playwright install chromium
```

## Discover tests without starting a server

```sh
npm test -- --list
npm run test:api -- --list
npm run test:ui -- --list
npm run test:go2rtc -- --list
```

Discovery does not build or inspect a native binary, acquire a runtime lease,
create a temporary server directory, or run global setup. It writes a unique
discovery report under `build/eng-artifacts/<task>/`. Empty selections and
invalid filters fail.

`npm test` selects all existing projects. API/UI scripts retain their `@api` and
`@ui` tag filters. The go2rtc script selects `@go2rtc` tests in the API project,
where the current `go2rtc.api.spec.ts` actually lives. The legacy project named
`go2rtc` currently matches no files; selecting it directly correctly fails.

## Run integration tests

```sh
npm run test:api
npm run test:ui -- --shard 1/2
npm run test:go2rtc
```

The scripts invoke the owned integration runner. Without `--binary`, it builds
a Release server with go2rtc enabled and SOD/LiteRT/MQTT disabled in a task-scoped
build directory. Initialize the `go2rtc` and FAAC sources and build
`go2rtc/go2rtc` before running media fixtures. This profile does not qualify
optional features. Supply an explicit appropriately built binary when testing
another feature profile:

```sh
node scripts/quality/run-integration.mjs \
  --task local-api --project api --grep @api \
  --binary build/bin/lightnvr
```

Use repeatable `--file` options for exact spec files. Paths must resolve inside
`tests/integration`; arbitrary Playwright configs, positional patterns, worker
counts, production URLs and environment-provided server configs are rejected.
Both `--shard 1/2` and `--shard=1/2` work. `--list` can be added to any selection
without needing the referenced binary to exist.

```sh
node scripts/quality/run-integration.mjs \
  --task local-streams --project ui \
  --file tests/integration/specs/streams.ui.spec.ts --list
```

The fixed test ports are 18080 (LightNVR) and 11984/18554/18555 (go2rtc).
Existing listeners cause failure; no process is adopted or killed by name.
Native and integration runs share an exclusive runtime lease, so run these
commands serially. A forced termination can leave a stale lease; inspect the
retained run/process records and confirm that no run is active before recovering
it. The runner never silently steals a lock.

## Health and MQTT lane

Use the bounded `health` configuration and a binary built with `ENABLE_MQTT=ON`
and `ENABLE_GO2RTC=OFF`. The profile keeps MQTT enabled, disables go2rtc, and
retains the existing health collector settings. An explicit binary is required
for execution so the default MQTT-disabled build cannot substitute for it.

```sh
node scripts/quality/run-integration.mjs \
  --task local-health --project all --config-profile health \
  --binary build/health-ci/bin/lightnvr \
  --file tests/integration/specs/system-health.api.spec.ts \
  --file tests/integration/specs/system-health-mqtt.api.spec.ts \
  --file tests/integration/specs/system-health.ui.spec.ts \
  --file tests/integration/specs/system-health-settings.ui.spec.ts
```

This API/UI lane intentionally uses an unreachable local broker at port 18884
for delivery-observability tests. The runner refuses to use an existing broker
on that port. The separate real-broker presence script remains a separate lane;
its broker-visible assertions have not been replaced by this configuration.

## Reports and cleanup

The runner prints the artifact directory. It contains discovery/results JSON,
HTML report, Playwright output, per-test screenshots/traces, process cleanup
results, and an allowlisted copy of the owned server configuration/logs under
`runtime/`. Database and recording directories are retained in the unique
`/tmp/lightnvr-eng-*` run directory for local diagnosis, but are not broadly
uploaded. No unrelated `/tmp` directories are collected or removed.

Open a specific report using its printed path:

```sh
npm run test:report -- build/eng-artifacts/local-api/integration-RUN_ID/html
```

A few older explicit screenshot calls still write to repository `test-results/`;
the serialized runner and CI's separate job checkouts prevent collisions, and
CI retains those screenshots separately. Normal Playwright artifacts use the
unique per-run directory.

Cleanup validates the owned executable, command line and kernel start time and
uses Linux pidfds. The platform requires Linux `/proc` and Python 3 with pidfd
support. A 60-second graceful shutdown timeout or leftover child processes
fails the run even when test bodies pass. There is no broad `pkill`, raw PID-file
fallback, or shared temporary-directory deletion.

## Native and static checks

```sh
python3 scripts/quality/run-native.py \
  --task local-native --regex '^test_system_health_proc$' --sanitizer asan-ubsan
npm run typecheck:tests
npm --prefix web run typecheck
npm --prefix web test -- --runInBand
python3 -m unittest discover -s tests/quality -p test_runners.py
```

The native runner defaults to the lean ASan/UBSan profile. `--profile integration`
enables go2rtc; `--profile full` enables SOD/LiteRT/go2rtc/MQTT. `--sanitizer none`
selects Release. It checks system FFmpeg header/runtime ABI agreement before
building and fails if no CTest test matches. Test registration fragments belong
under `tests/unit/cmake/eng/`.

Passing runner tests or discovery does not establish full application acceptance.
The [T13 evidence](../quality/evidence/T13.md) records real API and lifecycle
failures separately from harness success. Physical qualification and pending
security exception reviews remain separate acceptance gates.
