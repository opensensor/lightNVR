# Browser startup measurement

Build the production web assets (`npm --prefix web run build`) and a native
fixture binary, then run from the repository root:

```sh
node tests/performance/browser-startup.ts --binary build/mobile-review/bin/lightnvr --output build/browser-startup --samples 3
```

The output directory must be new. Requires Node 26, Playwright Chromium,
FFmpeg/ffprobe with libx264 and the native fixture dependencies. All media,
credentials and configuration belong to the isolated loopback fixture.

This visits Live, Recordings, Settings and API explorer three times with both
cold and warm browser caches, on desktop and a 390px touch/4x-CPU Chromium
profile (48 navigations). It records visible-control timing, script/task CPU,
heap, every resource's transfer/decoded sizes and actual decoded recording-frame
timing. Frame timing includes the automated Play action and uses a 96×64
synthetic MP4. It does not measure first live-camera frame or validator-only CPU.

The retained JSON contains browser/host/source/binary hashes and hashes of every
built asset. Run serially without other agent builds or tests for a useful
baseline. Neither loopback results nor CPU emulation qualify physical phones,
networks, NVR capacity or a production SLO. Failures retain fixture artifacts.
