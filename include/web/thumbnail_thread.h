/**
 * @file thumbnail_thread.h
 * @brief Detached pthread-based thumbnail generation with async completion
 *
 * This module offloads thumbnail generation to detached pthreads to avoid
 * blocking the libuv thread pool. When generation completes, a uv_async
 * callback is triggered to send the response back to the client.
 */

#ifndef THUMBNAIL_THREAD_H
#define THUMBNAIL_THREAD_H

#include <stdint.h>
#include <uv.h>

/**
 * @brief Opaque handle for a deferred response action
 *
 * This handle is stored in the connection and used to send the response
 * after the thumbnail generation completes on a detached pthread.
 * In practice, this is a pointer to the libuv_connection_t.
 */
typedef void* deferred_action_handle_t;

/**
 * @brief Callback function type for sending deferred responses
 *
 * This function is called from the uv_async callback when a thumbnail
 * generation completes. It should send the appropriate response based
 * on the result.
 *
 * @param handle Deferred action handle (connection pointer)
 * @param output_path Path to the generated thumbnail (or NULL on failure)
 * @param result 0 on success, -1 on failure
 */
typedef void (*deferred_response_callback_t)(deferred_action_handle_t handle,
                                             const char *output_path, int result);

/**
 * @brief Initialize the thumbnail thread subsystem
 *
 * Must be called once during server initialization, before any thumbnail
 * requests are processed.
 *
 * @param loop The libuv event loop (for uv_async callbacks)
 * @return 0 on success, -1 on error
 */
int thumbnail_thread_init(uv_loop_t *loop);

/**
 * @brief Shutdown the thumbnail thread subsystem
 *
 * Waits for all in-flight thumbnail generations to complete, then cleans up
 * resources. Must be called during server shutdown.
 */
void thumbnail_thread_shutdown(void);

/**
 * @brief Submit a thumbnail generation request
 *
 * Spawns a detached pthread to generate the thumbnail. When complete, the
 * uv_async callback will invoke the callback to send the response.
 *
 * @param recording_id Recording ID
 * @param index Thumbnail index (0, 1, or 2)
 * @param input_path Path to the recording file
 * @param output_path Path where the thumbnail should be saved
 * @param seek_seconds Seek time in the video
 * @param deferred_action Handle to the deferred response action
 * @param callback Function to call when generation completes
 * @return 0 on success (request submitted), -1 on error
 */
int thumbnail_thread_submit(uint64_t recording_id, int index,
                            const char *input_path, const char *output_path,
                            double seek_seconds, deferred_action_handle_t deferred_action,
                            deferred_response_callback_t callback);

/**
 * @brief Get the number of active thumbnail generation threads
 *
 * Useful for monitoring and debugging.
 *
 * @return Number of active threads
 */
int thumbnail_thread_get_active_count(void);

#endif /* THUMBNAIL_THREAD_H */

