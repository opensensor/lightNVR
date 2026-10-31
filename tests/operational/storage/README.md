# Isolated storage failure tests

Run the retention/SQLite/pressure baseline and operational scenarios together:

```sh
python3 scripts/quality/run-native.py --task T28 \
  --regex '^test_storage_(retention_sqlite|pressure|pressure_extended|manager_retention|recording_faults|writer_backpressure)$'
```

The runner serializes native and application workloads with the shared runtime lock,
checks FFmpeg headers against linked libraries, and defaults to ASan/UBSan. Logs,
JUnit results, CTest discovery and build provenance go under the printed task artifact
path. No camera, mounted test volume, external FFmpeg process or elevated privileges
are needed. All temporary database/media directories use `mkdtemp`.

| Scenario | Injection and bound | Assertions |
| --- | --- | --- |
| Enabled inherited/unlimited/finite retention | Real SQLite, global14days, per-stream -1/0/3days, five1KiB sparse files | Correct twoexpired files removed; recent, unlimited and protected rows/files retained; openprotected descriptor remains readable; SQLite integrity |
| Cleanup EROFS/EIO | Link wrapper fails `unlink` only for one exact owned path | File and metadata preserved, durable deletion journal pending/retry count, protected file retained, real retry succeeds after clearing fault |
| Root unavailable/restored | Rename owned root temporarily, restore before assertions | Expired and protected metadata remain while root unavailable; expired file reclaimed after restoration |
| SQLite read-only | Real connection `PRAGMA query_only=ON`, restored after test | Protection update rejected with SQLITE_READONLY; row unchanged; update succeeds after restoration |
| SQLite full | Real pager `max_page_count=page_count+8`, at most256 recording insert attempts | SQLITE_FULL, exact committed row count, prior protected row unchanged and integrity_check=ok; subsequent insert succeeds after limit lifted |
| Active segment ENOSPC/EIO/EROFS | Fourth output `av_interleaved_write_frame` fails; scoped to exact output URL | Segment returns I/O failure immediately, closes output/demuxer, telemetry records error; restored run produces decodable media |
| Read-only segment open | Exact output `avio_open` returns EROFS | No packet write/file created; restored run decodes |
| Emergency backpressure | Production writer loop with pressure seam and deterministic two1second sleep calls (no real sleep) | No segment before recovery, aggressive cleanup requested, one segment after recovery; shutdown while paused opens none |

The media fixture is generated inside the test using FFmpeg's native MPEG-4 encoder:
32×32 YUV420P,24fps, fourseconds, one-second GOP, noaudio. Real encoding,
demuxing, segment recording, muxing and post-recovery decoding run in-process.
The write-error seam models the muxer's observable error contract; it does not
claim to emulate a kernel block device. SQLite failures use the real pager.
The backpressure test stubs the segment boundary and does not claim end-to-end
recording metadata or API health coverage.

No test fills or remounts the host filesystem. Deletion retries advance only the
private database's retry deadline. An unavailable-root case restores its directory
before checking results so a failed regression does not strand test media.

Remaining coverage boundaries and any discovered product defects are recorded in
`quality/evidence/T28.md`; a green pressure-classifier test alone is not evidence
of full-volume recorder recovery.
