# Bounded active recording measurement

From the repository root, with a Release native build, Node 24, Linux `/proc`,
and FFmpeg/ffprobe with libx264:

```sh
node tests/performance/active-recording.mjs --binary build/stop-release/bin/lightnvr --output build/active-eight --streams 8 --seconds 60
```

The output directory must be new. The harness creates only private loopback
fixtures and synthetic sources: 1920×1080 H.264, 15fps, 15-frame GOP, 60-second
segments and no live viewers. It records list latency, RSS/high-water RSS,
descriptors and cumulative native-process CPU ticks once per second. Source
FFmpeg processes share the host; their CPU/memory are not included in the native
process samples. Streams start sequentially before the sampling interval.

After manual stop, every catalogued file must pass ffprobe and full FFmpeg
`-xerror` decoding, with footage present for every stream. Ordinary FFmpeg exit
status alone is insufficient: it can report corrupt frames while exiting zero.
The regression in `mediaLifecycle.api.spec.ts` uses one stream for ten seconds;
`LIGHTNVR_CONTRACT_BIN` selects the binary as in the other native contract tests.

Source/harness/native-file/binary hashes, build flags when available, machine
information, samples and failures are retained in `results.json`. The owned
source children have bounded shutdown. These are workstation observations, not
physical-camera, remote-storage, long-duration, active-viewer or sustainable
capacity qualifications. Use named target hardware and repeated serial runs
before setting supported capacity budgets.
