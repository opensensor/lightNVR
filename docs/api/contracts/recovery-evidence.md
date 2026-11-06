# Reconnect and lifecycle qualification

T27 uses `tests/reliability/lifecycle.py` with the real C server, managed go2rtc
and the owned FFmpeg/MPEG-TS source from T25. It uses private databases/storage,
dynamic loopback ports, synthetic credentials and retained logs. No production
camera, storage, database or process is touched.

## Declared experiment

The default is three cycles per phase; `--cycles` accepts 3–10. Bounds are set
before execution and saved in the result, not adjusted after seeing a failure:

| Observation | Fixture bound |
| --- | --- |
| Source outage | At least 5 seconds, including producer shutdown |
| New completed media after resume | 60 seconds |
| HTTP request | 10 seconds |
| Recorder graceful shutdown | 30 seconds |
| Synthetic segment / GOP | 6 seconds / 1 second at 10fps |
| Conservative catalog media gap | 72.2 seconds: outage + recovery + segment + GOP/rounding |
| Steady-state descriptor/thread jitter | Baseline + 8 descriptors / + 4 threads |

The gap is from the last completed row before the outage to the first qualifying
completed row after resume. It can include intervening usable footage, so it is
a conservative catalog gap, not a frame-perfect measurement of lost video.
ffprobe packet DTS must be monotonic, and FFmpeg must decode the selected and
final playable files. These are workstation fixture limits, not production SLOs
or physical-camera qualification.

The first phase pauses the source gate, terminates only that source's owned
FFmpeg processes, proves new connections receive 503 on the same URL, then
resumes it. It never kills managed go2rtc to simulate a source disconnect.
New completed media must have a post-resume timestamp and a new catalog ID.
The original protected recording must retain its ID, path, protection and hash.

Resource samples count `/proc/<pid>/fd` and `/proc/<pid>/task` for the recorder
and the verified managed child. Each sample is a median of five observations at
100ms intervals at the same phase (after completing recovered media). The
harness rejects the declared jitter overflow or strictly increasing values
across at least four samples with a net increase above two. Raw samples remain
in JSON; this short bounded check is not a long-duration leak qualification.

The second phase switches the fixture stream out of continuous recording and
uses the actual manual recording endpoint. The PUT response acknowledges
asynchronous work, so setup waits up to 30 seconds for the completion entry in
the private server log before issuing manual requests. This establishes the
mode; it does not qualify concurrent policy changes. After real media is written, a
barrier launches two stop requests and one start request concurrently. Successful
responses and honest 409 conflicts are accepted; 5xx responses are failures.
The resulting recorder is brought to idle before the next cycle.

The final phase overlaps one SIGTERM with real manual stop and start HTTP
requests. Transport closure during process shutdown is recorded as expected.
Only one signal is sent: the current server intentionally force-exits on a second
shutdown signal, so duplicate SIGTERM is not represented as graceful shutdown.
The harness waits for exit and checks for surviving managed children **before**
the fixture's safety cleanup can remove them. It then restarts the same private
installation and verifies catalog protection/identity, reconciliation, media and
SQLite integrity. No concurrent calls to fixture cleanup are used.

## Reproduction

Use a fresh managed binary with coherent FFmpeg headers/libraries and libyaml;
see `tests/reliability/README.md` for the diagnosed workstation toolchain issue.
On the current x86_64 Debian-layout workstation:

```sh
PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig \
PKG_CONFIG_PATH="$PWD/build/eng-api-T25-deps/sysroot/usr/lib/x86_64-linux-gnu/pkgconfig" \
cmake --fresh -S . -B build/eng-api-T27 -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON \
  -DENABLE_SOD=OFF -DENABLE_LITERT=OFF -DENABLE_GO2RTC=ON \
  -DGO2RTC_BINARY_PATH="$PWD/go2rtc/go2rtc" -DENABLE_MQTT=OFF -DENABLE_SSL=ON \
  '-DCMAKE_C_FLAGS=-isystem /usr/include/x86_64-linux-gnu' \
  '-DCMAKE_CXX_FLAGS=-isystem /usr/include/x86_64-linux-gnu'
cmake --build build/eng-api-T27 --target lightnvr test_shutdown_coordinator \
  test_stream_state test_stream_state_disposal test_go2rtc_lifecycle \
  test_go2rtc_process_lifecycle test_libuv_handler_lifecycle -j6
python3 tests/reliability/lifecycle.py \
  --binary build/eng-api-T27/bin/lightnvr --go2rtc ./go2rtc/go2rtc
ctest --test-dir build/eng-api-T27 --output-on-failure --no-tests=error \
  -R '^(test_shutdown_coordinator|test_stream_state|test_stream_state_disposal|test_go2rtc_lifecycle|test_go2rtc_process_lifecycle|test_libuv_handler_lifecycle)$'
```

CMake's shared version generation and native test execution must be coordinated
with other workers. `--build-manifest PATH` accepts the T25 fixture's source
manifest captured at compilation and verifies both binary hashes. `--output`
requires a new directory; otherwise evidence is retained under
`/tmp/lightnvr-lifecycle-*`. `result.json` follows the shared reliability schema
and records exact triggers, requests, time bounds, process resources, media
probe/decode results, source digest, executable hashes and cleanup outcome.

Same-second filename reuse and clock rollback remain a separate known writer
collision follow-up. These scenarios do not induce or qualify those conditions.
Concurrent policy changes, device firmware, network partitions beyond the owned
source gate, and very long recording segments are outside this bounded run.

## Initial diagnostic evidence

Two runs against the accepted T25a native binary and unchanged pinned go2rtc
reproduced a resource failure: go2rtc descriptors rose **24 → 29 → 34 → 39**
across the baseline and three source outages. The second report,
`/tmp/lightnvr-lifecycle-yre_m207/result.json`, records owned socket targets and
TCP states: **5 → 10 → 15 CLOSE_WAIT sockets**, all to the private synthetic
source, while the recorder descriptor count stayed stable. The HTTP producer's
non-200 response path in `go2rtc/internal/http/http.go` returns without closing
the response body. Each five-second outage produces five rejected retries.
These sockets are leaked rejected responses, not idle reusable connections.

The first run, `/tmp/lightnvr-lifecycle-6h8poc00/result.json`, recovered completed
playable media in 6.07 / 6.48 / 6.49 seconds with conservative catalog gaps of
6 / 7 / 7 seconds. Both runs failed the original descriptor bound. The harness
stops before manual-recording races and overlapping process shutdown after this
failure, so these reports do not qualify either later phase. No resource
threshold was relaxed.

The focused T27a repair adds `tcp.Close(res)` before returning the non-200
error. `go2rtc/internal/http/http_test.go` streams an unfinished error response
and waits for server-side request cancellation, so server-initiated closure
cannot hide the leak. All three statuses (503/404/500) failed before the change
in `/tmp/lightnvr-T27a-go-red.log`; `go test -race ./internal/http -count=1
-timeout=60s` passed afterward in `/tmp/lightnvr-T27a-go-green.log`. Successful
producer ownership and unrelated media handlers are unchanged.

The initial repair lived in the go2rtc submodule working tree. T27c delivered
the same source and regression through a parent-tracked patch while retaining
the original `6986a1b` pin. Those historical runs record that revision, the
exact tracked diff and SHA-256, and both changed source/test contents and hashes.
The rebuilt diagnostic go2rtc executable has SHA-256
`1784534bdaf809a11eb0bb4e38cbc354cccf0273c6e3f7e86faf41f5ceabab0b`.

After #804 merged, the fix and regression were committed directly to the
maintained fork as [opensensor/go2rtc `6c31199`](https://github.com/opensensor/go2rtc/commit/6c31199036913edbff0c3546c74469dfe593ff00).
The parent now pins that commit; a fresh submodule checkout includes the repair.
The temporary patch, application script and build hooks have been retired.
Current provenance records the go2rtc revision, source diff and binary hash.
Historical upgrade fixtures still build their original unmodified release pin;
the dated binary hashes and qualification results above are not rewritten.

The first post-repair diagnostic (`/tmp/lightnvr-lifecycle-g8smr8cn/result.json`)
passed unchanged reconnect resource bounds: recorder FDs 21/21/21/21 and
threads 83/83/83/83; go2rtc FDs 24/24/24/24 and threads 17/17/18/18. It then
failed during manual-mode setup because the asynchronous policy PUT had not
finished before a manual start. The native writer correctly rejected duplicate
writer registration, but the accepted manual request produced no media. The
harness now waits for policy completion before its deliberately concurrent
manual lifecycle phase. This diagnostic is retained and is not full T27 acceptance.

With the setup barrier, `/tmp/lightnvr-lifecycle-tkblwz9l/result.json` passed
three manual stop/stop/start races (200/409 stops and 202/409 starts), then
failed on the first real SIGTERM overlapping manual stop/start. The native
process exited 1 with an AddressSanitizer heap-use-after-free. The retained
`recorder-1.log` identifies a write in `http_response_set_json` from
`handle_post_stream_recording`, after `libuv_server_stop` closed and destroyed
the connection through `libuv_close_cb`. The worker still owned the embedded
request/response. This is a production ownership defect; T27 remains unaccepted.
The report records sanitizer detection during cleanup as well as the failed
shutdown, and the fixture cleans only its captured managed child identities.

T27b targets connection ownership until the queued handler and its completion
callback finish. The shutdown drain retains its existing three-second warning
but must continue draining before allowing server/loop/TLS and global subsystem
teardown. This deliberately favors memory safety over abandoning a blocking
handler. The fixture's 30-second shutdown bound qualifies its selected workload;
it cannot guarantee an arbitrary blocked handler will terminate. Regression
handlers have a five-second gate deadline and the long-handler case releases
after four seconds, so the tests themselves remain finite.

## Accepted recovery evidence

The unchanged full experiment passed with the repaired Go/C ownership paths:
`/tmp/lightnvr-lifecycle-fj8enmir/result.json`. Three reconnects recovered completed
media in 6.59 / 6.48 / 6.48 seconds. Recorder descriptors stayed21/21/21/21 and
threads83/83/83/83; go2rtc descriptors24/24/24/24 and threads18/18/18/18.
Three manual request races and three overlapping process shutdown/restart cycles
passed. Shutdown took4.47 / 4.12 / 4.12 seconds, each exited0, and no owned
managed children survived before safety cleanup. Protected file hashes/catalog
identity, packet/decode checks and SQLite integrity passed. The shared result
schema also validates the report. Its source digest is
`fc96e45d7a4a8443aad4ddcaab11992130d8970deabc43a4afbc83139602086c`, native hash
`885eeba495b1199c490545d6a010a3a9a69df1a14bb214c9437d09ca93102ae9`.

The deterministic native regression first reproduced the old lifetime failure
under ASan (`/tmp/lightnvr-T27b-red-test.log`). Review then added a never-started
server destruction case and an observable server-state read to the existing
disconnect/delayed-handler cases. All three pass in4.32s under ASan/LSan/UBSan
(`/tmp/lightnvr-T27b-final-test.log`); this small final cleanup follow-up has its
own source/binary/test manifest in `build/eng-api-T27/evidence-final.json`.
Five related lifecycle CTests passed in5.73s, and the go2rtc process lifecycle
CTest passed in2.19s (`/tmp/lightnvr-T27-{native,process}-test.log`). The full
experiment above and the final unit follow-up are distinct recorded builds;
final integration must run against its own fresh build.
