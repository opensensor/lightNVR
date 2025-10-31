/**
 * @file batch_delete_progress.h
 * @brief Progress tracking for batch delete operations
 */

#ifndef BATCH_DELETE_PROGRESS_H
#define BATCH_DELETE_PROGRESS_H

#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <time.h>

/**
 * @brief Batch delete job status
 */
typedef enum {
    BATCH_DELETE_STATUS_PENDING,    // Job created but not started
    BATCH_DELETE_STATUS_RUNNING,    // Job is currently running
    BATCH_DELETE_STATUS_COMPLETE,   // Job completed successfully
    BATCH_DELETE_STATUS_ERROR       // Job encountered an error
} batch_delete_status_t;

/**
 * @brief Batch delete progress information
 */
typedef struct {
    char job_id[64];                // Unique job identifier (UUID)
    batch_delete_status_t status;   // Current status
    int total;                      // Total number of recordings to delete
    int current;                    // Number of recordings processed so far
    int succeeded;                  // Number of successfully deleted recordings
    int failed;                     // Number of failed deletions
    char status_message[256];       // Current status message
    char error_message[256];        // Error message if status is ERROR
    time_t created_at;              // When the job was created
    time_t updated_at;              // When the job was last updated
    bool is_active;                 // Whether this slot is in use
} batch_delete_progress_t;

/**
 * @brief Initialize the batch delete progress tracking system
 * 
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_init(void);

/**
 * @brief Cleanup the batch delete progress tracking system
 */
void batch_delete_progress_cleanup(void);

/**
 * @brief Create a new batch delete job
 * 
 * @param total Total number of recordings to delete
 * @param job_id_out Buffer to store the generated job ID (must be at least 64 bytes)
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_create_job(int total, char *job_id_out);

/**
 * @brief Update progress for a batch delete job
 * 
 * @param job_id Job identifier
 * @param current Number of recordings processed
 * @param succeeded Number of successfully deleted recordings
 * @param failed Number of failed deletions
 * @param status_message Status message (can be NULL)
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_update(const char *job_id, int current, int succeeded, int failed, const char *status_message);

/**
 * @brief Mark a batch delete job as complete
 * 
 * @param job_id Job identifier
 * @param succeeded Number of successfully deleted recordings
 * @param failed Number of failed deletions
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_complete(const char *job_id, int succeeded, int failed);

/**
 * @brief Mark a batch delete job as failed
 * 
 * @param job_id Job identifier
 * @param error_message Error message
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_error(const char *job_id, const char *error_message);

/**
 * @brief Get progress information for a batch delete job
 * 
 * @param job_id Job identifier
 * @param progress_out Buffer to store progress information
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_get(const char *job_id, batch_delete_progress_t *progress_out);

/**
 * @brief Delete a batch delete job from tracking
 * 
 * @param job_id Job identifier
 * @return int 0 on success, non-zero on error
 */
int batch_delete_progress_delete(const char *job_id);

#endif /* BATCH_DELETE_PROGRESS_H */

