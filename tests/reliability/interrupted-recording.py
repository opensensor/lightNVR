#!/usr/bin/env python3
"""Owned synthetic recording/crash/reconciliation qualification; Linux, stdlib only."""
from __future__ import annotations

import argparse
import base64
import hashlib
import http.server
import json
import os
from pathlib import Path
import platform
import signal
import socket
import sqlite3
import subprocess
import tempfile
import threading
import time
import traceback
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
STARTUP_SECONDS = 60
SHUTDOWN_SECONDS = 30
REQUEST_SECONDS = 10
SEGMENT_SECONDS = 6
GOP_SECONDS = 1
STALE_AGE_SECONDS = 7200  # Explicit test-only clock seam; production cutoff is 3600s.
PASSWORD = "ReliabilityFixture123!"


class QualificationBlocked(RuntimeError):
    """A reproduced product gap, distinct from a broken fixture/assertion."""


def sha256(path):
    with Path(path).open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def wait_for(label, predicate, timeout=STARTUP_SECONDS):
    deadline = time.monotonic() + timeout
    while True:
        value = predicate()
        if value:
            return value
        if time.monotonic() >= deadline:
            raise AssertionError(f"{label} did not complete within {timeout}s")
        time.sleep(.1)


def process_identity(pid):
    try:
        data = Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()
        return (data[19], data[0])  # start time, state; PID reuse cannot authorize a kill.
    except (FileNotFoundError, ProcessLookupError):
        return None


def stop_owned_pid(pid, identity, sig=signal.SIGTERM):
    current = process_identity(pid)
    if current is None or current[0] != identity[0] or current[1] == 'Z':
        return
    os.kill(pid, sig)


def reserve_ports(count):
    sockets = []
    try:
        for _ in range(count):
            sock = socket.socket()
            sock.bind(('127.0.0.1', 0))
            sock.listen()
            sockets.append(sock)
        return sockets, [sock.getsockname()[1] for sock in sockets]
    except BaseException:
        for sock in sockets:
            sock.close()
        raise


def run_evidence(command, directory, name, timeout=30):
    result = subprocess.run(command, capture_output=True, timeout=timeout)
    (directory / f'{name}.stdout').write_bytes(result.stdout)
    (directory / f'{name}.stderr').write_bytes(result.stderr)
    (directory / f'{name}.command.json').write_text(json.dumps(command))
    return result


class SyntheticSource:
    """Loopback HTTP MPEG-TS source; each owned FFmpeg producer has a one-second GOP."""
    def __init__(self, directory):
        self.directory = directory
        self.processes = []
        self.lock = threading.Lock()
        self.paused = False
        source = self

        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, fmt, *args):
                with (source.directory / 'source-http.log').open('a') as out:
                    out.write(fmt % args + '\n')

            def do_GET(self):
                if self.path != '/synthetic.ts':
                    self.send_error(404)
                    return
                command = ['ffmpeg', '-nostdin', '-hide_banner', '-loglevel', 'warning',
                           '-re', '-f', 'lavfi', '-i', 'testsrc2=size=320x180:rate=10',
                           '-an', '-c:v', 'libx264', '-preset', 'ultrafast', '-tune', 'zerolatency',
                           '-g', '10', '-keyint_min', '10', '-sc_threshold', '0',
                           '-pix_fmt', 'yuv420p', '-f', 'mpegts', 'pipe:1']
                with source.lock:
                    if source.paused:
                        self.send_error(503, 'Synthetic source is deliberately disconnected')
                        return
                    self.send_response(200)
                    self.send_header('Content-Type', 'video/mp2t')
                    self.end_headers()
                    index = len(source.processes)
                    log = (source.directory / f'source-{index}.log').open('wb')
                    proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=log,
                                            start_new_session=True)
                    source.processes.append((proc, log))
                try:
                    while chunk := proc.stdout.read(1316):
                        self.wfile.write(chunk)
                        self.wfile.flush()
                except (BrokenPipeError, ConnectionResetError):
                    pass
                finally:
                    if proc.poll() is None:
                        proc.terminate()
                        try:
                            proc.wait(timeout=5)
                        except subprocess.TimeoutExpired:
                            proc.kill()
                            proc.wait(timeout=5)
                    proc.stdout.close()
                    log.close()

        self.server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        self.server.daemon_threads = True
        self.port = self.server.server_port
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.url = f'http://127.0.0.1:{self.port}/synthetic.ts'

    def pause(self):
        """Disconnect only owned producers; reject reconnects on the same URL."""
        with self.lock:
            self.paused = True
            affected = []
            for proc, _ in self.processes:
                if proc.poll() is None:
                    affected.append(proc.pid)
                    proc.terminate()
            for proc, _ in self.processes:
                try:
                    proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait(timeout=5)
            return affected

    def resume(self):
        with self.lock:
            self.paused = False

    def stop(self):
        self.pause()
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=5)


class Recorder:
    """Reusable T27/T28 fixture: private config, DB, ports and verified managed child."""
    def __init__(self, binary, go2rtc, directory):
        self.binary = Path(binary).resolve()
        self.go2rtc = Path(go2rtc).resolve()
        self.directory = Path(directory)
        for name in ('recordings', 'models', 'go2rtc', 'hls'):
            (self.directory / name).mkdir(exist_ok=True)
        self.reservations, ports = reserve_ports(4)
        self.port, self.go_port, self.rtsp_port, self.rtc_port = ports
        self.base_url = f'http://127.0.0.1:{self.port}'
        self.db = self.directory / 'lightnvr.db'
        self.config = self.directory / 'lightnvr.ini'
        self.config.write_text(f'''[general]
pid_file = {self.directory}/lightnvr.pid
log_file = {self.directory}/server.log
log_level = 2
syslog_enabled = false
[storage]
path = {self.directory}/recordings
path_hls = {self.directory}/hls
max_size = 0
retention_days = 30
auto_delete_oldest = false
min_free_pct = 1
record_mp4_directly = true
mp4_path = {self.directory}/recordings
mp4_directory_format = flat
mp4_segment_duration = {SEGMENT_SECONDS}
mp4_retention_days = 30
generate_thumbnails = false
[database]
path = {self.db}
[web]
port = {self.port}
bind_ip = 127.0.0.1
root = {ROOT}/web/dist
auth_enabled = true
username = admin
password = admin
[streams]
max_streams = 8
[models]
path = {self.directory}/models
[go2rtc]
enabled = true
binary_path = {self.go2rtc}
config_dir = {self.directory}/go2rtc
api_port = {self.go_port}
rtsp_port = {self.rtsp_port}
webrtc_listen_port = {self.rtc_port}
webrtc_enabled = false
stun_enabled = false
''')
        self.process = None
        self.log = None
        self.generation = 0
        self.managed = {}
        self.password = 'admin'

    def request(self, route, method='GET', body=None, headers=None):
        auth = base64.b64encode(f'admin:{self.password}'.encode()).decode()
        req = urllib.request.Request(self.base_url + route, method=method,
            data=None if body is None else json.dumps(body).encode(),
            headers={'Authorization': f'Basic {auth}', 'Content-Type': 'application/json', **(headers or {})})
        try:
            with urllib.request.urlopen(req, timeout=REQUEST_SECONDS) as response:
                return response.status, dict(response.headers), response.read()
        except urllib.error.HTTPError as error:
            return error.code, dict(error.headers), error.read()

    def json(self, route, method='GET', body=None):
        status, _, data = self.request(route, method, body)
        if status >= 400:
            raise AssertionError(f'{method} {route}: {status} {data.decode()[:500]}')
        return json.loads(data)

    def children(self):
        found = {}
        if not self.process:
            return found
        for entry in Path('/proc').iterdir():
            if not entry.name.isdigit():
                continue
            try:
                stat = (entry / 'stat').read_text().rsplit(')', 1)[1].split()
                args = (entry / 'cmdline').read_bytes().split(b'\0')
                if int(stat[1]) == self.process.pid and str(self.directory / 'go2rtc/go2rtc.yaml').encode() in args:
                    found[int(entry.name)] = process_identity(int(entry.name))
            except (FileNotFoundError, ProcessLookupError, PermissionError):
                continue
        self.managed.update(found)
        return found

    def start(self):
        assert self.process is None or self.process.poll() is not None
        for sock in self.reservations:
            sock.close()
        self.reservations = []
        self.generation += 1
        server_log = self.directory / 'server.log'
        log_offset = server_log.stat().st_size if server_log.exists() else 0
        self.log = (self.directory / f'recorder-{self.generation}.log').open('wb')
        env = {key: value for key, value in os.environ.items() if not key.startswith('LIGHTNVR_')}
        self.process = subprocess.Popen([str(self.binary), '-c', str(self.config)], cwd=self.directory,
                                        env=env, stdout=self.log, stderr=self.log, start_new_session=True)
        started = time.monotonic()

        def ready():
            if self.process.poll() is not None:
                raise AssertionError(f'Recorder exited during startup: {self.process.returncode}')
            try:
                own_pid = int((self.directory / 'lightnvr.pid').read_text().strip()) == self.process.pid
                # Require the owned process's bind log before issuing any mutation.
                with server_log.open() as log_file:
                    log_file.seek(log_offset)
                    log = log_file.read()
                if not own_pid or f'libuv_server_start: Listening on 127.0.0.1:{self.port}' not in log:
                    return False
                return self.request('/api/streams')[0] in (200, 403)
            except (FileNotFoundError, ValueError, urllib.error.URLError, TimeoutError):
                return False

        wait_for('recorder startup', ready)
        wait_for('owned managed go2rtc child', self.children)
        if self.password == 'admin':
            self.json('/api/auth/users/1/password', 'PUT', {'old_password': 'admin', 'new_password': PASSWORD})
            self.password = PASSWORD
        return time.monotonic() - started

    def stop(self, crash=False):
        if self.process is None:
            return
        self.children()
        started = time.monotonic()
        timed_out = False
        if self.process.poll() is None:
            self.process.send_signal(signal.SIGKILL if crash else signal.SIGTERM)
            try:
                self.process.wait(timeout=SHUTDOWN_SECONDS)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=5)
                timed_out = True
        # A killed recorder cannot supervise its child; terminate only captured
        # PID/start-time identities from that recorder's own config directory.
        for pid, identity in self.managed.items():
            if identity:
                stop_owned_pid(pid, identity)
        for pid, identity in self.managed.items():
            if identity:
                deadline = time.monotonic() + 5
                while (current := process_identity(pid)) and current[0] == identity[0] and current[1] != 'Z':
                    if time.monotonic() > deadline:
                        stop_owned_pid(pid, identity, signal.SIGKILL)
                        break
                    time.sleep(.05)
        if self.log:
            self.log.close()
            self.log = None
        log_text = (self.directory / f'recorder-{self.generation}.log').read_text(errors='replace')
        if 'ERROR: AddressSanitizer' in log_text or 'ERROR: LeakSanitizer' in log_text or 'runtime error:' in log_text:
            raise AssertionError('Native sanitizer finding; retained recorder log')
        if timed_out or (not crash and self.process.returncode != 0):
            raise AssertionError(f'Recorder shutdown failed: code={self.process.returncode}, timeout={timed_out}')
        return time.monotonic() - started

    def rows(self):
        with sqlite3.connect(f'file:{self.db}?mode=ro', uri=True, timeout=5) as db:
            db.row_factory = sqlite3.Row
            return [dict(row) for row in db.execute('SELECT id,file_path,start_time,end_time,size_bytes,is_complete,protected,playback_issue FROM recordings ORDER BY id')]


def inspect_media(path, directory, name):
    probe = run_evidence(['ffprobe', '-v', 'error', '-show_format', '-show_streams',
                         '-show_packets', '-select_streams', 'v:0', '-of', 'json', str(path)], directory, name + '-probe')
    decode = run_evidence(['ffmpeg', '-nostdin', '-v', 'error', '-xerror', '-i', str(path),
                          '-map', '0:v:0', '-f', 'null', '-'], directory, name + '-decode')
    payload = json.loads(probe.stdout) if probe.returncode == 0 else {}
    packets = payload.get('packets', [])
    timestamps = [float(p['dts_time']) for p in packets if 'dts_time' in p]
    monotonic = all(b >= a for a, b in zip(timestamps, timestamps[1:]))
    return {'probe_exit': probe.returncode, 'decode_exit': decode.returncode,
            'video_packets': len(packets), 'monotonic_dts': monotonic,
            'first_dts': timestamps[0] if timestamps else None,
            'last_dts': timestamps[-1] if timestamps else None,
            'duration': payload.get('format', {}).get('duration'),
            'sha256': sha256(path), 'bytes': Path(path).stat().st_size}


def source_evidence(binary, go2rtc):
    names = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard',
                                     '--', 'src', 'include', 'CMakeLists.txt', 'tests/reliability'], cwd=ROOT).decode().splitlines()
    digest = hashlib.sha256()
    manifest = []
    for name in sorted(set(names)):
        file = ROOT / name
        if not file.is_file() or '__pycache__' in name:
            continue
        data = file.read_bytes()
        digest.update(name.encode() + b'\0' + data + b'\0')
        manifest.append({'path': name, 'sha256': hashlib.sha256(data).hexdigest()})
    cache_path = Path(binary).resolve().parents[1] / 'CMakeCache.txt'
    selected = {'CMAKE_BUILD_TYPE', 'CMAKE_C_FLAGS', 'CMAKE_CXX_FLAGS', 'BUILD_TESTS', 'ENABLE_SOD', 'ENABLE_LITERT',
                'ENABLE_GO2RTC', 'ENABLE_MQTT', 'ENABLE_SSL', 'GO2RTC_BINARY_PATH', 'YAML_VERSION'}
    cache = {}
    if cache_path.is_file():
        for line in cache_path.read_text().splitlines():
            if '=' in line and ':' in line.split('=', 1)[0]:
                key = line.split(':', 1)[0]
                if key in selected:
                    cache[key] = line.split('=', 1)[1]
    return {'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode().strip(),
            'source_sha256': digest.hexdigest(), 'manifest': manifest,
            'binary_sha256': sha256(binary), 'go2rtc_sha256': sha256(go2rtc),
            'go2rtc_revision': subprocess.check_output(['git', '-C', str(ROOT / 'go2rtc'), 'rev-parse', 'HEAD']).decode().strip(),
            'build_flags': cache, 'environment': {'python': platform.python_version(),
                'platform': platform.platform(),
                'ffmpeg': subprocess.check_output(['ffmpeg', '-version']).decode().splitlines()[0],
                'go': subprocess.check_output(['go', 'version']).decode().strip()}}


def qualify(binary, go2rtc, directory, build_manifest=None, diagnostic_age=False):
    evidence = json.loads(build_manifest.read_text()) if build_manifest else source_evidence(binary, go2rtc)
    assert evidence['binary_sha256'] == sha256(binary), 'Build manifest does not match binary'
    assert evidence['go2rtc_sha256'] == sha256(go2rtc), 'Build manifest does not match go2rtc'
    report = {'format_version': 1, 'scenario': 'interrupted-recording', 'status': 'failed',
              'artifacts': str(directory), 'evidence': evidence,
              'harness_sha256': sha256(__file__),
              'bounds': {'startup_seconds': STARTUP_SECONDS, 'shutdown_seconds': SHUTDOWN_SECONDS,
                         'request_seconds': REQUEST_SECONDS, 'segment_seconds': SEGMENT_SECONDS,
                         'gop_seconds': GOP_SECONDS, 'timestamp_tolerance_seconds': GOP_SECONDS + 1.2,
                         'stale_age_injected_seconds': STALE_AGE_SECONDS if diagnostic_age else 0}, 'observations': {}}
    recorder = Recorder(binary, go2rtc, directory)
    source = SyntheticSource(directory)
    try:
        report['observations']['initial_startup_seconds'] = recorder.start()
        report['observations']['managed_go2rtc_pids'] = list(recorder.managed)
        recorder.json('/api/streams', 'POST', {'name': 'recovery', 'url': source.url,
                      'enabled': True, 'streaming_enabled': True, 'record': True,
                      'record_audio': False, 'segment_duration': SEGMENT_SECONDS})

        def ready_segments():
            rows = recorder.rows()
            complete = [r for r in rows if r['is_complete'] and Path(r['file_path']).is_file()]
            active = [r for r in rows if not r['is_complete'] and Path(r['file_path']).is_file()
                      and Path(r['file_path']).stat().st_size > 4096]
            return rows if len(complete) >= 2 and active else None

        before = wait_for('two completed segments and a nonempty active recording', ready_segments)
        native_log = (directory / 'server.log').read_text()
        assert "Using go2rtc RTSP URL for MP4 recording on stream recovery" in native_log
        assert f"rtsp://localhost:{recorder.rtsp_port}/recovery" in native_log
        report['observations']['managed_rtsp_recording_verified'] = True
        protected = next(row for row in before if row['is_complete'])
        recorder.json(f"/api/recordings/{protected['id']}/protect", 'PUT', {'protected': True})
        for row in before:
            if not row['is_complete']:
                recorder.json(f"/api/recordings/{row['id']}/protect", 'PUT', {'protected': True})
        protected_hash = sha256(protected['file_path'])
        report['observations']['before_crash'] = recorder.rows()
        report['observations']['crash_shutdown_seconds'] = recorder.stop(crash=True)
        crashed = [row for row in recorder.rows() if not row['is_complete']]
        assert crashed, 'Need actual incomplete row at kill point'
        crash_hashes = {row['id']: sha256(row['file_path']) for row in crashed}
        source.stop()
        source = None
        # Stop the fixture camera from creating new rows during recovery. This
        # offline mutation is only after owned recorder + managed child exit.
        with sqlite3.connect(recorder.db) as db:
            db.execute("UPDATE streams SET enabled=0,record=0,streaming_enabled=0 WHERE name='recovery'")
        report['observations']['immediate_restart_seconds'] = recorder.start()
        immediate_started = time.monotonic()
        while True:
            immediate = recorder.rows()
            current = {row['id']: row for row in immediate}
            immediate_complete = all(row['id'] in current and current[row['id']]['is_complete'] for row in crashed)
            if immediate_complete or time.monotonic() - immediate_started >= STARTUP_SECONDS:
                break
            time.sleep(.1)
        report['observations']['immediate_reconciliation_complete'] = immediate_complete
        report['observations']['immediate_reconciliation_wait_seconds'] = time.monotonic() - immediate_started
        report['observations']['immediate_restart_rows'] = immediate
        report['observations']['immediate_playback_status'] = {
            str(row['id']): recorder.request(f"/api/recordings/play/{row['id']}")[0] for row in crashed}
        crashed_ids = {row['id'] for row in crashed}
        def reconciled():
            rows = recorder.rows()
            selected = [row for row in rows if row['id'] in crashed_ids]
            return rows if len(selected) == len(crashed_ids) and all(row['is_complete'] for row in selected) else None
        if not immediate_complete:
            if not diagnostic_age:
                raise QualificationBlocked('Fresh crashed rows did not reconcile within the declared startup bound; no age injection performed.')
            recorder.stop()
            # Optional historical diagnostic only; never the default acceptance.
            with sqlite3.connect(recorder.db) as db:
                for row in crashed:
                    db.execute('UPDATE recordings SET start_time=? WHERE id=? AND is_complete=0',
                               (row['start_time'] - STALE_AGE_SECONDS, row['id']))
            report['observations']['eligible_restart_seconds'] = recorder.start()
            after = wait_for('eligible interrupted rows finalized', reconciled)
        else:
            after = immediate
        report['observations']['recovery_without_age_injection'] = immediate_complete
        report['observations']['after_reconciliation'] = after
        media = {}
        playable_count = 0
        explained_count = 0
        for row in after:
            path = Path(row['file_path'])
            assert path.is_file(), f"Surviving recording disappeared: {row['id']}"
            result = inspect_media(path, directory, f"recording-{row['id']}")
            status, _, body = recorder.request(f"/api/recordings/play/{row['id']}")
            result['playback_status'] = status
            if result['decode_exit'] == 0 and result['video_packets']:
                assert result['probe_exit'] == 0 and result['monotonic_dts']
                assert row['playback_issue'] == '' and status == 200
                playable_count += 1
                duration = float(result['duration'])
                assert duration > 0
                if row['id'] not in crashed_ids:
                    assert abs((row['end_time'] - row['start_time']) - duration) <= GOP_SECONDS + 1.2
            else:
                assert row['playback_issue'], 'Undecodable media must have an explicit issue'
                assert status == 422 and b'interrupted' in body
                explained_count += 1
            media[str(row['id'])] = result
        assert playable_count >= 2, 'Completed pre-crash footage must remain decodable'
        report['observations']['media'] = media
        report['observations']['playable_count'] = playable_count
        report['observations']['explained_unplayable_count'] = explained_count
        preserved = next(row for row in after if row['id'] == protected['id'])
        assert preserved['protected'] and sha256(preserved['file_path']) == protected_hash
        for old in crashed:
            current = next(row for row in after if row['id'] == old['id'])
            assert current['protected'] == old['protected']
            assert sha256(current['file_path']) == crash_hashes[old['id']]
        listed = recorder.json('/api/recordings?limit=100')
        (directory / 'recording-list.json').write_text(json.dumps(listed, indent=2))
        visible = {row['id']: row for row in listed['recordings']}
        for row in after:
            assert row['id'] in visible, f"Recording {row['id']} missing from API list"
            assert visible[row['id']]['playback_issue'] == row['playback_issue']
            assert visible[row['id']]['in_progress'] is False
        recorder.stop()
        report['observations']['idempotent_restart_seconds'] = recorder.start()
        assert recorder.rows() == after, 'Repeated restart changed reconciled catalog identities/metadata'
        for row in after:
            assert sha256(row['file_path']) == media[str(row['id'])]['sha256']
        with sqlite3.connect(f'file:{recorder.db}?mode=ro', uri=True) as db:
            integrity = db.execute('PRAGMA integrity_check').fetchone()[0]
        assert integrity == 'ok'
        report['observations']['database_integrity'] = integrity
        report['observations']['repeated_restart_idempotent'] = True
        report['observations']['stale_eligible_qualification'] = 'passed'
        report['status'] = 'passed' if immediate_complete else 'blocked'
        report['qualification'] = ('Immediate and eligible restart reconciliation verified.' if immediate_complete else
            'Eligible stale crash rows passed. Full interrupted-recording qualification is blocked: '
            'the startup-only scan excludes recent rows, leaving them in_progress until another eligible restart.')
    except QualificationBlocked as error:
        report['status'] = 'blocked'
        report['qualification'] = str(error)
    except BaseException as error:
        report['error'] = str(error)
        (directory / 'failure.txt').write_text(traceback.format_exc())
    finally:
        try:
            recorder.stop()
        except BaseException as error:
            report['status'] = 'failed'
            report['cleanup_error'] = str(error)
        if source:
            try:
                source.stop()
            except BaseException as error:
                report['status'] = 'failed'
                report['source_cleanup_error'] = str(error)
        (directory / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--go2rtc', required=True, type=Path)
    parser.add_argument('--build-manifest', type=Path,
                        help='Source evidence captured at native build completion; binary hashes must match')
    parser.add_argument('--diagnostic-age', action='store_true',
                        help='Historical diagnosis only: age stopped rows after an immediate-recovery failure (never a full pass)')
    parser.add_argument('--output', type=Path, help='New private output directory (never overwrites existing evidence)')
    args = parser.parse_args()
    if args.output:
        args.output.mkdir(mode=0o700, parents=True, exist_ok=False)
        directory = args.output.resolve()
    else:
        directory = Path(tempfile.mkdtemp(prefix='lightnvr-recovery-'))
    report = qualify(args.binary.resolve(), args.go2rtc.resolve(), directory, args.build_manifest, args.diagnostic_age)
    print(json.dumps({'status': report['status'], 'artifacts': str(directory), 'error': report.get('error')}))
    return {'passed': 0, 'failed': 1, 'blocked': 2}[report['status']]


if __name__ == '__main__':
    raise SystemExit(main())
