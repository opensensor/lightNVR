# Historical upgrade qualification

T28 creates a synthetic installation with the actual **0.45.3** binary and
upgrades a copy with current source. The original installation remains available
for recovery inspection, including after a failed qualification. This establishes
one forward upgrade path on this workstation; it does not claim downgrade
compatibility or physical-device qualification.

## Historical source and builds

The latest available local 0.45.x tag preceding the execution revision was
`0.45.3`, resolving to `c82d6ed17c7e8ec9038bf4ebcdaa96d3b8cd8163`, dated
2026-11-01T08:14:53-05:00. `git merge-base --is-ancestor` confirmed it precedes
execution HEAD `dea6094a731c86debb03cc1c75d0251ab3b96cec`. The other available
0.45.x tags were 0.45.0, 0.45.1 and 0.45.2.

The isolated checkout is `build/eng-api-T28-prior`; it retains the release's own
source, embedded migrations and pinned dependencies:

| Dependency | Historical revision |
| --- | --- |
| go2rtc | `6986a1b3f122651b318a957534da44bb4c65e271` |
| FAAC | `eb4928a76fd075f9bab20e8195df84f937da289b` |

FAAC's historical revision differs from the current checkout. It was fetched
explicitly from the upstream repository; current FAAC was not substituted.
LiteRT and SOD were disabled. The historical Go checkout is clean and does not
include T27a's response-body repair. Current Go includes that repair, captured
with its diff and executable hash in the current build manifest.

Source acquisition used:

```sh
git worktree add --detach build/eng-api-T28-prior '0.45.3^{commit}'
git clone --no-hardlinks --no-checkout go2rtc build/eng-api-T28-prior/go2rtc
git -C build/eng-api-T28-prior/go2rtc checkout --detach 6986a1b3f122651b318a957534da44bb4c65e271
git clone --no-hardlinks --no-checkout third_party/faac build/eng-api-T28-prior/third_party/faac
git -C build/eng-api-T28-prior/third_party/faac fetch https://github.com/FreewareAdvancedAudio/faac.git eb4928a76fd075f9bab20e8195df84f937da289b
git -C build/eng-api-T28-prior/third_party/faac checkout --detach eb4928a76fd075f9bab20e8195df84f937da289b
(cd build/eng-api-T28-prior/go2rtc && go build -trimpath -ldflags '-s -w' -o go2rtc .)
```

These commands require new task-owned directories; do not remove an existing
evidence checkout to reuse its name. Exact historical configure/build commands
and dependency paths are also machine-readable in
`tests/reliability/upgrade-fixture.json`.

Both native builds use coherent system FFmpeg headers/libraries, with the
workstation-specific correction described in `tests/reliability/README.md`:

```sh
PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig \
PKG_CONFIG_PATH="$PWD/build/eng-api-T25-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig" \
cmake --fresh -S build/eng-api-T28-prior -B build/eng-api-T28-prior/native \
  -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=OFF -DENABLE_SOD=OFF -DENABLE_LITERT=OFF \
  -DENABLE_GO2RTC=ON -DGO2RTC_BINARY_PATH="$PWD/build/eng-api-T28-prior/go2rtc/go2rtc" \
  -DENABLE_MQTT=OFF -DENABLE_SSL=ON \
  '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu' \
  '-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
cmake --build build/eng-api-T28-prior/native --target lightnvr -j6

PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig \
PKG_CONFIG_PATH="$PWD/build/eng-api-T25-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig" \
cmake --fresh -S . -B build/eng-api-T28 -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON \
  -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=ON \
  -DGO2RTC_BINARY_PATH="$PWD/go2rtc/go2rtc" -DENABLE_MQTT=OFF -DENABLE_SSL=ON \
  '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu' \
  '-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
cmake --build build/eng-api-T28 --target lightnvr -j4
```

Debug enables ASan/UBSan. Build logs are
`/tmp/lightnvr-T28-{prior,current}-{configure,build}.log`. The historical source
manifest is `build/eng-api-T28-prior/native/source-evidence.json`; current source,
flags, environment and Go diff are in `build/eng-api-T28/evidence.json`.

| Executable | SHA-256 |
| --- | --- |
| Historical native | `118c0ac764306eddd051a1447b2e1961a164c66662f627e90cf67be8f017f5da` |
| Current native | `d55d3db3d42453e36db8f5a91d8a0fd36af66c33c6677ae72b2c33de0159b822` |
| Historical go2rtc | `8a08d35b3e12550489d6430a44f99e7a40f56ebd166230a1edb43d8a2c36e5d4` |
| Current go2rtc | `1784534bdaf809a11eb0bb4e38cbc354cccf0273c6e3f7e86faf41f5ceabab0b` |

## Fixture and assertions

```sh
python3 tests/reliability/upgrade.py \
  --binary build/eng-api-T28/bin/lightnvr \
  --fixture-manifest tests/reliability/upgrade-fixture.json
```

The manifest pins the historical binary and dependencies; an unavailable or
mismatched build fails qualification instead of silently substituting current
migrations. Build that source before running. Every invocation creates a new
retained `/tmp/lightnvr-upgrade-*` directory. `--output` also requires a new path.

The historical process initializes its own database through migration **0089**,
records the owned FFmpeg source through its managed go2rtc child, and protects a
completed recording through the API. It creates a second disabled camera, a
viewer scoped to the first camera and a read-only scoped token owned by admin.
The admin credential is synthetic and rotated through the normal password API.

Before upgrade, the harness checks actual viewer/token allow and deny responses,
anonymous rejection, admin access, recording-list visibility, HTTP playback,
ffprobe packet timestamps and FFmpeg decode. It disables the fixture camera
through the API and gracefully stops the historical process and owned source.
The original database, configuration, media, sidecars and logs are then hashed
and copied. No current binary has opened the original.

The only offline copy mutations relocate absolute paths in `recordings.file_path`
and `storage_targets.root_path`, plus the copied configuration's private directory,
four reserved ports and Go executable path. The result records all affected
tables/columns/counts. Schema definitions and migration rows must remain identical
before the first current startup. The full original tree must retain its hashes
after the test and after cleanup, even if the upgrade assertions fail.

Current startup must genuinely apply **0090** (external-motion index) and **0091**
(`recordings.playback_issue`). Two current starts verify:

- SQLite integrity `ok`, no foreign-key violations, expected index/column and
  migration history.
- Existing viewer and token credentials preserve camera scope; forbidden PUTs
  cannot change the camera's enabled state. Current discovery advertises reads
  without the update operation for either principal.
- Protected recording ID, relocated path, protection, file hash and playable
  contents survive. The recording remains listed and serves HTTP 200.
- Persistent schema definitions, migration rows, all recording metadata and the
  relocated configuration are identical after the second current start.

SQLite's `PRAGMA schema_version` is a schema-cache invalidation counter, not the
application migration version. Existing `db_core.c` creates and drops
`test_table` for its startup write probe, so the counter increases by two on a
repeat start. The harness records that counter and explicitly checks the known
two-step increase while comparing persistent definitions/history exactly.

Bounds remain startup/media acquisition 60 seconds, request 10 seconds and
graceful shutdown 30 seconds, with finite media probes and owned-child cleanup
inherited from T25. These are fixture bounds, not production latency guarantees.
The compatibility subclass only reads historical recording columns; it does not
alter the historical schema. A remaining WAL is honored and copied, never
ignored using SQLite's immutable-reader option.

## Diagnostic runs

`/tmp/lightnvr-upgrade-0542h9sl/result.json` exposed an incorrect fixture assumption
that a gracefully stopped database must have an empty WAL. The harness was
corrected to close its readers explicitly and preserve/read the actual WAL.

`/tmp/lightnvr-upgrade-2599znml/result.json` passed migration, authorization and
playback but rejected the known startup schema-counter increment. Inspection
showed identical persistent definitions, migration rows and recording metadata;
the original full-tree hashes remained unchanged despite the failed assertion.
This is retained failure/recovery evidence, not a product migration defect.

The next run, `/tmp/lightnvr-upgrade-qlfzmbna/result.json`, passed the scenario
assertions. It upgraded two genuine recording rows and preserved a protected
278,388-byte, 6.1-second video with 61 monotonically timestamped packets. Current
starts took 2.456 / 2.455 seconds and shutdowns 3.920 / 3.919 seconds. Both
integrity checks passed and the original full tree remained unchanged. A final
run additionally asserts that forbidden PUTs leave enabled state unchanged.

The strengthened run `/tmp/lightnvr-upgrade-z5x4k8cf/result.json`,
using harness SHA-256
`f96fb55c3a1f090566636c02ebd81a2df30aaa2b9a5d21cfa4814c9eace22632`.
All 89 historical migration rows are retained and the two new migrations bring
the count to 91. The strengthened denied-write checks pass before and after
upgrade for both viewer and token. Both current starts preserve the protected
video SHA-256
`e54bc01f59e3cb1757b05d7f50de2fd0377d53d306eee62148f62138fed68940`.
The original installation remains byte-for-byte unchanged, including after
cleanup. No sanitizer finding or shutdown timeout was reported. Coordinator
review subsequently found that its report lacked the shared result schema's
required `qualification` field and used `current_build` instead of the required
`evidence` field. These earlier reports are retained unchanged as diagnostic
metadata failures; their `passed` status is not final acceptance. The harness
now emits the canonical evidence field and an explicit bounded qualification.

**Final qualification and report validation passed:**
`/tmp/lightnvr-upgrade-zdegojsx/result.json`, using harness SHA-256
`ecedb31b8a17821c05a4094f0aed40cc2467bcf6e12a665abd67a41012874db8`.
The complete scenario ran again in a new directory. Current starts took
2.458 / 2.456 seconds and shutdowns 3.870 / 3.869 seconds. Migration, authorization,
denied-write side-effect, protected-media, restart-idempotence and untouched-original
assertions all passed again. Ajv 2020 validated the actual final result against
`tests/reliability/results-schema.json`, including the required evidence and
qualification fields; log `/tmp/lightnvr-T28-result-schema.log`.

Reproduce that report check with the pinned root tooling:

```sh
node --input-type=module - /tmp/lightnvr-upgrade-zdegojsx/result.json <<'JS'
import fs from 'node:fs';
import Ajv2020 from 'ajv/dist/2020.js';
const schema = JSON.parse(fs.readFileSync('tests/reliability/results-schema.json', 'utf8'));
const result = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
const validate = new Ajv2020({strict: false, allErrors: true}).compile(schema);
if (!validate(result)) { console.error(validate.errors); process.exit(1); }
console.log('Result schema valid:', result.status);
JS
```

Current native source digest:
`44f7e5397c9ddd2c06dca140c45a794219b943d77aeba2f725fda70bff30b132`.
The build manifest captures source at compilation; each run separately records
its harness hash, so later fixture refinements are not attributed to the native
executable. The qualification changed no production source or migration files.


## Appendix: portable preparation and manual qualification (T28a)

The original evidence above and `upgrade-fixture.json` remain unchanged. The new
`tests/reliability/prepare-upgrade.py` supports a clean pinned historical build
and explicit verified reuse. Local verification used the existing T28 managed
current binary and its captured build manifest; it does not attribute later
working-tree changes to that executable. Fresh integrated T29 qualification is
separate.

The actual local preparation and upgrade commands were:

```sh
python3 tests/reliability/prepare-upgrade.py prepare \
  --output build/eng-api-T28a-prepared-final \
  --current-binary build/eng-api-T28/bin/lightnvr \
  --current-go2rtc go2rtc/go2rtc \
  --current-build-manifest build/eng-api-T28/evidence.json \
  --reuse-checkout build/eng-api-T28-prior \
  --reuse-manifest tests/reliability/upgrade-fixture.json
python3 tests/reliability/upgrade.py \
  --binary build/eng-api-T28/bin/lightnvr \
  --fixture-manifest build/eng-api-T28a-prepared-final/upgrade-manifest.json \
  --output /tmp/lightnvr-T28a-upgrade
python3 tests/reliability/prepare-upgrade.py validate-report --kind upgrade \
  --report /tmp/lightnvr-T28a-upgrade/result.json
```

**Preparation, full real upgrade and Ajv 2020 report validation passed.** The
new manifest identifies `verified-existing-build`, checks the exact prior
release and both gitlinks, verifies recorded historical source and executable
hashes, and checks source/submodule cleanliness. It then records a new source
walk explicitly rooted in the historical checkout: 616 files, SHA-256
`e153b20cfbcfacacc9b99377639b0d3a4ba2380cc5f130978c02df17af13b239`.
This walk additionally includes the tracked ARMv7 CMake toolchain file omitted
from the original 615-file manifest; it is not a historical source edit. Native
and Go executable hashes remain the four values in the table above. The actual
result contains ten observation groups, including both current starts and
untouched-original checks. Private report/log paths are
`/tmp/lightnvr-T28a-upgrade/result.json` and `/tmp/lightnvr-T28a-upgrade.log`.

Negative preparation checks rejected a mismatched historical revision and
binary hash. Missing required reports produced a failed sanitized summary with
zero of four required reports; no raw errors or report values were published.
Python syntax, workflow parsing, all nine shell steps, the single-file upload
allowlist and actionlint 1.7.7 passed. Local check metadata is retained in
`build/eng-api-T28a-prepared-final/local-validation.json`.

A second qualification exercised the default clean clone/build path with the
same captured current T28 binary, without reusing any historical executable:

```sh
PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig \
PKG_CONFIG_PATH="$PWD/build/eng-api-T25-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig" \
python3 tests/reliability/prepare-upgrade.py prepare \
  --output build/eng-api-T28a-clean --jobs 2 \
  --current-binary build/eng-api-T28/bin/lightnvr \
  --current-go2rtc go2rtc/go2rtc \
  --current-build-manifest build/eng-api-T28/evidence.json \
  '--cmake-arg=-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu' \
  '--cmake-arg=-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
python3 tests/reliability/upgrade.py \
  --binary build/eng-api-T28/bin/lightnvr \
  --fixture-manifest build/eng-api-T28a-clean/upgrade-manifest.json \
  --output /tmp/lightnvr-T28a-clean-upgrade
python3 tests/reliability/prepare-upgrade.py validate-report --kind upgrade \
  --report /tmp/lightnvr-T28a-clean-upgrade/result.json
```

**The clean preparation, full upgrade and report-schema validation passed.**
Its fresh historical source digest is the same 616-file digest above. Actual
new executable hashes are native
`feed66a05564b12e38857510578202f72c2bc3471245839ed2344c0292e7112e`
and historical Go
`fd1f7b8b783f837e9fa701ec0f29173cb2cee18c581a1353dbd6507e4cf9bed9`.
The generated manifest records `fresh-build`, exact commands and flags; build
logs are in `build/eng-api-T28a-clean`. Current binary and Go still match the
original captured T28 hashes. Both the default and reuse paths are now observed.

Final validator checks accepted actual reports of all four formats, rejected
five malformed/failed/pin/digest/storage-boundary cases, and omitted a synthetic
private extra field from the allowlisted summary. These validator replays do
not claim to rerun the other three scenarios against the T28 current binary.

The new manual workflow runs current patched Go, a fresh managed Debug recorder,
a clean unpatched historical build and all four bounded recovery scenarios.
It validates actual reports and uploads only allowlisted status, hashes, pins
and counts; missing/failed reports fail the job. It is opt-in and adds no branch
check. **Hosted CI has not been observed.** Local validation exercised both a clean
historical clone/build and explicit verified reuse, each followed by a real
upgrade. No harness behavior, migration or result schema was
changed for this automation.
