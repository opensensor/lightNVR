#include "video/stream_protocol.h"
#include "core/logger.h"
#include "video/ffmpeg_utils.h"
#include "video/ffmpeg_leak_detector.h"
#include <string.h>
#include <stdio.h>
#include <ctype.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <signal.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <unistd.h>
#include <sys/time.h>

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
 * Check if an RTSP stream exists by sending a simple HTTP request
 * This is a lightweight check to avoid FFmpeg crashes when trying to connect to non-existent streams
 *
 * @param url The RTSP URL to check
 * @return true if the stream exists, false otherwise
 */
static bool check_rtsp_stream_exists(const char *url) {
    if (!url || strncmp(url, "rtsp://", 7) != 0) {
        return true; // Not an RTSP URL, assume it exists
    }

    // Extract the host and port from the URL
    char host[256] = {0};
    int port = 554; // Default RTSP port

    // Skip the rtsp:// prefix
    const char *host_start = url + 7;

    // Skip any authentication info (user:pass@)
    const char *at_sign = strchr(host_start, '@');
    if (at_sign) {
        host_start = at_sign + 1;
    }

    // Find the end of the host part
    const char *host_end = strchr(host_start, ':');
    if (!host_end) {
        host_end = strchr(host_start, '/');
        if (!host_end) {
            host_end = host_start + strlen(host_start);
        }
    }

    // Copy the host part
    size_t host_len = host_end - host_start;
    if (host_len >= sizeof(host)) {
        host_len = sizeof(host) - 1;
    }
    memcpy(host, host_start, host_len);
    host[host_len] = '\0';

    // Extract the port if specified
    if (*host_end == ':') {
        port = atoi(host_end + 1);
        if (port <= 0) {
            port = 554; // Default RTSP port
        }
    }

    // Create a socket
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket for RTSP check");
        return true; // Assume the stream exists if we can't check
    }

    // Set a short timeout for the connection
    struct timeval tv;
    tv.tv_sec = 1;
    tv.tv_usec = 0;
    setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv);
    setsockopt(sock, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof tv);

    // Connect to the server
    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);

    // Convert hostname to IP address
    struct hostent *he = gethostbyname(host);
    if (!he) {
        log_error("Failed to resolve hostname: %s", host);
        close(sock);
        return true; // Assume the stream exists if we can't resolve the hostname
    }

    memcpy(&server_addr.sin_addr, he->h_addr_list[0], he->h_length);

    // Connect to the server
    if (connect(sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        log_error("Failed to connect to RTSP server: %s:%d", host, port);
        close(sock);
        return false; // Stream doesn't exist if we can't connect to the server
    }

    // Extract the path part of the URL
    const char *path = strchr(host_start, '/');
    if (!path) {
        path = "/";
    }

    // Send a simple RTSP OPTIONS request
    char request[1024];
    snprintf(request, sizeof(request),
             "OPTIONS %s RTSP/1.0\r\n"
             "CSeq: 1\r\n"
             "User-Agent: LightNVR\r\n"
             "\r\n",
             path);

    if (send(sock, request, strlen(request), 0) < 0) {
        log_error("Failed to send RTSP OPTIONS request");
        close(sock);
        return false; // Stream doesn't exist if we can't send the request
    }

    // Receive the response
    char response[1024] = {0};
    int bytes_received = recv(sock, response, sizeof(response) - 1, 0);
    close(sock);

    if (bytes_received <= 0) {
        log_error("Failed to receive RTSP OPTIONS response");
        return false; // Stream doesn't exist if we don't get a response
    }

    // Check if the response contains "404 Not Found"
    if (strstr(response, "404 Not Found") != NULL) {
        log_error("RTSP stream not found (404): %s", url);
        return false; // Stream doesn't exist
    }

    // Stream exists
    return true;
}

/**
 * Open input stream with appropriate options based on protocol
 * Enhanced with more robust error handling and synchronization for UDP streams
 */
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol) {
    int ret;
    AVDictionary *input_options = NULL;
    bool is_multicast = false;

    // CRITICAL FIX: Check for shutdown before opening a new input stream
    // This prevents opening new connections during shutdown which can cause memory leaks
    extern bool is_shutdown_initiated(void);
    if (is_shutdown_initiated()) {
        log_info("Skipping input stream open for %s during shutdown", url);
        return AVERROR(EINTR); // Interrupted system call
    }

    // Validate input parameters
    if (!input_ctx) {
        log_error("Invalid input_ctx parameter for open_input_stream: ctx=NULL");
        return AVERROR(EINVAL);
    }

    // CRITICAL FIX: Add more robust URL validation to prevent use-after-free
    if (!url) {
        log_error("Invalid URL parameter for open_input_stream: url=NULL");
        return AVERROR(EINVAL);
    }

    // CRITICAL FIX: Create a local copy of the URL to prevent use-after-free
    char local_url[1024];
    size_t url_len = strlen(url);

    // Validate URL length
    if (url_len < 5 || url_len >= sizeof(local_url)) {
        log_error("Invalid URL length for open_input_stream: %zu", url_len);
        return AVERROR(EINVAL);
    }

    // Copy URL to local buffer
    strncpy(local_url, url, sizeof(local_url) - 1);
    local_url[sizeof(local_url) - 1] = '\0';

    // Use local_url instead of url from this point forward

    // Make sure we're starting with a NULL context
    if (*input_ctx) {
        log_warn("Input context not NULL, closing existing context before opening new one");
        avformat_close_input(input_ctx);
    }

    // Check if the RTSP stream exists before trying to connect
    if (strncmp(local_url, "rtsp://", 7) == 0) {
        if (!check_rtsp_stream_exists(local_url)) {
            log_error("RTSP stream does not exist: %s", local_url);
            return AVERROR(ENOENT); // Return "No such file or directory" error
        }
    }

    // Log the stream opening attempt
    log_info("Opening input stream: %s (protocol: %s)",
            local_url, protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");

    // Set common options for all protocols
    av_dict_set(&input_options, "protocol_whitelist", "file,udp,rtp,rtsp,tcp,https,tls,http", 0);
    av_dict_set(&input_options, "reconnect", "1", 0); // Enable reconnection
    av_dict_set(&input_options, "reconnect_streamed", "1", 0); // Reconnect if streaming
    av_dict_set(&input_options, "reconnect_delay_max", "5", 0); // Max 5 seconds between reconnection attempts

    // MEMORY LEAK FIX: Disable parsers which are causing memory leaks
    av_dict_set(&input_options, "use_parser", "0", 0);
    av_dict_set(&input_options, "enable_parser", "0", 0);

    // MEMORY LEAK FIX: Limit the number of streams to reduce memory usage
    av_dict_set(&input_options, "max_streams", "4", 0);

    // MEMORY LEAK FIX: Reduce the amount of data analyzed to prevent memory leaks
    av_dict_set(&input_options, "analyzeduration", "2000000", 0); // 2 seconds
    av_dict_set(&input_options, "probesize", "2000000", 0); // 2MB

    if (protocol == STREAM_PROTOCOL_UDP) {
        // Check if this is a multicast stream with robust error handling
        is_multicast = is_multicast_url(local_url);

        log_info("Using UDP protocol for stream URL: %s (multicast: %s)",
                local_url, is_multicast ? "yes" : "no");

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
            log_info("Configuring multicast-specific settings for %s", local_url);

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
        log_info("Using TCP protocol for stream URL: %s", local_url);
        // TCP-specific options with improved reliability
        av_dict_set(&input_options, "stimeout", "5000000", 0); // 5 second timeout in microseconds
        av_dict_set(&input_options, "rtsp_transport", "tcp", 0); // Force TCP for RTSP
        av_dict_set(&input_options, "analyzeduration", "2000000", 0); // 2 seconds analyze duration
        av_dict_set(&input_options, "probesize", "1000000", 0); // 1MB probe size
        av_dict_set(&input_options, "reconnect", "1", 0); // Enable reconnection
        av_dict_set(&input_options, "reconnect_streamed", "1", 0); // Reconnect if streaming
        av_dict_set(&input_options, "reconnect_delay_max", "2", 0); // Max 2 seconds between reconnection attempts (reduced from 5s)

        // Add more tolerant timestamp handling for TCP streams as well
        av_dict_set(&input_options, "fflags", "genpts+discardcorrupt", 0);
    }

    // Check if this is an ONVIF stream and apply ONVIF-specific options
    // This allows ONVIF to work with either TCP or UDP protocol
    if (is_onvif_stream(local_url)) {
        log_info("Applying ONVIF-specific options for stream URL: %s", local_url);

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
        const char *auth_start = strstr(local_url, "://");
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

        // Try multiple authentication methods for onvif_simple_server
        // Some ONVIF implementations require specific auth methods
        log_info("Setting multiple auth options for ONVIF compatibility");
        av_dict_set(&input_options, "rtsp_transport", "tcp", 0);

        // Disable authentication requirement - some servers don't need it
        av_dict_set(&input_options, "rtsp_flags", "prefer_tcp", 0);

        // Increase timeout for RTSP connections
        av_dict_set(&input_options, "stimeout", "15000000", 0); // 15 seconds

        // Add detailed logging for RTSP
        av_dict_set(&input_options, "loglevel", "debug", 0);
    }

    // Open input with protocol-specific options and better error handling
    // Use a local variable to avoid modifying the input_ctx in case of error
    AVFormatContext *local_ctx = NULL;

    // Add extra safety options to prevent crashes
    av_dict_set(&input_options, "rtsp_flags", "prefer_tcp", 0);
    av_dict_set(&input_options, "allowed_media_types", "video+audio", 0);
    av_dict_set(&input_options, "max_analyze_duration", "5000000", 0); // 5 seconds
    av_dict_set(&input_options, "rw_timeout", "5000000", 0); // 5 seconds

    // CRITICAL FIX: Increase analyzeduration and probesize to handle streams with unspecified dimensions
    // This addresses the "Could not find codec parameters for stream 0" error
    av_dict_set(&input_options, "analyzeduration", "10000000", 0); // 10 seconds (increased from default)
    av_dict_set(&input_options, "probesize", "10000000", 0); // 10MB (increased from default 5MB)

    // CRITICAL FIX: Ensure local_ctx is NULL before calling avformat_open_input
    // This prevents potential double-free issues if avformat_open_input fails
    local_ctx = NULL;

    // Open the input stream
    ret = avformat_open_input(&local_ctx, local_url, NULL, &input_options);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);

        // Log the error with appropriate context
        log_error("Could not open input stream: %s (error code: %d, message: %s)",
                 local_url, ret, error_buf);

        // Log additional context for RTSP errors
        if (strstr(local_url, "rtsp://") != NULL) {
            log_error("RTSP connection failed - server may be down or URL may be incorrect: %s", local_url);

            // Log specific error for 404 Not Found
            if (strstr(error_buf, "404") != NULL || strstr(error_buf, "Not Found") != NULL) {
                log_error("Failed to connect to stream %s: %s (error code: %d)",
                         local_url, error_buf, ret);
            }
        }

        // Free options before returning
        av_dict_free(&input_options);

        // CRITICAL FIX: Use comprehensive cleanup to prevent segmentation faults
        // This is important because avformat_open_input might have allocated memory
        // even if it returned an error
        if (local_ctx) {
            // Use our comprehensive cleanup function to ensure all resources are freed
            comprehensive_ffmpeg_cleanup(&local_ctx, NULL, NULL, NULL);
            // The function already sets the pointer to NULL
        }

        // CRITICAL FIX: Always ensure the output parameter is set to NULL to prevent use-after-free
        // This is essential for preventing segmentation faults during shutdown
        if (input_ctx) {
            *input_ctx = NULL;
            log_debug("Set input_ctx to NULL after failed connection to prevent segmentation fault");
        }

        return ret;
    }

    // If we got here, the open was successful, so assign the local context to the output parameter
    *input_ctx = local_ctx;

    // Track the AVFormatContext to detect memory leaks
    TRACK_AVFORMAT_CTX(local_ctx);

    // Free options
    av_dict_free(&input_options);

    // Verify that the context was created with additional safety checks
    if (!*input_ctx) {
        log_error("Input context is NULL after successful open for URL: %s", local_url);
        return AVERROR(EINVAL);
    }

    // Get stream info with enhanced error handling
    log_debug("Getting stream info for %s", local_url);

    // MEMORY LEAK FIX: Create a new dictionary for find_stream_info options
    AVDictionary *find_stream_options = NULL;

    // Set options to prevent memory leaks during stream info discovery
    av_dict_set(&find_stream_options, "enable_parser", "0", 0); // Disable parser to prevent leaks
    av_dict_set(&find_stream_options, "use_parser", "0", 0); // Disable parser to prevent leaks
    av_dict_set(&find_stream_options, "analyzeduration", "1000000", 0); // 1 second (reduced from 5)
    av_dict_set(&find_stream_options, "probesize", "1000000", 0); // 1MB (reduced from 5)

    // Create an array of option dictionaries, one for each stream
    AVDictionary **options = NULL;
    if ((*input_ctx)->nb_streams > 0) {
        options = av_calloc((*input_ctx)->nb_streams, sizeof(*options));
        if (options) {
            // Set the same options for each stream
            for (unsigned int i = 0; i < (*input_ctx)->nb_streams; i++) {
                av_dict_copy(&options[i], find_stream_options, 0);
            }
        }
    }

    // CRITICAL FIX: Create a local copy of the context pointer to prevent race conditions
    AVFormatContext *local_input_ctx = *input_ctx;

    // CRITICAL FIX: Add memory barrier to ensure the context is fully initialized
    __sync_synchronize();

    // Call find_stream_info with the options
    ret = avformat_find_stream_info(local_input_ctx, options);

    // Free the options
    if (options) {
        for (unsigned int i = 0; i < local_input_ctx->nb_streams; i++) {
            av_dict_free(&options[i]);
        }
        av_free(options);
    }
    av_dict_free(&find_stream_options);

    // CRITICAL FIX: Verify that the context pointer hasn't changed during find_stream_info
    if (*input_ctx != local_input_ctx) {
        log_warn("Context pointer changed during find_stream_info, using original pointer");
        // Continue with the original pointer
    }

    if (ret < 0) {
        log_ffmpeg_error(ret, "Could not find stream info");

        // CRITICAL FIX: Use comprehensive cleanup instead of just avformat_close_input
        // This ensures all resources are properly freed, preventing memory leaks
        AVFormatContext *ctx_to_cleanup = *input_ctx;
        *input_ctx = NULL; // Clear the pointer first to prevent use-after-free

        // CRITICAL FIX: Add memory barrier to ensure the pointer is cleared before cleanup
        __sync_synchronize();

        // Use our comprehensive cleanup function
        comprehensive_ffmpeg_cleanup(&ctx_to_cleanup, NULL, NULL, NULL);

        // CRITICAL FIX: Verify that the context is actually NULL after cleanup
        if (ctx_to_cleanup) {
            log_warn("Failed to clean up context, forcing NULL");
            ctx_to_cleanup = NULL;
        }

        // Note: We're not forcing garbage collection here to avoid segmentation faults
        // Memory cleanup will be handled by the periodic cleanup in hls_unified_thread_func

        log_debug("Comprehensive cleanup completed after find_stream_info failure");
        return ret;
    }

    // Log successful stream opening with safety checks
    if (*input_ctx && (*input_ctx)->nb_streams > 0) {
        // CRITICAL FIX: Sanitize the URL before logging to prevent displaying non-printable characters
        // This prevents potential issues with corrupted stream names
        char sanitized_url[1024] = {0};
        size_t i;

        // Copy and sanitize the URL
        for (i = 0; i < sizeof(sanitized_url) - 1 && local_url[i] != '\0'; i++) {
            // Check if character is printable (ASCII 32-126 plus tab and newline)
            if ((local_url[i] >= 32 && local_url[i] <= 126) || local_url[i] == '\t' || local_url[i] == '\n') {
                sanitized_url[i] = local_url[i];
            } else {
                sanitized_url[i] = '?'; // Replace non-printable characters
            }
        }
        sanitized_url[i] = '\0'; // Ensure null termination

        log_info("Successfully opened input stream: %s with %d streams",
                sanitized_url, (*input_ctx)->nb_streams);

        // Log information about detected streams with safety checks
        for (unsigned int i = 0; i < (*input_ctx)->nb_streams; i++) {
            // CRITICAL FIX: Add safety checks to prevent segmentation faults
            if (!(*input_ctx)->streams[i]) {
                log_warn("Stream %d is NULL, skipping", i);
                continue;
            }

            AVStream *stream = (*input_ctx)->streams[i];

            // CRITICAL FIX: Check if codecpar is valid
            if (!stream->codecpar) {
                log_warn("Stream %d has NULL codecpar, skipping", i);
                continue;
            }

            if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
                log_info("Stream %d: Video stream detected (codec: %d, width: %d, height: %d)",
                        i, stream->codecpar->codec_id, stream->codecpar->width, stream->codecpar->height);
            } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
                // CRITICAL FIX: Use the correct field for channels based on FFmpeg version
                #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                    log_info("Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)",
                            i, stream->codecpar->codec_id, stream->codecpar->ch_layout.nb_channels, stream->codecpar->sample_rate);
                #else
                    log_info("Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)",
                            i, stream->codecpar->codec_id, stream->codecpar->channels, stream->codecpar->sample_rate);
                #endif
            } else {
                log_info("Stream %d: Other stream type detected (type: %d)",
                        i, stream->codecpar->codec_type);
            }
        }
    } else {
        log_warn("Opened input stream but no streams found: %s", local_url);
    }

    // Note: We're not forcing garbage collection here to avoid segmentation faults
    // Memory cleanup will be handled by the periodic cleanup in hls_unified_thread_func

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

/**
 * Find audio stream index in the input context
 */
int find_audio_stream_index(AVFormatContext *input_ctx) {
    if (!input_ctx) {
        return -1;
    }

    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        if (input_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
            return i;
        }
    }

    return -1;
}
