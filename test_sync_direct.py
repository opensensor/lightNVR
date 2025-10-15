#!/usr/bin/env python3
"""
Direct database sync script for LightNVR recordings
This script updates the database with actual file sizes from disk
"""

import sqlite3
import os
import sys

DB_PATH = "/var/lib/lightnvr/lightnvr.db"

def sync_recordings():
    """Sync recording file sizes with actual files on disk"""
    try:
        # Connect to database
        conn = sqlite3.connect(DB_PATH)
        cursor = conn.cursor()
        
        # Get all recordings
        cursor.execute("SELECT id, file_path, size_bytes FROM recordings ORDER BY id")
        recordings = cursor.fetchall()
        
        if not recordings:
            print("No recordings found in database")
            return 0
        
        updated = 0
        errors = 0
        not_found = 0
        
        print(f"Found {len(recordings)} recordings in database")
        print("Syncing file sizes...")
        
        for rec_id, file_path, current_size in recordings:
            if not file_path:
                print(f"  Recording {rec_id}: No file path")
                errors += 1
                continue
                
            if os.path.exists(file_path):
                try:
                    actual_size = os.path.getsize(file_path)
                    if actual_size != current_size:
                        cursor.execute("UPDATE recordings SET size_bytes = ? WHERE id = ?", 
                                     (actual_size, rec_id))
                        updated += 1
                        print(f"  Recording {rec_id}: {current_size:,} -> {actual_size:,} bytes ({file_path})")
                except Exception as e:
                    print(f"  Recording {rec_id}: Error getting file size: {e}")
                    errors += 1
            else:
                not_found += 1
                if current_size != 0:
                    print(f"  Recording {rec_id}: File not found (marked as {current_size:,} bytes): {file_path}")
        
        # Commit changes
        conn.commit()
        conn.close()
        
        print(f"\nSync complete:")
        print(f"  - {updated} recordings updated")
        print(f"  - {len(recordings) - updated - errors - not_found} recordings already correct")
        print(f"  - {not_found} files not found")
        print(f"  - {errors} errors")
        
        return updated
        
    except sqlite3.OperationalError as e:
        if "readonly" in str(e).lower() or "locked" in str(e).lower():
            print(f"Database error: {e}")
            print("\nThe database may be locked by the running LightNVR service.")
            print("Please stop the service first with: sudo systemctl stop lightnvr")
            return -1
        else:
            raise
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return -1

if __name__ == "__main__":
    result = sync_recordings()
    sys.exit(0 if result >= 0 else 1)

