#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "generate_embedded_migrations.py"
SPEC = importlib.util.spec_from_file_location("embedded_migrations", SCRIPT)
assert SPEC and SPEC.loader
generator = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = generator
SPEC.loader.exec_module(generator)


def migration_sql(up: str = "CREATE TABLE one (id INTEGER);",
                  down: str = "DROP TABLE one;") -> str:
    return f"-- comment\n-- migrate:up\n\n{up}\n\n-- migrate:down\n\n{down}\n"


def header(version: str = "0001", description: str = "historical_name") -> str:
    return f'''#ifndef TEST_MIGRATIONS_H
#define TEST_MIGRATIONS_H
static const char migration_{version}_up[] =
    "CREATE TABLE one (id INTEGER);";

static const char migration_{version}_down[] =
    "DROP TABLE one;";

static const migration_t embedded_migrations_data[] = {{
    {{
        .version = "{version}",
        .description = "{description}",
        .sql_up = migration_{version}_up,
        .sql_down = migration_{version}_down,
        .is_embedded = true
    }},
}};

#define EMBEDDED_MIGRATIONS_COUNT 1
#endif
'''


class EmbeddedMigrationGeneratorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.migrations = self.root / "migrations"
        self.migrations.mkdir()
        self.output = self.root / "embedded.h"

    def tearDown(self) -> None:
        self.temp.cleanup()

    def write_migration(self, name: str, sql: str | None = None) -> None:
        (self.migrations / name).write_text(sql or migration_sql(), encoding="utf-8")

    def test_noop_update_preserves_every_byte_and_description_mismatch(self) -> None:
        self.write_migration("0001_new_filename.sql")
        original = header(description="old_description_is_allowed")
        self.output.write_text(original, encoding="utf-8")
        self.assertEqual(0, generator.run("--update", self.migrations, self.output))
        self.assertEqual(original, self.output.read_text(encoding="utf-8"))
        self.assertEqual(0, generator.run("--check", self.migrations, self.output))

    def test_update_appends_compact_entry_and_is_then_a_noop(self) -> None:
        self.write_migration("0001_first.sql")
        self.write_migration(
            "0002_second.sql",
            migration_sql("CREATE INDEX idx_one ON one(id);", "DROP INDEX idx_one;"),
        )
        self.output.write_text(header(), encoding="utf-8")
        self.assertEqual(0, generator.run("--update", self.migrations, self.output))
        once = self.output.read_text(encoding="utf-8")
        self.assertIn("migration_0002_up", once)
        self.assertIn("EMBEDDED_MIGRATIONS_COUNT 2", once)
        self.assertEqual(0, generator.run("--update", self.migrations, self.output))
        self.assertEqual(once, self.output.read_text(encoding="utf-8"))

    def test_duplicate_version_is_rejected(self) -> None:
        self.write_migration("0001_first.sql")
        self.write_migration("0001_duplicate.sql")
        with self.assertRaises(generator.MigrationError):
            generator.load_migrations(self.migrations)

    def test_missing_section_and_changed_sql_are_rejected(self) -> None:
        self.write_migration("0001_first.sql", "-- migrate:up\nSELECT 1;\n")
        with self.assertRaises(generator.MigrationError):
            generator.load_migrations(self.migrations)
        (self.migrations / "0001_first.sql").write_text(
            migration_sql("CREATE TABLE changed (id INTEGER);"), encoding="utf-8")
        self.output.write_text(header(), encoding="utf-8")
        with self.assertRaises(generator.MigrationError):
            generator.run("--check", self.migrations, self.output)


if __name__ == "__main__":
    unittest.main()
