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
#include <sys/wait.h>

#include "database/db_core.h"
#include "database/db_backup.h"
#include "core/logger.h"

// Test database path
#define TEST_DB_PATH "/tmp/test_db.sqlite"
#define TEST_BACKUP_PATH "/tmp/test_db.sqlite.bak"

// Signal handler for simulating a crash
static void simulate_crash(int sig) {
    printf("Simulating application crash...\n");
    _exit(1); // Force exit without cleanup
}

// Create a test database with some data
static int create_test_database(void) {
    sqlite3 *db;
    int rc;
    char *err_msg = NULL;
    
    // Remove any existing test database
    unlink(TEST_DB_PATH);
    unlink(TEST_BACKUP_PATH);
    
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
    
    // Open the database file
    file = fopen(TEST_DB_PATH, "r+b");
    if (!file) {
        printf("Failed to open database file for corruption\n");
        return -1;
    }
    
    // Seek to a position in the file
    fseek(file, 100, SEEK_SET);
    
    // Write some random data to corrupt the file
    const char *corrupt_data = "CORRUPTED_DATA";
    fwrite(corrupt_data, 1, strlen(corrupt_data), file);
    
    fclose(file);
    
    printf("Database file corrupted\n");
    return 0;
}

// Test backup functionality
static int test_backup(void) {
    // Create a backup of the database
    int rc = backup_database(TEST_DB_PATH, TEST_BACKUP_PATH);
    if (rc != 0) {
        printf("Failed to create backup\n");
        return -1;
    }
    
    printf("Database backup created successfully\n");
    return 0;
}

// Test restore functionality
static int test_restore(void) {
    // Restore the database from backup
    int rc = restore_database_from_backup(TEST_BACKUP_PATH, TEST_DB_PATH);
    if (rc != 0) {
        printf("Failed to restore database from backup\n");
        return -1;
    }
    
    printf("Database restored successfully from backup\n");
    return 0;
}

// Main test function
int main(void) {
    // Initialize logger
    init_logger();
    
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
