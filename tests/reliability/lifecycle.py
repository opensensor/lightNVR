#!/usr/bin/env python3
"""Bounded real-source reconnect and overlapping stream/process lifecycle qualification."""
from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import signal
import sqlite3
import statistics
import subprocess
import tempfile
import threading
import time
import traceback
import urllib.error
import urllib.request

spec = importlib.util.spec_from_file_location('recovery_fixture', Path(__file__).with_name('interrupted-recording.py'))
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)

DISCONNECT_SECONDS = 5
RECOVERY_SECONDS = 60
SHUTDOWN_SECONDS = 30
REQUEST_SECONDS = 10
# Gap includes the injected outage, the declared recovery budget, a segment
# needed to observe completed footage, plus GOP/metadata quantization.
MAX_MEDIA_GAP_SECONDS = DISCONNECT_SECONDS + RECOVERY_SECONDS + fixture.SEGMENT_SECONDS + 1.2
FD_JITTER = 8
THREAD_JITTER = 4
STREAM = 'lifecycle'


def go_source_evidence():
    directory = fixture.ROOT / 'go2rtc'
    diff = subprocess.check_output(['git', 'diff', 'HEAD', '--', 'internal/http'], cwd=directory)
    paths = ['internal/http/http.go', 'internal/http/http_test.go']
    return {'revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=directory).decode().strip(),
            'tracked_diff': diff.decode(), 'tracked_diff_sha256': hashlib.sha256(diff).hexdigest(),
            'files': [{'path': path, 'sha256': fixture.sha256(directory / path),
                       'content': (directory / path).read_text()} for path in paths],
            'delivery': 'Uncommitted submodule working-tree repair; pinned revision alone does not contain T27a.'}


def resource_sample(recorder):
    def counts(pid):
        directory = Path(f'/proc/{pid}')
        return {'pid': pid, 'fds': len(list((directory / 'fd').iterdir())),
                'threads': len(list((directory / 'task').iterdir()))}
    children = recorder.children()
    assert len(children) == 1, f'Expected one owned managed child, found {children}'
    samples = []
    for _ in range(5):
        samples.append({'recorder': counts(recorder.process.pid), 'go2rtc': counts(next(iter(children)))})
        time.sleep(.1)
    result = {name: {'pid': samples[0][name]['pid'],
                   **{key: statistics.median(s[name][key] for s in samples) for key in ('fds', 'threads')}}
            for name in ('recorder', 'go2rtc')}
    for item in result.values():
        directory = Path(f"/proc/{item['pid']}")
        targets = {}
        for fd in (directory / 'fd').iterdir():
            try:
                targets[fd.name] = os.readlink(fd)
            except FileNotFoundError:
                continue
        inodes = {target[8:-1] for target in targets.values() if target.startswith('socket:[')}
        item['fd_targets'] = targets
        item['owned_socket_rows'] = {}
        for kind in ('tcp', 'tcp6', 'udp', 'udp6'):
            item['owned_socket_rows'][kind] = [line.split() for line in
                (directory / 'net' / kind).read_text().splitlines()[1:]
                if len(line.split()) > 9 and line.split()[9] in inodes]
    return result


def assert_bounded_resources(samples):
    trends = {}
    for name in ('recorder', 'go2rtc'):
        for key, allowance in [('fds', FD_JITTER), ('threads', THREAD_JITTER)]:
            values = [sample[name][key] for sample in samples]
            trends[f'{name}_{key}'] = values
            assert max(values) <= values[0] + allowance, f'{name} {key} exceeds declared jitter: {values}'
            # All successive steady-state samples rising is different from a
            # short-lived socket/thread fluctuation. Allow one unit of rounding.
            growing = len(values) >= 4 and all(b > a for a, b in zip(values, values[1:]))
            assert not (growing and values[-1] - values[0] > 2), f'Sustained {name} {key} growth: {values}'
    return trends


def completed_rows(recorder):
    return [row for row in recorder.rows() if row['is_complete'] and row['size_bytes'] > 0]


def verify_media(recorder, row, directory, label):
    result = fixture.inspect_media(row['file_path'], directory, label)
    assert result['probe_exit'] == 0 and result['decode_exit'] == 0 and result['video_packets'], result
    assert result['monotonic_dts'], result
    assert row['playback_issue'] == '', row
    status, _, _ = recorder.request(f"/api/recordings/play/{row['id']}")
    assert status == 200, (row['id'], status)
    result['playback_status'] = status
    return result


def state(recorder):
    return recorder.json(f'/api/streams/{STREAM}/recording')


def wait_idle(recorder):
    return fixture.wait_for('manual recorder idle', lambda: not state(recorder)['recording_active'], SHUTDOWN_SECONDS)


def start_manual(recorder):
    response = recorder.json(f'/api/streams/{STREAM}/recording', 'POST', {'action': 'start'})
    assert response['action'] == 'start'
    def nonempty():
        current = state(recorder)
        row = next((r for r in recorder.rows() if r['id'] == current.get('recording_id')), None)
        return row if row and Path(row['file_path']).is_file() and Path(row['file_path']).stat().st_size > 4096 else None
    return fixture.wait_for('manual writer with real media', nonempty, RECOVERY_SECONDS)


def stop_if_active(recorder):
    if state(recorder)['recording_active']:
        status, _, body = recorder.request(f'/api/streams/{STREAM}/recording', 'POST', {'action': 'stop'})
        assert status in (200, 409), (status, body)
    wait_idle(recorder)


def concurrent_actions(recorder, actions):
    barrier = threading.Barrier(len(actions))
    def invoke(action):
        barrier.wait(timeout=5)
        started = time.monotonic()
        status, _, body = recorder.request(f'/api/streams/{STREAM}/recording', 'POST', {'action': action})
        return {'action': action, 'status': status, 'seconds': time.monotonic() - started,
                'response': json.loads(body)}
    with concurrent.futures.ThreadPoolExecutor(max_workers=len(actions)) as executor:
        futures = [executor.submit(invoke, action) for action in actions]
        results = [future.result(timeout=REQUEST_SECONDS + 5) for future in futures]
    for result in results:
        allowed = (202, 409) if result['action'] == 'start' else (200, 409)
        assert result['status'] in allowed, results
    return results


def concurrent_process_shutdown(recorder):
    """One SIGTERM overlaps real stop/start HTTP requests, then one serial cleanup."""
    barrier = threading.Barrier(3)
    recorder.children()
    managed = dict(recorder.managed)
    identity = fixture.process_identity(recorder.process.pid)
    def request(action):
        barrier.wait(timeout=5)
        started = time.monotonic()
        try:
            status, _, body = recorder.request(f'/api/streams/{STREAM}/recording', 'POST', {'action': action})
            return {'action': action, 'status': status, 'response': body.decode(errors='replace')[:500],
                    'seconds': time.monotonic() - started}
        except (urllib.error.URLError, TimeoutError, ConnectionError, OSError) as error:
            return {'action': action, 'transport_closed': type(error).__name__,
                    'seconds': time.monotonic() - started}
    with concurrent.futures.ThreadPoolExecutor(max_workers=2) as executor:
        futures = [executor.submit(request, action) for action in ('stop', 'start')]
        barrier.wait(timeout=5)
        started = time.monotonic()
        fixture.stop_owned_pid(recorder.process.pid, identity, signal.SIGTERM)
        recorder.process.wait(timeout=SHUTDOWN_SECONDS)
        elapsed = time.monotonic() - started
        results = [future.result(timeout=REQUEST_SECONDS + 5) for future in futures]
    assert recorder.process.returncode == 0, f'Nonzero shutdown: {recorder.process.returncode}'
    for result in results:
        if 'status' in result:
            allowed = (202, 409) if result['action'] == 'start' else (200, 409)
            assert result['status'] in allowed, results
    # Inspect before Recorder.stop's safety cleanup, so it cannot hide a child
    # leaked by the service's own shutdown.
    leaked = []
    for pid, old in managed.items():
        current = fixture.process_identity(pid)
        if old and current and current[0] == old[0] and current[1] != 'Z':
            leaked.append(pid)
    assert not leaked, f'Managed children outlived graceful recorder shutdown: {leaked}'
    recorder.stop()  # Process has exited: this sends no second signal.
    return {'seconds': elapsed, 'exit_code': recorder.process.returncode,
            'requests': results, 'owned_children_live_before_safety_cleanup': leaked}


def check_preserved(recorder, protected, hashes):
    rows = {row['id']: row for row in recorder.rows()}
    for recording_id, original in protected.items():
        assert recording_id in rows, f'Protected recording disappeared: {recording_id}'
        row = rows[recording_id]
        assert row['protected'] and row['file_path'] == original['file_path']
        assert row['start_time'] == original['start_time'] and row['is_complete']
        assert fixture.sha256(row['file_path']) == hashes[recording_id]


def qualify(binary, go2rtc, directory, cycles, build_manifest=None):
    (directory / 'lifecycle.py.snapshot').write_bytes(Path(__file__).read_bytes())
    (directory / 'interrupted-recording.py.snapshot').write_bytes(Path(fixture.__file__).read_bytes())
    evidence = json.loads(build_manifest.read_text()) if build_manifest else fixture.source_evidence(binary, go2rtc)
    if not build_manifest:
        evidence['go2rtc_source'] = go_source_evidence()
    assert evidence['binary_sha256'] == fixture.sha256(binary)
    assert evidence['go2rtc_sha256'] == fixture.sha256(go2rtc)
    report = {'format_version': 1, 'scenario': 'reconnect-and-lifecycle', 'status': 'failed',
        'artifacts': str(directory), 'evidence': evidence,
        'harness_sha256': fixture.sha256(__file__), 'shared_fixture_sha256': fixture.sha256(fixture.__file__),
        'bounds': {'cycles': cycles, 'disconnect_seconds': DISCONNECT_SECONDS,
                   'recovery_seconds': RECOVERY_SECONDS, 'shutdown_seconds': SHUTDOWN_SECONDS,
                   'request_seconds': REQUEST_SECONDS, 'max_media_gap_seconds': MAX_MEDIA_GAP_SECONDS,
                   'gop_seconds': fixture.GOP_SECONDS, 'fd_jitter': FD_JITTER, 'thread_jitter': THREAD_JITTER},
        'observations': {'reconnect_cycles': [], 'stream_races': [], 'shutdown_cycles': []}}
    recorder = fixture.Recorder(binary, go2rtc, directory)
    source = fixture.SyntheticSource(directory)
    try:
        recorder.start()
        recorder.json('/api/streams', 'POST', {'name': STREAM, 'url': source.url,
            'enabled': True, 'streaming_enabled': True, 'record': True,
            'record_audio': False, 'segment_duration': fixture.SEGMENT_SECONDS})
        baseline = fixture.wait_for('two initial completed recordings',
            lambda: (rows if len(rows := completed_rows(recorder)) >= 2 else None), RECOVERY_SECONDS)
        protected = {baseline[0]['id']: baseline[0]}
        recorder.json(f"/api/recordings/{baseline[0]['id']}/protect", 'PUT', {'protected': True})
        hashes = {row['id']: fixture.sha256(row['file_path']) for row in protected.values()}
        report['observations']['protected_baseline'] = list(protected.values())
        report['observations']['baseline_media'] = verify_media(recorder, baseline[0], directory, 'baseline')
        # Warm the same request paths before fixed-phase resource observations.
        for _ in range(5):
            state(recorder)
        resources = [resource_sample(recorder)]
        for cycle in range(cycles):
            before = completed_rows(recorder)
            latest = max(before, key=lambda row: row['end_time'])
            max_id = max(row['id'] for row in recorder.rows())
            started_wall, started = time.time(), time.monotonic()
            killed = source.pause()
            assert killed, 'Source fault did not interrupt an active owned producer'
            try:
                urllib.request.urlopen(source.url, timeout=2)
            except urllib.error.HTTPError as error:
                assert error.code == 503
                error.close()
            else:
                raise AssertionError('Paused source accepted a connection')
            assert all(proc.poll() is not None for proc, _ in source.processes)
            while time.monotonic() - started < DISCONNECT_SECONDS:
                assert recorder.process.poll() is None, 'Recorder died during source outage'
                time.sleep(.1)
            source.resume()
            resumed_wall, resumed = time.time(), time.monotonic()
            def recovered():
                rows = [row for row in completed_rows(recorder)
                        if row['id'] > max_id and row['start_time'] >= resumed_wall - fixture.GOP_SECONDS - 1]
                return min(rows, key=lambda row: row['id']) if rows else None
            row = fixture.wait_for('new completed media after source reconnect', recovered, RECOVERY_SECONDS)
            elapsed = time.monotonic() - resumed
            media = verify_media(recorder, row, directory, f'reconnect-{cycle}')
            gap = max(0, row['start_time'] - latest['end_time'])
            assert gap <= MAX_MEDIA_GAP_SECONDS, f'Media gap {gap}s exceeds declared bound'
            check_preserved(recorder, protected, hashes)
            resources.append(resource_sample(recorder))
            report['observations']['reconnect_cycles'].append({
                'cycle': cycle, 'paused_at': started_wall, 'resumed_at': resumed_wall,
                'owned_source_pids_stopped': killed, 'completed_media_recovery_seconds': elapsed,
                'previous_recording_id': latest['id'], 'recovered_recording_id': row['id'],
                'catalog_gap_seconds': gap, 'media': media, 'resources': resources[-1]})
        report['observations']['reconnect_resource_trends'] = assert_bounded_resources(resources)
        # PUT acknowledges asynchronous configuration work. Establish the new
        # mode before testing manual lifecycle races; policy-update concurrency
        # is a different scenario. Read only this private server's new log tail.
        server_log = recorder.directory / 'server.log'
        offset = server_log.stat().st_size
        recorder.json(f'/api/streams/{STREAM}', 'PUT', {'record': False})
        def mode_applied():
            with server_log.open() as log:
                log.seek(offset)
                return f'Successfully completed stream update for: {STREAM}' in log.read()
        fixture.wait_for('continuous recording mode change applied', mode_applied, SHUTDOWN_SECONDS)
        wait_idle(recorder)
        report['observations']['manual_mode_setup'] = 'Awaited private asynchronous PUT worker completion before races'
        manual_resources = [resource_sample(recorder)]
        for cycle in range(cycles):
            start_manual(recorder)
            responses = concurrent_actions(recorder, ['stop', 'stop', 'start'])
            stop_if_active(recorder)
            check_preserved(recorder, protected, hashes)
            manual_resources.append(resource_sample(recorder))
            report['observations']['stream_races'].append({'cycle': cycle, 'requests': responses,
                                                         'resources_after_idle': manual_resources[-1]})
        report['observations']['manual_resource_trends'] = assert_bounded_resources(manual_resources)
        restart_resources = []
        for cycle in range(cycles):
            start_manual(recorder)
            stopped = concurrent_process_shutdown(recorder)
            restarted = recorder.start()
            check_preserved(recorder, protected, hashes)
            fixture.wait_for('startup reconciliation after lifecycle overlap',
                lambda: all(row['is_complete'] for row in recorder.rows()), RECOVERY_SECONDS)
            restart_resources.append(resource_sample(recorder))
            report['observations']['shutdown_cycles'].append({
                'cycle': cycle, 'shutdown': stopped, 'restart_seconds': restarted,
                'resources_after_restart': restart_resources[-1]})
        if restart_resources:
            report['observations']['restart_resource_trends'] = assert_bounded_resources(restart_resources)
        for row in completed_rows(recorder):
            if row['playback_issue']:
                status, _, _ = recorder.request(f"/api/recordings/play/{row['id']}")
                assert status == 422
            else:
                verify_media(recorder, row, directory, f"final-{row['id']}")
        with sqlite3.connect(f'file:{recorder.db}?mode=ro', uri=True) as db:
            assert db.execute('PRAGMA integrity_check').fetchone()[0] == 'ok'
        report['observations']['final_rows'] = recorder.rows()
        report['observations']['database_integrity'] = 'ok'
        report['status'] = 'passed'
        report['qualification'] = ('Owned source disconnect/reconnect, overlapping manual stop/stop/start, '
            'and one SIGTERM overlapping stop/start HTTP requests; same-path/clock-rollback collisions excluded.')
    except Exception as error:
        report['error'] = str(error)
        (directory / 'failure.txt').write_text(traceback.format_exc())
    finally:
        fixture.finalize_report(report, directory, [
            ('cleanup_error', recorder.stop), ('source_cleanup_error', source.stop)])
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--go2rtc', required=True, type=Path)
    parser.add_argument('--build-manifest', type=Path)
    parser.add_argument('--cycles', type=int, default=3)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    if not 3 <= args.cycles <= 10:
        parser.error('--cycles must be between 3 and 10')
    if args.output:
        args.output.mkdir(mode=0o700, parents=True, exist_ok=False)
        directory = args.output.resolve()
    else:
        directory = Path(tempfile.mkdtemp(prefix='lightnvr-lifecycle-'))
    report = qualify(args.binary.resolve(), args.go2rtc.resolve(), directory, args.cycles, args.build_manifest)
    print(json.dumps({'status': report['status'], 'artifacts': str(directory), 'error': report.get('error')}))
    return 0 if report['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
