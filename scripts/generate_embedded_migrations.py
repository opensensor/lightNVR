#!/usr/bin/env python3
"""Check or append SQL files to LightNVR's embedded migration header.

The checked-in header predates this updater and intentionally retains its
formatting.  ``--update`` therefore performs no write when SQL is already
embedded, and only appends migrations with versions newer than the header.
SQL comparison is token based so comments and formatting do not create churn.
"""

from __future__ import annotations

import argparse
import ast
from dataclasses import dataclass
import hashlib
from pathlib import Path
import re
import sys


MIGRATION_NAME = re.compile(r"^(\d{4})_(.+)\.sql$")
SECTION_MARKER = re.compile(r"^\s*--\s*migrate:(up|down)\s*$", re.IGNORECASE)
CONSTANT = re.compile(
    r"static const char migration_(\d{4})_(up|down)\[\]\s*=\s*\n"
    r"(?P<body>(?:\s*\"(?:\\.|[^\"\\])*\"\s*\n?)+);",
    re.MULTILINE,
)
ARRAY_VERSION = re.compile(r'\.version\s*=\s*"(\d{4})"')
COUNT = re.compile(r"(#define\s+EMBEDDED_MIGRATIONS_COUNT\s+)(\d+)")
ARRAY_MARKER = "static const migration_t embedded_migrations_data[] = {"

# Locked fingerprints for three known pre-generator differences.  Both sides
# must remain byte-semantically identical to this baseline; arbitrary drift is
# still rejected.  0045/0046 were embedded as a no-op for old SQLite, while
# 0068's embedded form predates idempotent CREATE clauses in its source file.
LEGACY_SQL_VARIANTS = {
    ("0045", "down"): (
        "eec89a0b94da992659771264a2f1c3f02591427ed41f5c5d4cfd4f2d8818ea45",
        "bd8b05aa590ad00c651b1482829242b5954f8a6752bc37c5142632ccdcf3746a"),
    ("0046", "down"): (
        "8ab58d73926d0d77d866be567f91a5bb420bd13281f1c7a458e3feb2ad0e890a",
        "bd8b05aa590ad00c651b1482829242b5954f8a6752bc37c5142632ccdcf3746a"),
    ("0068", "up"): (
        "0dc54f336f0fa2da4c2e6ad04ed9a41c36c03f4754e30522b2f5f300dd01300d",
        "0df2bcdfe72b7911026d6f6037861e006f7458a82a27b475adc168486f9a0505"),
}


class MigrationError(ValueError):
    """A migration set or embedded header is unsafe to update."""


@dataclass(frozen=True)
class Migration:
    version: str
    description: str
    up: str
    down: str


def _sections(path: Path) -> tuple[str, str]:
    sections: dict[str, list[str]] = {}
    current: str | None = None
    for line in path.read_text(encoding="utf-8").splitlines():
        marker = SECTION_MARKER.match(line)
        if marker:
            current = marker.group(1).lower()
            if current in sections:
                raise MigrationError(f"{path}: duplicate migrate:{current} section")
            sections[current] = []
        elif current is not None:
            sections[current].append(line)
    missing = [name for name in ("up", "down") if not "\n".join(sections.get(name, [])).strip()]
    if missing:
        raise MigrationError(f"{path}: missing or empty migrate:{'/'.join(missing)} section")
    return ("\n".join(sections["up"]).strip(),
            "\n".join(sections["down"]).strip())


def load_migrations(directory: Path) -> list[Migration]:
    if not directory.is_dir():
        raise MigrationError(f"migration directory does not exist: {directory}")
    migrations: list[Migration] = []
    seen: dict[str, Path] = {}
    for path in sorted(directory.glob("*.sql")):
        match = MIGRATION_NAME.match(path.name)
        if not match:
            raise MigrationError(f"malformed migration filename: {path.name}")
        version, description = match.groups()
        if version in seen:
            raise MigrationError(
                f"duplicate migration version {version}: {seen[version].name}, {path.name}")
        seen[version] = path
        up, down = _sections(path)
        migrations.append(Migration(version, description, up, down))
    if not migrations:
        raise MigrationError(f"no SQL migrations found in {directory}")
    return sorted(migrations, key=lambda item: int(item.version))


def _decode_c_strings(body: str) -> str:
    literals = re.findall(r'\"(?:\\.|[^\"\\])*\"', body)
    if not literals:
        raise MigrationError("embedded SQL constant contains no C strings")
    try:
        return "".join(ast.literal_eval(literal) for literal in literals)
    except (SyntaxError, ValueError) as exc:
        raise MigrationError(f"invalid C string in embedded header: {exc}") from exc


def parse_header(text: str) -> tuple[dict[str, dict[str, str]], list[str]]:
    constants: dict[str, dict[str, str]] = {}
    for match in CONSTANT.finditer(text):
        version, direction = match.group(1), match.group(2)
        bucket = constants.setdefault(version, {})
        if direction in bucket:
            raise MigrationError(f"duplicate embedded migration_{version}_{direction}")
        bucket[direction] = _decode_c_strings(match.group("body"))

    array_versions = ARRAY_VERSION.findall(text)
    if len(array_versions) != len(set(array_versions)):
        raise MigrationError("duplicate migration version in embedded migration table")
    count_match = COUNT.search(text)
    if not count_match:
        raise MigrationError("missing EMBEDDED_MIGRATIONS_COUNT")
    if int(count_match.group(2)) != len(array_versions):
        raise MigrationError("EMBEDDED_MIGRATIONS_COUNT does not match migration table")
    for version in array_versions:
        if set(constants.get(version, {})) != {"up", "down"}:
            raise MigrationError(f"embedded migration {version} lacks up/down SQL")
    if set(constants) != set(array_versions):
        raise MigrationError("embedded SQL constants and migration table disagree")
    return constants, array_versions


def _sql_tokens(sql: str) -> tuple[str, ...]:
    """Return formatting/comment-insensitive SQL tokens, preserving literals."""
    tokens: list[str] = []
    i = 0
    length = len(sql)
    while i < length:
        char = sql[i]
        if char.isspace():
            i += 1
            continue
        if sql.startswith("--", i):
            end = sql.find("\n", i + 2)
            i = length if end < 0 else end + 1
            continue
        if sql.startswith("/*", i):
            end = sql.find("*/", i + 2)
            if end < 0:
                raise MigrationError("unterminated SQL block comment")
            i = end + 2
            continue
        if char in "'\"`":
            quote = char
            start = i
            i += 1
            while i < length:
                if sql[i] == quote:
                    if i + 1 < length and sql[i + 1] == quote:
                        i += 2
                        continue
                    i += 1
                    break
                i += 1
            else:
                raise MigrationError("unterminated quoted SQL token")
            tokens.append(sql[start:i])
            continue
        if char == "[":
            end = sql.find("]", i + 1)
            if end < 0:
                raise MigrationError("unterminated bracketed SQL identifier")
            tokens.append(sql[i:end + 1])
            i = end + 1
            continue
        operator = next((op for op in ("->>", "||", "!=", "<=", ">=", "<>", "==", "->")
                         if sql.startswith(op, i)), None)
        if operator:
            tokens.append(operator)
            i += len(operator)
            continue
        match = re.match(r"[A-Za-z_][A-Za-z0-9_$]*|(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?", sql[i:])
        if match:
            token = match.group(0)
            tokens.append(token.lower())
            i += len(token)
            continue
        tokens.append(char)
        i += 1
    normalized = tuple(tokens)
    # Historical down migrations use either a comment-only no-op or SELECT 1.
    # They are operationally equivalent and must not force legacy header churn.
    if not normalized or normalized in (("select", "1"), ("select", "1", ";")):
        return ("__noop__",)
    return normalized


def _validate_existing(migrations: list[Migration], constants: dict[str, dict[str, str]],
                       array_versions: list[str]) -> list[Migration]:
    by_version = {migration.version: migration for migration in migrations}
    for version in array_versions:
        migration = by_version.get(version)
        if migration is None:
            # Some legacy files were intentionally retired after being embedded.
            # The header remains their immutable release record.
            continue
        for direction in ("up", "down"):
            source_tokens = _sql_tokens(getattr(migration, direction))
            header_tokens = _sql_tokens(constants[version][direction])
            if source_tokens == header_tokens:
                continue
            fingerprints = (
                hashlib.sha256("\0".join(source_tokens).encode()).hexdigest(),
                hashlib.sha256("\0".join(header_tokens).encode()).hexdigest(),
            )
            if LEGACY_SQL_VARIANTS.get((version, direction)) != fingerprints:
                raise MigrationError(
                    f"migration {version} {direction} SQL differs from embedded header")

    missing = [migration for migration in migrations if migration.version not in constants]
    if missing and array_versions:
        last = int(array_versions[-1])
        if any(int(migration.version) <= last for migration in missing):
            versions = ", ".join(migration.version for migration in missing)
            raise MigrationError(f"cannot insert migration(s) behind embedded tip: {versions}")
    return missing


def _c_literal_lines(sql: str) -> str:
    lines = sql.splitlines()
    encoded: list[str] = []
    for index, line in enumerate(lines):
        value = line + ("\n" if index + 1 < len(lines) else "")
        value = value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
        encoded.append(f'    "{value}"')
    return "\n".join(encoded)


def _constant_block(migration: Migration) -> str:
    return (
        f"static const char migration_{migration.version}_up[] =\n"
        f"{_c_literal_lines(migration.up)};\n\n"
        f"static const char migration_{migration.version}_down[] =\n"
        f"{_c_literal_lines(migration.down)};\n\n"
    )


def _array_entry(migration: Migration) -> str:
    description = migration.description.replace("\\", "").replace('"', "")[:255]
    return (
        "    {\n"
        f'        .version = "{migration.version}",\n'
        f'        .description = "{description}",\n'
        f"        .sql_up = migration_{migration.version}_up,\n"
        f"        .sql_down = migration_{migration.version}_down,\n"
        "        .is_embedded = true\n"
        "    },\n"
    )


def updated_header(text: str, migrations: list[Migration]) -> tuple[str, int]:
    constants, versions = parse_header(text)
    missing = _validate_existing(migrations, constants, versions)
    if not missing:
        return text, 0
    marker_at = text.find(ARRAY_MARKER)
    if marker_at < 0:
        raise MigrationError("missing embedded migration table")
    constants_text = "".join(_constant_block(migration) for migration in missing)
    text = text[:marker_at] + constants_text + text[marker_at:]

    marker_at = text.find(ARRAY_MARKER)
    array_end = text.find("\n};", marker_at)
    if array_end < 0:
        raise MigrationError("unterminated embedded migration table")
    entries = "".join(_array_entry(migration) for migration in missing)
    text = text[:array_end] + "\n" + entries.rstrip("\n") + text[array_end:]
    text, replacements = COUNT.subn(
        lambda match: match.group(1) + str(len(versions) + len(missing)), text, count=1)
    if replacements != 1:
        raise MigrationError("could not update EMBEDDED_MIGRATIONS_COUNT")
    parse_header(text)
    return text, len(missing)


def run(mode: str, migrations_dir: Path, header_path: Path) -> int:
    migrations = load_migrations(migrations_dir)
    if not header_path.is_file():
        raise MigrationError(f"embedded header does not exist: {header_path}")
    original = header_path.read_text(encoding="utf-8")
    updated, appended = updated_header(original, migrations)
    if mode == "--check":
        if appended:
            versions = ", ".join(m.version for m in migrations if m.version not in parse_header(original)[0])
            raise MigrationError(f"embedded header is missing migration(s): {versions}")
        return 0
    if updated != original:
        header_path.write_text(updated, encoding="utf-8")
        print(f"Appended {appended} migration(s) to {header_path}")
    else:
        print(f"{header_path} is already up to date")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", dest="mode", action="store_const", const="--check")
    mode.add_argument("--update", dest="mode", action="store_const", const="--update")
    parser.add_argument("migrations_dir", type=Path)
    parser.add_argument("header", type=Path)
    try:
        args = parser.parse_args(argv)
        return run(args.mode, args.migrations_dir, args.header)
    except MigrationError as exc:
        print(f"migration generation failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
