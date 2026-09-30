# Issue #620 follow-up: detection consumer recovery

Investigated against `658a03ad` (0.42.8).

The [September 26 report](https://github.com/opensensor/lightNVR/issues/620#issuecomment-5845557708)
says remote cameras still stop producing detection recordings on 0.42.8, while
their live view and local-camera detection continue working. The reporter has
offered fresh logs but has not attached them yet. The September 23 logs predate
the recovery changes; they cannot establish the cause of this recurrence.

## Confirmed code defect

Both readers in `src/video/unified_detection_thread.c` set the obsolete RTSP
option `stimeout`. This repository requires libavformat >= 61; its RTSP option
is `timeout`, in microseconds ([FFmpeg documentation](https://ffmpeg.org/ffmpeg-protocols.html#rtsp)).
The obsolete option is left unused in the dictionary and then discarded.

Both FFmpeg interrupt callbacks previously checked only stop/shutdown flags.
The main reader's ten-second packet watchdog runs *after* `av_read_frame`
returns. It cannot recover a connection stuck inside open, stream probing, or
packet reading. The optional detection-source reader has the same exposure;
while stuck reading, its `detection_stream_connected` flag also suppresses
main-stream inference.

The earlier go2rtc fixes address registration and producer recovery. They do
not interrupt a wedged detection consumer. The health monitor additionally
treats BUFFERING/RECORDING/POST_BUFFER as RUNNING without checking packet age,
and go2rtc byte counters may advance for a working live-view consumer. Thus
working live view does not exclude a stuck detection connection. This is a
plausible explanation of the new report, not confirmation of its field cause.

## Change

- Use `timeout` and `rw_timeout` for five-second socket I/O timeouts.
- Give each reader its own monotonic FFmpeg interrupt deadline: five seconds
  for opening and ten seconds for probing/reading. Bound normal disconnect
  I/O as well. Stop/shutdown checks remain active.
- On an expired read deadline, enter the existing reconnect/backoff path even
  if wall time has not advanced. Each subsequent operation gets a fresh deadline.
- Clear the optional source's connected flag before closing it so main-stream
  inference can resume during its reconnect.

## Verification

`test_detection_io_recovery` exercises the production reader loops and callbacks,
using a real media fixture for decoder setup and controlled libav stalls for
open/probe/read. It checks that both readers interrupt and attempt a fresh open,
that their deadlines are independent, and that stop requests still interrupt.
A separate real TCP listener accepts connections but never answers RTSP.

All eight cases pass in both Release and Debug (AddressSanitizer and
UndefinedBehaviorSanitizer) builds. Against the original production source,
all six stalled operation cases fail to interrupt, the deadline check fails, and the real silent
RTSP peer blocks until the external 12-second test limit kills the process.
With the patch, that real connection returns in about five seconds.

Existing `test_go2rtc_unregister_recovery`, `test_go2rtc_stream_activity`, and
`test_annotation_writer_registry` also pass. The local build needed its temporary
system-header include directory restored: installed `/usr/local` FFmpeg headers
are version 7 while the linked system libraries are version 8. Validation uses
the matching system headers, with no repository build configuration changes.

## Evidence still needed from the deployment

Before restarting the affected instance, capture a few minutes of INFO-level
LightNVR logs, including detection heartbeats and go2rtc recovery messages for
both affected cameras, plus `/go2rtc/api/streams` with credentials redacted.
Include each camera's detection engine and whether a separate detection URL
is configured. An absent heartbeat points toward blocked reader work; advancing
packet counts with absent detection checks points toward inference/decoder
handling; continuing detections without clips points toward recording handling.
