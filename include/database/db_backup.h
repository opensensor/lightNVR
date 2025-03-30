#ifndef LIGHTNVR_DB_BACKUP_H
#define LIGHTNVR_DB_BACKUP_H

/**
 * Backup the database to a specified path
 * 
 * @param source_path Path to the source database file
 * @param dest_path Path to the destination backup file
 * @return 0 on success, non-zero on failure
 */
int backup_database(const char *source_path, const char *dest_path);

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
