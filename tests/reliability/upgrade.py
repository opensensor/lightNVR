#!/usr/bin/env python3
"""Upgrade a copy of an installation created by the pinned historical binary."""
from __future__ import annotations

import argparse
import base64
from contextlib import closing
import importlib.util
import json
from pathlib import Path
import shutil
import sqlite3
import subprocess
import tempfile
import time
import traceback

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('recovery_fixture', Path(__file__).with_name('interrupted-recording.py'))
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)


class UpgradeRecorder(fixture.Recorder):
    def rows(self):
        # The prior release predates playback_issue. Read its genuine schema;
        # never add a column or invoke current migrations while seeding it.
        with closing(sqlite3.connect(f'file:{self.db}?mode=ro', uri=True, timeout=5)) as db:
            db.row_factory = sqlite3.Row
            return [dict(row) for row in db.execute('SELECT * FROM recordings ORDER BY id')]


def git(checkout, *args):
    return subprocess.check_output(['git', '-C', str(checkout), *args], text=True).strip()


def require_status(recorder, route, expected, headers=None, method='GET', body=None):
    status, _, payload = recorder.request(route, method, body, headers)
    assert status == expected, f'{method} {route}: expected {expected}, got {status}: {payload[:250]!r}'
    return payload


def basic(username, password):
    return {'Authorization': 'Basic ' + base64.b64encode(f'{username}:{password}'.encode()).decode()}


def schema_snapshot(database):
    # Called only after graceful stop and before sealing the original. Honor
    # its real WAL if present; immutable=1 would silently ignore committed WAL
    # transactions. The complete installation, including sidecars, is copied.
    with closing(sqlite3.connect(f'file:{database}?mode=ro', uri=True)) as db:
        db.row_factory = sqlite3.Row
        migrations = [dict(row) for row in db.execute('SELECT * FROM schema_migrations ORDER BY version')]
        return {'integrity': db.execute('PRAGMA integrity_check').fetchone()[0],
                'foreign_key_errors': [list(row) for row in db.execute('PRAGMA foreign_key_check')],
                'schema_version': db.execute('PRAGMA schema_version').fetchone()[0],
                'migration_rows': migrations,
                'recording_columns': [row['name'] for row in db.execute('PRAGMA table_info(recordings)')],
                'objects': [dict(row) for row in db.execute(
                    "SELECT type,name,tbl_name,sql FROM sqlite_master WHERE name NOT LIKE 'sqlite_%' ORDER BY type,name")]}


def hashes(directory):
    return {str(path.relative_to(directory)): fixture.sha256(path)
            for path in sorted(directory.rglob('*')) if path.is_file()}


def path_from_manifest(value):
    path = Path(value)
    return path if path.is_absolute() else ROOT / path


def seed(recorder, source):
    recorder.json('/api/setup/status', 'POST', {'complete': True})
    recorder.json('/api/streams', 'POST', {'name': 'upgrade', 'url': source.url,
        'enabled': True, 'streaming_enabled': True, 'record': True,
        'record_audio': False, 'segment_duration': fixture.SEGMENT_SECONDS})
    recorder.json('/api/streams', 'POST', {'name': 'denied', 'url': 'rtsp://fixture.invalid/denied',
        'enabled': False, 'streaming_enabled': False, 'record': False})
    camera = recorder.json('/api/streams/upgrade/full')['stream']
    user = recorder.json('/api/auth/users', 'POST', {'username': 'upgrade_viewer',
        'password': fixture.PASSWORD, 'role': 2, 'is_active': True})
    scope = {'type': 'selector', 'selector': {'version': 1,
        'expression': {'op': 'camera_uuid', 'values': [camera['camera_uuid']]}}}
    policy = recorder.json(f"/api/authorization/users/{user['id']}")
    recorder.json(f"/api/authorization/users/{user['id']}", 'PUT', {'mode': 'policy',
        'expected_policy_version': policy['policy_version'], 'grants': [{
            'role_uuid': '00000000-0000-4000-8000-000000000003', 'scope': scope}]})
    token = recorder.json('/api/authorization/users/1/tokens', 'POST', {
        'description': 'upgrade-read-only', 'expires_at': int(time.time()) + 3600,
        'actions': ['live.view'], 'scope': scope})
    return {'user_id': user['id'], 'camera_uuid': camera['camera_uuid'], 'token': token['secret']}


def auth_checks(recorder, principals, current=False):
    evidence = {}
    for name, headers in [('viewer', basic('upgrade_viewer', fixture.PASSWORD)),
                          ('token', {'Authorization': 'Bearer ' + principals['token']})]:
        allowed = json.loads(require_status(recorder, '/api/streams/upgrade/full', 200, headers))['stream']
        assert allowed['camera_uuid'] == principals['camera_uuid']
        assert not allowed.get('password'), 'Read-only response exposed camera password'
        require_status(recorder, '/api/streams/denied/full', 403, headers)
        require_status(recorder, '/api/streams/upgrade', 403, headers, 'PUT', {'enabled': not allowed['enabled']})
        unchanged = json.loads(require_status(recorder, '/api/streams/upgrade/full', 200, headers))['stream']
        assert unchanged['enabled'] == allowed['enabled'], 'Denied write changed camera configuration'
        evidence[name] = {'allowed_read': 200, 'denied_camera': 403, 'denied_write': 403, 'denied_write_unchanged': True}
        if current:
            projection = json.loads(require_status(recorder, '/api/openapi.json', 200, headers))
            assert 'get' in projection['paths']['/api/streams/{name}/full']
            assert 'put' not in projection['paths'].get('/api/streams/{name}', {})
            evidence[name]['discovery_read_only'] = True
    require_status(recorder, '/api/streams', 401, {'Authorization': ''})
    require_status(recorder, '/api/streams/denied/full', 200)
    evidence['anonymous'] = 401
    evidence['admin_other_camera'] = 200
    return evidence


def media_check(recorder, row, artifacts, label):
    payload = require_status(recorder, f"/api/recordings/play/{row['id']}", 200)
    assert len(payload) > 4096, 'Playback response is not a substantive recording'
    media = fixture.inspect_media(Path(row['file_path']), artifacts, label)
    assert media['probe_exit'] == media['decode_exit'] == 0 and media['video_packets'] > 0
    assert media['monotonic_dts']
    listing = recorder.json('/api/recordings?limit=100')
    assert any(int(item['id']) == row['id'] for item in listing['recordings'])
    return media


def relocate_copy(original, copied, prior, current):
    """Explicit fixture relocation only; preserve schema/migration history."""
    old_text = (original / 'lightnvr.ini').read_text()
    text = old_text.replace(str(original), str(copied)).replace(str(prior.go2rtc), str(current.go2rtc))
    for key, old, new in [('port', prior.port, current.port), ('api_port', prior.go_port, current.go_port),
                          ('rtsp_port', prior.rtsp_port, current.rtsp_port),
                          ('webrtc_listen_port', prior.rtc_port, current.rtc_port)]:
        text = text.replace(f'{key} = {old}\n', f'{key} = {new}\n')
    current.config.write_text(text)
    updates = []
    # Historical rows contain absolute fixture paths. Relocate only those
    # values, after copying and while both recorder processes are stopped.
    with closing(sqlite3.connect(current.db)) as db, db:
        for table, in db.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'").fetchall():
            quoted_table = '"' + table.replace('"', '""') + '"'
            for column in db.execute(f'PRAGMA table_info({quoted_table})').fetchall():
                name = column[1]
                quoted_column = '"' + name.replace('"', '""') + '"'
                cursor = db.execute(f'UPDATE {quoted_table} SET {quoted_column}=replace({quoted_column},?,?) '
                    f'WHERE typeof({quoted_column})=\'text\' AND instr({quoted_column},?)>0',
                    (str(original), str(copied), str(original)))
                if cursor.rowcount:
                    updates.append({'table': table, 'column': name, 'rows': cursor.rowcount})
    return {'config_original_sha256': fixture.sha256(original / 'lightnvr.ini'),
            'config_relocated_sha256': fixture.sha256(current.config), 'database_path_updates': updates,
            'ports': {'prior': [prior.port, prior.go_port, prior.rtsp_port, prior.rtc_port],
                      'current': [current.port, current.go_port, current.rtsp_port, current.rtc_port]}}


def qualify(binary, manifest, directory):
    report = {'format_version': 1, 'scenario': 'historical-upgrade', 'status': 'failed',
        'artifacts': str(directory), 'manifest': manifest, 'harness_sha256': fixture.sha256(__file__),
        'bounds': {'startup_seconds': 60, 'shutdown_seconds': 30, 'request_seconds': 10,
                   'media_wait_seconds': 60, 'current_restarts': 2}, 'observations': {}}
    prior = current = source = None
    seal = None
    original = directory / 'original'
    try:
        prior_spec = manifest['prior']
        checkout = path_from_manifest(prior_spec['checkout'])
        prior_binary = path_from_manifest(prior_spec['binary'])
        prior_go = path_from_manifest(prior_spec['go2rtc'])
        current_go = path_from_manifest(manifest['current']['go2rtc'])
        assert git(checkout, 'rev-parse', 'HEAD') == prior_spec['revision']
        assert fixture.sha256(prior_binary) == prior_spec['binary_sha256']
        assert fixture.sha256(prior_go) == prior_spec['go2rtc_sha256']
        for submodule, revision in prior_spec['submodules'].items():
            assert git(checkout / submodule, 'rev-parse', 'HEAD') == revision
            assert not git(checkout / submodule, 'diff', '--name-only'), f'Dirty prior submodule {submodule}'
        changed = git(checkout, 'diff', '--name-only').splitlines()
        assert set(changed) <= {'web/js/version.js'}, f'Historical implementation changed: {changed}'
        current_evidence = json.loads(path_from_manifest(manifest['current']['build_manifest']).read_text())
        assert current_evidence['binary_sha256'] == fixture.sha256(binary)
        assert current_evidence['go2rtc_sha256'] == fixture.sha256(current_go)
        report['evidence'] = current_evidence
        original.mkdir()
        prior = UpgradeRecorder(prior_binary, prior_go, original)
        source = fixture.SyntheticSource(original)
        report['observations']['prior_startup_seconds'] = prior.start()
        principals = seed(prior, source)
        report['observations']['prior_auth'] = auth_checks(prior, principals)
        completed = fixture.wait_for('prior release completed recording', lambda: next((row for row in prior.rows()
            if row['is_complete'] and Path(row['file_path']).is_file() and Path(row['file_path']).stat().st_size > 4096), None))
        prior.json(f"/api/recordings/{completed['id']}/protect", 'PUT', {'protected': True})
        report['observations']['prior_media'] = media_check(prior, completed, directory, 'prior-media')
        prior.json('/api/streams/upgrade', 'PUT', {'enabled': False, 'record': False, 'streaming_enabled': False})
        fixture.wait_for('fixture camera disabled', lambda: not prior.json('/api/streams/upgrade/full')['stream']['enabled'])
        prior.stop()
        source.stop(); source = None
        # Clear only consumed process references so finally never reopens the
        # sealed installation's logs or starts another historical generation.
        prior.process = None
        before = schema_snapshot(prior.db)
        assert before['integrity'] == 'ok' and not before['foreign_key_errors']
        assert 'playback_issue' not in before['recording_columns']
        assert before['migration_rows'][-1]['version'] == prior_spec['last_migration']
        report['observations']['before_schema'] = before
        rows_before = prior.rows()
        report['observations']['before_rows'] = rows_before
        selected = next(row for row in rows_before if row['id'] == completed['id'])
        assert selected['protected'] and selected['is_complete']
        seal = hashes(original)
        (directory / 'original-hashes.json').write_text(json.dumps(seal, indent=2) + '\n')
        copied = directory / 'upgraded'
        shutil.copytree(original, copied)
        current = UpgradeRecorder(binary, current_go, copied)
        current.password = fixture.PASSWORD
        report['observations']['relocation'] = relocate_copy(original, copied, prior, current)
        relocated = schema_snapshot(current.db)
        assert relocated['objects'] == before['objects'] and relocated['migration_rows'] == before['migration_rows']
        stable = None
        for restart in range(2):
            round_result = {'startup_seconds': current.start()}
            round_result['auth'] = auth_checks(current, principals, current=True)
            row = next(row for row in current.rows() if row['id'] == selected['id'])
            assert row['protected'] == selected['protected'] and row['is_complete']
            assert row['file_path'] == selected['file_path'].replace(str(original), str(copied))
            round_result['media'] = media_check(current, row, directory, f'current-{restart}-media')
            assert round_result['media']['sha256'] == report['observations']['prior_media']['sha256']
            round_result['shutdown_seconds'] = current.stop()
            round_result['schema'] = schema_snapshot(current.db)
            assert round_result['schema']['integrity'] == 'ok' and not round_result['schema']['foreign_key_errors']
            assert 'playback_issue' in round_result['schema']['recording_columns']
            assert round_result['schema']['migration_rows'][-1]['version'] == manifest['current']['last_migration']
            assert any(item['name'] == 'idx_detections_external_motion_camera_end' for item in round_result['schema']['objects'])
            round_result['rows'] = current.rows()
            round_result['config_sha256'] = fixture.sha256(current.config)
            assert round_result['config_sha256'] == report['observations']['relocation']['config_relocated_sha256']
            report['observations'][f'current_{restart}'] = round_result
            # db_core.c creates/drops test_table as its startup write probe.
            # That advances SQLite's schema cookie twice even when persistent
            # objects and migration history are unchanged. Check both facts.
            invariant = {'schema': {key: value for key, value in round_result['schema'].items()
                                    if key != 'schema_version'}, 'rows': round_result['rows']}
            if stable is not None:
                assert invariant == stable, 'Second current startup changed schema/migrations or recording metadata'
                previous_cookie = report['observations']['current_0']['schema']['schema_version']
                assert round_result['schema']['schema_version'] == previous_cookie + 2, 'Unexpected schema cookie change'
            stable = invariant
        assert hashes(original) == seal, 'Original installation was modified'
        report['observations']['original_untouched'] = True
        report['qualification'] = (
            'Bounded workstation forward upgrade from the pinned 0.45.3 release: '
            'one synthetic installation, protected playable media and scoped principals, '
            'two current-version starts within the declared deadlines, with the original '
            'installation preserved. No downgrade or physical-device qualification.')
        report['status'] = 'passed'
    except Exception as error:
        report['error'] = str(error)
        report['traceback'] = traceback.format_exc()
    finally:
        cleanup = []
        for name, recorder in [('current', current), ('prior', prior)]:
            if recorder:
                cleanup.append((f'{name}_cleanup_error', recorder.stop))
                cleanup.extend((f'{name}_reservation_cleanup_error', sock.close)
                               for sock in recorder.reservations)
        if source:
            cleanup.append(('source_cleanup_error', source.stop))
        if seal is not None:
            def check_original():
                report['observations']['original_untouched_after_cleanup'] = hashes(original) == seal
                if not report['observations']['original_untouched_after_cleanup']:
                    raise AssertionError('Original hashes changed')
            cleanup.append(('original_preservation_error', check_original))
        fixture.finalize_report(report, directory, cleanup)
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--fixture-manifest', required=True, type=Path)
    parser.add_argument('--output', type=Path, help='New retained evidence directory')
    args = parser.parse_args()
    if args.output:
        args.output.mkdir(mode=0o700, parents=True, exist_ok=False)
        directory = args.output.resolve()
    else:
        directory = Path(tempfile.mkdtemp(prefix='lightnvr-upgrade-'))
    report = qualify(args.binary.resolve(), json.loads(args.fixture_manifest.read_text()), directory)
    print(json.dumps({'status': report['status'], 'artifacts': str(directory), 'error': report.get('error')}))
    return 0 if report['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
