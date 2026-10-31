#!/usr/bin/env python3
"""Real disposable SQLite upgrade from pinned0.45.0 sources; no native process."""
import importlib.util
import json
from pathlib import Path
import sqlite3
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
MIGRATION = ROOT / 'db/migrations/0090_add_stale_deletion_guards.sql'
SPEC = importlib.util.spec_from_file_location('lifecycle_fixture', Path(__file__).with_name('fixture.py'))
fixture = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(fixture)


class StaleGuardUpgradeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='lightnvr-stale-guard-')
        self.path = Path(self.temp.name) / 'old.db'
        self.db = sqlite3.connect(self.path)
        old = json.loads((ROOT / 'tests/fixtures/eng/upgrade/0.45.0-migrations.json').read_text())
        for entry in old['migrations']:
            fixture.apply_historical_migration(self.db, entry['up'])
        self.db.execute("INSERT INTO storage_deletions(uuid,recording_id,reason) VALUES('historical-deletion',7,'ordinary deletion')")
        self.db.execute("INSERT INTO storage_deletion_objects(id,deletion_uuid,file_path,size_bytes) VALUES(1,'historical-deletion','/synthetic/ordinary.mp4',123)")
        self.db.commit()
        if MIGRATION.exists():
            self.db.executescript(MIGRATION.read_text().split('-- migrate:up', 1)[1].split('-- migrate:down', 1)[0])

    def tearDown(self):
        self.db.close()
        self.temp.cleanup()

    def test_upgrade_preserves_ordinary_row_with_null_guards(self):
        row = self.db.execute('SELECT file_path,size_bytes,state,stale_guard_kind,stale_guard_dev,stale_guard_ino FROM storage_deletion_objects WHERE id=1').fetchone()
        self.assertEqual(row, ('/synthetic/ordinary.mp4', 123, 'pending', None, None, None))
        self.assertEqual(self.db.execute('PRAGMA integrity_check').fetchone(), ('ok',))

    def test_guards_survive_database_reopen(self):
        self.db.execute("UPDATE storage_deletion_objects SET stale_guard_kind='missing' WHERE id=1")
        self.db.execute("INSERT INTO storage_deletion_objects(id,deletion_uuid,file_path,stale_guard_kind,stale_guard_dev,stale_guard_ino) VALUES(2,'historical-deletion','/synthetic/empty.mp4','empty',2049,123456)")
        self.db.commit()
        self.db.close()
        self.db = sqlite3.connect(self.path)
        self.assertEqual(self.db.execute('SELECT stale_guard_kind,stale_guard_dev,stale_guard_ino FROM storage_deletion_objects ORDER BY id').fetchall(), [('missing', None, None), ('empty', 2049, 123456)])

    def test_inconsistent_guard_shapes_are_rejected(self):
        invalid = [(None, 1, 2), ('missing', 1, None), ('empty', None, 2), ('empty', 1, None), ('empty', -1, 2), ('empty', 1, 0), ('unknown', None, None)]
        for kind, dev, ino in invalid:
            with self.subTest(kind=kind, dev=dev, ino=ino):
                with self.assertRaises(sqlite3.IntegrityError):
                    self.db.execute('UPDATE storage_deletion_objects SET stale_guard_kind=?,stale_guard_dev=?,stale_guard_ino=? WHERE id=1', (kind, dev, ino))
        self.assertEqual(self.db.execute('SELECT stale_guard_kind,stale_guard_dev,stale_guard_ino FROM storage_deletion_objects WHERE id=1').fetchone(), (None, None, None))

    def test_unsupported_downgrade_cannot_remove_pending_guards(self):
        self.db.execute("UPDATE storage_deletion_objects SET stale_guard_kind='empty',stale_guard_dev=2049,stale_guard_ino=123456 WHERE id=1")
        self.db.commit()
        down = MIGRATION.read_text().split('-- migrate:down', 1)[1]
        with self.assertRaisesRegex(sqlite3.DatabaseError, 'stale deletion guards cannot be downgraded'):
            self.db.executescript(down)
        self.assertEqual(self.db.execute('SELECT stale_guard_kind,stale_guard_dev,stale_guard_ino FROM storage_deletion_objects WHERE id=1').fetchone(), ('empty', 2049, 123456))


if __name__ == '__main__':
    unittest.main()
