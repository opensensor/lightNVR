# Synthetic media recovery regression

Run under the coordinator's exclusive native/server/media lease:

```sh
python3 scripts/quality/run-native.py --task T27 \
  --regex '^test_mp4_(audio_recovery|recovery_watchdog|writer_watchdog|segment_boundaries|segment_timestamps)$'
python3 tests/operational/media/recovery.py --binary /absolute/path/to/current/lightnvr
python3 -m unittest discover -s tests/operational/media -p test_assertions.py
```

The real-media runner requires Linux pidfds, Python 3, FFmpeg/ffprobe with libx264
and AAC, the repository go2rtc executable, and an explicitly supplied LightNVR
binary. `--go2rtc` overrides that binary; `--scenario video-only|h264-aac` selects
one scenario. It takes the same atomic runtime lock as T13 and never adopts an
existing server. Dynamic loopback ports, fresh NVR config/database/recordings,
owned processes and source logs are retained under `build/eng-artifacts/T27/`.
Only pinned child identities are signalled during cleanup. Forced shutdown is a
failure, even if media assertions otherwise pass.

`tests/fixtures/eng/media/scenarios.json` declares every functional threshold.
These host regression deadlines are not constrained-device qualification or a
replacement for T5's deferred performance budgets. The source is live FFmpeg
`testsrc2`, 160x96, 10fps, H.264 with one-second GOP/no B frames, optionally AAC
48kHz mono sine audio. go2rtc provides the actual RTSP socket. After three
complete three-second segments, its owned process and observed FFmpeg children
are stopped, the socket stays unavailable for four seconds, and the same source
configuration restarts. Three new complete segments must appear within 60 seconds.

The H264/AAC case also exercises the **real** storage-controller pause decision.
It marks its fixture recordings protected in its private SQLite database, raises
only its private NVR's configured emergency threshold above the observed free
percentage, and requests a controller check. No host filesystem is filled,
remounted or made read-only. `/api/storage/health` must report emergency. After
an already-open segment settles, no new database recording row may open while
the source disconnects/restores. Restoring thresholds below actual free space
and waking the controller must report normal and allow recording to resume.
This is a controlled threshold-induced pressure state, not a physical ENOSPC
simulation. T28 separately owns actual-write fault injection.

Each selected file must match its database byte size and start/end interval,
contain the intended video/audio codecs, start with a keyframe, have strictly
increasing DTS per stream, satisfy declared duration/count bounds, and decode
fully through FFmpeg with `-xerror`. Continuous sequences must stay within the
stated integer-wall-clock tolerance. The existing recorder deliberately retains
one shared boundary video keyframe; **one** video frame of overlap is allowed,
not two. Unit coverage checks exact source-frame conservation, shared keyframe
bytes, per-frame cadence and decoding across three consecutive segments. The
operational oracle rejects two duplicated boundary packets. Packet timestamps
reset relative to each segment; raw per-file DTS values are not incorrectly
compared as a single global clock. Database ordering is checked across reconnect.

The native watchdog case advances wrapped one-second sleeps deterministically:
five failed retries must lead to exactly 300 cooldown ticks before another retry;
shutdown after seven cooldown ticks must stop without another restart. Existing
G.711/audio-transcoder recovery tests are preserved.
