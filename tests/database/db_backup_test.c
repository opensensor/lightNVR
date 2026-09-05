#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sqlite3.h>
#include <unistd.h>
#include <signal.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/wait.h>

#include "database/db_core.h"
#include "database/db_backup.h"
#include "core/config.h"
#include "core/logger.h"

// Test database path
#define TEST_DB_PATH "/tmp/test_db.sqlite"
#define TEST_BACKUP_PATH "/tmp/test_db.sqlite.bak"
#define TEST_LARGE_DB_PATH "/tmp/test_db_large.sqlite"
#define TEST_LARGE_BACKUP_PATH "/tmp/test_db_large.sqlite.bak"

static void remove_database_files(const char *path) {
    char sidecar[256];

    unlink(path);
    snprintf(sidecar, sizeof(sidecar), "%s-wal", path);
    unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-shm", path);
    unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-journal", path);
    unlink(sidecar);
}

/* POSIX record locks are process-associated, so query them from a child.
 * This detects accidental close(open(database)) patterns that silently drop
 * SQLite's locks while its real descriptors and mappings remain live. */
static int path_has_parent_process_lock(const char *path) {
    pid_t pid = fork();
    if (pid < 0) return 0;
    if (pid == 0) {
        int fd = open(path, O_RDWR | O_CLOEXEC);
        if (fd < 0) _exit(2);
        struct flock lock;
        memset(&lock, 0, sizeof(lock));
        lock.l_type = F_WRLCK;
        lock.l_whence = SEEK_SET;
        lock.l_start = 0;
        lock.l_len = 0;
        int query_rc = fcntl(fd, F_GETLK, &lock);
        close(fd);
        _exit(query_rc == 0 && lock.l_type != F_UNLCK ? 0 : 1);
    }

    int status = 0;
    return waitpid(pid, &status, 0) == pid && WIFEXITED(status) &&
           WEXITSTATUS(status) == 0;
}

// Signal handler for simulating a crash
static void simulate_crash(int sig) {
    (void)sig;
    printf("Simulating application crash...\n");
    _exit(1); // Force exit without cleanup
}

// Create a test database with some data
static int create_test_database(void) {
    sqlite3 *db;
    int rc;
    char *err_msg = NULL;
    
    // Remove any existing test database
    remove_database_files(TEST_DB_PATH);
    remove_database_files(TEST_BACKUP_PATH);
    remove_database_files(TEST_BACKUP_PATH ".tmp");
    
    // Initialize the database
    rc = init_database(TEST_DB_PATH);
    if (rc != 0) {
        printf("Failed to initialize database\n");
        return -1;
    }
    
    // Get the database handle
    db = get_db_handle();
    if (!db) {
        printf("Failed to get database handle\n");
        return -1;
    }
    
    // Create a test table
    const char *create_table = "CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT);";
    rc = sqlite3_exec(db, create_table, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        printf("Failed to create test table: %s\n", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    // Insert some test data
    const char *insert_data = "INSERT INTO test (id, value) VALUES (1, 'test data 1'), (2, 'test data 2');";
    rc = sqlite3_exec(db, insert_data, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        printf("Failed to insert test data: %s\n", err_msg);
        sqlite3_free(err_msg);
        return -1;
    }
    
    printf("Test database created successfully\n");
    return 0;
}

// Verify the database contains the expected data
static int verify_database(void) {
    sqlite3 *db;
    int rc;
    sqlite3_stmt *stmt;
    int count = 0;
    
    // Open the database
    rc = sqlite3_open_v2(TEST_DB_PATH, &db, SQLITE_OPEN_READONLY, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to open database for verification: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
        return -1;
    }
    
    // Check if the test table exists
    const char *check_table = "SELECT name FROM sqlite_master WHERE type='table' AND name='test';";
    rc = sqlite3_prepare_v2(db, check_table, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
        return -1;
    }
    
    if (sqlite3_step(stmt) != SQLITE_ROW) {
        printf("Test table does not exist\n");
        sqlite3_finalize(stmt);
        sqlite3_close(db);
        return -1;
    }
    
    sqlite3_finalize(stmt);
    
    // Count the number of rows in the test table
    const char *count_rows = "SELECT COUNT(*) FROM test;";
    rc = sqlite3_prepare_v2(db, count_rows, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
        return -1;
    }
    
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        count = sqlite3_column_int(stmt, 0);
    }
    
    sqlite3_finalize(stmt);
    sqlite3_close(db);
    
    if (count != 2) {
        printf("Expected 2 rows, found %d\n", count);
        return -1;
    }
    
    printf("Database verification successful\n");
    return 0;
}

// Corrupt the database file
static int corrupt_database(void) {
    FILE *file;
    const unsigned char corrupt_header[] = {
        0x00, 0x00, 0x00, 0x00, 'B', 'A', 'D', '-',
        'H', 'E', 'A', 'D', 'E', 'R', 0x00, 0x00
    };

    // Close the live database handle first so corruption hits the on-disk file deterministically.
    shutdown_database();

    /* A cleanly shut down standalone file has no live WAL index. Persistent
     * sidecars are deliberately enabled in production, so remove them before
     * corrupting the main-file header rather than letting WAL page 1 mask it. */
    char sidecar[256];
    snprintf(sidecar, sizeof(sidecar), "%s-wal", TEST_DB_PATH);
    unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-shm", TEST_DB_PATH);
    unlink(sidecar);
    
    // Open the database file
    file = fopen(TEST_DB_PATH, "r+b");
    if (!file) {
        printf("Failed to open database file for corruption\n");
        return -1;
    }
    
    // Overwrite the SQLite header so subsequent opens fail reliably.
    fseek(file, 0, SEEK_SET);
    fwrite(corrupt_header, 1, sizeof(corrupt_header), file);
    fflush(file);
    fsync(fileno(file));
    
    fclose(file);
    
    printf("Database file corrupted\n");
    return 0;
}

// Test backup functionality
static int test_backup(void) {
    char backup_wal[256];
    char backup_shm[256];
    char primary_shm[256];
    char temporary_wal[256];
    char temporary_shm[256];
    struct stat shm_before;
    struct stat shm_after;

    snprintf(primary_shm, sizeof(primary_shm), "%s-shm", TEST_DB_PATH);
    snprintf(temporary_wal, sizeof(temporary_wal), "%s.tmp-wal",
             TEST_BACKUP_PATH);
    snprintf(temporary_shm, sizeof(temporary_shm), "%s.tmp-shm",
             TEST_BACKUP_PATH);
    snprintf(backup_wal, sizeof(backup_wal), "%s-wal", TEST_BACKUP_PATH);
    snprintf(backup_shm, sizeof(backup_shm), "%s-shm", TEST_BACKUP_PATH);
    if (stat(primary_shm, &shm_before) != 0 || shm_before.st_size < 32768) {
        printf("Primary WAL shared-memory file is not initialized\n");
        return -1;
    }
    if (!path_has_parent_process_lock(TEST_DB_PATH) ||
        !path_has_parent_process_lock(primary_shm)) {
        printf("SQLite database/WAL-index locks are missing before backup\n");
        return -1;
    }

    /* Seed sidecars from a hypothetical older WAL-mode backup. Publishing a
     * replacement main file must remove them. */
    int stale_fd = open(backup_wal, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (stale_fd < 0 || write(stale_fd, "old-wal", 7) != 7) {
        if (stale_fd >= 0) close(stale_fd);
        printf("Failed to create stale published-backup WAL fixture\n");
        return -1;
    }
    close(stale_fd);
    stale_fd = open(backup_shm, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (stale_fd < 0 || write(stale_fd, "old-shm", 7) != 7) {
        if (stale_fd >= 0) close(stale_fd);
        printf("Failed to create stale published-backup SHM fixture\n");
        return -1;
    }
    close(stale_fd);

    // Create a backup of the database
    int rc = backup_database(TEST_DB_PATH, TEST_BACKUP_PATH);
    if (rc != 0) {
        printf("Failed to create backup\n");
        return -1;
    }

    /* Opening and closing the backup source must not replace, truncate, or
     * unlink the live database's mmap-backed WAL index. */
    if (stat(primary_shm, &shm_after) != 0 ||
        shm_after.st_ino != shm_before.st_ino ||
        shm_after.st_size < 32768 || shm_after.st_size % 32768 != 0) {
        printf("Live database WAL shared-memory mapping changed during backup\n");
        return -1;
    }
    if (!path_has_parent_process_lock(TEST_DB_PATH) ||
        !path_has_parent_process_lock(primary_shm)) {
        printf("Backup dropped live SQLite database/WAL-index locks\n");
        return -1;
    }

    /* A completed backup is a single recovery file. Temporary WAL sidecars
     * must not survive publication or be required to read the backup. */
    if (access(temporary_wal, F_OK) == 0 ||
        access(temporary_shm, F_OK) == 0 ||
        access(backup_wal, F_OK) == 0 || access(backup_shm, F_OK) == 0) {
        printf("Backup WAL/SHM sidecars were left behind\n");
        return -1;
    }

    sqlite3 *backup_db = NULL;
    sqlite3_stmt *journal_mode = NULL;
    rc = sqlite3_open_v2(TEST_BACKUP_PATH, &backup_db, SQLITE_OPEN_READONLY,
                         NULL);
    if (rc != SQLITE_OK ||
        sqlite3_prepare_v2(backup_db, "PRAGMA journal_mode;", -1,
                           &journal_mode, NULL) != SQLITE_OK ||
        sqlite3_step(journal_mode) != SQLITE_ROW ||
        strcmp((const char *)sqlite3_column_text(journal_mode, 0), "delete") != 0) {
        printf("Published backup is not self-contained rollback-journal mode\n");
        if (journal_mode) sqlite3_finalize(journal_mode);
        if (backup_db) sqlite3_close(backup_db);
        return -1;
    }
    sqlite3_finalize(journal_mode);
    sqlite3_close(backup_db);

    /* Restore owns no database handle and must never close db_core's borrowed
     * global pointer behind the rest of the process. */
    if (restore_database_from_backup(TEST_BACKUP_PATH, TEST_DB_PATH) == 0) {
        printf("Restore unexpectedly accepted a live database handle\n");
        return -1;
    }
    
    printf("Database backup created successfully\n");
    return 0;
}

// Exercise more than one bounded sqlite3_backup_step() batch.  The production
// regression only appears once databases are large enough to fill the cgroup's
// filesystem cache, so the tiny recovery fixture above is not sufficient.
static int test_large_incremental_backup(void) {
    sqlite3 *source = NULL;
    sqlite3 *backup = NULL;
    sqlite3_stmt *stmt = NULL;
    int result = -1;
    int rc;

    remove_database_files(TEST_LARGE_DB_PATH);
    remove_database_files(TEST_LARGE_BACKUP_PATH);
    remove_database_files(TEST_LARGE_BACKUP_PATH ".tmp");

    rc = sqlite3_open(TEST_LARGE_DB_PATH, &source);
    if (rc != SQLITE_OK) {
        printf("Failed to create large backup fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source, "PRAGMA page_size=4096;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to set large backup fixture page size: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to create large backup table: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_prepare_v2(source,
        "INSERT INTO payload(data) VALUES(zeroblob(?));", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to prepare large backup fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_bind_int(stmt, 1, 20 * 1024 * 1024);
    if (sqlite3_step(stmt) != SQLITE_DONE) {
        printf("Failed to populate large backup fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_finalize(stmt);
    stmt = NULL;
    sqlite3_close(source);
    source = NULL;

    if (backup_database(TEST_LARGE_DB_PATH, TEST_LARGE_BACKUP_PATH) != 0) {
        printf("Failed to create multi-batch database backup\n");
        goto cleanup;
    }

    rc = sqlite3_open_v2(TEST_LARGE_BACKUP_PATH, &backup, SQLITE_OPEN_READONLY, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to open multi-batch backup: %s\n", sqlite3_errmsg(backup));
        goto cleanup;
    }
    rc = sqlite3_prepare_v2(backup,
        "SELECT length(data) FROM payload WHERE id = 1;", -1, &stmt, NULL);
    if (rc != SQLITE_OK || sqlite3_step(stmt) != SQLITE_ROW ||
        sqlite3_column_int(stmt, 0) != 20 * 1024 * 1024) {
        printf("Multi-batch backup did not preserve its payload\n");
        goto cleanup;
    }

    printf("Large incremental database backup verified successfully\n");
    result = 0;

cleanup:
    if (stmt) sqlite3_finalize(stmt);
    if (source) sqlite3_close(source);
    if (backup) sqlite3_close(backup);
    remove_database_files(TEST_LARGE_DB_PATH);
    remove_database_files(TEST_LARGE_BACKUP_PATH);
    remove_database_files(TEST_LARGE_BACKUP_PATH ".tmp");
    return result;
}

// Test restore functionality
static int test_restore(void) {
    char stale_wal[256];
    char stale_shm[256];
    snprintf(stale_wal, sizeof(stale_wal), "%s-wal", TEST_DB_PATH);
    snprintf(stale_shm, sizeof(stale_shm), "%s-shm", TEST_DB_PATH);

    /* Simulate sidecars left by an uncleanly terminated prior database. They
     * must not be paired with the newly restored main-file inode. */
    int fd = open(stale_wal, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (fd < 0 || write(fd, "stale-wal", 9) != 9) {
        if (fd >= 0) close(fd);
        printf("Failed to create stale WAL restore fixture\n");
        return -1;
    }
    close(fd);
    fd = open(stale_shm, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (fd < 0 || write(fd, "stale-shm", 9) != 9) {
        if (fd >= 0) close(fd);
        printf("Failed to create stale SHM restore fixture\n");
        return -1;
    }
    close(fd);

    // Restore the database from backup
    int rc = restore_database_from_backup(TEST_BACKUP_PATH, TEST_DB_PATH);
    if (rc != 0) {
        printf("Failed to restore database from backup\n");
        return -1;
    }
    if (access(stale_wal, F_OK) == 0 || access(stale_shm, F_OK) == 0) {
        printf("Restore left stale WAL/SHM sidecars behind\n");
        return -1;
    }
    
    printf("Database restored successfully from backup\n");
    return 0;
}

// Main test function
int main(void) {
    // Initialize logger
    init_logger();
    load_default_config(&g_config);
    
    printf("=== Database Backup and Recovery Test ===\n");
    
    // Create a test database
    if (create_test_database() != 0) {
        printf("Test failed: Could not create test database\n");
        return 1;
    }
    
    // Verify the database
    if (verify_database() != 0) {
        printf("Test failed: Database verification failed after creation\n");
        return 1;
    }
    
    // Create a backup
    if (test_backup() != 0) {
        printf("Test failed: Could not create backup\n");
        return 1;
    }

    if (test_large_incremental_backup() != 0) {
        printf("Test failed: Large incremental backup failed\n");
        return 1;
    }
    
    // Corrupt the database
    if (corrupt_database() != 0) {
        printf("Test failed: Could not corrupt database\n");
        return 1;
    }
    
    // Try to verify the corrupted database (should fail)
    if (verify_database() == 0) {
        printf("Test failed: Database verification succeeded with corrupted database\n");
        return 1;
    } else {
        printf("Database verification failed as expected with corrupted database\n");
    }
    
    // Restore from backup
    if (test_restore() != 0) {
        printf("Test failed: Could not restore database from backup\n");
        return 1;
    }
    
    // Verify the restored database
    if (verify_database() != 0) {
        printf("Test failed: Database verification failed after restore\n");
        return 1;
    }
    
    // Test crash recovery
    printf("\n=== Testing Crash Recovery ===\n");
    
    // Set up signal handler for simulating a crash
    signal(SIGUSR1, simulate_crash);
    
    // Fork a child process to simulate a crash
    pid_t pid = fork();
    
    if (pid == 0) {
        // Child process
        
        // Initialize the database
        if (init_database(TEST_DB_PATH) != 0) {
            printf("Child: Failed to initialize database\n");
            exit(1);
        }
        
        // Get the database handle
        sqlite3 *db = get_db_handle();
        if (!db) {
            printf("Child: Failed to get database handle\n");
            exit(1);
        }
        
        // Start a transaction
        char *err_msg = NULL;
        int rc = sqlite3_exec(db, "BEGIN TRANSACTION;", NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            printf("Child: Failed to begin transaction: %s\n", err_msg);
            sqlite3_free(err_msg);
            exit(1);
        }
        
        // Insert some data
        const char *insert_data = "INSERT INTO test (id, value) VALUES (3, 'test data 3');";
        rc = sqlite3_exec(db, insert_data, NULL, NULL, &err_msg);
        if (rc != SQLITE_OK) {
            printf("Child: Failed to insert test data: %s\n", err_msg);
            sqlite3_free(err_msg);
            exit(1);
        }
        
        printf("Child: Inserted data, simulating crash before commit...\n");
        
        // Simulate a crash before committing
        raise(SIGUSR1);
        
        // Should not reach here
        exit(0);
    } else if (pid > 0) {
        // Parent process
        int status;
        
    // Wait for the child to exit
    waitpid(pid, &status, 0);
        
        printf("Parent: Child process exited with status %d\n", WEXITSTATUS(status));
        
        // Verify the database integrity
        printf("Parent: Verifying database integrity after crash...\n");
        
        // Initialize the database (this should trigger recovery if needed)
        if (init_database(TEST_DB_PATH) != 0) {
            printf("Parent: Failed to initialize database after crash\n");
            return 1;
        }
        
        // Verify the database
        if (verify_database() != 0) {
            printf("Test failed: Database verification failed after crash recovery\n");
            return 1;
        }
        
        printf("Parent: Database integrity verified after crash\n");
    } else {
        printf("Failed to fork child process\n");
        return 1;
    }
    
    printf("\n=== All tests passed successfully ===\n");
    
    // Clean up
    shutdown_database();
    
    return 0;
}
