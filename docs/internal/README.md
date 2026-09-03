# Internal notes

Design notes, subsystem writeups, and historical implementation records. These are written
for people working on LightNVR's internals.

**They are not user documentation, and they are not guaranteed to describe current
behavior.** Several describe a change at the time it was made; the code has moved on. If
you are trying to run or configure LightNVR, start at [the docs index](../README.md)
instead.

| Document | What it is |
|---|---|
| [STATE_MANAGEMENT.md](STATE_MANAGEMENT.md) | The stream/recording state management architecture and the races it was built to resolve |
| [MOTION_DETECTION_OPTIMIZATION.md](MOTION_DETECTION_OPTIMIZATION.md) | Optimizations made to motion detection for embedded targets |
| [PRE_DETECTION_BUFFER_IMPLEMENTATION.md](PRE_DETECTION_BUFFER_IMPLEMENTATION.md) | Implementation record for the pre-detection buffer (see [MOTION_BUFFER.md](../MOTION_BUFFER.md) for how to use it) |
| [HLS_REFACTORING.md](HLS_REFACTORING.md) | The HLS architecture refactor |
| [GO2RTC_PROCESS_MONITORING.md](GO2RTC_PROCESS_MONITORING.md) | How the go2rtc child process is supervised |
| [GO2RTC_CAMERA_RECONNECTION_FIX.md](GO2RTC_CAMERA_RECONNECTION_FIX.md) | Post-mortem of a camera reconnection bug |
| [SOD_UNIFIED_DETECTION.md](SOD_UNIFIED_DETECTION.md) | The unified interface over RealNet and CNN models |
| [SOD_REALNET.md](SOD_REALNET.md) | RealNet face detection integration |
| [SUMMARY_Recording_Retention.md](SUMMARY_Recording_Retention.md) | Summary of the retention policy work (the PRD is in [prd/](../prd/PRD_Recording_Retention_Policies.md)) |

Adding something here? If it describes a change you made rather than how the system
behaves, say so in the opening line and date it — that is the difference between a useful
record and a document someone later mistakes for the truth.
