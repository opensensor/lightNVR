#!/usr/bin/env python3
"""Materialize only synthetic historical state inside a token-owned test directory."""
import hashlib
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import sys
import time

FIXTURES = Path(__file__).resolve().parents[2] / 'fixtures/eng/upgrade'


def owned(directory):
    root = Path(directory).resolve()
    assert root.parent == Path('/tmp') and root.name.startswith('lightnvr-eng-T29-')
    owner = json.loads((root / 'owner.json').read_text())
    assert owner['dir'] == str(root) and owner['token'] == os.environ['LIGHTNVR_RUN_TOKEN']
    return root


def query(root, sql):
    with sqlite3.connect(f'file:{root}/lightnvr.db?mode=ro', uri=True) as db:
        db.row_factory = sqlite3.Row
        return [dict(row) for row in db.execute(sql)]


def apply_historical_migration(db, sql):
    # Match0.45.0 sqlite_migrate.c's idempotent already-existing object policy.
    # sqlite3.complete_statement preserves trigger bodies containing semicolons.
    pending = ''
    for char in sql:
        pending += char
        if char == ';' and sqlite3.complete_statement(pending):
            try:
                db.executescript(pending)
            except sqlite3.OperationalError as error:
                message = str(error)
                if not ('duplicate column name' in message or
                        ('already exists' in message and ('table' in message or 'index' in message))):
                    raise
            pending = ''
    if pending.strip():
        db.executescript(pending)


def seed(root):
    database = root / 'lightnvr.db'
    assert not database.exists()
    media = root / 'recordings/mp4'
    media.mkdir(parents=True)
    complete = media / 'historical-complete.mp4'
    subprocess.run(['ffmpeg', '-nostdin', '-hide_banner', '-loglevel', 'error', '-f', 'lavfi',
                    '-i', 'testsrc=size=160x120:rate=5', '-t', '2', '-c:v', 'libx264',
                    '-threads', '1', '-pix_fmt', 'yuv420p', str(complete)], check=True)
    (media / 'stale-valid.mp4').write_bytes(complete.read_bytes())
    (media / 'stale-zero.mp4').touch()
    (media / 'stale-truncated.mp4').write_bytes(complete.read_bytes()[:40])
    (media / 'recent-incomplete.mp4').write_bytes(complete.read_bytes())
    fixture = json.loads((FIXTURES / '0.45.0-migrations.json').read_text())
    now = int(time.time())
    with sqlite3.connect(database) as db:
        db.execute("CREATE TABLE schema_migrations(version TEXT PRIMARY KEY, applied_at INTEGER NOT NULL DEFAULT(strftime('%s','now')))")
        for migration in fixture['migrations']:
            apply_historical_migration(db, migration['up'])
            db.execute('INSERT INTO schema_migrations(version) VALUES(?)', (migration['version'],))
        db.execute("INSERT INTO streams(name,url,enabled,record,streaming_enabled,camera_uuid) VALUES('historical','rtsp://127.0.0.1:1/unused',0,0,0,'d27404f7-5bc9-43e6-ae54-88a605be6929')")
        for filename, age, complete_flag in [('historical-complete.mp4', 7200, 1), ('stale-valid.mp4', 7200, 0), ('stale-zero.mp4', 7200, 0), ('stale-missing.mp4', 7200, 0), ('stale-truncated.mp4', 7200, 0), ('recent-incomplete.mp4', 0, 0)]:
            file = media / filename
            size = file.stat().st_size if file.exists() and complete_flag else 0
            db.execute('INSERT INTO recordings(stream_name,file_path,start_time,end_time,size_bytes,width,height,fps,codec,is_complete) VALUES(?,?,?,?,?,?,?,?,?,?)', ('historical', str(file), now-age, now-age+2 if complete_flag else 0, size, 160, 120, 5, 'h264', complete_flag))
        db.execute("INSERT OR REPLACE INTO system_settings(key,value) VALUES('t29.synthetic.preference','preserve-me')")
        db.execute("UPDATE system_settings SET value='1' WHERE key='setup_complete'")
        assert db.execute('PRAGMA integrity_check').fetchone() == ('ok',)
    result = {'tag': fixture['tag'], 'revision': fixture['revision'], 'seeded_at': now,
              'complete_sha256': hashlib.sha256(complete.read_bytes()).hexdigest(),
              'recordings': query(root, 'SELECT id,file_path,start_time,end_time,size_bytes,is_complete FROM recordings'),
              'schema_versions': query(root, 'SELECT version FROM schema_migrations ORDER BY version')}
    (root / 'fixture-state.json').write_text(json.dumps(result, indent=2))
    return result


if __name__ == '__main__':
    directory = owned(sys.argv[2])
    if sys.argv[1] == 'seed': result = seed(directory)
    elif sys.argv[1] == 'query': result = query(directory, sys.argv[3])
    else: raise SystemExit('Unknown fixture operation')
    print(json.dumps(result))
