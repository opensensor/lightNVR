#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "core/logger.h"
#include "video/stream_protocol.h"

/**
 * Get the protocol type from a URL
 */
stream_protocol_t get_protocol_from_url(const char *url) {
    if (!url) {
        return STREAM_PROTOCOL_UNKNOWN;
    }
    
    // Check for UDP protocol
    if (strstr(url, "udp://") == url) {
        return STREAM_PROTOCOL_UDP;
    }
    
    // Check for RTP protocol (which uses UDP)
    if (strstr(url, "rtp://") == url) {
        return STREAM_PROTOCOL_UDP;
    }
    
    // Check for file protocol
    if (strstr(url, "file://") == url || url[0] == '/') {
        return STREAM_PROTOCOL_FILE;
    }
    
    // Default to TCP for all other protocols (RTSP, HTTP, etc.)
    return STREAM_PROTOCOL_TCP;
}

/**
 * Get the protocol name as a string
 */
const char *get_protocol_name(stream_protocol_t protocol) {
    switch (protocol) {
        case STREAM_PROTOCOL_TCP:
            return "TCP";
        case STREAM_PROTOCOL_UDP:
            return "UDP";
        case STREAM_PROTOCOL_FILE:
            return "FILE";
        case STREAM_PROTOCOL_UNKNOWN:
        default:
            return "UNKNOWN";
    }
}
