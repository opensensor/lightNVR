#!/usr/bin/env python3
"""Bounded real-server storage fault qualification; never fills/remounts a disk.

All logs/media/DB/results are retained in a private temporary directory.
ENOSPC/EROFS are explicitly injected at a process-local libc boundary; chmod
EACCES and renamed-directory ENOENT are real kernel failures. Run unprivileged.
"""
from __future__ import annotations

import argparse
import base64
import errno
import hashlib
import json
import os
from pathlib import Path
import socket
import sqlite3
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = Path(__file__).resolve().parent / "storage-fixtures"
DEADLINES = {"startup_seconds": 30, "request_seconds": 10,
             "cleanup_seconds": 30, "shutdown_seconds": 15,
             "tool_seconds": 30}
PASSWORD = "StorageFaultFixture123!"  # synthetic and local only


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def command(argv, *, env=None, timeout=30, log=None):
    completed = subprocess.run([str(x) for x in argv], env=env, cwd=ROOT,
                               stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                               text=True, timeout=timeout, check=False)
    if log:
        Path(log).write_text(completed.stdout)
    require(completed.returncode == 0,
            f"Tool exited {completed.returncode}: {argv[0]}; {completed.stdout[-2000:]}")
    return completed.stdout


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def source_evidence():
    paths = command(["git", "ls-files", "-c", "-o", "--exclude-standard", "--",
                     "src", "include", "CMakeLists.txt", "tests/reliability/storage-faults.py",
                     "tests/reliability/storage-fixtures"]).splitlines()
    digest = hashlib.sha256()
    manifest = {}
    for name in sorted(set(paths)):
        path = ROOT / name
        if path.is_file():
            data = path.read_bytes()
            digest.update(name.encode() + b"\0" + data + b"\0")
            manifest[name] = hashlib.sha256(data).hexdigest()
    return {"head": command(["git", "rev-parse", "HEAD"]).strip(),
            "sha256": digest.hexdigest(), "files": manifest,
            "note": "Source observed at run time; use the named fresh build and its manifest for binary provenance."}


def compile_boundary(directory):
    library = directory / "fault_boundary.so"
    probe = directory / "probe_boundary"
    command(["cc", "-std=c11", "-Wall", "-Wextra", "-Werror", "-fPIC", "-shared",
             FIXTURES / "fault_boundary.c", "-ldl", "-o", library],
            log=directory / "compile-boundary.log")
    command(["cc", "-std=c11", "-Wall", "-Wextra", "-Werror",
             FIXTURES / "probe_boundary.c", "-o", probe],
            log=directory / "compile-probe.log")
    return library, probe


def boundary_environment(directory, library, binary=None):
    env = {k: v for k, v in os.environ.items()
           if not k.startswith("LIGHTNVR_") and k != "LD_PRELOAD"}
    preload = str(library)
    if binary:
        # Keep ASan first for the project's default Debug build. Use the exact
        # runtime linked by this binary, not a guessed compiler installation.
        linked = command(["ldd", binary])
        for line in linked.splitlines():
            if "libasan.so" in line and "=>" in line:
                preload = line.split("=>", 1)[1].strip().split()[0] + ":" + preload
    env.update({"LD_PRELOAD": preload,
                "LIGHTNVR_TEST_FAULT_ROOT": str(directory / "recordings"),
                "LIGHTNVR_TEST_FAULT_CONTROL": str(directory / "fault-mode"),
                "LIGHTNVR_TEST_FAULT_LOG": str(directory / "fault-observed.log"),
                "LC_ALL": "C"})
    return env


def self_test(directory, library, probe):
    require(sys.platform.startswith("linux"), "Linux process-local boundary is required")
    require(os.geteuid() != 0, "Run unprivileged: root can bypass the chmod fault")
    recording = directory / "recordings"
    recording.mkdir(exist_ok=True)
    sibling = directory / "recordings-other"
    sibling.mkdir()
    control = directory / "fault-mode"
    env = boundary_environment(directory, library)
    cases = []
    for mode, expected in (("none", 0), ("full", errno.ENOSPC), ("readonly", errno.EROFS)):
        control.write_text(mode)
        value = json.loads(command([probe, recording], env=env))
        require(value["errno"] == expected, f"{mode} fault not reached: {value}")
        if mode == "full":
            require(value["available"] == 0, "Full statvfs was not injected")
        if mode == "readonly":
            require(value["readonly"], "Read-only flag was not injected")
        outside = json.loads(command([probe, sibling], env=env))
        require(outside["errno"] == 0, "Fault escaped the exact private directory boundary")
        # The same process without preload must not experience the injected error.
        ordinary = json.loads(command([probe, recording]))
        require(ordinary["errno"] == 0, "Fault modified the real filesystem")
        cases.append({"mode": mode, "result": value, "outside": outside})
    control.write_text("none")
    try:
        recording.chmod(0o500)
        denied = json.loads(command([probe, recording]))
        require(denied["errno"] == errno.EACCES, "Kernel permission fault not observed")
        cases.append({"mode": "kernel-permission", "result": denied})
    finally:
        recording.chmod(0o700)
        control.write_text("none")
    require((directory / "fault-observed.log").stat().st_size > 0, "Missing fault observations")
    return cases


class Server:
    def __init__(self, directory, binary, env):
        self.directory, self.binary, self.env = directory, binary, env
        self.child = None
        self.output = None
        self.generation = 0
        self.password = "admin"
        self.base = ""
        self.db = directory / "lightnvr.db"

    def request(self, route, method="GET", body=None, expected=200):
        data = json.dumps(body).encode() if body is not None else None
        auth = base64.b64encode(f"admin:{self.password}".encode()).decode()
        request = urllib.request.Request(self.base + route, data=data, method=method,
                                        headers={"Authorization": "Basic " + auth,
                                                 "Content-Type": "application/json"})
        try:
            response = urllib.request.urlopen(request, timeout=DEADLINES["request_seconds"])
        except urllib.error.HTTPError as error:
            response = error
        with response:
            status = response.status
            value = json.loads(response.read())
        require(status == expected, f"{method} {route}: expected {expected}, got {status}: {value}")
        return value

    def start(self):
        require(self.child is None, "Server already running")
        self.generation += 1
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        self.base = f"http://127.0.0.1:{port}"
        self.log = self.directory / f"server-{self.generation}.log"
        self.process_log = self.directory / f"process-{self.generation}.log"
        config = self.directory / "lightnvr.ini"
        config.write_text(f"""[general]
pid_file = {self.directory}/lightnvr.pid
log_file = {self.log}
log_level = 2
syslog_enabled = false
[storage]
path = {self.directory}/recordings
max_size = 0
record_mp4_directly = false
[database]
path = {self.db}
[web]
port = {port}
bind_ip = 127.0.0.1
root = {ROOT}/web/dist
auth_enabled = true
username = admin
password = admin
web_thread_pool_size = 4
[streams]
max_streams = 8
[models]
path = {self.directory}/models
[go2rtc]
enabled = false
auto_start = false
[health]
enabled = false
""")
        self.output = self.process_log.open("wb")
        self.child = subprocess.Popen([str(self.binary), "-c", str(config)],
                                      cwd=ROOT, env=self.env,
                                      stdin=subprocess.DEVNULL,
                                      stdout=self.output, stderr=self.output)
        deadline = time.monotonic() + DEADLINES["startup_seconds"]
        while time.monotonic() < deadline:
            require(self.child.poll() is None, f"Server exited during startup: {self.child.returncode}")
            pid_file = self.directory / "lightnvr.pid"
            log = self.log.read_text(errors="replace") if self.log.exists() else ""
            if (pid_file.exists() and pid_file.read_text().strip() == str(self.child.pid)
                    and f"libuv_server_start: Listening on 127.0.0.1:{port}" in log):
                return
            time.sleep(0.1)
        raise AssertionError("Owned server did not bind within startup deadline")

    def stop(self):
        child, self.child = self.child, None
        if child is None:
            return
        timed_out = False
        if child.poll() is None:
            child.terminate()
            try:
                child.wait(timeout=DEADLINES["shutdown_seconds"])
            except subprocess.TimeoutExpired:
                timed_out = True
                child.kill()
                child.wait(timeout=5)
        self.output.close()
        logs = self.process_log.read_text(errors="replace")
        require(not timed_out, "Owned server exceeded graceful shutdown deadline")
        require(child.returncode == 0, f"Native process exit: {child.returncode}")
        require(not any(marker in logs for marker in
                        ("ERROR: AddressSanitizer", "ERROR: LeakSanitizer", "runtime error:",
                         "AddressSanitizer:DEADLYSIGNAL")), "Native sanitizer failure")

    def cleanup(self):
        previous = self.log.read_text(errors="replace").count("Standard cleanup cycle complete")
        self.request("/api/storage/cleanup", "POST", {"aggressive": False})
        deadline = time.monotonic() + DEADLINES["cleanup_seconds"]
        while time.monotonic() < deadline:
            require(self.child.poll() is None, "Native server exited during cleanup")
            if self.log.read_text(errors="replace").count("Standard cleanup cycle complete") > previous:
                return self.request("/api/storage/health")
            time.sleep(0.1)
        raise AssertionError("Forced storage cleanup did not finish within declared deadline")


def catalog_identity(server, recording_id):
    with sqlite3.connect(f"file:{server.db}?mode=ro", uri=True, timeout=2) as db:
        db.row_factory = sqlite3.Row
        row = db.execute("SELECT id,camera_uuid,stream_name,file_path,start_time,end_time,"
                         "size_bytes,protected,is_complete,storage_target_uuid,object_key,"
                         "deletion_pending FROM recordings WHERE id=?", (recording_id,)).fetchone()
        require(row is not None, "Protected recording disappeared from catalog")
        require(db.execute("PRAGMA quick_check").fetchone()[0] == "ok", "Catalog integrity failed")
        return dict(row)


def run_matrix(directory, binary, library):
    recording = directory / "recordings"
    recording.mkdir(exist_ok=True)
    (directory / "models").mkdir()
    control = directory / "fault-mode"
    control.write_text("none")
    server = Server(directory, binary, boundary_environment(directory, library, binary))
    parked = directory / "recordings-unavailable"
    results = []
    try:
        server.start()
        server.request("/api/auth/users/1/password", "PUT",
                       {"old_password": "admin", "new_password": PASSWORD})
        server.password = PASSWORD
        server.request("/api/setup/status", "POST", {"complete": True})
        server.request("/api/streams", "POST", {"name": "storage-fixture", "url": "rtsp://127.0.0.1:9/test",
                       "enabled": False, "streaming_enabled": False, "record": False}, expected=201)
        targets = server.request("/api/storage-targets")["targets"]
        target = next(t for t in targets if t["is_default"])
        target_uuid = target["uuid"]
        server.stop()
        media = recording / "protected.mp4"
        command(["ffmpeg", "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-f", "lavfi",
                 "-i", "color=c=blue:s=64x64:r=10", "-t", "2", "-c:v", "mpeg4", "-an", media],
                log=directory / "ffmpeg-generate.log")
        with sqlite3.connect(server.db) as db:
            camera_uuid = db.execute("SELECT camera_uuid FROM streams WHERE name='storage-fixture'").fetchone()[0]
            now = int(time.time())
            cursor = db.execute("INSERT INTO recordings(stream_name,camera_uuid,file_path,start_time,end_time,"
                                "size_bytes,width,height,fps,codec,is_complete,protected,storage_target_uuid,"
                                "object_key,retention_override_days,disk_pressure_eligible) "
                                "VALUES(?,?,?,?,?,?,?,?,?,?,1,1,?,?,-1,0)",
                                ("storage-fixture", camera_uuid, str(media), now - 2, now,
                                 media.stat().st_size, 64, 64, 10, "mpeg4", target_uuid, "protected.mp4"))
            recording_id = cursor.lastrowid
        initial_hash = sha(media)
        before = catalog_identity(server, recording_id)
        (directory / "catalog-before.json").write_text(json.dumps(before, indent=2) + "\n")
        server.start()
        server.request(f"/api/storage-targets/{target_uuid}/probe", "POST", {})
        for mode in ("full", "readonly", "permission", "unavailable"):
            mode_started = time.monotonic()
            observed_before = (directory / "fault-observed.log").read_text() if (directory / "fault-observed.log").exists() else ""
            try:
                if mode in ("full", "readonly"):
                    control.write_text(mode)
                elif mode == "permission":
                    recording.chmod(0o500)
                    try:
                        descriptor = os.open(recording / "must-not-write", os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
                    except OSError as error:
                        require(error.errno == errno.EACCES, f"Expected kernel EACCES, got {error}")
                    else:
                        os.close(descriptor)
                        raise AssertionError("Permission fault bypassed; qualification requires unprivileged execution")
                else:
                    recording.rename(parked)
                    require(not recording.exists(), "Missing-directory fault was not established")
                rejected = server.request(f"/api/storage-targets/{target_uuid}/probe", "POST", {}, expected=422)
                target_state = server.request(f"/api/storage-targets/{target_uuid}")
                require(target_state["health"]["status"] == "unavailable", f"Fault not reported: {target_state}")
                error_expected = {"full": "No space left", "readonly": "Read-only file system",
                                  "permission": "Permission denied", "unavailable": "unavailable"}[mode]
                require(error_expected in target_state["health"]["last_error"],
                        f"Wrong fault explanation: {target_state['health']}")
                health = server.cleanup()
                if mode == "full":
                    require(health["pressure_level_num"] == 3 and health["free_space_bytes"] == 0,
                            f"Full storage pressure was not observed: {health}")
                current_media = parked / media.name if mode == "unavailable" else media
                require(sha(current_media) == initial_hash, "Protected media changed during fault cleanup")
                require(catalog_identity(server, recording_id) == before, "Protected catalog identity changed during fault")
                if mode in ("full", "readonly"):
                    observed = (directory / "fault-observed.log").read_text()[len(observed_before):]
                    require("mkstemp" in observed, "Server never reached injected write fault")
                results.append({"mode": mode, "kind": "injected-libc" if mode in ("full", "readonly") else "kernel",
                                "status": "passed", "probe_rejection": rejected,
                                "target_health": target_state["health"], "controller_health": health,
                                "protected_sha256": initial_hash, "catalog_identity": before,
                                "fault_seconds": round(time.monotonic() - mode_started, 3)})
            finally:
                control.write_text("none")
                if parked.exists():
                    parked.rename(recording)
                recording.chmod(0o700)
            recovered = server.request(f"/api/storage-targets/{target_uuid}/probe", "POST", {})
            require(recovered["health"]["status"] in ("healthy", "degraded"), "Target did not recover")
            require(recovered["health"]["last_error"] == "", "Recovered target retained old error")
            require(catalog_identity(server, recording_id) == before, "Recovery changed protected catalog identity")
            require(sha(media) == initial_hash, "Recovery changed protected footage")
            results[-1]["recovered_health"] = recovered["health"]
        server.stop()
        server.start()
        require(catalog_identity(server, recording_id) == before, "Restart changed protected catalog identity")
        require(sha(media) == initial_hash, "Restart changed protected footage")
        server.stop()
        command(["ffmpeg", "-hide_banner", "-loglevel", "error", "-xerror", "-nostdin", "-i", media,
                 "-f", "null", "-"], log=directory / "ffmpeg-decode.log")
        probe = json.loads(command(["ffprobe", "-v", "error", "-show_format", "-show_streams", "-of", "json", media],
                                  log=directory / "ffprobe.json"))
        require(float(probe["format"]["duration"]) >= 1.9, "Surviving protected media is truncated")
        return results
    finally:
        control.write_text("none")
        if parked.exists():
            parked.rename(recording)
        recording.chmod(0o700)
        server.stop()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--self-test", action="store_true", help="Test fault isolation only; no server started")
    args = parser.parse_args()
    directory = Path(tempfile.mkdtemp(prefix="lightnvr-storage-fault-"))
    result = {"format_version": 1, "task": "T26", "status": "failed", "directory": str(directory),
              "deadlines": DEADLINES, "uid": os.geteuid(), "cases": []}
    try:
        require(os.geteuid() != 0, "Run unprivileged; root bypasses permission tests")
        binary = args.binary.resolve(strict=True)
        result.update({"binary": str(binary), "binary_sha256": sha(binary), "source": source_evidence(),
                       "python": sys.version, "platform": os.uname()._asdict() if hasattr(os.uname(), "_asdict") else list(os.uname())})
        library, probe = compile_boundary(directory)
        result["boundary_sha256"] = sha(library)
        result["self_tests"] = self_test(directory, library, probe)
        if not args.self_test:
            result["cases"] = run_matrix(directory, binary, library)
        result["status"] = "passed"
    except Exception as error:
        result["error"] = f"{type(error).__name__}: {error}"
        print(result["error"], file=sys.stderr)
    finally:
        # Also restore when a self-test or tool failed before run_matrix's
        # cleanup boundary. Everything here belongs to this invocation.
        try:
            (directory / "fault-mode").write_text("none")
            recording, parked = directory / "recordings", directory / "recordings-unavailable"
            if parked.exists() and not recording.exists():
                parked.rename(recording)
            if recording.is_dir():
                recording.chmod(0o700)
        except OSError as error:
            result["status"] = "failed"
            result["cleanup_error"] = str(error)
        (directory / "result.json").write_text(json.dumps(result, indent=2) + "\n")
        print(f"{result['status']}: {directory / 'result.json'}")
    return 0 if result["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
