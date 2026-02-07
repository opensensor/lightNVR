/**
 * @file libuv_connection.h
 * @brief libuv connection management and async file serving
 * 
 * Internal header for connection handling, buffer management,
 * and async file serving operations.
 */

#ifndef LIBUV_CONNECTION_H
#define LIBUV_CONNECTION_H

#ifdef HTTP_BACKEND_LIBUV

#include <uv.h>
#include "web/libuv_server.h"

// Default buffer sizes
#define LIBUV_RECV_BUFFER_INITIAL   4096
#define LIBUV_RECV_BUFFER_MAX       (1024 * 1024)  // 1MB max request size
#define LIBUV_FILE_BUFFER_SIZE      (64 * 1024)    // 64KB file read chunks
#define LIBUV_SEND_BUFFER_SIZE      (64 * 1024)    // 64KB send buffer

/**
 * @brief Context for async file serving
 */
typedef struct file_serve_ctx {
    uv_fs_t open_req;                   // File open request
    uv_fs_t read_req;                   // File read request
    uv_fs_t stat_req;                   // File stat request
    uv_fs_t close_req;                  // File close request
    uv_write_t write_req;               // TCP write request
    
    libuv_connection_t *conn;           // Connection to serve on
    uv_file fd;                         // File descriptor
    char *buffer;                       // Read buffer
    size_t buffer_size;                 // Buffer size
    
    // File info
    size_t file_size;                   // Total file size
    size_t offset;                      // Current read offset
    size_t remaining;                   // Bytes remaining to send
    
    // Range request support
    bool has_range;                     // Range header present
    size_t range_start;                 // Range start (inclusive)
    size_t range_end;                   // Range end (inclusive)
    
    // Response state
    bool headers_sent;                  // Headers already sent
    char content_type[128];             // MIME type
} file_serve_ctx_t;

/**
 * @brief Write request with buffer
 */
typedef struct libuv_write_ctx {
    uv_write_t req;                     // Write request (must be first)
    uv_buf_t buf;                       // Buffer being written
    libuv_connection_t *conn;           // Connection
    bool free_buffer;                   // Whether to free buf.base on completion
} libuv_write_ctx_t;

/**
 * @brief Create a new connection
 * 
 * Allocates and initializes a libuv_connection_t, sets up the TCP handle,
 * and configures llhttp parser callbacks.
 * 
 * @param server Server that owns this connection
 * @return libuv_connection_t* New connection or NULL on error
 */
libuv_connection_t *libuv_connection_create(libuv_server_t *server);

/**
 * @brief Reset connection for keep-alive
 * 
 * Resets request/response state while keeping the TCP connection open.
 * 
 * @param conn Connection to reset
 */
void libuv_connection_reset(libuv_connection_t *conn);

/**
 * @brief Destroy a connection
 * 
 * Frees all resources associated with the connection.
 * The TCP handle should already be closed.
 * 
 * @param conn Connection to destroy
 */
void libuv_connection_destroy(libuv_connection_t *conn);

/**
 * @brief Allocate buffer callback for uv_read_start
 */
void libuv_alloc_cb(uv_handle_t *handle, size_t suggested_size, uv_buf_t *buf);

/**
 * @brief Read callback for incoming data
 */
void libuv_read_cb(uv_stream_t *stream, ssize_t nread, const uv_buf_t *buf);

/**
 * @brief Handle close callback
 */
void libuv_close_cb(uv_handle_t *handle);

/**
 * @brief Write complete callback
 */
void libuv_write_cb(uv_write_t *req, int status);

/**
 * @brief Send raw data on connection
 * 
 * @param conn Connection
 * @param data Data to send
 * @param len Length of data
 * @param free_after_send Whether to free data after sending
 * @return int 0 on success, -1 on error
 */
int libuv_connection_send(libuv_connection_t *conn, char *data, size_t len, 
                          bool free_after_send);

/**
 * @brief Parse Range header for byte range requests
 * 
 * @param range_header Range header value (e.g., "bytes=0-1023")
 * @param file_size Total file size
 * @param start Output: range start
 * @param end Output: range end
 * @return bool true if valid range parsed
 */
bool libuv_parse_range_header(const char *range_header, size_t file_size,
                               size_t *start, size_t *end);

/**
 * @brief Get MIME type for file extension
 * 
 * @param path File path
 * @return const char* MIME type (static string, do not free)
 */
const char *libuv_get_mime_type(const char *path);

#endif /* HTTP_BACKEND_LIBUV */

#endif /* LIBUV_CONNECTION_H */

