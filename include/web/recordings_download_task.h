#ifndef RECORDINGS_DOWNLOAD_TASK_H
#define RECORDINGS_DOWNLOAD_TASK_H

#include <stdint.h>
#include "mongoose.h"

/**
 * @brief Structure for download recording task
 */
typedef struct {
    struct mg_connection *connection;  // Mongoose connection
    uint64_t id;                       // Recording ID
} download_recording_task_t;

/**
 * @brief Create a download recording task
 * 
 * @param c Mongoose connection
 * @param id Recording ID
 * @return download_recording_task_t* Pointer to the task or NULL on error
 */
download_recording_task_t *download_recording_task_create(struct mg_connection *c, uint64_t id);

/**
 * @brief Free a download recording task
 * 
 * @param task Task to free
 */
void download_recording_task_free(download_recording_task_t *task);

/**
 * @brief Direct handler for GET /api/recordings/download/:id
 * Uses Mongoose's built-in file serving capability to serve the recording file
 */
void mg_handle_download_recording(struct mg_connection *c, struct mg_http_message *hm);

#endif // RECORDINGS_DOWNLOAD_TASK_H
