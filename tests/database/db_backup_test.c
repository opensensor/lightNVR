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
#include <stdbool.h>
#include <stdatomic.h>
#include <time.h>

#include "database/db_core.h"
#include "database/db_backup.h"

// Test-only hook into db_backup.c's internal duration safety valve. Not
// declared in the public header (production code has no business mutating
// a global backup timeout at runtime), so it's declared here instead.
extern void db_backup_set_max_duration_seconds_for_testing(int seconds);
extern int db_backup_pin_source_snapshot(sqlite3 *source_db);

// Test-only hook into db_core.c's scheduled-backup worker, same convention:
// swaps the cycle the worker runs so a test can hold it in flight or delay it
// deterministically. The override receives the real cycle so it can still
// delegate to it (and so the real copy/abort path stays under test).
extern void db_scheduled_backup_set_cycle_fn_for_testing(int (*fn)(int (*real_cycle)(void)));
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
#define TEST_SNAPSHOT_DB_PATH "/tmp/test_db_snapshot.sqlite"
#define TEST_SNAPSHOT_COPY_PATH "/tmp/test_db_snapshot_copy.sqlite"

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

/* [database] backup_verify selects the post-copy scan (full, quick, off).
 * Whatever the operator picks, the published file must still be a complete,
 * self-contained, independently verifiable copy (issue #580). */
static int check_published_backup_is_intact(const char *label) {
    sqlite3 *backup_db = NULL;
    sqlite3_stmt *stmt = NULL;
    int ok = -1;
    if (sqlite3_open_v2(TEST_BACKUP_PATH, &backup_db, SQLITE_OPEN_READONLY, NULL) != SQLITE_OK) {
        printf("%s: cannot open published backup\n", label);
        return -1;
    }
    if (sqlite3_prepare_v2(backup_db, "PRAGMA integrity_check;", -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW &&
        strcmp((const char *)sqlite3_column_text(stmt, 0), "ok") == 0) {
        ok = 0;
    } else {
        printf("%s: published backup failed an independent integrity_check\n", label);
    }
    if (stmt) sqlite3_finalize(stmt);
    sqlite3_close(backup_db);
    return ok;
}

static int test_backup_verify_modes(void) {
    const int modes[] = { DB_BACKUP_VERIFY_QUICK, DB_BACKUP_VERIFY_OFF, DB_BACKUP_VERIFY_FULL };
    const char *names[] = { "quick", "off", "full" };
    for (size_t i = 0; i < sizeof(modes) / sizeof(modes[0]); i++) {
        g_config.db_backup_verify = modes[i];
        unlink(TEST_BACKUP_PATH);
        if (backup_database(TEST_DB_PATH, TEST_BACKUP_PATH, true) != 0) {
            printf("backup_verify=%s: backup failed\n", names[i]);
            g_config.db_backup_verify = DB_BACKUP_VERIFY_FULL;
            return -1;
        }
        if (check_published_backup_is_intact(names[i]) != 0) {
            g_config.db_backup_verify = DB_BACKUP_VERIFY_FULL;
            return -1;
        }
    }
    g_config.db_backup_verify = DB_BACKUP_VERIFY_FULL;
    printf("Backup verify modes (quick/off/full) all produced intact backups\n");
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

// Regression test for a bug found live in production: a scheduled backup
// that hung during its copy phase blocked the main loop (which runs
// maybe_run_scheduled_database_backup() synchronously) for almost 12 hours
// straight, silently skipping every other scheduled backup in that window,
// with no way to recover short of an operator happening to trigger a
// restart. Verifies the new stuck-backup safety valve: an abortable backup
// whose duration budget is already exhausted aborts on the very next
// between-batches check instead of running unbounded.
static int test_backup_aborts_early_when_duration_exceeded(void) {
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
        printf("Failed to create duration-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source, "PRAGMA page_size=4096;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to set duration-test fixture page size: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to create duration-test table: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_prepare_v2(source,
        "INSERT INTO payload(data) VALUES(zeroblob(?));", -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to prepare duration-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    // Larger than one BACKUP_STEP_PAGES batch (16MB) so the between-batches
    // deadline check actually gets exercised before the copy would finish.
    sqlite3_bind_int(stmt, 1, 20 * 1024 * 1024);
    if (sqlite3_step(stmt) != SQLITE_DONE) {
        printf("Failed to populate duration-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_finalize(stmt);
    stmt = NULL;
    sqlite3_close(source);
    source = NULL;

    // Force an already-expired deadline deterministically, rather than
    // waiting out a real timeout.
    db_backup_set_max_duration_seconds_for_testing(-60);

    rc = backup_database(TEST_ABORT_DB_PATH, TEST_ABORT_BACKUP_PATH, true);
    db_backup_set_max_duration_seconds_for_testing(DB_BACKUP_MAX_DURATION_SECONDS_DEFAULT);
    if (rc == 0) {
        printf("Backup should have aborted early on an exhausted duration budget but reported success\n");
        goto cleanup;
    }

    if (stat(temp_path, &st) == 0) {
        printf("Duration-aborted backup left behind a temp file: %s\n", temp_path);
        goto cleanup;
    }
    if (stat(TEST_ABORT_BACKUP_PATH, &st) == 0) {
        printf("Duration-aborted backup should not have produced a final backup file\n");
        goto cleanup;
    }

    printf("Backup aborted early on an exhausted duration budget, as expected\n");
    result = 0;

cleanup:
    if (stmt) sqlite3_finalize(stmt);
    if (source) sqlite3_close(source);
    unlink(TEST_ABORT_DB_PATH);
    unlink(TEST_ABORT_BACKUP_PATH);
    unlink(temp_path);
    return result;
}

// Same production bug as above, but for the verification phase: once the
// copy loop reaches SQLITE_DONE it isn't re-checked, so a stuck
// PRAGMA integrity_check needs its own deadline check (progress_during_
// verification) to ever be interrupted. Uses the same cell-dense,
// single-batch fixture as the shutdown-request verification-abort test so
// the copy loop finishes without consulting the deadline, forcing this test
// to exercise the verification-phase check specifically.
static int test_backup_aborts_during_verification_when_duration_exceeded(void) {
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
        printf("Failed to create verification-duration-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source, "PRAGMA page_size=4096;", NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to set verification-duration-test fixture page size: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to create verification-duration-test table: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    rc = sqlite3_exec(source,
        "WITH RECURSIVE seq(x) AS ("
        "  SELECT 1 UNION ALL SELECT x+1 FROM seq WHERE x < 300000"
        ") INSERT INTO payload(data) SELECT randomblob(16) FROM seq;",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) {
        printf("Failed to populate verification-duration-test fixture: %s\n", sqlite3_errmsg(source));
        goto cleanup;
    }
    sqlite3_close(source);
    source = NULL;

    db_backup_set_max_duration_seconds_for_testing(-60);

    rc = backup_database(TEST_ABORT_DB_PATH, TEST_ABORT_BACKUP_PATH, true);
    db_backup_set_max_duration_seconds_for_testing(DB_BACKUP_MAX_DURATION_SECONDS_DEFAULT);
    if (rc == 0) {
        printf("Backup should have aborted during verification on an exhausted duration budget but reported success\n");
        goto cleanup;
    }
    if (stat(temp_path, &st) == 0) {
        printf("Backup aborted during verification (duration) left behind a temp file: %s\n", temp_path);
        goto cleanup;
    }
    if (stat(TEST_ABORT_BACKUP_PATH, &st) == 0) {
        printf("Backup aborted during verification (duration) should not have produced a final backup file\n");
        goto cleanup;
    }

    printf("Backup aborted during post-copy verification on an exhausted duration budget, as expected\n");
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

// Partial copies (.sqlite3.tmp and its -wal/-shm/-journal) an aborted or
// still-running backup_database() would leave in the backup directory.
static int count_temporary_backup_artifacts(const char *db_path) {
    char backup_dir[PATH_MAX];
    snprintf(backup_dir, sizeof(backup_dir), "%s.backups", db_path);
    DIR *dir = opendir(backup_dir);
    if (!dir) return 0;
    int count = 0;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        if (strstr(entry->d_name, ".sqlite3.tmp") != NULL) {
            count++;
        }
    }
    closedir(dir);
    return count;
}

// Create a database file up front so init_database() sees an existing
// database and does not take its own "initial backup" of a new one, which
// would otherwise refresh last_backup_time and pre-empt the scheduler.
static int create_existing_database_fixture(const char *db_path) {
    sqlite3 *existing = NULL;
    if (sqlite3_open(db_path, &existing) != SQLITE_OK) {
        if (existing) sqlite3_close(existing);
        return -1;
    }
    int rc = sqlite3_exec(existing, "CREATE TABLE existing_fixture(id INTEGER);",
                          NULL, NULL, NULL);
    sqlite3_close(existing);
    return rc == SQLITE_OK ? 0 : -1;
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

// A failed scheduled attempt must wait for the next interval, but must not
// count as a successful snapshot when deciding whether to back up at shutdown.
// The cycle now runs on the worker thread: the tick only *starts* it, so the
// test waits for the worker before checking the outcome, and the cooldown is
// observed on the following tick exactly as before.
static int test_failed_scheduled_backup_waits_without_suppressing_shutdown_backup(void) {
    shutdown_database();
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    // Simulate an existing database with no successful backup yet.
    if (create_existing_database_fixture(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    g_config.db_backup_interval_minutes = 60;

    // An indexed fixture large enough to invoke the verification callback
    // deterministically reaches the expired deadline without sleeping.
    int rc = sqlite3_exec(get_db_handle(),
        "CREATE TABLE retry_fixture(id INTEGER PRIMARY KEY, value TEXT);"
        "WITH RECURSIVE n(i) AS (VALUES(1) UNION ALL SELECT i+1 FROM n WHERE i<20000) "
        "INSERT INTO retry_fixture SELECT i,printf('value-%d',i) FROM n;"
        "CREATE INDEX retry_value ON retry_fixture(value);",
        NULL, NULL, NULL);
    if (rc != SQLITE_OK) return -1;

    db_backup_set_max_duration_seconds_for_testing(-60);
    int first = maybe_run_scheduled_database_backup();
    bool finished = db_scheduled_backup_wait_idle(30000);
    int second = maybe_run_scheduled_database_backup();
    db_backup_set_max_duration_seconds_for_testing(DB_BACKUP_MAX_DURATION_SECONDS_DEFAULT);
    if (first != 1 || !finished || second != 0 ||
        count_timestamped_backups(TEST_SHUTDOWN_DB_PATH) != 0) {
        printf("Failed backup was not started (%d), never finished (%d), was retried "
               "immediately (%d) or was published as successful\n",
               first, finished, second);
        shutdown_database();
        remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
        return -1;
    }

    shutdown_database();
    int count = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    if (count != 1) {
        printf("Failed scheduled attempt incorrectly suppressed shutdown backup\n");
        return -1;
    }
    return 0;
}

// A shutdown shortly after a successful scheduled backup should not repeat
// the full copy and verification of a potentially multi-gigabyte database.
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

    // Existing database, so the backup counted below is the scheduled
    // cycle's own rather than init_database()'s initial backup of a new one.
    if (create_existing_database_fixture(TEST_SHUTDOWN_DB_PATH) != 0) {
        printf("Failed to create fixture for shutdown-skip test\n");
        return -1;
    }
    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) {
        printf("Failed to init database for shutdown-skip test\n");
        return -1;
    }

    g_config.db_backup_interval_minutes = 60;

    if (maybe_run_scheduled_database_backup() != 1) {
        printf("Scheduled backup was not started in shutdown-skip test setup\n");
        goto cleanup;
    }
    if (!db_scheduled_backup_wait_idle(30000)) {
        printf("Scheduled backup did not finish in shutdown-skip test setup\n");
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

// ---- Scheduled-backup worker ------------------------------------------------
//
// State the injected cycle publishes for the tests below. The worker runs the
// hook on its own thread, so everything is atomic.
static atomic_int hook_cycles_entered;
static atomic_bool hook_release;
static atomic_bool hook_finished;
static atomic_bool hook_db_open_at_end;
static atomic_int hook_real_cycle_result;

static void reset_hook_state(void) {
    atomic_store(&hook_cycles_entered, 0);
    atomic_store(&hook_release, false);
    atomic_store(&hook_finished, false);
    atomic_store(&hook_db_open_at_end, false);
    atomic_store(&hook_real_cycle_result, 0);
}

static void publish_hook_outcome(int rc) {
    atomic_store(&hook_real_cycle_result, rc);
    // Ordering witness: was the global handle still open when the cycle
    // finished? shutdown_database() must wait for the worker before closing
    // it, so this is true whenever shutdown honoured that ordering.
    atomic_store(&hook_db_open_at_end, get_db_handle() != NULL);
    atomic_store(&hook_finished, true);
}

// Holds the cycle in flight until the test releases it, then runs the real
// one so the ordinary completion path (timestamps, backup file) is exercised.
static int gated_cycle(int (*real_cycle)(void)) {
    atomic_fetch_add(&hook_cycles_entered, 1);
    for (int i = 0; i < 1000 && !atomic_load(&hook_release); i++) {
        usleep(10000);  // 10 s cap so a broken test cannot hang the binary
    }
    int rc = real_cycle();
    publish_hook_outcome(rc);
    return rc;
}

// Simulates a cycle that is still mid-copy when shutdown arrives: stays in
// flight for a moment, then runs the real large-fixture copy -- which is what
// the abort/cancel raised meanwhile must actually stop.
#define DELAYED_CYCLE_HOLD_US 500000
static int delayed_real_cycle(int (*real_cycle)(void)) {
    atomic_fetch_add(&hook_cycles_entered, 1);
    usleep(DELAYED_CYCLE_HOLD_US);
    int rc = real_cycle();
    publish_hook_outcome(rc);
    return rc;
}

static int wait_for_hook_entry(void) {
    for (int i = 0; i < 500; i++) {  // 5 s
        if (atomic_load(&hook_cycles_entered) >= 1) return 0;
        usleep(10000);
    }
    printf("Scheduled backup worker never entered the injected cycle\n");
    return -1;
}

static double elapsed_ms_since(const struct timespec *start) {
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    return (double)(now.tv_sec - start->tv_sec) * 1000.0 +
           (double)(now.tv_nsec - start->tv_nsec) / 1e6;
}

// Seed the live database with one blob larger than a single 16 MiB
// sqlite3_backup_step() batch, so backup_database() reaches its
// between-batches abort check before the copy would otherwise finish.
static int seed_multi_batch_payload(sqlite3 *db) {
    sqlite3_stmt *stmt = NULL;
    if (sqlite3_exec(db, "CREATE TABLE payload (id INTEGER PRIMARY KEY, data BLOB);",
                     NULL, NULL, NULL) != SQLITE_OK) {
        printf("Failed to create multi-batch payload table: %s\n", sqlite3_errmsg(db));
        return -1;
    }
    if (sqlite3_prepare_v2(db, "INSERT INTO payload(data) VALUES(zeroblob(?));",
                           -1, &stmt, NULL) != SQLITE_OK) {
        printf("Failed to prepare multi-batch payload: %s\n", sqlite3_errmsg(db));
        return -1;
    }
    sqlite3_bind_int(stmt, 1, 20 * 1024 * 1024);
    int rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if (rc != SQLITE_DONE) {
        printf("Failed to populate multi-batch payload: %s\n", sqlite3_errmsg(db));
        return -1;
    }
    return 0;
}

// The scheduler must never overlap cycles now that they run off the main
// loop: a tick that arrives while the worker is still busy is a no-op, and
// once the cycle completes its result feeds the same cooldown and
// shutdown-skip decisions as the synchronous version did.
static int test_scheduler_does_not_start_second_cycle_while_in_flight(void) {
    int result = -1;

    shutdown_database();
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    if (create_existing_database_fixture(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    g_config.db_backup_interval_minutes = 60;

    reset_hook_state();
    db_scheduled_backup_set_cycle_fn_for_testing(gated_cycle);

    int first = maybe_run_scheduled_database_backup();
    if (first != 1) {
        printf("First tick should have started a cycle, returned %d\n", first);
        goto cleanup;
    }
    if (wait_for_hook_entry() != 0) goto cleanup;
    if (!db_scheduled_backup_in_flight()) {
        printf("Scheduler does not report the running cycle as in flight\n");
        goto cleanup;
    }

    int second = maybe_run_scheduled_database_backup();
    if (second != 0 || atomic_load(&hook_cycles_entered) != 1) {
        printf("A tick during an in-flight cycle started another one (rc=%d, cycles=%d)\n",
               second, atomic_load(&hook_cycles_entered));
        goto cleanup;
    }

    atomic_store(&hook_release, true);
    if (!db_scheduled_backup_wait_idle(30000)) {
        printf("Released cycle never finished\n");
        goto cleanup;
    }
    if (db_scheduled_backup_in_flight() ||
        atomic_load(&hook_real_cycle_result) != 0 ||
        count_timestamped_backups(TEST_SHUTDOWN_DB_PATH) != 1) {
        printf("Background cycle did not complete with one published backup (rc=%d, backups=%d)\n",
               atomic_load(&hook_real_cycle_result),
               count_timestamped_backups(TEST_SHUTDOWN_DB_PATH));
        goto cleanup;
    }

    // Completion feeds the cooldown: the next tick within the interval is a
    // no-op ...
    int third = maybe_run_scheduled_database_backup();
    if (third != 0 || atomic_load(&hook_cycles_entered) != 1) {
        printf("Tick right after a completed cycle started another one (rc=%d)\n", third);
        goto cleanup;
    }

    // ... and the shutdown decision: the backup the worker just took is recent.
    db_scheduled_backup_set_cycle_fn_for_testing(NULL);
    shutdown_database();
    if (count_timestamped_backups(TEST_SHUTDOWN_DB_PATH) != 1) {
        printf("Shutdown took a redundant backup after a completed background cycle\n");
        goto cleanup;
    }

    printf("Scheduler refused to overlap an in-flight cycle and honoured its completion\n");
    result = 0;

cleanup:
    atomic_store(&hook_release, true);
    (void)db_scheduled_backup_wait_idle(30000);
    db_scheduled_backup_set_cycle_fn_for_testing(NULL);
    shutdown_database();
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    return result;
}

// Shared body for the two shutdown-with-in-flight-cycle tests below. Starts
// a scheduled cycle on a multi-batch fixture that is deliberately still in
// flight when shutdown_database() is called, and checks that shutdown
// (1) waited for the worker instead of closing the handle under it, (2) got
// the worker to abort within the bounded wait, (3) did not treat the aborted
// cycle as a fresh backup, so its own final backup still ran, and (4) left
// no partial temporary copy behind.
//
// raise_process_abort selects the caller being modelled: main.c's restart/
// shutdown path raises request_background_abort() before shutting the
// database down; a settings-driven database restart keeps the process
// running, so it must not touch that never-reset flag and relies on
// shutdown_database() cancelling the in-flight backup on its own.
static int run_shutdown_with_in_flight_cycle(bool raise_process_abort) {
    int result = -1;
    struct timespec shutdown_started;
    double shutdown_ms = 0;

    shutdown_database();
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    if (create_existing_database_fixture(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    if (seed_multi_batch_payload(get_db_handle()) != 0) goto cleanup;
    g_config.db_backup_interval_minutes = 60;

    reset_hook_state();
    db_scheduled_backup_set_cycle_fn_for_testing(delayed_real_cycle);

    if (maybe_run_scheduled_database_backup() != 1) {
        printf("Scheduled cycle was not started\n");
        goto cleanup;
    }
    if (wait_for_hook_entry() != 0) goto cleanup;

    if (raise_process_abort) {
        // What signal_handler() / request_restart() do the instant a
        // restart or shutdown is requested, before the main loop even exits.
        request_background_abort();
    }

    clock_gettime(CLOCK_MONOTONIC, &shutdown_started);
    shutdown_database();
    shutdown_ms = elapsed_ms_since(&shutdown_started);

    if (!atomic_load(&hook_finished)) {
        printf("shutdown_database() returned while the scheduled cycle was still running\n");
        goto cleanup;
    }
    if (!atomic_load(&hook_db_open_at_end)) {
        printf("Database handle was closed before the in-flight scheduled cycle finished\n");
        goto cleanup;
    }
    if (atomic_load(&hook_real_cycle_result) == 0) {
        printf("In-flight scheduled cycle should have been aborted but completed\n");
        goto cleanup;
    }
    if (db_scheduled_backup_in_flight()) {
        printf("Worker still reported in flight after shutdown_database()\n");
        goto cleanup;
    }
    // Waited for the held cycle (so the join is real), but well inside the
    // bounded wait (so the abort was honoured promptly rather than the
    // cycle running to completion or the budget expiring).
    if (shutdown_ms < DELAYED_CYCLE_HOLD_US / 1000.0 * 0.8 ||
        shutdown_ms > DB_SCHEDULED_BACKUP_JOIN_TIMEOUT_MS / 2.0) {
        printf("shutdown_database() took %.0f ms; expected to join the held cycle "
               "within the bounded wait\n", shutdown_ms);
        goto cleanup;
    }
    if (count_temporary_backup_artifacts(TEST_SHUTDOWN_DB_PATH) != 0) {
        printf("Aborted scheduled cycle left a partial temporary backup behind\n");
        goto cleanup;
    }
    // Exactly one backup: the shutdown backup. The aborted cycle must neither
    // have published one nor refreshed last_backup_time and suppressed it.
    int count = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);
    if (count != 1) {
        printf("Expected only the shutdown backup after an aborted scheduled cycle, found %d\n",
               count);
        goto cleanup;
    }
    if (!raise_process_abort && is_background_abort_requested()) {
        printf("shutdown_database() must not raise the process-wide abort flag on a database restart\n");
        goto cleanup;
    }

    printf("Shutdown %s and joined the in-flight scheduled backup in %.0f ms, then took its own backup\n",
           raise_process_abort ? "aborted" : "cancelled", shutdown_ms);
    result = 0;

cleanup:
    db_scheduled_backup_set_cycle_fn_for_testing(NULL);
    (void)db_scheduled_backup_wait_idle(30000);
    shutdown_database();
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    return result;
}

// Settings-driven database restart (api_handlers_settings.c calls
// shutdown_database() + init_database() with the process still running):
// shutdown must cancel and join the in-flight cycle on its own, without the
// process-wide abort flag. Runs before any test raises that flag.
static int test_shutdown_cancels_and_joins_in_flight_scheduled_backup(void) {
    if (is_background_abort_requested()) {
        printf("Test-order bug: the process-wide abort flag is already set\n");
        return -1;
    }
    if (run_shutdown_with_in_flight_cycle(false) != 0) return -1;

    // The database must be usable again afterwards, including its scheduler:
    // the cancel raised during shutdown must not leak into the next cycle.
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    if (create_existing_database_fixture(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    if (init_database(TEST_SHUTDOWN_DB_PATH) != 0) return -1;
    g_config.db_backup_interval_minutes = 60;
    int rc = maybe_run_scheduled_database_backup();
    bool finished = db_scheduled_backup_wait_idle(30000);
    int count = count_timestamped_backups(TEST_SHUTDOWN_DB_PATH);
    shutdown_database();
    remove_test_db_and_backups(TEST_SHUTDOWN_DB_PATH);
    if (rc != 1 || !finished || count != 1) {
        printf("Scheduler did not recover after a cancelled cycle (rc=%d, finished=%d, backups=%d)\n",
               rc, finished, count);
        return -1;
    }
    return 0;
}

// Process restart/shutdown (main.c): the abort flag is raised the moment the
// restart or shutdown is requested; shutdown_database() must then join the
// aborting worker within its bounded wait before taking the final backup and
// closing the handle. Raises the never-reset process-wide flag, so it runs
// last among the scheduler tests.
static int test_shutdown_aborts_and_joins_in_flight_scheduled_backup(void) {
    return run_shutdown_with_in_flight_cycle(true);
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

static int test_backup_snapshot_stays_pinned_across_writer_commit(void) {
    sqlite3 *seed = NULL, *source = NULL, *writer = NULL, *dest = NULL;
    sqlite3_backup *backup = NULL;
    int result = -1;
    remove_database_files(TEST_SNAPSHOT_DB_PATH);
    remove_database_files(TEST_SNAPSHOT_COPY_PATH);

    if (sqlite3_open(TEST_SNAPSHOT_DB_PATH, &seed) != SQLITE_OK ||
        sqlite3_exec(seed,
            "PRAGMA journal_mode=WAL;"
            "CREATE TABLE payload(data BLOB);"
            "WITH RECURSIVE n(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM n WHERE x<100) "
            "INSERT INTO payload SELECT zeroblob(3000) FROM n;",
            NULL, NULL, NULL) != SQLITE_OK) goto cleanup;
    sqlite3_close(seed);
    seed = NULL;

    if (sqlite3_open_v2(TEST_SNAPSHOT_DB_PATH, &source,
                        SQLITE_OPEN_READONLY | SQLITE_OPEN_PRIVATECACHE,
                        NULL) != SQLITE_OK ||
        sqlite3_open(TEST_SNAPSHOT_DB_PATH, &writer) != SQLITE_OK ||
        sqlite3_open(TEST_SNAPSHOT_COPY_PATH, &dest) != SQLITE_OK ||
        db_backup_pin_source_snapshot(source) != SQLITE_OK) goto cleanup;

    backup = sqlite3_backup_init(dest, "main", source, "main");
    if (!backup || sqlite3_backup_step(backup, 1) != SQLITE_OK) goto cleanup;
    int remaining_before = sqlite3_backup_remaining(backup);
    int pages_before = sqlite3_backup_pagecount(backup);
    if (remaining_before < 2 ||
        sqlite3_exec(writer, "INSERT INTO payload VALUES(zeroblob(3000));",
                     NULL, NULL, NULL) != SQLITE_OK ||
        sqlite3_backup_step(backup, 1) != SQLITE_OK ||
        sqlite3_backup_pagecount(backup) != pages_before ||
        sqlite3_backup_remaining(backup) != remaining_before - 1) {
        printf("Concurrent write restarted incremental backup despite pinned snapshot\n");
        goto cleanup;
    }
    result = 0;

cleanup:
    if (backup) sqlite3_backup_finish(backup);
    if (source) sqlite3_exec(source, "ROLLBACK;", NULL, NULL, NULL);
    if (dest) sqlite3_close(dest);
    if (writer) sqlite3_close(writer);
    if (source) sqlite3_close(source);
    if (seed) sqlite3_close(seed);
    remove_database_files(TEST_SNAPSHOT_COPY_PATH);
    remove_database_files(TEST_SNAPSHOT_DB_PATH);
    return result;
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

    if (test_backup_snapshot_stays_pinned_across_writer_commit() != 0) {
        printf("Test failed: incremental backup snapshot was not pinned\n");
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

    if (test_backup_verify_modes() != 0) {
        printf("Test failed: backup_verify quick/off/full did not all yield an intact backup\n");
        return 1;
    }

    // Scheduler / worker tests run before any test raises the never-reset
    // process-wide abort flag: the scheduler refuses to start a cycle once
    // it is set (a restart/shutdown is pending), and the last of these
    // raises it itself.
    if (test_failed_scheduled_backup_waits_without_suppressing_shutdown_backup() != 0) {
        printf("Test failed: failed scheduled backup cooldown\n");
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

    if (test_scheduler_does_not_start_second_cycle_while_in_flight() != 0) {
        printf("Test failed: scheduler overlapped an in-flight backup cycle\n");
        return 1;
    }

    if (test_shutdown_cancels_and_joins_in_flight_scheduled_backup() != 0) {
        printf("Test failed: shutdown did not cancel and join an in-flight scheduled backup (database restart)\n");
        return 1;
    }

    if (test_shutdown_aborts_and_joins_in_flight_scheduled_backup() != 0) {
        printf("Test failed: shutdown did not abort and join an in-flight scheduled backup (process shutdown)\n");
        return 1;
    }

    // Restore the interval this file's own load_default_config() set, in
    // case any later step in this binary implicitly depends on it.
    load_default_config(&g_config);

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

    if (test_backup_aborts_early_when_duration_exceeded() != 0) {
        printf("Test failed: Backup did not abort early on an exhausted duration budget\n");
        return 1;
    }

    if (test_backup_aborts_during_verification_when_duration_exceeded() != 0) {
        printf("Test failed: Backup did not abort during post-copy verification on an exhausted duration budget\n");
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
