#include "video/stream_protocol.h"
#include "core/logger.h"
#include "video/ffmpeg_utils.h"
#include <string.h>
#include <stdio.h>
#include <ctype.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>

/**
 * Check if a URL is a multicast address
 * Multicast IPv4 addresses are in the range 224.0.0.0 to 239.255.255.255
 */
bool is_multicast_url(const char *url) {
    // Validate input
    if (!url || strlen(url) < 7) {  // Minimum length for "udp://1"
        log_warn("Invalid URL for multicast detection: %s", url ? url : "NULL");
        return false;
    }
    
    // Extract IP address from URL with more robust parsing
    const char *ip_start = NULL;
    
    // Skip protocol prefix with safer checks
    if (strncmp(url, "udp://", 6) == 0) {
        ip_start = url + 6;
    } else if (strncmp(url, "rtp://", 6) == 0) {
        ip_start = url + 6;
    } else {
        // Not a UDP or RTP URL
        log_debug("Not a UDP/RTP URL for multicast detection: %s", url);
        return false;
    }
    
    // Skip any authentication info (user:pass@)
    const char *at_sign = strchr(ip_start, '@');
    if (at_sign) {
        ip_start = at_sign + 1;
    }
    
    // Make a copy of the IP part to avoid modifying the original
    char ip_buffer[256];
    strncpy(ip_buffer, ip_start, sizeof(ip_buffer) - 1);
    ip_buffer[sizeof(ip_buffer) - 1] = '\0';
    
    // Remove port and path information
    char *colon = strchr(ip_buffer, ':');
    if (colon) {
        *colon = '\0';
    }
    
    char *slash = strchr(ip_buffer, '/');
    if (slash) {
        *slash = '\0';
    }
    
    // Parse IP address with additional validation
    unsigned int a = 0, b = 0, c = 0, d = 0;
    if (sscanf(ip_buffer, "%u.%u.%u.%u", &a, &b, &c, &d) == 4) {
        // Validate IP address components
        if (a > 255 || b > 255 || c > 255 || d > 255) {
            log_warn("Invalid IP address components in URL: %s", url);
            return false;
        }
        
        // Check if it's in multicast range (224.0.0.0 - 239.255.255.255)
        if (a >= 224 && a <= 239) {
            log_info("Detected multicast address: %u.%u.%u.%u in URL: %s", a, b, c, d, url);
            return true;
        }
    } else {
        log_debug("Could not parse IP address from URL: %s", url);
    }
    
    return false;
}

/**
 * Open input stream with appropriate options based on protocol
 * Enhanced with more robust error handling and synchronization for UDP streams
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol) {
    int ret;
    AVDictionary *input_options = NULL;
    bool is_multicast = false;
    
    // Validate input parameters
    if (!input_ctx || !url || strlen(url) < 5) {
        log_error("Invalid parameters for open_input_stream: ctx=%p, url=%s", 
                 (void*)input_ctx, url ? url : "NULL");
        return AVERROR(EINVAL);
    }
    
    // Make sure we're starting with a NULL context
    if (*input_ctx) {
        log_warn("Input context not NULL, closing existing context before opening new one");
        avformat_close_input(input_ctx);
    }
    
    // Log the stream opening attempt
    log_info("Opening input stream: %s (protocol: %s)", 
            url, protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");
    
    // Set common options for all protocols
    av_dict_set(&input_options, "protocol_whitelist", "file,udp,rtp,rtsp,tcp,https,tls,http", 0);
    av_dict_set(&input_options, "reconnect", "1", 0); // Enable reconnection
    av_dict_set(&input_options, "reconnect_streamed", "1", 0); // Reconnect if streaming
    av_dict_set(&input_options, "reconnect_delay_max", "5", 0); // Max 5 seconds between reconnection attempts
    
    if (protocol == STREAM_PROTOCOL_UDP) {
        // Check if this is a multicast stream with robust error handling
        is_multicast = is_multicast_url(url);
        
        log_info("Using UDP protocol for stream URL: %s (multicast: %s)", 
                url, is_multicast ? "yes" : "no");
        
        // UDP-specific options with improved buffering for smoother playback
        // Increased buffer size to 16MB as recommended for UDP jitter handling
        av_dict_set(&input_options, "buffer_size", "16777216", 0); // 16MB buffer
        // Expanded protocol whitelist to support more UDP variants
        av_dict_set(&input_options, "protocol_whitelist", "file,udp,rtp,rtsp,tcp,https,tls,http", 0);
        av_dict_set(&input_options, "buffer_size", "8388608", 0); // 8MB buffer
        av_dict_set(&input_options, "max_delay", "1000000", 0); // 1000ms max delay

        // Allow port reuse
        av_dict_set(&input_options, "reuse", "1", 0);
        
        // Extended timeout for UDP streams which may have more jitter
        av_dict_set(&input_options, "timeout", "10000000", 0); // 10 second timeout in microseconds
        
        // Increased max delay for UDP streams
        av_dict_set(&input_options, "max_delay", "2000000", 0); // 2000ms max delay
        
        // More tolerant timestamp handling for UDP streams with ultra-low latency flags
        av_dict_set(&input_options, "fflags", "genpts+discardcorrupt+nobuffer+flush_packets", 0);
        
        // Set UDP-specific socket options
        av_dict_set(&input_options, "recv_buffer_size", "16777216", 0); // 16MB socket receive buffer
        
        // UDP-specific packet reordering settings
        av_dict_set(&input_options, "max_interleave_delta", "1000000", 0); // 1 second max interleave
        
        // Multicast-specific settings with enhanced error handling
        if (is_multicast) {
            log_info("Configuring multicast-specific settings for %s", url);
            
            // Set appropriate TTL for multicast
            av_dict_set(&input_options, "ttl", "32", 0);
            
            // Join multicast group
            av_dict_set(&input_options, "multiple_requests", "1", 0);
            
            // Auto-detect the best network interface
            av_dict_set(&input_options, "localaddr", "0.0.0.0", 0);
            
            // Additional multicast settings for better reliability
            av_dict_set(&input_options, "pkt_size", "1316", 0); // Standard UDP packet size for MPEG-TS
            av_dict_set(&input_options, "rw_timeout", "10000000", 0); // 10 second read/write timeout
        }
    } else {
        log_info("Using TCP protocol for stream URL: %s", url);
        // TCP-specific options with improved reliability
        av_dict_set(&input_options, "stimeout", "5000000", 0); // 5 second timeout in microseconds
        av_dict_set(&input_options, "rtsp_transport", "tcp", 0); // Force TCP for RTSP
        av_dict_set(&input_options, "analyzeduration", "2000000", 0); // 2 seconds analyze duration
        av_dict_set(&input_options, "probesize", "1000000", 0); // 1MB probe size
        av_dict_set(&input_options, "reconnect", "1", 0); // Enable reconnection
        av_dict_set(&input_options, "reconnect_streamed", "1", 0); // Reconnect if streaming
        av_dict_set(&input_options, "reconnect_delay_max", "2", 0); // Max 2 seconds between reconnection attempts (reduced from 5s)
    }
    
    // Check if this is an ONVIF stream and apply ONVIF-specific options
    // This allows ONVIF to work with either TCP or UDP protocol
    if (is_onvif_stream(url)) {
        log_info("Applying ONVIF-specific options for stream URL: %s", url);
        
        // ONVIF-specific options for better reliability
        av_dict_set(&input_options, "stimeout", "10000000", 0); // 10 second timeout in microseconds
        av_dict_set(&input_options, "analyzeduration", "3000000", 0); // 3 seconds analyze duration
        av_dict_set(&input_options, "probesize", "2000000", 0); // 2MB probe size
        
        // More tolerant timestamp handling for ONVIF streams
        av_dict_set(&input_options, "fflags", "genpts+discardcorrupt", 0);
        
        // ONVIF streams may need more time to start
        av_dict_set(&input_options, "rw_timeout", "15000000", 0); // 15 second read/write timeout
        
        // For onvif_simple_server compatibility
        // Extract username and password from URL if present
        const char *auth_start = strstr(url, "://");
        if (auth_start && strchr(auth_start + 3, '@')) {
            // URL contains authentication, extract it for RTSP auth
            char username[64] = {0};
            char password[64] = {0};
            
            // Parse username and password from URL
            if (sscanf(auth_start + 3, "%63[^:]:%63[^@]@", username, password) == 2) {
                log_info("Extracted credentials from URL for RTSP authentication");
                
                // Set RTSP authentication options
                av_dict_set(&input_options, "rtsp_transport", "tcp", 0);
                av_dict_set(&input_options, "rtsp_flags", "prefer_tcp", 0);
                av_dict_set(&input_options, "rtsp_user", username, 0);
                av_dict_set(&input_options, "rtsp_pass", password, 0);
            }
        } else {
            // No authentication in URL, set default RTSP options for onvif_simple_server
            log_info("No credentials in URL, using default RTSP options for onvif_simple_server");
            av_dict_set(&input_options, "rtsp_transport", "tcp", 0);
            av_dict_set(&input_options, "rtsp_flags", "prefer_tcp", 0);
        }
    }
    
    // Open input with protocol-specific options
    ret = avformat_open_input(input_ctx, url, NULL, &input_options);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not open input stream");
        av_dict_free(&input_options);
        return ret;
    }
    
    // Free options
    av_dict_free(&input_options);
    
    // Verify that the context was created
    if (!*input_ctx) {
        log_error("Input context is NULL after successful open");
        return AVERROR(EINVAL);
    }

    // Get stream info with enhanced error handling
    log_debug("Getting stream info for %s", url);
    ret = avformat_find_stream_info(*input_ctx, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not find stream info");
        avformat_close_input(input_ctx);
        return ret;
    }
    
    // Log successful stream opening
    if (*input_ctx && (*input_ctx)->nb_streams > 0) {
        log_info("Successfully opened input stream: %s with %d streams", 
                url, (*input_ctx)->nb_streams);
    } else {
        log_warn("Opened input stream but no streams found: %s", url);
    }

    return 0;
}

/**
 * Check if a URL is an ONVIF stream
 */
bool is_onvif_stream(const char *url) {
    if (!url) {
        return false;
    }
    
    // Check if URL contains "onvif" substring
    if (strstr(url, "onvif") != NULL) {
        log_info("Detected ONVIF stream URL: %s", url);
        return true;
    }
    
    return false;
}

/**
 * Find video stream index in the input context
 */
int find_video_stream_index(AVFormatContext *input_ctx) {
    if (!input_ctx) {
        return -1;
    }

    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            return i;
        }
    }

    return -1;
}
