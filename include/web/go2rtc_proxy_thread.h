/**
 * @file go2rtc_proxy_thread.h
 * @brief One-and-done proxy threads for go2rtc reverse proxy
 *
 * Instead of using the shared libuv worker thread pool (which can be
 * exhausted by long-running curl requests), each proxy request spawns
 * a short-lived detached pthread.  The thread performs the blocking
 * curl call, enqueues the result on a lock-free done list, and wakes
 * the event loop via uv_async_send so the response is sent from the
 * loop thread (as libuv requires).
 */

#ifndef GO2RTC_PROXY_THREAD_H
#define GO2RTC_PROXY_THREAD_H

#ifdef HTTP_BACKEND_LIBUV

#include <stdbool.h>
#include <uv.h>
#include "web/libuv_server.h"

/**
 * Initialise the proxy async mechanism.
 * Must be called once from the event-loop thread after the loop is running.
 *
 * @param loop  The libuv event loop
 * @return 0 on success, -1 on error
 */
int go2rtc_proxy_thread_init(uv_loop_t *loop);

/**
 * Submit a proxy request.  A detached thread is spawned to perform the
 * blocking curl call.  When complete the event loop is woken and the
 * response is sent from the loop thread.
 *
 * The caller must have already stopped reading on the connection and
 * paused the HTTP parser.
 *
 * @param conn    Connection (already stopped reading)
 * @param action  Post-response action (keep-alive or close)
 * @return 0 on success, -1 on error
 */
int go2rtc_proxy_thread_submit(libuv_connection_t *conn,
                               write_complete_action_t action);

/**
 * Shut down: close the uv_async handle.
 * Any in-flight threads will still complete but their results are discarded
 * (the connection will already be torn down by libuv shutdown).
 */
void go2rtc_proxy_thread_shutdown(void);

/**
 * Quick prefix check – returns true when @p path should be handled by the
 * proxy rather than the normal handler registry.
 */
bool go2rtc_proxy_path_matches(const char *path);

#endif /* HTTP_BACKEND_LIBUV */
#endif /* GO2RTC_PROXY_THREAD_H */

