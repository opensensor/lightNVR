/**
 * @file mongoose_server_multithreading.c
 * @brief Multithreading support for Mongoose server
 *
 * This file implements multithreading support for the Mongoose server,
 * allowing it to handle multiple requests in parallel.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>

#include "web/mongoose_server.h"
#include "web/mongoose_server_multithreading.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"

// Thread data structure is defined in the header file

/**
 * @brief Start a thread
 *
 * @param f Thread function
 * @param p Thread data
 */
void mg_start_thread(void *(*f)(void *), void *p) {
#ifdef _WIN32
  _beginthread((void(__cdecl *)(void *)) f, 0, p);
#else
#define closesocket(x) close(x)
#include <pthread.h>
  pthread_t thread_id = (pthread_t) 0;
  pthread_attr_t attr;
  (void) pthread_attr_init(&attr);
  (void) pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
  pthread_create(&thread_id, &attr, f, p);
  pthread_attr_destroy(&attr);
#endif
}

/**
 * @brief Thread function that processes the request
 *
 * @param param Thread data
 * @return void* NULL
 */
void *mg_thread_function(void *param) {
  struct mg_thread_data *p = (struct mg_thread_data *) param;

  // Log thread start
  log_debug("Worker thread started for connection ID %lu", p->conn_id);

  // Create a fake connection for the handler
  struct mg_connection fake_conn = {0};
  fake_conn.mgr = p->mgr;
  fake_conn.id = p->conn_id;

  // Create HTTP message from the stored message
  struct mg_http_message hm = {0};
  if (mg_http_parse((char *)p->message.buf, p->message.len, &hm) > 0) {
    // Extract URI for logging
    char uri[256] = {0};
    if (hm.uri.len > 0) {
      size_t uri_len = hm.uri.len < sizeof(uri) - 1 ? hm.uri.len : sizeof(uri) - 1;
      memcpy(uri, hm.uri.buf, uri_len);
      uri[uri_len] = '\0';
    }

    // Call the handler function if provided
    if (p->handler_func) {
      log_debug("Calling handler function for URI: %s", uri);

      // Set up a buffer to capture the response
      fake_conn.send.buf = NULL;
      fake_conn.send.len = 0;
      fake_conn.send.size = 0;

      // Execute the handler function
      p->handler_func(&fake_conn, &hm);

      // Check if the handler sent a response directly
      if (fake_conn.send.buf && fake_conn.send.len > 0) {
        // Send the response back to the parent connection
        log_debug("Handler sent response of length %zu", fake_conn.send.len);
        mg_wakeup(p->mgr, p->conn_id, fake_conn.send.buf, fake_conn.send.len);

        // Free the send buffer if it was allocated
        free((void *)fake_conn.send.buf);
      } else {
        // No response was sent, send a default response
        log_debug("Handler did not send a response, sending default");
        mg_wakeup(p->mgr, p->conn_id, "Handler completed", 16);
      }
    } else {
      // Try to find a handler for this URI
      log_error("No handler function provided for URI: %s", uri);

      // Special handling for root path
      if (strcmp(uri, "/") == 0) {
        log_info("Root path detected in thread function, sending redirect to static handler");
        mg_wakeup(p->mgr, p->conn_id, "HTTP/1.1 302 Found\r\nLocation: /index.html\r\nContent-Length: 0\r\n\r\n", 65);
      } else {
        mg_wakeup(p->mgr, p->conn_id, "No handler for request", 21);
      }
    }
  } else {
    // Failed to parse HTTP message
    log_error("Failed to parse HTTP message");
    mg_wakeup(p->mgr, p->conn_id, "Failed to parse request", 22);
  }

  // Save connection ID before freeing the structure
  unsigned long conn_id = p->conn_id;

  // Free resources
  free((void *) p->message.buf);
  free(p);

  log_debug("Worker thread completed for connection ID %lu", conn_id);
  return NULL;
}

/**
 * @brief Handle HTTP request with multithreading
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 * @param fast_path If true, handle the request in the main thread
 * @param handler_func Handler function to call
 * @return true if the request was handled, false otherwise
 */
bool mg_handle_request_with_threading(struct mg_connection *c,
                                     struct mg_http_message *hm,
                                     bool fast_path) {
  if (fast_path) {
    // Fast path - handle in the main thread
    // This is useful for simple requests that don't need to be processed in a separate thread
    log_debug("Handling request in main thread: %.*s",
             (int)hm->uri.len, hm->uri.buf);

    // Return false to let the normal request handling continue
    return false;
  } else {
    // Multithreading path - spawn a thread to handle the request
    log_debug("Spawning thread to handle request: %.*s",
             (int)hm->uri.len, hm->uri.buf);

    // Extract URI for logging
    char uri[256];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';

    log_debug("Handling request with threading: %s", uri);

    // Allocate thread data
    struct mg_thread_data *data =
        (struct mg_thread_data *) calloc(1, sizeof(*data));

    if (!data) {
      log_error("Failed to allocate memory for thread data");
      mg_http_reply(c, 500, "", "Internal Server Error\n");
      return true;
    }

    // Copy the HTTP message
    data->message = mg_strdup(hm->message);
    if (data->message.len == 0) {
      log_error("Failed to duplicate HTTP message");
      free(data);
      mg_http_reply(c, 500, "", "Internal Server Error\n");
      return true;
    }

    // Set connection ID, manager, and handler function
    data->conn_id = c->id;
    data->mgr = c->mgr;

    // Start thread
    mg_start_thread(mg_thread_function, data);

    return true;
  }
}

/**
 * @brief Wakeup event handler
 *
 * @param c Mongoose connection
 * @param ev_data Event data (mg_str *)
 */
void mg_handle_wakeup_event(struct mg_connection *c, void *ev_data) {
  // Validate connection pointer
  if (!c) {
    log_error("Wakeup event received with NULL connection");
    return;
  }

  // Check if connection is closing or already closed
  if (c->is_closing) {
    log_debug("Wakeup event received for closing connection ID %lu, ignoring", c->id);
    return;
  }

  struct mg_str *data = (struct mg_str *) ev_data;

  // Validate event data
  if (!data || !data->buf || data->len == 0) {
    log_warn("Wakeup event received with invalid data for connection ID %lu", c->id);
    return;
  }

  // Log the wakeup event
  log_debug("Received wakeup event for connection ID %lu: %.*s",
           c->id, (int)data->len, data->buf);

  // Check if the data is a complete HTTP response
  if (data->len > 12 && strncmp(data->buf, "HTTP/1.", 7) == 0) {
    // This is a complete HTTP response, send it as is
    mg_send(c, data->buf, data->len);
  } else {
    // This is just a message, wrap it in an HTTP response
    mg_http_reply(c, 200, "Content-Type: application/json\r\n", "{\"result\": \"%.*s\"}\n",
                 (int)data->len, data->buf);
  }
}
