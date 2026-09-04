#ifndef LIGHTNVR_DB_BACKUP_H
#define LIGHTNVR_DB_BACKUP_H

#include <stdbool.h>

/**
 * Backup the database to a specified path
 *
 * @param source_path Path to the source database file
 * @param dest_path Path to the destination backup file
 * @param abortable When true, the copy bails out early (returning non-zero,
 *     no partial file left behind) if a restart/shutdown is requested
 *     mid-copy -- appropriate for a periodic/scheduled backup, where
 *     finishing this run is never more important than letting the process
 *     stop promptly. Pass false for a backup that must complete once
 *     started (e.g. the deliberate one-time backup taken while already
 *     shutting down) -- there, the request is already guaranteed to be
 *     pending, so an abortable copy would always bail out immediately and
 *     never actually produce a backup.
 * @return 0 on success, non-zero on failure
 */
int backup_database(const char *source_path, const char *dest_path, bool abortable);

/**
 * Restore database from backup
 * 
 * @param backup_path Path to the backup file
 * @param db_path Path to the database file to restore to
 * @return 0 on success, non-zero on failure
 */
int restore_database_from_backup(const char *backup_path, const char *db_path);

/**
 * Check and repair database
 * 
 * @return 0 on success, non-zero on failure
 */
int check_and_repair_database(void);

#endif // LIGHTNVR_DB_BACKUP_H
