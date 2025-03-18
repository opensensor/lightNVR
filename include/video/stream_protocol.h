#ifndef STREAM_PROTOCOL_H
#define STREAM_PROTOCOL_H

#include "core/config.h"

// Additional protocol types extending the ones in config.h
#define STREAM_PROTOCOL_FILE 2 // Local file
#define STREAM_PROTOCOL_UNKNOWN 3 // Unknown protocol

/**
 * Get the protocol type from a URL
 * 
 * @param url URL to check
 * @return Protocol type
 */
stream_protocol_t get_protocol_from_url(const char *url);

/**
 * Get the protocol name as a string
 * 
 * @param protocol Protocol type
 * @return Protocol name as a string
 */
const char *get_protocol_name(stream_protocol_t protocol);

#endif /* STREAM_PROTOCOL_H */
