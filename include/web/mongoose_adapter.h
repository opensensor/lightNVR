/**
 * @file mongoose_adapter.h
 * @brief Adapter for Mongoose API to handle different versions
 */

#ifndef MONGOOSE_ADAPTER_H
#define MONGOOSE_ADAPTER_H

#include "../../external/mongoose/mongoose.h"

/**
 * @brief Get a pointer to the data in a mg_str
 * 
 * @param str Mongoose string
 * @return const char* Pointer to the string data
 */
static inline const char* mg_str_get_ptr(const struct mg_str *str) {
    if (!str) return NULL;
    
    // For the specific version of Mongoose we're using
    return str->buf;
}

/**
 * @brief Get the length of a mg_str
 * 
 * @param str Mongoose string
 * @return size_t Length of the string
 */
static inline size_t mg_str_get_len(const struct mg_str *str) {
    if (!str) return 0;
    return str->len;
}

/**
 * @brief Copy data from a mg_str to a buffer
 * 
 * @param str Mongoose string
 * @param buf Buffer to copy to
 * @param buf_size Size of the buffer
 * @return size_t Number of bytes copied
 */
static inline size_t mg_str_copy(const struct mg_str *str, char *buf, size_t buf_size) {
    if (!str || !buf || buf_size == 0) return 0;
    
    size_t to_copy = str->len < buf_size - 1 ? str->len : buf_size - 1;
    const char *ptr = mg_str_get_ptr(str);
    
    if (ptr) {
        memcpy(buf, ptr, to_copy);
        buf[to_copy] = '\0';
    } else {
        buf[0] = '\0';
        to_copy = 0;
    }
    
    return to_copy;
}

/**
 * @brief Check if HTTP message matches a URI
 * 
 * @param hm HTTP message
 * @param uri URI to match
 * @return bool True if the URI matches
 */
static inline bool mg_http_match_uri(const struct mg_http_message *hm, const char *uri) {
    if (!hm || !uri) return false;
    
    size_t uri_len = strlen(uri);
    if (hm->uri.len != uri_len) return false;
    
    return strncmp(mg_str_get_ptr(&hm->uri), uri, uri_len) == 0;
}

#endif /* MONGOOSE_ADAPTER_H */
