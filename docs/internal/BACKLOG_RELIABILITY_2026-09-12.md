# Reliability backlog work — 2026-09-12

Based on main `7f91155c9f7a0958fe6737b11c56d8090527779f` (0.41.17).
The affected cameras are unavailable in this workspace. No additional SQLite
error log or ODM capture is available beyond the issue reports.

| Issue | Implemented or verified locally | Remaining evidence |
| --- | --- | --- |
| [#600](https://github.com/opensensor/lightNVR/issues/600) | Removed HLS `alarm()` calls and temporary SIGALRM/SIGSEGV dispositions; removed the freed-memory probe. Concurrent writer closure and full stream shutdown/restart pass. Fixed a codec metadata leak exposed by the lifecycle test. | The reported connection to a particular deadlock remains unproven. The system API suite completes. These tests do not establish freedom from every existing HLS context lifetime race. |
| [#603](https://github.com/opensensor/lightNVR/issues/603) | Parse the subscription destination from the XML response's `SubscriptionReference`, including arbitrary prefixes and escaped URL characters. Escape `wsa:To`. Reject empty HTTP 200 responses, disable curl signal use, and log action/curl code/HTTP code/elapsed time. Test dropped connections and subsequent subscription reuse. | Tapo C530WS model/hardware revision/firmware, working ODM exchanges, and matching lightNVR exchanges are still needed. Keep open; this does not establish that the report is a duplicate of #567 or that these changes fix its empty connection response. |
| [#580](https://github.com/opensensor/lightNVR/issues/580) | Removed whole-file cache eviction during `integrity_check`; retain full verification, copy-batch cache release, final eviction, shutdown abort and deadline checks. Schedule the next backup interval from completion and defer failed attempts without counting them as successful backups. Performance work remains linked to [#604](https://github.com/opensensor/lightNVR/issues/604). | Persistent timeline `SQLITE_IOERR` remains a separate investigation. Need the full failing operation and `code`/`extended_code`, with filesystem/device context. No database recovery or error suppression was added. |
| [#568](https://github.com/opensensor/lightNVR/issues/568) | Native video play/pause updates shared investigation playback. Apply shared changes before delayed native events can undo a pause. Automatic pauses during media reload, clip end or errors do not stop the shared clock. Browser test exercises real media events and reload. | Reporter already confirmed the cropping fix; no further cropping change was needed. |
| [#494](https://github.com/opensensor/lightNVR/issues/494) | Real RTSP input plus simulated ONVIF events produces a decodable detection-only MP4 containing footage before the trigger. Eight-second pre-buffer and three-second post-buffer are exercised. | Still needs footage from the affected camera on the current release. The synthetic source has a one-second GOP; it does not establish behavior for every camera's GOP, timestamps or reconnect history. |
| [#579](https://github.com/opensensor/lightNVR/issues/579) | Existing browser tests pass: 1×1 selects main, multi-cell selects sub, and fullscreen upgrades only its selected cell. No new stream-selection change needed. | Retest the affected camera with main-only recording/viewing, then add sub-stream consumers. Its later failure in the manufacturer app remains a separate symptom; browser tests cannot validate camera stability. |

## HLS lifecycle validation

`test_hls_cleanup` runs three writers through three closure/recreation cycles,
then three full unified HLS streams through two simultaneous restart cycles and
simultaneous shutdown. The writer test reopens finalized playlists and reads
media. The stream test paces local H.264 input to keep threads active during
shutdown. Link wrappers reject alarm use or signal-disposition changes.

The expanded test initially found 981 leaked bytes across nine stream lifetimes.
LeakSanitizer traced them to codec extradata allocated during
`avformat_find_stream_info`. `comprehensive_ffmpeg_cleanup` discarded those
pointers and zeroed the codec parameters before FFmpeg could free them. It now
leaves demuxer-owned parameters intact for normal input closure. The test passes
with address/leak sanitizers enabled; the test does not suppress leak detection.

## Backup measurement

Reproduce on a disk-backed directory; `/tmp` on this workspace is tmpfs:

```sh
python3 tests/database/benchmark_backup_verification.py build
```

The script creates its own indexed audit fixture from migration 0055, starts
each verification with cold file cache, measures `/proc/self/io` read bytes, and
removes the fixture. It never opens an application database.

One local run with 100,000 rows and an 81,793,024-byte fixture:

| Verification policy | Read bytes | Elapsed | Integrity result |
| --- | ---: | ---: | --- |
| Evict whole file every 100,000 VM operations | 178,323,456 | 1.235 s | ok |
| Retain cache during verification | 81,776,640 | 0.924 s | ok |

An earlier 67 MB audit fixture showed 368 MB versus 67 MB read, and 3.437 s
versus 0.844 s. The amplification depends on table/index layout and available
memory; neither small local run predicts the reported 38–48-minute production
backup duration. The repeatable result is reduced rereading while retaining the
same integrity check. Verification may occupy more reclaimable OS page cache
until completion; the SQLite heap limit and copy-phase controls remain.

## Actual ONVIF pre-buffer footage

```sh
python3 tests/integration/synthetic-onvif-prebuffer.py --go2rtc /path/to/go2rtc
```

Requires the built application, frontend assets, FFmpeg and go2rtc. The harness
uses dedicated temporary files, loopback camera ports and separately owned
processes. It retains application/source logs, the MP4 and `result.json` in the
printed `/tmp/lightnvr-prebuffer-*` directory.

The source changes from red to blue at media second 12. Motion begins after
second 15 and stops after second 18. Continuous recording is disabled, and the
harness checks that there was no recording before the event. The MP4 must start
red, end blue, and carry a pre-trigger interval in its metadata. This validates
the footage, independently of a settings value or packet-buffer count.

The first successful local run produced a 32.2-second clip with a red first
frame (`255,0,0`) and a start time eight seconds before the simulated trigger.
Logs showed 92 buffered packets flushed, 91 written from a keyframe, and entry
into a three-second post-buffer. The total clip also includes the existing
15-second ONVIF motion hold and detection grace interval.

The checked-in harness also passed after the final C changes: a 31.4-second
clip started red (`255,0,0`), ended blue (`0,0,255`), and began about 7.8 seconds
before the trigger. Its local artifacts are in
`/tmp/lightnvr-prebuffer-m2pnwzlh/result.json` and the sibling recording directory.

## Checks

Configured Debug build with tests enabled and SOD, LiteRT, MQTT and SSL disabled.
Frontend production build passed. Targeted C tests passed:

```sh
cmake --build build -j8 --target lightnvr test_hls_cleanup \
  test_detection_system_onvif test_db_backup test_packet_buffer \
  test_api_handlers_system test_mp4_segment_timestamps
ctest --test-dir build --output-on-failure --timeout 30 \
  -R '^(test_hls_cleanup|test_detection_system_onvif|test_db_backup|test_packet_buffer|test_api_handlers_system|test_mp4_segment_timestamps)$'
```

Six C suites passed in 31.48 s; `test_api_handlers_system` completed in 20.67 s.
The ONVIF suite includes 14 cases. Investigation browser tests passed (2 cases),
as did the existing sub-stream/fullscreen tests (2 cases). Browser tests used
`LIGHTNVR_TEST_DIR`, `LIGHTNVR_TEST_CONFIG`, and `LIGHTNVR_TEST_BIN` pointing at
an isolated test instance, with `LIGHTNVR_SKIP_GO2RTC=1` for mocked live endpoints.

These results support the local fixes and regression coverage. They do not
close the hardware-dependent reports or identify the persistent SQLite I/O
error's extended cause.
