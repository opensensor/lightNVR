/**
 * @file batch_delete_progress.c
 * @brief Progress tracking for batch delete operations
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "web/batch_delete_progress.h"
#include "core/logger.h"

/**
 * @brief Generate a simple UUID v4 without external dependencies
 *
 * This is a lightweight UUID generator that doesn't require libuuid.
 * It generates a random UUID v4 using /dev/urandom or rand() as fallback.
 *
 * @param uuid_str Output buffer for UUID string (must be at least 37 bytes)
 */
static void generate_uuid_v4(char *uuid_str) {
    unsigned char uuid[16];

    // Try to read from /dev/urandom for better randomness
    FILE *f = fopen("/dev/urandom", "rb");
    if (f) {
        if (fread(uuid, 1, 16, f) != 16) {
            // Fallback to rand() if read fails
            for (int i = 0; i < 16; i++) {
                uuid[i] = rand() & 0xFF;
            }
        }
        fclose(f);
    } else {
        // Fallback to rand() if /dev/urandom is not available
        for (int i = 0; i < 16; i++) {
            uuid[i] = rand() & 0xFF;
        }
    }

    // Set version (4) and variant bits according to RFC 4122
    uuid[6] = (uuid[6] & 0x0F) | 0x40;  // Version 4
    uuid[8] = (uuid[8] & 0x3F) | 0x80;  // Variant 10

    // Format as UUID string: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    snprintf(uuid_str, 37,
             "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
             uuid[0], uuid[1], uuid[2], uuid[3],
             uuid[4], uuid[5],
             uuid[6], uuid[7],
             uuid[8], uuid[9],
             uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15]);
}

// Maximum number of concurrent batch delete jobs
#define MAX_BATCH_DELETE_JOBS 10

// How long to keep completed jobs (in seconds)
#define JOB_RETENTION_TIME 300  // 5 minutes

// Global state
static batch_delete_progress_t g_jobs[MAX_BATCH_DELETE_JOBS];
static pthread_mutex_t g_jobs_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool g_initialized = false;

/**
 * @brief Initialize the batch delete progress tracking system
 */
int batch_delete_progress_init(void) {
    pthread_mutex_lock(&g_jobs_mutex);
    
    if (g_initialized) {
        pthread_mutex_unlock(&g_jobs_mutex);
        return 0;
    }
    
    // Initialize all job slots
    memset(g_jobs, 0, sizeof(g_jobs));
    for (int i = 0; i < MAX_BATCH_DELETE_JOBS; i++) {
        g_jobs[i].is_active = false;
    }
    
    g_initialized = true;
    pthread_mutex_unlock(&g_jobs_mutex);
    
    log_info("Batch delete progress tracking initialized");
    return 0;
}

/**
 * @brief Cleanup the batch delete progress tracking system
 */
void batch_delete_progress_cleanup(void) {
    pthread_mutex_lock(&g_jobs_mutex);
    
    if (!g_initialized) {
        pthread_mutex_unlock(&g_jobs_mutex);
        return;
    }
    
    // Clear all jobs
    memset(g_jobs, 0, sizeof(g_jobs));
    g_initialized = false;
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    log_info("Batch delete progress tracking cleaned up");
}

/**
 * @brief Find an available job slot
 * 
 * @return int Index of available slot, or -1 if none available
 */
static int find_available_slot(void) {
    time_t now = time(NULL);
    
    // First, try to find an empty slot
    for (int i = 0; i < MAX_BATCH_DELETE_JOBS; i++) {
        if (!g_jobs[i].is_active) {
            return i;
        }
    }
    
    // If no empty slots, try to find an old completed job to replace
    for (int i = 0; i < MAX_BATCH_DELETE_JOBS; i++) {
        if ((g_jobs[i].status == BATCH_DELETE_STATUS_COMPLETE || 
             g_jobs[i].status == BATCH_DELETE_STATUS_ERROR) &&
            (now - g_jobs[i].updated_at) > JOB_RETENTION_TIME) {
            log_info("Reusing slot %d for new job (old job: %s)", i, g_jobs[i].job_id);
            return i;
        }
    }
    
    return -1;
}

/**
 * @brief Find a job by ID
 * 
 * @param job_id Job identifier
 * @return int Index of job, or -1 if not found
 */
static int find_job_by_id(const char *job_id) {
    if (!job_id) {
        return -1;
    }
    
    for (int i = 0; i < MAX_BATCH_DELETE_JOBS; i++) {
        if (g_jobs[i].is_active && strcmp(g_jobs[i].job_id, job_id) == 0) {
            return i;
        }
    }
    
    return -1;
}

/**
 * @brief Create a new batch delete job
 */
int batch_delete_progress_create_job(int total, char *job_id_out) {
    if (!g_initialized) {
        log_error("Batch delete progress tracking not initialized");
        return -1;
    }
    
    if (!job_id_out) {
        log_error("Invalid job_id_out parameter");
        return -1;
    }
    
    pthread_mutex_lock(&g_jobs_mutex);
    
    // Find an available slot
    int slot = find_available_slot();
    if (slot < 0) {
        pthread_mutex_unlock(&g_jobs_mutex);
        log_error("No available slots for new batch delete job");
        return -1;
    }

    // Generate a UUID for the job
    generate_uuid_v4(g_jobs[slot].job_id);
    
    // Initialize the job
    g_jobs[slot].status = BATCH_DELETE_STATUS_PENDING;
    g_jobs[slot].total = total;
    g_jobs[slot].current = 0;
    g_jobs[slot].succeeded = 0;
    g_jobs[slot].failed = 0;
    snprintf(g_jobs[slot].status_message, sizeof(g_jobs[slot].status_message), 
             "Preparing to delete %d recordings...", total);
    g_jobs[slot].error_message[0] = '\0';
    g_jobs[slot].created_at = time(NULL);
    g_jobs[slot].updated_at = g_jobs[slot].created_at;
    g_jobs[slot].is_active = true;
    
    // Copy job ID to output
    strncpy(job_id_out, g_jobs[slot].job_id, 64);
    job_id_out[63] = '\0';
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    log_info("Created batch delete job: %s (total: %d)", g_jobs[slot].job_id, total);
    return 0;
}

/**
 * @brief Update progress for a batch delete job
 */
int batch_delete_progress_update(const char *job_id, int current, int succeeded, int failed, const char *status_message) {
    if (!g_initialized) {
        log_error("Batch delete progress tracking not initialized");
        return -1;
    }
    
    if (!job_id) {
        log_error("Invalid job_id parameter");
        return -1;
    }
    
    pthread_mutex_lock(&g_jobs_mutex);
    
    int slot = find_job_by_id(job_id);
    if (slot < 0) {
        pthread_mutex_unlock(&g_jobs_mutex);
        log_error("Job not found: %s", job_id);
        return -1;
    }
    
    // Update progress
    g_jobs[slot].status = BATCH_DELETE_STATUS_RUNNING;
    g_jobs[slot].current = current;
    g_jobs[slot].succeeded = succeeded;
    g_jobs[slot].failed = failed;
    
    if (status_message) {
        strncpy(g_jobs[slot].status_message, status_message, sizeof(g_jobs[slot].status_message) - 1);
        g_jobs[slot].status_message[sizeof(g_jobs[slot].status_message) - 1] = '\0';
    }
    
    g_jobs[slot].updated_at = time(NULL);
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    return 0;
}

/**
 * @brief Mark a batch delete job as complete
 */
int batch_delete_progress_complete(const char *job_id, int succeeded, int failed) {
    if (!g_initialized) {
        log_error("Batch delete progress tracking not initialized");
        return -1;
    }
    
    if (!job_id) {
        log_error("Invalid job_id parameter");
        return -1;
    }
    
    pthread_mutex_lock(&g_jobs_mutex);
    
    int slot = find_job_by_id(job_id);
    if (slot < 0) {
        pthread_mutex_unlock(&g_jobs_mutex);
        log_error("Job not found: %s", job_id);
        return -1;
    }
    
    // Mark as complete
    g_jobs[slot].status = BATCH_DELETE_STATUS_COMPLETE;
    g_jobs[slot].current = g_jobs[slot].total;
    g_jobs[slot].succeeded = succeeded;
    g_jobs[slot].failed = failed;
    snprintf(g_jobs[slot].status_message, sizeof(g_jobs[slot].status_message),
             "Batch delete operation complete");
    g_jobs[slot].updated_at = time(NULL);
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    log_info("Batch delete job completed: %s (succeeded: %d, failed: %d)", job_id, succeeded, failed);
    return 0;
}

/**
 * @brief Mark a batch delete job as failed
 */
int batch_delete_progress_error(const char *job_id, const char *error_message) {
    if (!g_initialized) {
        log_error("Batch delete progress tracking not initialized");
        return -1;
    }
    
    if (!job_id) {
        log_error("Invalid job_id parameter");
        return -1;
    }
    
    pthread_mutex_lock(&g_jobs_mutex);
    
    int slot = find_job_by_id(job_id);
    if (slot < 0) {
        pthread_mutex_unlock(&g_jobs_mutex);
        log_error("Job not found: %s", job_id);
        return -1;
    }
    
    // Mark as error
    g_jobs[slot].status = BATCH_DELETE_STATUS_ERROR;
    
    if (error_message) {
        strncpy(g_jobs[slot].error_message, error_message, sizeof(g_jobs[slot].error_message) - 1);
        g_jobs[slot].error_message[sizeof(g_jobs[slot].error_message) - 1] = '\0';
        
        snprintf(g_jobs[slot].status_message, sizeof(g_jobs[slot].status_message),
                 "Error: %s", error_message);
    } else {
        snprintf(g_jobs[slot].status_message, sizeof(g_jobs[slot].status_message),
                 "Error: Unknown error");
    }
    
    g_jobs[slot].updated_at = time(NULL);
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    log_error("Batch delete job failed: %s (%s)", job_id, error_message ? error_message : "Unknown error");
    return 0;
}

/**
 * @brief Get progress information for a batch delete job
 */
int batch_delete_progress_get(const char *job_id, batch_delete_progress_t *progress_out) {
    if (!g_initialized) {
        log_error("Batch delete progress tracking not initialized");
        return -1;
    }
    
    if (!job_id || !progress_out) {
        log_error("Invalid parameters");
        return -1;
    }
    
    pthread_mutex_lock(&g_jobs_mutex);
    
    int slot = find_job_by_id(job_id);
    if (slot < 0) {
        pthread_mutex_unlock(&g_jobs_mutex);
        log_debug("Job not found: %s", job_id);
        return -1;
    }
    
    // Copy progress information
    memcpy(progress_out, &g_jobs[slot], sizeof(batch_delete_progress_t));
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    return 0;
}

/**
 * @brief Delete a batch delete job from tracking
 */
int batch_delete_progress_delete(const char *job_id) {
    if (!g_initialized) {
        log_error("Batch delete progress tracking not initialized");
        return -1;
    }
    
    if (!job_id) {
        log_error("Invalid job_id parameter");
        return -1;
    }
    
    pthread_mutex_lock(&g_jobs_mutex);
    
    int slot = find_job_by_id(job_id);
    if (slot < 0) {
        pthread_mutex_unlock(&g_jobs_mutex);
        log_debug("Job not found: %s", job_id);
        return -1;
    }
    
    // Clear the slot
    memset(&g_jobs[slot], 0, sizeof(batch_delete_progress_t));
    g_jobs[slot].is_active = false;
    
    pthread_mutex_unlock(&g_jobs_mutex);
    
    log_info("Deleted batch delete job: %s", job_id);
    return 0;
}

