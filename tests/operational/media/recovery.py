#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Real RTSP source loss/restart, MP4 decode, timestamp and SQLite assertions.

Owns fresh loopback listeners and child identities under the T13 runtime lease.
Never attaches to an existing NVR/go2rtc or signals by process name. Retains all
configuration, logs, reports and small synthetic recordings under build/.
"""
import argparse
import base64
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import signal
import socket
import sqlite3
import subprocess
import sys
import time
import urllib.error
import urllib.request
import uuid

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('native_runner', ROOT / 'scripts/quality/run-native.py')
native_runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(native_runner)
LIMITS = json.loads((ROOT / 'tests/fixtures/eng/media/scenarios.json').read_text())


def wait_for(predicate, seconds, description):
    deadline = time.monotonic() + seconds
    last_error = None
    while time.monotonic() < deadline:
        try:
            result = predicate()
            if result:
                return result
        except (OSError, sqlite3.Error, urllib.error.URLError) as exc:
            last_error = str(exc)
        time.sleep(0.2)
    raise AssertionError(f'{description} exceeded {seconds}s: {last_error}')


def free_port():
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


def descendants(parent):
    processes = {}
    for item in Path('/proc').iterdir():
        if not item.name.isdigit():
            continue
        try:
            stat = (item / 'stat').read_text()
            fields = stat[stat.rindex(')') + 2:].split()
            processes[int(item.name)] = int(fields[1])
        except (OSError, ValueError):
            continue
    owned = {parent}
    while True:
        expanded = owned | {pid for pid, ppid in processes.items() if ppid in owned}
        if expanded == owned:
            return owned
        owned = expanded


def stop(process, deadline):
    # Pin observed descendants before their parent exits and reparents them.
    handles = []
    for pid in descendants(process.pid):
        try:
            handles.append((pid, os.pidfd_open(pid)))
        except ProcessLookupError:
            pass
    forced = False
    try:
        for _, handle in reversed(handles):
            try:
                signal.pidfd_send_signal(handle, signal.SIGTERM)
            except ProcessLookupError:
                pass
        try:
            process.wait(timeout=deadline)
        except subprocess.TimeoutExpired:
            forced = True
        # Any observed child still alive is an owned cleanup failure. Kill only
        # those pinned identities; PID reuse cannot target an unrelated process.
        for pid, handle in handles:
            try:
                signal.pidfd_send_signal(handle, 0)
                if pid != process.pid:
                    # Give orderly go2rtc/FFmpeg teardown a short bounded grace.
                    import select
                    if select.select([handle], [], [], 2)[0]:
                        continue
                forced = True
                signal.pidfd_send_signal(handle, signal.SIGKILL)
            except ProcessLookupError:
                pass
        process.wait(timeout=5)
    finally:
        for _, handle in handles:
            os.close(handle)
    return {'pid':process.pid,'observed_pids':[pid for pid,_ in handles],
            'returncode':process.returncode,'forced':forced}


def api(port, route, payload=None):
    request = urllib.request.Request(f'http://127.0.0.1:{port}/api/{route}',
        data=json.dumps(payload).encode() if payload is not None else None,
        headers={'Authorization':'Basic ' + base64.b64encode(b'admin:admin').decode(),
                 'Content-Type':'application/json'})
    with urllib.request.urlopen(request, timeout=5) as response:
        return json.load(response)


def completed(database):
    if not database.exists():
        return []
    with sqlite3.connect(f'file:{database}?mode=ro', uri=True) as db:
        db.row_factory = sqlite3.Row
        return [dict(row) for row in db.execute(
            'SELECT id,file_path,start_time,end_time,size_bytes FROM recordings '
            'WHERE stream_name=? AND is_complete=1 ORDER BY start_time,id', ('media-recovery',))]


def probe_recording(row, audio, output):
    media = Path(row['file_path'])
    if media.stat().st_size != row['size_bytes']:
        raise AssertionError(f'DB/file size differs: {media}')
    command = ['ffprobe','-v','error','-show_format','-show_streams','-show_packets',
               '-show_data_hash','sha256','-of','json',str(media)]
    result = subprocess.run(command, capture_output=True, text=True, timeout=15, check=True)
    report = json.loads(result.stdout)
    output.write_text(json.dumps(report, indent=2) + '\n')
    streams = report['streams']
    video = next(s for s in streams if s['codec_type'] == 'video')
    assert video['codec_name'] == 'h264', video
    audio_streams = [s for s in streams if s['codec_type'] == 'audio']
    assert bool(audio_streams) == audio, streams
    if audio:
        assert audio_streams[0]['codec_name'] == 'aac', audio_streams
    duration = float(report['format']['duration'])
    assert abs(duration - LIMITS['segment_seconds']) <= LIMITS['segment_duration_tolerance_seconds'], duration
    assert abs((row['end_time'] - row['start_time']) - duration) <= LIMITS['database_time_tolerance_seconds'], row
    previous = {}
    counts = {}
    videos = []
    for packet in report['packets']:
        index = packet['stream_index']
        dts = float(packet['dts_time'])
        if index in previous:
            assert dts > previous[index], f'Non-monotonic DTS {media}: {packet}'
        previous[index] = dts
        counts[index] = counts.get(index, 0) + 1
        if index == video['index']:
            videos.append(packet)
    assert len(videos) >= LIMITS['fps'] * (LIMITS['segment_seconds'] - 1), counts
    assert 'K' in videos[0]['flags'], 'Segment does not start with a keyframe'
    if audio:
        assert counts.get(audio_streams[0]['index'], 0) > 0, counts
    decode = subprocess.run(['ffmpeg','-v','error','-xerror','-i',str(media),'-map','0:v:0',
        '-map','0:a?','-f','null','-'], capture_output=True, text=True, timeout=20)
    output.with_suffix('.decode.log').write_text(decode.stderr)
    assert decode.returncode == 0, decode.stderr
    return {'row':row,'duration':duration,'video_packets':len(videos),
            'first_hash':videos[0]['data_hash'],'last_hash':videos[-1]['data_hash'],
            'first_two_hashes':[p['data_hash'] for p in videos[:2]],
            'last_two_hashes':[p['data_hash'] for p in videos[-2:]],
            'sha256':hashlib.sha256(media.read_bytes()).hexdigest()}


def check_sequence(reports):
    assert len(reports) >= LIMITS['consecutive_segments'], 'Missing complete media sequence'
    for previous, current in zip(reports, reports[1:]):
        gap = current['row']['start_time'] - previous['row']['end_time']
        assert abs(gap) <= LIMITS['continuous_gap_tolerance_seconds'], f'DB segment gap/overlap: {gap}s'
        assert current['row']['start_time'] > previous['row']['start_time'], 'DB ordering regressed'
        # Existing recorder deliberately retains one boundary keyframe. Two
        # repeated packets exceed the declared one-video-frame overlap tolerance.
        assert previous['last_two_hashes'] != current['first_two_hashes'], 'Multiple duplicated boundary frames'


def scenario(binary, go2rtc, case, directory):
    directory.mkdir(parents=True)
    web_port, api_port, rtsp_port = free_port(), free_port(), free_port()
    while len({web_port, api_port, rtsp_port}) != 3:
        web_port, api_port, rtsp_port = free_port(), free_port(), free_port()
    for subdir in ('recordings/mp4','recordings/hls','models'):
        (directory / subdir).mkdir(parents=True)
    ini = (ROOT / 'config/lightnvr-health-test.ini').read_text().replace('/tmp/lightnvr-test',str(directory))
    ini = ini.replace('port = 18080',f'port = {web_port}').replace('root = ./web/dist',f'root = {ROOT}/web/dist')
    ini = ini.replace('[mqtt]\nenabled = true','[mqtt]\nenabled = false').replace('record_mp4_directly = false','record_mp4_directly = true')
    ini = ini.replace('mp4_segment_duration = 60',f"mp4_segment_duration = {LIMITS['segment_seconds']}")
    (directory / 'lightnvr.ini').write_text(ini)
    ffmpeg = 'ffmpeg -hide_banner -loglevel warning -re -f lavfi -i testsrc2=size=160x96:rate=10 '
    if case['audio']:
        ffmpeg += '-re -f lavfi -i sine=frequency=1000:sample_rate=48000 '
    ffmpeg += '-c:v libx264 -preset ultrafast -tune zerolatency -pix_fmt yuv420p -threads 1 -g 10 -keyint_min 10 -sc_threshold 0 -bf 0 '
    ffmpeg += '-c:a aac -ar 48000 -ac 1 ' if case['audio'] else '-an '
    ffmpeg += '-rtsp_transport tcp -f rtsp {output}'
    source_config = {'api':{'listen':f'127.0.0.1:{api_port}'},'rtsp':{'listen':f'127.0.0.1:{rtsp_port}'},
                     'webrtc':{'listen':''},'streams':{'fixture':'exec:' + ffmpeg}}
    # JSON is valid YAML and avoids a runtime YAML dependency.
    (directory / 'go2rtc.yaml').write_text(json.dumps(source_config))
    env = native_runner.build_env()
    env['LD_LIBRARY_PATH'] = str(binary.parent)
    logs, processes, cleanup = [], [], []
    outcome = {'scenario':case,'limits':LIMITS,'failures':[]}
    def launch(command, name):
        log = (directory / name).open('w'); logs.append(log)
        proc = subprocess.Popen(command, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        processes.append(proc)
        return proc
    try:
        source = launch([str(go2rtc),'-config',str(directory / 'go2rtc.yaml')],'source-before.log')
        server = launch([str(binary),'-c',str(directory / 'lightnvr.ini')],'server.log')
        wait_for(lambda: api(web_port,'system/info'),30,'server startup')
        api(web_port,'streams',{'name':'media-recovery','url':f'rtsp://127.0.0.1:{rtsp_port}/fixture',
            'enabled':True,'streaming_enabled':False,'width':160,'height':96,'fps':10,'codec':'h264',
            'record':True,'record_audio':case['audio'],'segment_duration':LIMITS['segment_seconds']})
        database = directory / 'lightnvr.db'
        before = wait_for(lambda: rows if len(rows := completed(database)) >= LIMITS['consecutive_segments'] else None,
                          LIMITS['startup_deadline_seconds'],'initial segments')[:LIMITS['consecutive_segments']]
        paused_count = None
        free_pct = None
        if case.get('pause_during_restore'):
            # Change only this fresh test NVR's configurable thresholds. Actual
            # statvfs -> controller -> storage_should_pause_recording stays real.
            with sqlite3.connect(database) as db:
                db.execute('UPDATE recordings SET protected=1,disk_pressure_eligible=0')
            free_pct = float(api(web_port,'storage/health')['free_space_pct'])
            assert 0 < free_pct < 100, 'Cannot induce bounded pressure thresholds on this filesystem'
            emergency = (free_pct + 100) / 2
            pressure_settings = {'storage_pressure_warning_pct':100,
                'storage_pressure_critical_pct':(emergency + 100) / 2,
                'storage_pressure_emergency_pct':emergency}
            outcome['pressure_settings'] = pressure_settings
            api(web_port,'settings',pressure_settings)
            api(web_port,'storage/cleanup',{'aggressive':False})
            wait_for(lambda: api(web_port,'storage/health')['pressure_level'].lower() == 'emergency',10,'real pressure state')
            # Allow the one already-open segment to finish, then count all rows,
            # including open recordings. No new segment may open while paused.
            time.sleep(LIMITS['segment_seconds'] + 2)
            with sqlite3.connect(database) as db:
                db.execute('UPDATE recordings SET protected=1,disk_pressure_eligible=0')
                paused_count = db.execute('SELECT count(*) FROM recordings').fetchone()[0]
        cleanup.append(stop(source,10)); processes.remove(source)
        time.sleep(LIMITS['disconnect_seconds'])
        baseline_ids = {row['id'] for row in completed(database)}
        restored_wall = time.time(); restored = time.monotonic()
        source = launch([str(go2rtc),'-config',str(directory / 'go2rtc.yaml')],'source-after.log')
        if paused_count is not None:
            time.sleep(LIMITS['segment_seconds'] + 2)
            with sqlite3.connect(database) as db:
                actual_count = db.execute('SELECT count(*) FROM recordings').fetchone()[0]
            assert actual_count == paused_count, f'Recording opened while emergency pressure was active: {paused_count}->{actual_count}'
            outcome['rows_while_paused'] = actual_count
            api(web_port,'settings',{'storage_pressure_warning_pct':free_pct / 2,
                'storage_pressure_critical_pct':free_pct / 3,'storage_pressure_emergency_pct':free_pct / 4})
            api(web_port,'storage/cleanup',{'aggressive':False})
            wait_for(lambda: api(web_port,'storage/health')['pressure_level'].lower() == 'normal',10,'pressure recovery')
            restored = time.monotonic()
            restored_wall = time.time()
        after = wait_for(lambda: rows if len(rows := [r for r in completed(database)
            if r['id'] not in baseline_ids and r['start_time'] >= restored_wall]) >= LIMITS['consecutive_segments'] else None,
            LIMITS['recovery_deadline_seconds'],'completed post-reconnect segments')[:LIMITS['consecutive_segments']]
        outcome['recovery_seconds'] = time.monotonic() - restored
        outcome['before'], outcome['after'] = [], []
        for phase, rows in [('before',before),('after',after)]:
            for index, row in enumerate(rows):
                try:
                    outcome[phase].append(probe_recording(row,case['audio'],directory / f'{phase}-{index}.ffprobe.json'))
                except (AssertionError, OSError, KeyError, ValueError, StopIteration, subprocess.SubprocessError) as exc:
                    outcome['failures'].append(f'{phase} segment {index}: {exc}')
            try:
                check_sequence(outcome[phase])
            except AssertionError as exc:
                outcome['failures'].append(f'{phase}: {exc}')
        assert after[0]['start_time'] > before[-1]['start_time'], 'Time regressed across reconnect'
        assert server.poll() is None, 'NVR exited during recovery'
    except Exception as exc:
        outcome['failures'].append(f'{type(exc).__name__}: {exc}')
    finally:
        for process in reversed(processes):
            cleanup.append(stop(process,LIMITS['shutdown_deadline_seconds']))
        for log in logs:
            log.close()
        outcome['cleanup'] = cleanup
        if any(row['forced'] for row in cleanup):
            outcome['failures'].append('Owned process required forced shutdown')
        (directory / 'result.json').write_text(json.dumps(outcome,indent=2) + '\n')
    return outcome


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary',required=True,type=Path)
    parser.add_argument('--go2rtc',type=Path,default=ROOT / 'go2rtc/go2rtc')
    parser.add_argument('--scenario',choices=[c['id'] for c in LIMITS['scenarios']])
    args = parser.parse_args()
    binary, go2rtc = args.binary.resolve(strict=True), args.go2rtc.resolve(strict=True)
    artifacts = ROOT / 'build/eng-artifacts/T27' / ('media-' + uuid.uuid4().hex[:12])
    artifacts.mkdir(parents=True)
    outcomes = []
    with native_runner.runtime_lock():
        for case in LIMITS['scenarios']:
            if not args.scenario or args.scenario == case['id']:
                outcomes.append(scenario(binary,go2rtc,case,artifacts / case['id']))
    metadata = {'binary':str(binary),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
                'go2rtc':str(go2rtc),'go2rtc_sha256':hashlib.sha256(go2rtc.read_bytes()).hexdigest(),
                'revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                'ffmpeg_version':subprocess.check_output(['ffmpeg','-version'],text=True).splitlines()[0],
                'ffprobe_version':subprocess.check_output(['ffprobe','-version'],text=True).splitlines()[0],
                'results':outcomes}
    (artifacts / 'run.json').write_text(json.dumps(metadata,indent=2) + '\n')
    print(artifacts)
    return 1 if any(row['failures'] for row in outcomes) else 0


if __name__ == '__main__':
    raise SystemExit(main())
