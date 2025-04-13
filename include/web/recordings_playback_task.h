#ifndef RECORDINGS_PLAYBACK_TASK_H
#define RECORDINGS_PLAYBACK_TASK_H

#include <stdint.h>
#include "mongoose.h"

/**
 * @brief Structure for playback recording task
 */
typedef struct {
    struct mg_connection *connection;  // Mongoose connection
    uint64_t id;                       // Recording ID
    struct mg_http_message *hm;        // HTTP message (for range requests)
    char *range_header;                // Range header (if present)
} playback_recording_task_t;

/**
 * @brief Create a playback recording task
 *
 * @param c Mongoose connection
 * @param id Recording ID
 * @param hm HTTP message
 * @return playback_recording_task_t* Pointer to the task or NULL on error
 */
playback_recording_task_t *playback_recording_task_create(struct mg_connection *c, uint64_t id, struct mg_http_message *hm);

/**
 * @brief Free a playback recording task
 *
 * @param task Task to free
 * @param free_http_message Whether to free the HTTP message
 */
void playback_recording_task_free(playback_recording_task_t *task, bool free_http_message);

/**
 * @brief Playback recording task function
 *
 * @param arg Task argument (playback_recording_task_t*)
 */
void playback_recording_task_function(void *arg);

/**
 * @brief Direct handler for GET /api/recordings/play/:id
 */
void mg_handle_play_recording(struct mg_connection *c, struct mg_http_message *hm);

#endif // RECORDINGS_PLAYBACK_TASK_H
