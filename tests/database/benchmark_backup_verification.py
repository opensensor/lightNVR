#!/usr/bin/env python3
"""Compare integrity_check I/O with and without repeated whole-file eviction.

Linux-only diagnostic, not a timing assertion for CI. Pass a directory on the
filesystem being measured (avoid tmpfs). Creates and removes its own fixture.
"""

import argparse
import ast
import json
import os
from pathlib import Path
import re
import sqlite3
import tempfile
import time


def read_bytes():
    values = dict(line.split(": ") for line in Path("/proc/self/io").read_text().splitlines())
    return int(values["read_bytes"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--rows", type=int, default=100000)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    source = (root / "include/database/db_embedded_migrations.h").read_text()
    migration = source.split("static const char migration_0055_up[] =", 1)[1]
    migration = migration.split("static const char migration_0055_down", 1)[0]
    schema = "".join(ast.literal_eval(part) for part in re.findall(r'"(?:[^"\\]|\\.)*"', migration))

    with tempfile.TemporaryDirectory(prefix="backup-verification-", dir=args.directory) as folder:
        path = Path(folder) / "audit.db"
        with sqlite3.connect(path) as db:
            db.executescript("CREATE TABLE users(id INTEGER PRIMARY KEY);"
                             "CREATE TABLE system_settings(key TEXT PRIMARY KEY,value TEXT);" + schema)
            db.executemany(
                "INSERT INTO audit_events(uuid,occurred_at,request_id,principal_user_id,"
                "principal_username,auth_method,action,target_type,target_uuid,outcome,"
                "remote_address,details_json) VALUES(?,?,?,?,?,?,?,?,?,?,?,?)",
                ((f"event-{i:032d}", 1700000000 + i, f"request-{i}", i % 4,
                  "admin", "session", "recordings.list", "camera", f"camera-{i % 16}",
                  "success", "127.0.0.1", json.dumps({"detail": "x" * 400}))
                 for i in range(args.rows)),
            )
        db.close()
        print(json.dumps({"rows": args.rows, "file_bytes": path.stat().st_size}))
        fd = os.open(path, os.O_RDONLY)
        try:
            # Both runs begin cold, outside the measurement interval.
            for evict in (True, False):
                os.fsync(fd)
                os.posix_fadvise(fd, 0, 0, os.POSIX_FADV_DONTNEED)
                callbacks = 0

                def progress():
                    nonlocal callbacks
                    callbacks += 1
                    if evict:
                        os.posix_fadvise(fd, 0, 0, os.POSIX_FADV_DONTNEED)
                    return 0

                db = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
                db.set_progress_handler(progress, 100000)
                before = read_bytes()
                started = time.monotonic()
                result = db.execute("PRAGMA integrity_check").fetchall()
                elapsed = time.monotonic() - started
                fetched = read_bytes() - before
                db.close()
                print(json.dumps(dict(evict_each_callback=evict, seconds=round(elapsed, 3),
                                      read_bytes=fetched, callbacks=callbacks, integrity=result)))
                assert result == [("ok",)]
        finally:
            os.close(fd)


if __name__ == "__main__":
    main()
