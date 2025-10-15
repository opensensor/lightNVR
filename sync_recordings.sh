#!/bin/bash
# Script to manually sync recording file sizes with the database

DB_PATH="/var/lib/lightnvr/lightnvr.db"
MP4_PATH="/var/lib/lightnvr/recordings/mp4"

echo "Syncing recording file sizes with database..."

# Get all recordings from database and update their sizes
sqlite3 "$DB_PATH" <<EOF
-- Update file sizes for all recordings
UPDATE recordings 
SET size_bytes = (
    SELECT CAST(
        SUBSTR(
            (SELECT group_concat(hex(substr(quote(zeroblob((
                SELECT length(readfile(file_path))
            ))), 3, 6)), '') FROM (SELECT 1)),
            1, 16
        ) AS INTEGER
    )
    FROM (SELECT file_path AS fp WHERE fp = recordings.file_path)
)
WHERE EXISTS (
    SELECT 1 FROM (
        SELECT file_path AS fp 
        WHERE fp = recordings.file_path
    )
);
EOF

# Alternative simpler approach - use a Python script
python3 << 'PYTHON_SCRIPT'
import sqlite3
import os
import sys

db_path = "/var/lib/lightnvr/lightnvr.db"
mp4_base = "/var/lib/lightnvr/recordings/mp4"

try:
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Get all recordings
    cursor.execute("SELECT id, file_path, size_bytes FROM recordings")
    recordings = cursor.fetchall()
    
    updated = 0
    errors = 0
    
    for rec_id, file_path, current_size in recordings:
        if os.path.exists(file_path):
            actual_size = os.path.getsize(file_path)
            if actual_size != current_size:
                cursor.execute("UPDATE recordings SET size_bytes = ? WHERE id = ?", 
                             (actual_size, rec_id))
                updated += 1
                print(f"Updated recording {rec_id}: {current_size} -> {actual_size} bytes")
        else:
            errors += 1
            print(f"File not found for recording {rec_id}: {file_path}", file=sys.stderr)
    
    conn.commit()
    conn.close()
    
    print(f"\nSync complete: {updated} recordings updated, {errors} errors")
    sys.exit(0 if errors == 0 else 1)
    
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    sys.exit(1)
PYTHON_SCRIPT

echo "Done!"

