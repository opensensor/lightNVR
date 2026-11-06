# Storage fault boundary

These files are test fixtures, never compiled into or installed with LightNVR.
`../storage-faults.py` compiles them into its private temporary evidence directory.

`fault_boundary.c` is a Linux `LD_PRELOAD` library intercepting only `statvfs`,
`statvfs64`, `mkstemp` and `mkstemp64`, and only for the exact private recording
directory or its descendants. A sibling whose name merely shares the prefix is
unaffected. Its control file selects:

- `none`: ordinary libc behavior;
- `full`: zero available blocks plus ENOSPC on temporary-file creation;
- `readonly`: the read-only filesystem flag plus EROFS on temporary-file creation.

The independent `probe_boundary.c` helper proves the calls are intercepted,
outside paths are unaffected, and a process without the preload sees the real
filesystem. Audit lines record the owned PID, intercepted symbol and errno.
No disk is filled, no mount changed, and no system binary or environment modified.
The script restores the fault control and directory permissions in `finally`.

Real-server qualification adds kernel-enforced EACCES with mode 0500 under an
unprivileged UID, and ENOENT by renaming only the private recording directory.
It refuses root execution because root can bypass the permission case.

The boundary qualifies LightNVR's direct storage-target write probe and controller
pressure/retention behavior. It does not simulate every filesystem call, FFmpeg
writer failure, real filesystem exhaustion, physical disk failure or a read-only
mount. The server's separate host-health collector is disabled during this matrix
to avoid mixing its clean-environment subprocess samples with the explicitly
injected direct probe; ordinary scheduled health collection has its own native
tests. Recording interruption/reconnect is covered by the separate T25/T27 work.

`ffmpeg_abi_probe.c` checks compile-time versus linked FFmpeg major versions.
Compile it with the same include/link selection as the native test build before
qualifying media tests on hosts with multiple FFmpeg installations. A mismatch
is an environment failure; a crashing mismatched build is not product evidence.

Commands from repository root:

```sh
python3 tests/reliability/storage-faults.py --binary build/eng-api-T26/bin/lightnvr --self-test
python3 tests/reliability/storage-faults.py --binary build/eng-api-T26/bin/lightnvr
```

Both commands retain JSON results, source manifests and compiler logs in a newly
created `/tmp/lightnvr-storage-fault-*` directory. The full matrix also retains
all recorder logs, the current-schema SQLite DB, the protected synthetic MP4,
FFprobe output and FFmpeg decode diagnostics. No existing directory is adopted
or removed. Exit code 1 means a failed or unavailable qualification, never a
silent skip. Default limits are startup 30 s, HTTP 10 s, cleanup 30 s, shutdown
15 s plus an owned-PID kill/reap deadline, and external tools 30 s.
