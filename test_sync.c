/**
 * Test program for recording sync functionality
 */

#include <stdio.h>
#include <stdlib.h>
#include "core/logger.h"
#include "database/database_manager.h"
#include "database/db_recordings_sync.h"

int main(int argc, char *argv[]) {
    // Initialize logger
    set_log_level(LOG_LEVEL_INFO);
    
    // Initialize database
    if (init_database("/var/lib/lightnvr/lightnvr.db") != 0) {
        fprintf(stderr, "Failed to initialize database\n");
        return 1;
    }
    
    printf("Running recording sync...\n");
    
    // Force a sync
    int result = force_recording_sync();
    
    if (result < 0) {
        fprintf(stderr, "Sync failed\n");
        return 1;
    }
    
    printf("Sync complete: %d recordings updated\n", result);
    
    // Cleanup
    shutdown_database();
    
    return 0;
}

