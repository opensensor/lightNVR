# Synthetic0.45.0 compatibility state

`provenance.json` pins tag0.45.0 to commit
`fba2cc52ea37fc61ecb03781e81af2a1e545ed17`. The migration bundle contains only
up sections from that revision, with each original file's SHA256 (and exact symbols for embedded-only entries). The config
is the byte-for-byte historical isolated health-test config. Preference values
are synthetic choices using keys consumed by that revision's browser sources.
No production/user state, authentication secrets or camera URLs were exported.

`tests/operational/lifecycle/fixture.py` applies89 migration up sections (87 filesystem files and embedded-only0043/0044) to a
new temporary database. Its SQLite statement splitter uses
`sqlite3.complete_statement`, including trigger bodies; already-existing
columns/tables/indexes follow the historical `sqlite_migrate.c` idempotence
policy. Other SQL failures abort. It seeds an explicitly completed setup flag,
one disabled synthetic stream, a sentinel setting and generated media rows.
Valid MP4 footage is generated locally with FFmpeg; missing, zero-length and
truncated files are intentional recovery controls.

The checked-in0.45.0 config is adapted only at execution to token-owned paths,
the existing reserved integration ports, managed go2rtc, disabled MQTT and short
synthetic recording segments. All mutations and their resulting config are
retained with run provenance. No downgrade is attempted. Returning an upgraded
database to an older binary is unsupported by this evidence; restore a separately
validated backup under a separately qualified procedure instead.

This qualifies compatibility with the previous tagged release, not an invented
support matrix. Current0.45.1 and0.45.0 have the same migration set through0089;
these fixtures demonstrate persisted state/config/preference continuity without
claiming that a schema-version transition occurred.
