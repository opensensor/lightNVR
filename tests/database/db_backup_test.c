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
#include <sys/stat.h>
#include <limits.h>
#include <dirent.h>

#include "database/db_core.h"
#include "database/db_backup.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"

// Test database path
#define TEST_DB_PATH "/tmp/test_db.sqlite"
#define TEST_BACKUP_PATH "/tmp/test_db.sqlite.bak"
#define TEST_LARGE_DB_PATH "/tmp/test_db_large.sqlite"
#define TEST_LARGE_BACKUP_PATH "/tmp/test_db_large.sqlite.bak"
#define TEST_ABORT_DB_PATH "/tmp/test_db_abort.sqlite"
#define TEST_ABORT_BACKUP_PATH "/tmp/test_db_abort.sqlite.bak"
#define TEST_SHUTDOWN_DB_PATH "/tmp/test_db_shutdown.sqlite"

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
    int rc = backup_database(TEST_DB_PATH, TEST_BACKUP_PATH, true);
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

    if (backup_database(TEST_LARGE_DB_PATH, TEST_LARGE_BACKUP_PATH, true) != 0) {
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

// Regression test: a stuck main loop couldn't notice a pending restart while
// backup_database() was mid-copy on a large database, because the batch loop
// never checked for one. This silently swallowed every "Restart LightNVR"
// click for as long as the backup took (tens of minutes on a multi-GB
// production database). Verifies a backup requested to abort mid-flight
// bails out promptly and leaves no partial temp file behind, rather than
// running to completion.
static int test_backup_aborts_when_shutdown_requested(void) {
    sqlite3 *source = NULL;
    sqlite3_stmt *stmt = NULL;
    int result = -1;
    int rc;
    char temp_path[PATH_MAX];
    struct stat st;

    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);
    snprintf(temp_path, sizeof(temp_path), "%s.tmp", TEST_ABORT_BACKUP_PATH);
    unlink(temp_path);

    rc = sqlite3_open(TEST_ABORT_DB_PATH, &source);
    if (rc != SQLITE_OK) {
        printf("Failed to create abort-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source, "PRAGMA page_size=4096;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to set abort-test fixture page size: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to create abort-test table: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_prepare_v2(source,
        "INSERT INTO payload(data) VALUES(zeroblob(?));", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to prepare abort-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    // Larger than one BACKUP_STEP_PAGES batch (16MB) so the abort check
    // between batches actually gets exercised before the copy would finish.
    sqlite3_bind_int(stmt, 1, 20 * 1024 * 1024);
    if (sqlite3_step(stmt) != SQLITE_DONE) {
        printf("Failed to populate abort-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_finalize(stmt);
    stmt = NULL;
    sqlite3_close(source);
    source = NULL;

    // Simulate a restart/shutdown request arriving before the backup starts.
    request_background_abort();

    rc = backup_database(TEST_ABORT_DB_PATH, TEST_ABORT_BACKUP_PATH, true);
    if (rc == 0) {
        printf("Backup should have aborted early but reported success\n");
        goto cleanup;
    }

    if (stat(temp_path, &st) == 0) {
        printf("Aborted backup left behind a temp file: %s\n", temp_path);
        goto cleanup;
    }
    if (stat(TEST_ABORT_BACKUP_PATH, &st) == 0) {
        printf("Aborted backup should not have produced a final backup file\n");
        goto cleanup;
    }

    printf("Backup aborted early on restart/shutdown request, as expected\n");
    result = 0;

cleanup:
    if (stmt) sqlite3_finalize(stmt);
    if (source) sqlite3_close(source);
    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);
    unlink(temp_path);
    return result;
}

// Regression test for a bug introduced by the abort check above: the
// deliberate one-time backup taken while the process is already shutting
// down (shutdown_database()'s "final backup") shares this same code path,
// but the abort flag is *always* already set by the time that one runs --
// checking it there made every graceful shutdown's final backup bail out
// instantly and never actually produce a backup. Sets the (idempotent,
// never-reset -- see request_background_abort()'s header comment)
// process-wide abort flag explicitly rather than relying on a previous
// test having left it set, so this test doesn't depend on execution order.
static int test_backup_completes_when_not_abortable_even_if_abort_requested(void) {
    sqlite3 *source = NULL;
    sqlite3_stmt *stmt = NULL;
    int result = -1;
    int rc;

    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);

    rc = sqlite3_open(TEST_ABORT_DB_PATH, &source);
    if (rc != SQLITE_OK) {
        printf("Failed to create not-abortable-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source, "PRAGMA page_size=4096;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to set not-abortable-test fixture page size: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to create not-abortable-test table: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_prepare_v2(source,
        "INSERT INTO payload(data) VALUES(zeroblob(?));", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to prepare not-abortable-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    // Larger than one BACKUP_STEP_PAGES batch (16MB), so if abortable were
    // mistakenly honored here, it would bail out before finishing.
    sqlite3_bind_int(stmt, 1, 20 * 1024 * 1024);
    if (sqlite3_step(stmt) != SQLITE_DONE) {
        printf("Failed to populate not-abortable-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_finalize(stmt);
    stmt = NULL;
    sqlite3_close(source);
    source = NULL;

    request_background_abort();

    rc = backup_database(TEST_ABORT_DB_PATH, TEST_ABORT_BACKUP_PATH, false);
    if (rc != 0) {
        printf("Non-abortable backup should have completed despite the pending abort request\n");
        goto cleanup;
    }
    {
        struct stat st;
        if (stat(TEST_ABORT_BACKUP_PATH, &st) != 0 || st.st_size == 0) {
            printf("Non-abortable backup reported success but produced no output file\n");
            goto cleanup;
        }
    }

    printf("Non-abortable backup completed despite a pending restart/shutdown request, as expected\n");
    result = 0;

cleanup:
    if (stmt) sqlite3_finalize(stmt);
    if (source) sqlite3_close(source);
    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);
    return result;
}

// Regression test for a bug found live in production: once the copy loop's
// abort check lets a batch that reaches SQLITE_DONE finish (by design -- see
// the comment in backup_database()), the *next* step is a full
// PRAGMA integrity_check verification pass over the whole backup file. For a
// multi-gigabyte database that scan alone can take as long as the copy it
// verifies, and had no abort check at all -- a live gdb backtrace during a
// stuck restart showed the main thread parked in sqlite3BtreeIntegrityCheck
// long after the copy loop's own fix should have made it responsive.
// Uses a single-batch fixture (under one BACKUP_STEP_PAGES) so the copy loop
// itself reaches SQLITE_DONE without ever consulting the abort flag, forcing
// this test to exercise the verification-phase check specifically. Sets the
// abort flag explicitly (it's idempotent) rather than relying on an earlier
// test having already set it, so this test doesn't depend on execution order.
static int test_backup_aborts_during_verification_when_shutdown_requested(void) {
    sqlite3 *source = NULL;
    sqlite3_stmt *stmt = NULL;
    int result = -1;
    int rc;
    char temp_path[PATH_MAX];
    struct stat st;

    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);
    snprintf(temp_path, sizeof(temp_path), "%s.tmp", TEST_ABORT_BACKUP_PATH);
    unlink(temp_path);

    rc = sqlite3_open(TEST_ABORT_DB_PATH, &source);
    if (rc != SQLITE_OK) {
        printf("Failed to create verification-abort-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source, "PRAGMA page_size=4096;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to set verification-abort-test fixture page size: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to create verification-abort-test table: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    // Many small rows rather than one big blob: integrity_check's VM cost is
    // dominated by per-cell/per-page bookkeeping (ordering, checksums), not
    // raw bytes, so a cell-dense B-tree crosses BACKUP_VERIFY_PROGRESS_OPS
    // reliably while still fitting in well under one BACKUP_STEP_PAGES batch
    // (16MB) -- a single large blob (tried first) didn't generate enough VM
    // ops to ever invoke the progress callback.
    rc = sqlite3_exec(source,
        "WITH RECURSIVE seq(x) AS ("
        "  SELECT 1 UNION ALL SELECT x+1 FROM seq WHERE x < 300000"
        ") INSERT INTO payload(data) SELECT randomblob(16) FROM seq;",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to populate verification-abort-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_close(source);
    source = NULL;

    request_background_abort();

    rc = backup_database(TEST_ABORT_DB_PATH, TEST_ABORT_BACKUP_PATH, true);
    if (rc == 0) {
        printf("Backup should have aborted during verification but reported success\n");
        goto cleanup;
    }
    if (stat(temp_path, &st) == 0) {
        printf("Backup aborted during verification left behind a temp file: %s\n", temp_path);
        goto cleanup;
    }
    if (stat(TEST_ABORT_BACKUP_PATH, &st) == 0) {
        printf("Backup aborted during verification should not have produced a final backup file\n");
        goto cleanup;
    }

    printf("Backup aborted during post-copy verification on restart/shutdown request, as expected\n");
    result = 0;

cleanup:
    if (stmt) sqlite3_finalize(stmt);
    if (source) sqlite3_close(source);
    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);
    unlink(temp_path);
    return result;
}

static int count_timestamped_backups(const char *db_path) {
    char backup_dir[PATH_MAX];
    snprintf(backup_dir, sizeof(backup_dir), "%s.backups", db_path);
    DIR *dir = opendir(backup_dir);
    if (!dir) return 0;
    int count = 0;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        size_t name_len = strlen(entry->d_name);
        // Count only completed backups (name.sqlite3), not .tmp/-wal/-shm/
        // -journal debris from an interrupted or in-progress copy.
        if (name_len > 8 && strcmp(entry->d_name + name_len - 8, ".sqlite3") == 0) {
            count++;
        }
    }
    closedir(dir);
    return count;
}

static void remove_test_db_and_backups(const char *db_path) {
    char path[PATH_MAX];
    static const char *sidecar_suffixes[] = {"", ".bak", "-wal", "-shm", "-journal"};
    for (size_t i = 0; i < sizeof(sidecar_suffixes) / sizeof(sidecar_suffixes[0]); i++) {
        snprintf(path, sizeof(path), "%s%s", db_path, sidecar_suffixes[i]);
        unlink(path);
    }

    // A stale <db_path>.bak or -wal/-shm/-journal sidecar left behind by an
    // earlier test would otherwise let init_database_ex() pick up a bogus
    // last_backup_time (it stat()s the .bak path at startup) or a stale WAL,
    // causing cross-test interference in this shared-connection test binary.
    char backup_dir[PATH_MAX];
    snprintf(backup_dir, sizeof(backup_dir), "%s.backups", db_path);
    DIR *dir = opendir(backup_dir);
    if (dir) {
        struct dirent *entry;
        while ((entry = readdir(dir)) != NULL) {
            if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
                continue;
            }
            snprintf(path, sizeof(path), "%s/%s", backup_dir, entry->d_name);
            unlink(path);
        }
        closedir(dir);
        rmdir(backup_dir);
    }
}

// Regression test: shutdown_database() used to unconditionally take a full
// backup-and-verify snapshot on every clean shutdown, no matter how recently
// the hourly scheduled backup had already run -- on a multi-gigabyte
// production database this made every restart take minutes just to
// re-capture a few minutes of additional changes. Verifies that a shutdown
// happening shortly after a scheduled backup skips the redundant one.
static int test_shutdown_skips_backup_when_recent_backup_exists(void) {
    int result = -1;

    // db_core.c holds a single global connection shared across this whole
    // file's main(); create_test_database() (run earlier in main()) opened
    // TEST_DB_PATH and left it open -- nothing in between closes it until
    // corrupt_database() does so itself, later, right before corrupting the
    // raw file. This test needs a second, separate database open at the same
    // time, so close the existing connection first, same as corrupt_database()
    // does for its own reason.
    shutdown_database();

    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);

    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) {
        printf("Failed to init database for shutdown-skip test\n");
        return -1;
    }

    g_config.db_backup_interval_minutes = 60;

    if (maybe_run_scheduled_database_backup() != 0) {
        printf("Scheduled backup failed in shutdown-skip test setup\n");
        goto cleanup;
    }
    int count_after_scheduled = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);
    if (count_after_scheduled != 1) {
        printf("Expected exactly 1 backup after the scheduled cycle, found %d\n",
               count_after_scheduled);
        goto cleanup;
    }

    shutdown_database();

    int count_after_shutdown = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);
    if (count_after_shutdown != count_after_scheduled) {
        printf("Shutdown took a redundant backup: had %d, now %d\n",
               count_after_scheduled, count_after_shutdown);
        return -1;
    }

    printf("Shutdown correctly skipped a redundant backup\n");
    result = 0;

cleanup:
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    return result;
}

// Regression test for the same fix: when scheduled backups are disabled
// (db_backup_interval_minutes <= 0), there is no defined freshness window,
// so shutdown must still take its final backup rather than skipping
// unconditionally.
static int test_shutdown_backs_up_when_scheduled_backups_disabled(void) {
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);

    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) {
        printf("Failed to init database for shutdown-disabled-interval test\n");
        return -1;
    }

    g_config.db_backup_interval_minutes = 0;

    // init_database() takes its own "initial backup" of a brand-new database
    // regardless of the scheduled interval, so the baseline here is 1, not
    // 0 -- what this test actually verifies is that shutdown adds another
    // one on top, rather than caring about that unrelated initial-backup
    // implementation detail.
    int count_before_shutdown = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);

    shutdown_database();

    int count = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    if (count <= count_before_shutdown) {
        printf("Expected shutdown to take a backup with scheduled backups "
               "disabled: had %d before, %d after\n", count_before_shutdown,
               count);
        return -1;
    }

    printf("Shutdown correctly backed up despite scheduled backups being disabled\n");
    return 0;
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

    if (test_backup_aborts_when_shutdown_requested() != 0) {
        printf("Test failed: Backup did not abort early on restart/shutdown request\n");
        return 1;
    }

    if (test_backup_aborts_during_verification_when_shutdown_requested() != 0) {
        printf("Test failed: Backup did not abort during post-copy verification on restart/shutdown request\n");
        return 1;
    }

    if (test_backup_completes_when_not_abortable_even_if_abort_requested() != 0) {
        printf("Test failed: Non-abortable backup did not complete despite a pending abort request\n");
        return 1;
    }

    if (test_shutdown_skips_backup_when_recent_backup_exists() != 0) {
        printf("Test failed: shutdown did not skip a redundant backup\n");
        return 1;
    }

    if (test_shutdown_backs_up_when_scheduled_backups_disabled() != 0) {
        printf("Test failed: shutdown did not back up with scheduled backups disabled\n");
        return 1;
    }

    // Restore the interval this file's own load_default_config() set, in
    // case any later step in this binary implicitly depends on it.
    load_default_config(&g_config);

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
