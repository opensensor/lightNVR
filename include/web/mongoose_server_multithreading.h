/**
 * @file mongoose_server_multithreading.h
 * @brief Multithreading support for Mongoose server
 */

#ifndef MONGOOSE_SERVER_MULTITHREADING_H
#define MONGOOSE_SERVER_MULTITHREADING_H

#include "mongoose.h"

/**
 * @brief Thread data structure for worker threads
 */
struct mg_thread_data {
  struct mg_mgr *mgr;
  unsigned long conn_id;  // Parent connection ID
  struct mg_str message;  // Original HTTP request
  void (*handler_func)(struct mg_connection *c, struct mg_http_message *hm);  // Handler function
};

/**
 * @brief Start a thread
 * 
 * @param f Thread function
 * @param p Thread data
 */
void mg_start_thread(void *(*f)(void *), void *p);

/**
 * @brief Thread function that processes the request
 * 
 * @param param Thread data
 * @return void* NULL
 */
void *mg_thread_function(void *param);

/**
 * @brief Handle HTTP request with multithreading
 * 
 * @param c Mongoose connection
 * @param hm HTTP message
 * @param fast_path If true, handle the request in the main thread
 * @return true if the request was handled, false otherwise
 */
bool mg_handle_request_with_threading(struct mg_connection *c, 
                                     struct mg_http_message *hm,
                                     bool fast_path);

/**
 * @brief Wakeup event handler
 * 
 * @param c Mongoose connection
 * @param ev_data Event data (mg_str *)
 */
void mg_handle_wakeup_event(struct mg_connection *c, void *ev_data);

#endif /* MONGOOSE_SERVER_MULTITHREADING_H */
