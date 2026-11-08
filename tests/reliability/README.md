# Bounded recovery qualification

These scripts use private temporary directories, synthetic video and loopback
ports. They do not access production cameras, configuration, databases or
recordings. Every run retains its complete evidence tree, including failures.
They are workstation qualifications with declared test deadlines, not embedded
production SLOs.

## Interrupted recording (T25)

Build the pinned go2rtc source and a fresh managed-integration native binary:

```sh
git submodule update --init go2rtc
(cd go2rtc && go build -trimpath -ldflags '-s -w' -o go2rtc .)
cmake -S . -B build/eng-api-T25 -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON \
  -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=ON \
  -DGO2RTC_BINARY_PATH="$PWD/go2rtc/go2rtc" -DENABLE_MQTT=OFF -DENABLE_SSL=ON
cmake --build build/eng-api-T25 --target lightnvr test_storage_reconcile_incomplete -j6
python3 tests/reliability/interrupted-recording.py \
  --binary build/eng-api-T25/bin/lightnvr --go2rtc ./go2rtc/go2rtc
ctest --test-dir build/eng-api-T25 --output-on-failure --no-tests=error \
  -R '^test_storage_reconcile_incomplete$'
```

Prerequisites: native dependencies from `docs/BUILD.md`, libyaml, Go compatible
with `go2rtc/go.mod`, Python 3.11+, FFmpeg/ffprobe with libx264 and Linux `/proc`.
No Python packages or Node process are required. Debug enables ASan/UBSan.
During shared-workspace parallel execution, acquire the coordinator's native
build lease first: CMake generates shared `web/js/version.js`.

The harness starts an HTTP MPEG-TS source backed by owned FFmpeg processes.
The recorder supervises its own real go2rtc child; the harness verifies parent
PID and its private go2rtc configuration path. This is not a source-only stand-in
for managed integration. All four recorder/go2rtc ports are reserved before
startup. Successful recorder bind is checked in that generation's own log before
HTTP bootstrap. Collisions fail the bounded startup rather than attaching to a
foreign service. Crash cleanup uses captured PID/start-time identities of only
the managed children, including go2rtc's separate process group leader.

The scenario records at least two completed segments plus a nonempty active
segment, protects both a completed and active recording through the API, and kills only the
owned recorder with SIGKILL. It preserves completed footage and the actual
partial file. After the killed recorder and managed child exit, it disables
only its fixture camera offline so recovery itself cannot create new rows.

**Fresh recovery is required:** the default run never ages recording timestamps.
It observes immediate completion for up to 60 seconds and reports the full
scenario **blocked (exit 2)** if crashed rows remain incomplete. T25 originally
reproduced a startup-only scan that excluded rows newer than 3600 seconds: a
quick restart could leave a crashed row in progress indefinitely. T25a replaces
that production startup call with a scan bounded by IDs captured synchronously
before new writers start. The ordinary manual stale API keeps its age filter.

`--diagnostic-age` preserves the original investigation method separately: after
an immediate-recovery failure, it subtracts 7200 seconds from only this fixture's
stopped, incomplete rows' `start_time`, then verifies eligible reconciliation.
That optional run remains blocked even if its aged assertions pass. It does not
alter media, file mtime, completion status, playback flags or end timestamps.
It proves neither immediate recovery nor natural elapsed-time scheduling; aged
rows' catalog duration cannot be compared to their short media duration.

After restart, surviving files must remain listed. Decodable files must serve
playback successfully; undecodable files must carry a nonempty `playback_issue`
and return the existing 422 interrupted-media explanation. A stopped writer is
not proof of playable media. The harness preserves file hashes, protected state,
recording IDs and the reconciled metadata across another restart, and checks
SQLite integrity. It does not delete evidence to make a check pass.

Bounds are declared before running: startup/reconciliation 60 seconds, request
10 seconds, graceful shutdown 30 seconds, owned-child cleanup 5 seconds,
ffprobe/decode 30 seconds. Synthetic segments target six seconds with a one-second
GOP at 10fps. Non-injected completed rows' catalog duration may differ from
media by at most 2.2 seconds (one GOP plus integer-second metadata rounding).
Packet DTS must be monotonic. These are fixture bounds, not service guarantees.

`--output PATH` accepts only a new directory. By default a new
`/tmp/lightnvr-recovery-*` tree is retained. `result.json` records the source
manifest/digest, HEAD, binary and go2rtc hashes, bounds, before/after database
rows, API status and media observations. FFmpeg/ffprobe command/stdout/stderr,
recorder generations, go2rtc logs, source logs and the SQLite database remain
alongside it. Schema: `results-schema.json`. Exit 1 means a failed assertion or
cleanup; exit 2 means the recorded immediate-recovery gap remains even if the
separate eligible-reconciliation assertions pass. A full pass requires actual
immediate recovery as well as the media/protection/idempotence assertions.

### This workstation's native dependency correction

Initial qualification exposed stale FFmpeg 61 headers and `.pc` files under
`/usr/local`, with only system FFmpeg 62 shared libraries present. The existing
reconciliation CTest failed in its MPEG4 fixture encoder before entering its
tests. Rebuild with coherent headers/libraries; do not interpret that mixed-ABI
run as a product regression or accept its media results. The verified probe
reported avcodec 62/62, avformat 62/62 and avutil 60/60 (header/runtime majors).

On this x86_64 Debian-layout host the corrected configure used `cmake --fresh`
with the above feature flags plus:

```sh
export PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig
export PKG_CONFIG_PATH="$PWD/build/eng-api-T25-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig"
# Add to cmake --fresh:
# '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
# '-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
```

The PKG_CONFIG_PATH entry is only this task's extracted libyaml 0.2.5 development
package/runtime, obtained with `apt-get download libyaml-dev libyaml-0-2` and
`dpkg-deb -x` under `build/eng-api-T25-deps`, with the extracted `.pc` prefix
changed to that sysroot. No host packages were changed. Hosts with a coherent
installed toolchain do not need these workstation-specific paths.

`--build-manifest build/eng-api-T25/build-evidence-system-ffmpeg.json` supplies
the source manifest captured at native build completion. The harness verifies
both native and go2rtc binary hashes before use and records its own current
script hash separately, avoiding attribution of subsequent concurrent source
edits to an older executable. The fixture's `source_evidence()` helper produces
this manifest; without the option, it records the current workspace directly.

### Recorded results and startup-boundary reasoning

At HEAD `dea6094a` plus the T25a working-tree changes, the coherent Debug build
passed all **25** native reconciliation cases under ASan/UBSan (19 existing and
six new), with zero ignored tests. CTest log: `/tmp/lightnvr-T25a-ctest.log`.
The full fresh crash scenario passed at `/tmp/lightnvr-recovery-b0yo0x2v/result.json`
with `stale_age_injected_seconds=0`: two decodable recordings, one retained
`invalid_media` crash file returning 422, completed and interrupted protection
preserved, SQLite integrity `ok`, and repeated restart unchanged. Restart took
2.454 seconds; the interrupted row was already reconciled when HTTP became ready.
This sample is evidence for the declared fixture, not a production latency bound.
The report validates against `results-schema.json` (JSON Schema 2020-12).

Build manifest: `build/eng-api-T25/build-evidence-T25a.json`; native SHA-256
`6671d5250a44ead0f6ac4b01086d74a2f0140b32ed62a18dce9df68f78a21ae2`, source
digest `8b99f58f2f01d73f9415f06206415ed1b1e39864ac56ef9760ebe3d3423b284f`.
The pinned go2rtc revision was `6986a1b3f122651b318a957534da44bb4c65e271`,
binary SHA-256 `8a08d35b3e12550489d6430a44f99e7a40f56ebd166230a1edb43d8a2c36e5d4`.
Original coherent pre-fix reproduction is retained at
`/tmp/lightnvr-recovery-570eu21o/result.json`: exit 2 after 60 seconds of continued
409/in-progress responses, with the age-only phase passing separately. Earlier
mixed-ABI diagnostics are retained but are not accepted native qualification.

In `src/core/main.c`, storage initialization occurs after database migration and
metadata bootstrap (around line 856), before stream manager initialization
(around 977), HTTP server start (around 1151), and recording startup. The ID
boundary is captured inside `init_storage_manager` before creating its worker.
The background page helper receives that immutable value and cannot recapture
it on page one. A failed capture skips reconciliation; zero means empty.
Migration `db/migrations/0001_initial_schema.sql` declares recording IDs
`INTEGER PRIMARY KEY AUTOINCREMENT`; the sole supported recording insert in
`db_recordings.c` supplies no ID. Tests delete the captured highest row and
insert through the production API to verify non-reuse. They also cover recent
protected damage, post-boundary rows with old timestamps, empty/failed captures,
and paging beyond retained rows. Existing protection, transfer and unavailable
storage guards remain in the shared reconciliation/deletion path.

Separate limitation discovered during review: recording paths have second-level
timestamp names, and existing writer open paths can overwrite an existing name.
This test does not qualify clock rollback or same-path reuse; ID-boundary safety
does not prove file identity when a separate writer reuses that pathname. Writer
collision handling was reported as a separate follow-up, not changed in T25a.

## Reusable fixture interface

Load `interrupted-recording.py` through `importlib.util.spec_from_file_location`
(the hyphenated filename is intentional to match the plan). T27/T28 may reuse:

- `Recorder(binary, go2rtc, directory)`: the directory must already exist.
  `start()`, `stop(crash=False)`, `json(route, method='GET', body=None)`,
  `request(...)` returning `(status, headers, bytes)`, and `rows()`.
- `SyntheticSource(directory)`: `.url`, `.pause()`, `.resume()`, `.stop()`; owns
  only its own producers. `pause()` terminates active producer processes and
  returns their PIDs; all new requests receive 503 until `resume()` reopens the
  same URL. This source-only fault leaves managed go2rtc running. Call lifecycle
  methods serially; use actual HTTP/server operations for concurrency tests.
- `wait_for(label, predicate, timeout)`, `inspect_media(path, directory, name)`,
  `source_evidence(binary, go2rtc)`.

The caller must always stop both source and recorder in `finally`. Successful
cleanup never removes the evidence directory. Avoid repeated process polling
without a deadline. All fixture database mutations must happen after the owned
recorder is stopped and be recorded explicitly in the scenario report.

## Portable upgrade preparation and manual CI (T28a)

`prepare-upgrade.py` prepares binaries and provenance; `upgrade.py` creates the
historical database by running the real old recorder. The dated
`upgrade-fixture.json` remains the original workstation evidence, not a portable
build recipe. Every preparation and scenario needs a new output directory.

Build the current managed Debug recorder using the command above and build
the pinned go2rtc submodule, which includes the earlier relay repair. Do not
reapply that historical patch to current source. Capture its manifest immediately
after compilation, before changing any source or executables. With a coherent
native toolchain, prepare the old release and run the upgrade:

```sh
npm ci --no-audit --no-fund
mkdir -p build/reliability-local web/dist
python3 tests/reliability/prepare-upgrade.py capture-current \
  --binary build/eng-api-T25/bin/lightnvr --go2rtc go2rtc/go2rtc \
  --output build/reliability-local/current-build.json
python3 tests/reliability/prepare-upgrade.py prepare \
  --output build/reliability-local/prior-build --jobs 2 \
  --current-binary build/eng-api-T25/bin/lightnvr \
  --current-go2rtc go2rtc/go2rtc \
  --current-build-manifest build/reliability-local/current-build.json
python3 tests/reliability/upgrade.py \
  --binary build/eng-api-T25/bin/lightnvr \
  --fixture-manifest build/reliability-local/prior-build/upgrade-manifest.json \
  --output build/reliability-local/upgrade
python3 tests/reliability/prepare-upgrade.py validate-report --kind upgrade \
  --report build/reliability-local/upgrade/result.json
```

Preparation clones the local repository into its new private directory, checks
that tag `0.45.3` resolves to `c82d6ed17c7e8ec9038bf4ebcdaa96d3b8cd8163`,
checks ancestry, and fetches the release's own pinned Go and FAAC submodules.
The current checkout must contain that tag and history (`fetch-depth: 0` in CI).
Historical Go is **never patched** with current changes. The old native build
uses Debug/ASan/UBSan and managed Go; SOD, LiteRT and MQTT are disabled. The new
manifest records actual binary hashes, a source manifest walked from that
historical checkout, dependency pins, commands and CMake flags. No old database
or migration table is fabricated. The current native and Go hashes must match
the supplied current build manifest, and managed Go must be enabled.

For the workstation correction above, pass explicit
`--cmake-arg='-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu'` and the
corresponding CXX argument to `prepare`, with the stated pkg-config environment.
These flags are recorded; no host-specific flags or executable hashes are built
into the preparer. The default builds an isolated historical checkout. An
existing qualified checkout can be reused only explicitly, for example:

```sh
python3 tests/reliability/prepare-upgrade.py prepare \
  --output build/reliability-local/verified-prior \
  --current-binary build/eng-api-T28/bin/lightnvr \
  --current-go2rtc go2rtc/go2rtc \
  --current-build-manifest build/eng-api-T28/evidence.json \
  --reuse-checkout build/eng-api-T28-prior \
  --reuse-manifest tests/reliability/upgrade-fixture.json
```

Reuse fails if pins, recorded source-file hashes, aggregate source digest,
executable hashes or clean dependency/source checks differ. Only the generated
historical `web/js/version.js` and the recorded native build directory are
allowed exceptions. Its manifest says `verified-existing-build`; original build
commands/environment are retained separately from tools observed during
preparation. It does not claim to have recompiled those executables.

The manual workflow `.github/workflows/reliability-qualification.yml` runs only
through **workflow_dispatch**. It adds no push/PR or branch-protection check.
It installs coherent Debian sid dependencies, Node 24 and Go matching `go.mod`,
then builds and runs as an unprivileged user. Its private `TMPDIR` isolates
storage-fault results. Current Go patch tests and the native reconciliation test
run before interrupted-recording, storage-fault, reconnect/lifecycle and genuine
historical-upgrade scenarios. There is no separate UI qualification in this job.

The three shared reports must validate against `results-schema.json` using
repository-pinned Ajv 2020; storage uses its existing separate result format and
must contain all four passed fault cases and boundary self-tests. All four
reports must belong to the declared current native binary. Missing, ambiguous,
failed or malformed reports fail summary generation and the job. Node and root
`npm ci` are needed for report validation, although the media harnesses themselves
use the Python standard library.

Only `build/reliability-public/status.json` is uploaded: enumerated scenario/status,
validated provenance hashes/pins and counts. Raw reports, free-form errors,
databases, configurations, media, logs and traces remain private and are never
artifact-upload inputs. A failed job still attempts this sanitized summary;
missing summary files fail the upload. Full local runs retain their private
diagnostics for inspection. Hosted execution has not yet been observed; local
preparer/upgrade evidence is recorded in `docs/api/contracts/upgrade-evidence.md`.

## Fresh post-merge local evidence

The [2026-11-07 qualification](../../docs/validation/recovery-qualification-2026-11-07.md)
repeats all four scenarios on one current Release binary and extends lifecycle
to ten cycles per phase. Named hardware and long-duration acceptance remain open.

## Observation evidence crash boundaries (INT_01 L11, A6)

`tests/unit/test_observation_crash_boundaries.c` forks a process that runs the
real evidence worker, relay, router and outbox on a migrated database, SIGKILLs
it at one persistence boundary (after the batch insert, after the tmp write,
after rename before commit, after commit before relay, after the outbox step
before the publication mark, and after the mark before delivery), restarts the
`main.c` sequence (reconcile -> evidence -> relay) in a second process and
asserts that no ready asset points at an unfinished file and that every
committed change reaches the outbox exactly once. The kill points are stage
hooks (`observation_evidence_set_stage_hook`, `observation_relay_set_stage_hook`)
that only linked test code can install; no configuration, environment variable
or API reaches them.

```sh
make -C <build> test_observation_crash_boundaries && <build>/bin/test_observation_crash_boundaries
```

Binary-level restarts (SIGKILL with committed-but-unrelayed changes, relayed but
unacknowledged MQTT delivery, DB restore, broker outage with 100 observations)
are Playwright contracts in `tests/contracts/observationsReliability.api.spec.ts`;
`tests/contracts/observationsPipeline.api.spec.ts` drives a real detection
stream into evidence and MQTT. Both need `LIGHTNVR_CONTRACT_MQTT=1`.
