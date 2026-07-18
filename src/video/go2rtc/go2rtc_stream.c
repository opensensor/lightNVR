/**
 * @file go2rtc_stream.c
 * @brief Implementation of the go2rtc stream integration module
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/time.h>
#include <errno.h>
#include <curl/curl.h>
#include <fcntl.h>
#include <netdb.h>

#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/dns_cleanup.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/url_utils.h"
#include "utils/strings.h"
#include "database/db_streams.h"

// Default API host
#define DEFAULT_API_HOST "localhost"

// Buffer sizes
#define URL_BUFFER_SIZE 2048

extern config_t g_config;

// Stream integration state
static bool g_initialized = false;
static int g_api_port = 0;
static char *g_config_dir = NULL;  // Store config directory for later use

// Cached result for go2rtc_stream_is_ready to avoid excessive HTTP requests
static bool g_ready_cache_valid = false;
static bool g_ready_cache_value = false;
static time_t g_ready_cache_time = 0;
#define READY_CACHE_TTL_SEC 5  // Cache result for 5 seconds

bool go2rtc_stream_init(const char *binary_path, const char *config_dir, int api_port) {
    if (g_initialized) {
        log_warn("go2rtc stream integration already initialized");
        return false;
    }

    if (!config_dir || api_port <= 0) {
        log_error("Invalid parameters for go2rtc_stream_init");
        return false;
    }

    // Store config directory
    g_config_dir = strdup(config_dir);
    if (!g_config_dir) {
        log_error("Failed to allocate memory for config directory");
        return false;
    }

    // Initialize process manager - binary_path can be NULL, in which case
    // go2rtc_process_init will try to find the binary or use an existing service
    if (!go2rtc_process_init(binary_path, config_dir, api_port)) {
        log_error("Failed to initialize go2rtc process manager");
        free(g_config_dir);
        g_config_dir = NULL;
        return false;
    }

    // Initialize API client
    if (!go2rtc_api_init(DEFAULT_API_HOST, api_port)) {
        log_error("Failed to initialize go2rtc API client");
        go2rtc_process_cleanup();
        free(g_config_dir);
        g_config_dir = NULL;
        return false;
    }

    g_api_port = api_port;
    g_initialized = true;

    log_info("go2rtc stream integration initialized with config dir: %s, API port: %d",
             config_dir, api_port);

    return true;
}

/*
 * go2rtc's RTSP source only understands a fixed set of '#' fragment keys
 * (internal/rtsp/rtsp.go: transport, timeout, backchannel, media, pkt_size,
 * log_level, source, mp4, weak_timeout). It parses the fragment with strings.Cut + ParseQuery,
 * so any other token an operator appends to the camera URL — most commonly
 * "#noaudio" — is silently swallowed: it neither errors nor does what the
 * operator intended, and it rides along in the stored source confusing later
 * inspection. We sanitize the fragment of native rtsp:// sources here so only
 * supported keys survive, and we promote the common "#noaudio" intent into an
 * actual audio-producer suppression instead of a no-op. (#429)
 *
 * Only valueless flags and the known keys are considered; non-RTSP sources
 * (ffmpeg:, wyze://, onvif://, http://, ...) are never passed here, so their
 * fragments are left untouched.
 */
static bool go2rtc_rtsp_fragment_key_supported(const char *key, size_t len) {
    static const char *const known[] = {
        "transport", "timeout", "backchannel", "media",
        "pkt_size", "log_level", "source", "mp4", "weak_timeout"
    };
    for (size_t i = 0; i < sizeof(known) / sizeof(known[0]); i++) {
        if (strlen(known[i]) == len && strncmp(known[i], key, len) == 0) {
            return true;
        }
    }
    return false;
}

// Rewrite `url` in place, dropping unsupported fragment tokens. Returns true if a
// "#noaudio" intent token was present so the caller can suppress the audio source.
static bool go2rtc_sanitize_rtsp_fragments(char *url, size_t url_size, const char *stream_id) {
    char *hash = strchr(url, '#');
    if (!hash) {
        return false;
    }

    bool suppress_audio = false;
    char rebuilt[URL_BUFFER_SIZE];
    size_t base_len = (size_t)(hash - url);
    if (base_len >= sizeof(rebuilt)) {
        return false; // pathologically long base; leave the URL untouched
    }
    memcpy(rebuilt, url, base_len);
    rebuilt[base_len] = '\0';
    size_t out = base_len;

    for (const char *p = hash + 1; *p; ) {
        const char *tok = p;
        const char *end = strchr(p, '#');
        size_t tok_len = end ? (size_t)(end - tok) : strlen(tok);

        // Key is the part before '=' (or the whole token for valueless flags).
        const char *eq = memchr(tok, '=', tok_len);
        size_t key_len = eq ? (size_t)(eq - tok) : tok_len;

        if ((key_len == 7 && strncasecmp(tok, "noaudio", 7) == 0) ||
            (key_len == 8 && strncasecmp(tok, "no-audio", 8) == 0)) {
            suppress_audio = true;
            log_info("Stream %s: '#noaudio' in source URL -> suppressing go2rtc audio producer",
                     stream_id);
        } else if (go2rtc_rtsp_fragment_key_supported(tok, key_len)) {
            if (out + tok_len + 1 < url_size && out + tok_len + 1 < sizeof(rebuilt)) {
                rebuilt[out++] = '#';
                memcpy(rebuilt + out, tok, tok_len);
                out += tok_len;
                rebuilt[out] = '\0';
            }
        } else {
            log_warn("Stream %s: dropping unsupported RTSP source fragment '#%.*s' "
                     "(value redacted; go2rtc would ignore it)", stream_id, (int)key_len, tok);
        }

        if (!end) {
            break;
        }
        p = end + 1;
    }

    safe_strcpy(url, rebuilt, url_size, 0);
    return suppress_audio;
}

bool go2rtc_stream_register(const char *stream_id, const char *stream_url,
                           const char *username, const char *password,
                           bool backchannel_enabled, stream_protocol_t protocol,
                           bool record_audio, const char *codec) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }

    if (!stream_id || !stream_url) {
        log_error("Invalid parameters for go2rtc_stream_register");
        return false;
    }

    // Use the raw stream_id for registration. The go2rtc API client
    // (go2rtc_api_add_stream / _multi) URL-escapes the name itself when it
    // builds the ?name= query parameter, exactly like every other API call
    // (unregister, webrtc-url, preload, exists) and the recording consumer.
    // Pre-escaping it here would double-encode the name (e.g. "Front Cam" ->
    // "Front%2520Cam"), registering the stream under a name that none of the
    // later lookups can match. The ffmpeg fallback sources below likewise
    // reference the stream by its raw name; go2rtc_api_add_stream_multi()
    // escapes each source string as a whole before sending it.

    // Ensure go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_info("go2rtc not running, starting service");
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            return false;
        }

        // Wait for service to start with increased retries
        int retries = 10;
        while (retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to be ready (%d retries left)...", retries);
            sleep(1);
            retries--;
        }

        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service failed to start in time");
            return false;
        }
    }

    // Use a static buffer for the modified URL to avoid memory allocation issues
    char modified_url[URL_BUFFER_SIZE];
    safe_strcpy(modified_url, stream_url, URL_BUFFER_SIZE, 0);

    // Inject credentials into URL if provided and not already embedded
    {
        char credentialed_url[URL_BUFFER_SIZE];
        if (url_apply_credentials(modified_url, username, password,
                                  credentialed_url, sizeof(credentialed_url)) == 0) {
            if (strcmp(credentialed_url, modified_url) != 0) {
                safe_strcpy(modified_url, credentialed_url, URL_BUFFER_SIZE, 0);
                log_info("Applied credentials to go2rtc source URL for registration");
            }
        }
    }

    // Build fragment parameters for go2rtc
    // go2rtc uses fragment (#) parameters for stream options like timeout, backchannel, transport.
    // These are RTSP-specific — skip them for non-RTSP schemes (wyze://, onvif://, http://, etc.)
    // so we don't corrupt the source URL for other go2rtc source modules.
    bool is_rtsp = (strncmp(modified_url, "rtsp://", 7) == 0 ||
                    strncmp(modified_url, "rtsps://", 8) == 0);

    // Drop fragment tokens go2rtc can't act on (e.g. a hand-added "#noaudio")
    // before we append our own, and honor a "#noaudio" request by suppressing the
    // audio producer below. Only native rtsp:// sources are sanitized. (#429)
    bool suppress_audio = false;

    if (is_rtsp) {
        suppress_audio = go2rtc_sanitize_rtsp_fragments(modified_url, URL_BUFFER_SIZE, stream_id);

        char fragment_params[256] = {0};
        int offset = 0;

        if (strstr(modified_url, "#transport=") == NULL) {
            if (protocol == STREAM_PROTOCOL_UDP) {
                offset += snprintf(fragment_params + offset, sizeof(fragment_params) - offset, "#transport=udp");
                log_info("Adding UDP transport parameter for stream");
            } else {
                offset += snprintf(fragment_params + offset, sizeof(fragment_params) - offset, "#transport=tcp");
                log_info("Adding TCP transport parameter for stream");
            }
        }

        offset += snprintf(fragment_params + offset, sizeof(fragment_params) - offset, "#timeout=30");

        if (backchannel_enabled) {
            snprintf(fragment_params + offset, sizeof(fragment_params) - offset, "#backchannel=1");
        }

        char new_url[URL_BUFFER_SIZE];
        snprintf(new_url, URL_BUFFER_SIZE, "%s%s", modified_url, fragment_params);
        safe_strcpy(modified_url, new_url, URL_BUFFER_SIZE, 0);
    } else {
        log_info("Non-RTSP source URL for stream %s, skipping RTSP fragment parameters", stream_id);
    }

    log_info("Prepared go2rtc source URL for stream registration of %s: %s", stream_id, modified_url);

    /*
     * Compose the multi-source list for go2rtc.
     *
     * Source 0 is always the primary source URL. We optionally append:
     *
     *   - ffmpeg:<id>#audio=aac#audio=opus   when record_audio is true.
     *     A single ffmpeg process emits two audio tracks:
     *       * AAC  feeds the MP4 recorder and the MPEG-TS HLS consumer.
     *       * OPUS is what WebRTC needs. go2rtc's pure-Go build does NOT
     *         transcode AAC -> OPUS: its WebRTC consumer only repackages
     *         OPUS/PCMU/PCMA (no AAC decode) and codec matching is name-exact
     *         (pkg/core/codec.go Codec.Match). Without an OPUS producer the
     *         browser negotiates an opus track in the SDP that never carries
     *         any audio — the exact "speaker icon but no sound" symptom. (#429)
     *
     *   - ffmpeg:<id>#video=h264[#hardware]   when the source codec is
     *     anything other than "h264" (including unknown/empty). Browsers'
     *     WebRTC stacks don't accept H.265, so go2rtc needs a transcoded
     *     fallback to negotiate with them — this one only spawns its
     *     ffmpeg process when a consumer actually asks for H.264 and the
     *     primary doesn't supply it (H.264 sources leave it idle).
     *     #hardware (VAAPI/NVENC/v4l2m2m) is only appended when
     *     hw_accel_enabled is set; otherwise we force software libx264 so a
     *     broken host encoder can't crash the producer. (#429)
     *     (Neither #video=copy on the audio source nor transcoding from
     *     the AAC feed — each ffmpeg: entry opens its own producer.)
     *
     * codec may be NULL/empty on first registration since the detection
     * thread only populates stream_config_t.codec once it has read a
     * packet. Treat unknown as "might be H.265" — safer to pay the idle
     * cost of an unused ffmpeg entry than to ship a stream that breaks
     * WebRTC silently. When the detection thread later learns the real
     * codec, go2rtc_integration_reregister_stream() re-issues this call
     * with the corrected value.
     */
    const char *sources[3];
    int num_sources = 0;
    sources[num_sources++] = modified_url;

    char ffmpeg_audio_source[URL_BUFFER_SIZE];
    if (record_audio && !suppress_audio) {
        // Emit AAC (for MP4 recording + MPEG-TS HLS) and OPUS (for WebRTC)
        // from one ffmpeg process — go2rtc cannot transcode AAC->OPUS itself,
        // so the OPUS track is what actually makes audio audible in the
        // browser's WebRTC player. (#429)
        snprintf(ffmpeg_audio_source, sizeof(ffmpeg_audio_source),
                 "ffmpeg:%s#audio=aac#audio=opus", stream_id);
        sources[num_sources++] = ffmpeg_audio_source;
    }

    bool is_h264 = (codec && codec[0] != '\0' && strcasecmp(codec, "h264") == 0);
    char ffmpeg_h264_source[URL_BUFFER_SIZE];
    if (!is_h264) {
        // Only ask go2rtc for the #hardware encoder when the operator opted in
        // via hw_accel_enabled. Some hosts (e.g. LXC containers where vainfo
        // probes succeed but VAAPI encoding fails with "Function not
        // implemented") have a broken hardware encoder that crashes the ffmpeg
        // producer, leaving go2rtc with a null consumer and a 404 stream.
        // Falling back to software libx264 keeps the WebRTC fallback working. (#429)
        if (g_config.hw_accel_enabled) {
            snprintf(ffmpeg_h264_source, sizeof(ffmpeg_h264_source),
                     "ffmpeg:%s#video=h264#hardware", stream_id);
        } else {
            snprintf(ffmpeg_h264_source, sizeof(ffmpeg_h264_source),
                     "ffmpeg:%s#video=h264", stream_id);
        }
        sources[num_sources++] = ffmpeg_h264_source;
        log_info("Stream %s codec=%s; adding ffmpeg H.264 fallback source for WebRTC (hw_accel=%s)",
                 stream_id, (codec && codec[0]) ? codec : "unknown",
                 g_config.hw_accel_enabled ? "on" : "off");
    } else {
        log_info("Stream %s codec=h264; no transcoding fallback needed", stream_id);
    }

    bool result;
    if (num_sources > 1) {
        result = go2rtc_api_add_stream_multi(stream_id, sources, num_sources);
        if (!result) {
            log_error("Failed to register stream %s with go2rtc (%d sources); falling back to primary-only",
                      stream_id, num_sources);
            result = go2rtc_api_add_stream(stream_id, modified_url);
        }
    } else {
        result = go2rtc_api_add_stream(stream_id, modified_url);
    }

    if (result) {
        log_info("Successfully registered stream %s with go2rtc (%d source%s)",
                 stream_id, num_sources, num_sources == 1 ? "" : "s");
    } else {
        log_error("Failed to register stream %s with go2rtc", stream_id);
    }

    // Intentionally do NOT preload here.
    //
    // Registration happens during startup for every enabled stream.  Preloading
    // an unreachable camera can block for up to ~20 seconds (video+audio then
    // video-only fallback), which used to delay the entire application startup
    // and postpone the web UI becoming available.
    //
    // Instead, preload on demand from the actual stream-start paths
    // (go2rtc_integration_start_hls / detection keepalive) after the web server
    // is already listening.
    if (result) {
        log_debug("Registered stream %s with go2rtc without preloading; startup paths will preload on demand",
                  stream_id);
    }

    // RTMP restream (publish) — issue #455.
    //
    // The static `publish:` block written into go2rtc.yaml only works for
    // streams present in go2rtc's config at startup (its publish loop runs once,
    // ~1s after boot, and silently skips any stream not yet in its map). Streams
    // registered dynamically via the API — the common case — are never in that
    // config, so their publish would never fire. Re-issue it here via the API
    // now that the stream is registered. Re-registration replaces go2rtc's
    // Stream object (tearing down any prior publisher), so this must run on every
    // (re-)registration, not just the first.
    //
    // publish_url lives on the main stream config only; the "<name>_sub" lookup
    // for sub-streams simply won't match a row, so sub-streams never publish.
    if (result) {
        stream_config_t pub_cfg;
        memset(&pub_cfg, 0, sizeof(pub_cfg));
        if (get_stream_config_by_name(stream_id, &pub_cfg) == 0 &&
            pub_cfg.publish_url[0] != '\0') {
            if (!go2rtc_api_publish_stream(stream_id, pub_cfg.publish_url)) {
                log_warn("Failed to start RTMP restream for %s; will retry on next registration",
                         stream_id);
            }
        }
    }

    return result;
}

bool go2rtc_stream_unregister(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }

    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_stream_unregister");
        return false;
    }

    // Check if go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service not running, cannot unregister stream");
        return false;
    }

    // Unregister stream from go2rtc
    bool result = go2rtc_api_remove_stream(stream_id);

    if (result) {
        log_info("Unregistered stream from go2rtc: %s", stream_id);
    } else {
        log_error("Failed to unregister stream from go2rtc: %s", stream_id);
    }

    return result;
}

bool go2rtc_stream_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }

    if (!stream_id || !buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_stream_get_webrtc_url");
        return false;
    }

    // Check if go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service not running, cannot get WebRTC URL");
        return false;
    }

    // Get WebRTC URL from API client
    return go2rtc_api_get_webrtc_url(stream_id, buffer, buffer_size);
}

bool go2rtc_stream_get_rtsp_url(const char *stream_id, char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }

    if (!stream_id || !buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_stream_get_rtsp_url");
        return false;
    }

    // Check if go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service not running, cannot get RTSP URL");
        return false;
    }

    // Get the RTSP port from the API
    int rtsp_port = 0;
    if (!go2rtc_api_get_server_info(&rtsp_port)) {
        log_warn("Failed to get RTSP port from go2rtc API, falling back to process manager");
        // Fall back to the process manager's stored port
        rtsp_port = go2rtc_process_get_rtsp_port();
    } else {
        log_info("Retrieved RTSP port from go2rtc API: %d", rtsp_port);
    }

    // URL-encode the stream name so that names containing spaces or other
    // special characters produce a valid RTSP URL.  Without encoding, a name
    // like "My Camera" would yield "rtsp://localhost:8554/My Camera" which
    // FFmpeg (and other RTSP clients) reject, resulting in a 404.
    char encoded_id[MAX_STREAM_NAME * 3];
    simple_url_escape(stream_id, encoded_id, MAX_STREAM_NAME * 3);

    // Format the RTSP URL
    snprintf(buffer, buffer_size, "rtsp://localhost:%d/%s", rtsp_port, encoded_id);
    log_info("Generated RTSP URL for stream %s: %s", stream_id, buffer);

    return true;
}

// Callback function for libcurl to discard response data
static size_t discard_response(void *ptr, size_t size, size_t nmemb, void *userdata) {
    // Just return the size of the data to indicate we handled it
    return size * nmemb;
}

/**
 * @brief Check if a TCP port is open and accepting connections
 *
 * @param host Hostname or IP address
 * @param port Port number
 * @param timeout_ms Timeout in milliseconds
 * @return true if port is open, false otherwise
 */
static bool is_port_open(const char *host, int port, int timeout_ms) {
    struct addrinfo hints, *res = NULL, *rp;
    fd_set fdset;
    struct timeval tv;
    bool result = false;

    // Set up hints for getaddrinfo
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_INET;    // IPv4
    hints.ai_socktype = SOCK_STREAM;

    // Convert port to string
    char port_str[16];
    snprintf(port_str, sizeof(port_str), "%d", port);

    // CRITICAL FIX: Use a local variable to track if we need to free addrinfo
    // This ensures we always free the resources even if we return early
    bool need_to_free_addrinfo = false;

    // Resolve hostname
    int status = getaddrinfo(host, port_str, &hints, &res);
    if (status != 0) {
        log_warn("is_port_open: getaddrinfo failed: %s", gai_strerror(status));
        return false;
    }

    // CRITICAL FIX: Set the flag to indicate we need to free addrinfo
    need_to_free_addrinfo = true;

    // Try each address until we successfully connect
    for (rp = res; rp != NULL; rp = rp->ai_next) {
        // Create socket
        int sockfd = socket(rp->ai_family, rp->ai_socktype, rp->ai_protocol);
        if (sockfd < 0) {
            continue; // Try next address
        }

        // Set non-blocking
        int flags = fcntl(sockfd, F_GETFL, 0);
        fcntl(sockfd, F_SETFL, flags | O_NONBLOCK);

        // Try to connect
        int connect_res = connect(sockfd, rp->ai_addr, rp->ai_addrlen);
        if (connect_res < 0) {
            if (errno == EINPROGRESS) {
                // Connection in progress, wait for it
                tv.tv_sec = timeout_ms / 1000;
                tv.tv_usec = (long)(timeout_ms % 1000) * 1000;
                FD_ZERO(&fdset);
                FD_SET(sockfd, &fdset);

                // Wait for connect to complete or timeout
                int select_res = select(sockfd + 1, NULL, &fdset, NULL, &tv);
                if (select_res < 0) {
                    log_warn("is_port_open: select error: %s", strerror(errno));
                    close(sockfd);
                    continue; // Try next address
                } else if (select_res == 0) {
                    // Timeout
                    log_warn("is_port_open: connection timeout");
                    close(sockfd);
                    continue; // Try next address
                }

                // Check if we actually connected
                int so_error;
                socklen_t len = sizeof(so_error);
                getsockopt(sockfd, SOL_SOCKET, SO_ERROR, &so_error, &len);
                if (so_error != 0) {
                    log_warn("is_port_open: connection failed: %s", strerror(so_error));
                    close(sockfd);
                    continue; // Try next address
                }
            } else {
                // Immediate connection failure
                log_warn("is_port_open: connection failed: %s", strerror(errno));
                close(sockfd);
                continue; // Try next address
            }
        }

        // Connection successful
        close(sockfd);
        result = true;
        break; // Exit the loop
    }

    // CRITICAL FIX: Always free the address info to prevent memory leaks
    if (need_to_free_addrinfo && res) {
        freeaddrinfo(res);
        res = NULL; // Set to NULL to prevent double-free
    }

    // CRITICAL FIX: Clean up DNS resolver resources to prevent memory leaks
    // This addresses the 106-byte memory leak shown in Valgrind
    cleanup_dns_resolver();

    return result;
}

bool go2rtc_stream_is_initialized(void) {
    return g_initialized;
}

bool go2rtc_stream_is_ready(void) {
    // CRITICAL FIX: Add safety checks to prevent memory corruption
    if (!g_initialized) {
        log_warn("go2rtc_stream_is_ready: not initialized");
        return false;
    }

    // Return cached result if still fresh (avoids excessive HTTP requests on
    // resource-constrained devices where many callers check readiness in quick
    // succession during startup and stream registration).
    time_t now = time(NULL);
    if (g_ready_cache_valid && (now - g_ready_cache_time) < READY_CACHE_TTL_SEC) {
        return g_ready_cache_value;
    }

    // Check if API port is valid
    if (g_api_port <= 0 || g_api_port > 65535) {
        log_warn("go2rtc_stream_is_ready: invalid API port: %d", g_api_port);
        return false;
    }

    // Check if process is running with safety checks
    bool process_running = false;

    // Safely check if process is running
    process_running = go2rtc_process_is_running();

    if (!process_running) {
        log_warn("go2rtc_stream_is_ready: process not running");
        return false;
    }

    // First check if the port is open with safety checks
    bool port_open = false;

    // Safely check if port is open
    port_open = is_port_open("localhost", g_api_port, 1000);

    if (!port_open) {
        log_warn("go2rtc_stream_is_ready: port %d is not open", g_api_port);
        return false;
    }

    // Use libcurl to check if the API is responsive
    CURL *curl = NULL;
    CURLcode res;

    // Initialize curl with safety checks
    curl = curl_easy_init();
    if (!curl) {
        log_warn("go2rtc_stream_is_ready: failed to initialize curl");
        return false;
    }

    char url[URL_BUFFER_SIZE];
    long http_code = 0;

    // Format the URL for the API endpoint with safety checks
    int url_result = snprintf(url, sizeof(url), "http://localhost:%d" GO2RTC_BASE_PATH "/api/streams", g_api_port);
    if (url_result < 0 || url_result >= (int)sizeof(url)) {
        log_warn("go2rtc_stream_is_ready: failed to format URL");
        curl_easy_cleanup(curl);
        return false;
    }

    // Set curl options with safety checks
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, discard_response);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 2L); // 2 second timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 2L); // 2 second connect timeout
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals
    curl_easy_setopt(curl, CURLOPT_FAILONERROR, 1L); // Fail on HTTP errors

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors with safety checks
    if (res != CURLE_OK) {
        log_warn("go2rtc_stream_is_ready: curl request failed: %s", curl_easy_strerror(res));

        // Try a simpler HTTP request using a socket with safety checks
        int sockfd = -1;
        struct sockaddr_in server_addr;
        char request[256] = {0}; // Initialize to zeros
        char response[1024] = {0}; // Initialize to zeros

        // Create socket with safety checks
        sockfd = socket(AF_INET, SOCK_STREAM, 0);
        if (sockfd < 0) {
            log_warn("go2rtc_stream_is_ready: socket creation failed: %s", strerror(errno));
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free
            return false;
        }

        // Set up server address with safety checks
        memset(&server_addr, 0, sizeof(server_addr));
        server_addr.sin_family = AF_INET;
        server_addr.sin_port = htons((uint16_t)g_api_port);

        // Use inet_pton for safer address conversion
        if (inet_pton(AF_INET, "127.0.0.1", &server_addr.sin_addr) != 1) {
            log_warn("go2rtc_stream_is_ready: invalid IP address");
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free
            return false;
        }

        // Set socket timeout with safety checks
        struct timeval tv;
        tv.tv_sec = 2;
        tv.tv_usec = 0;
        if (setsockopt(sockfd, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv) < 0) {
            log_warn("go2rtc_stream_is_ready: failed to set receive timeout: %s", strerror(errno));
            // Continue anyway
        }
        if (setsockopt(sockfd, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof tv) < 0) {
            log_warn("go2rtc_stream_is_ready: failed to set send timeout: %s", strerror(errno));
            // Continue anyway
        }

        // Connect to server with safety checks
        if (connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
            log_warn("go2rtc_stream_is_ready: socket connect failed: %s", strerror(errno));
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free
            return false;
        }

        // Prepare HTTP request with safety checks
        // Use GO2RTC_BASE_PATH to match the base_path in go2rtc config
        int req_result = snprintf(request, sizeof(request),
                "GET " GO2RTC_BASE_PATH "/api/streams HTTP/1.1\r\n"
                "Host: localhost:%d\r\n"
                "Connection: close\r\n"
                "\r\n", g_api_port);

        if (req_result < 0 || req_result >= (int)sizeof(request)) {
            log_warn("go2rtc_stream_is_ready: failed to format HTTP request");
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free
            return false;
        }

        // Send request with safety checks
        ssize_t sent = send(sockfd, request, strlen(request), 0);
        if (sent < 0) {
            log_warn("go2rtc_stream_is_ready: socket send failed: %s", strerror(errno));
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free
            return false;
        }

        // Receive response with safety checks
        ssize_t bytes = recv(sockfd, response, sizeof(response) - 1, 0);
        if (bytes <= 0) {
            log_warn("go2rtc_stream_is_ready: socket recv failed: %s", strerror(errno));
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free
            return false;
        }

        // Null-terminate response with safety checks
        if (bytes < (int)sizeof(response)) {
            response[bytes] = '\0';
        } else {
            response[sizeof(response) - 1] = '\0';
        }

        // Check if we got a valid HTTP response with safety checks
        if (strstr(response, "HTTP/1.1 200") || strstr(response, "HTTP/1.1 302")) {
            log_debug("go2rtc_stream_is_ready: socket HTTP request succeeded");
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL; // Set to NULL to prevent double-free

            // CRITICAL FIX: Clean up DNS resolver resources to prevent memory leaks
            // This addresses the 106-byte memory leak shown in Valgrind
            cleanup_dns_resolver();

            g_ready_cache_value = true;
            g_ready_cache_time = time(NULL);
            g_ready_cache_valid = true;
            return true;
        }

        // Log a truncated response to avoid buffer overflows in logging
        char truncated_response[64];
        safe_strcpy(truncated_response, response, sizeof(truncated_response), 0);
        log_warn("go2rtc_stream_is_ready: socket HTTP request failed: %s...", truncated_response);

        close(sockfd);
        curl_easy_cleanup(curl);
        curl = NULL; // Set to NULL to prevent double-free

        // CRITICAL FIX: Clean up DNS resolver resources to prevent memory leaks
        // This addresses the 106-byte memory leak shown in Valgrind
        cleanup_dns_resolver();

        g_ready_cache_value = false;
        g_ready_cache_time = time(NULL);
        g_ready_cache_valid = true;
        return false;
    }

    // Get the HTTP response code with safety checks
    CURLcode info_result = curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    if (info_result != CURLE_OK) {
        log_warn("go2rtc_stream_is_ready: failed to get HTTP response code: %s", curl_easy_strerror(info_result));
        curl_easy_cleanup(curl);
        curl = NULL; // Set to NULL to prevent double-free
        return false;
    }

    // Clean up with safety checks
    curl_easy_cleanup(curl);
    curl = NULL; // Set to NULL to prevent double-free

    // CRITICAL FIX: Clean up DNS resolver resources to prevent memory leaks
    // This addresses the 106-byte memory leak shown in Valgrind
    cleanup_dns_resolver();

    // Check if we got a successful HTTP response (200) with safety checks
    if (http_code == 200) {
        log_debug("go2rtc_stream_is_ready: API is responsive (HTTP %ld)", http_code);
        g_ready_cache_value = true;
        g_ready_cache_time = time(NULL);
        g_ready_cache_valid = true;
        return true;
    } else if (http_code > 0) {
        // We got some HTTP response, which means the server is running
        log_warn("go2rtc_stream_is_ready: API returned HTTP %ld", http_code);
        g_ready_cache_value = true;
        g_ready_cache_time = time(NULL);
        g_ready_cache_valid = true;
        return true;
    } else {
        log_warn("go2rtc_stream_is_ready: API is not responsive (HTTP %ld)", http_code);
        g_ready_cache_value = false;
        g_ready_cache_time = time(NULL);
        g_ready_cache_valid = true;
        return false;
    }
}

int go2rtc_stream_get_api_port(void) {
    return g_api_port;
}

bool go2rtc_stream_start_service(void) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }

    // Check if go2rtc is already running
    if (go2rtc_process_is_running()) {
        log_info("go2rtc service already running, checking if it's responsive");

        // Check if the service is already responsive
        if (go2rtc_stream_is_ready()) {
            log_info("Existing go2rtc service is responsive and ready to use");

            // Register all existing streams with go2rtc if integration module is initialized
            if (go2rtc_integration_is_initialized()) {
                log_info("Registering all existing streams with go2rtc");
                if (!go2rtc_integration_register_all_streams()) {
                    log_warn("Failed to register all streams with go2rtc");
                    // Continue anyway
                }
            } else {
                log_info("go2rtc integration module not initialized, skipping stream registration");
            }

            return true;
        }

        // If not immediately responsive, wait a bit and try again
        log_warn("Existing go2rtc service is not responding, will try to wait for it");

        // Wait for the service to be ready
        int retries = 10; // Increased retries to give more time for the service to become responsive
        while (retries > 0) {
            sleep(1);
            if (go2rtc_stream_is_ready()) {
                log_info("go2rtc service is now ready");

                // Register all existing streams with go2rtc if integration module is initialized
                if (go2rtc_integration_is_initialized()) {
                    log_info("Registering all existing streams with go2rtc");
                    if (!go2rtc_integration_register_all_streams()) {
                        log_warn("Failed to register all streams with go2rtc");
                        // Continue anyway
                    }
                } else {
                    log_info("go2rtc integration module not initialized, skipping stream registration");
                }

                return true;
            }
            log_info("Waiting for go2rtc service to be ready (%d retries left)...", retries);
            retries--;
        }

        // If still not responsive, log a warning but don't stop it
        log_warn("Existing go2rtc service is not responding to API requests");
        log_warn("Will continue using the existing service, but it may not work properly");
        return false;
    }

    // Start go2rtc process with our configuration
    log_info("Starting go2rtc process with API port %d", g_api_port);
    bool result = go2rtc_process_start(g_api_port);

    if (result) {
        log_info("go2rtc service started successfully");

        // Wait for the service to be ready
        int retries = 10;
        while (retries > 0) {
            sleep(1); // Sleep first to give the process time to start
            if (go2rtc_stream_is_ready()) {
                log_info("go2rtc service is ready");

                // Register all existing streams with go2rtc if integration module is initialized
                if (go2rtc_integration_is_initialized()) {
                    log_info("Registering all existing streams with go2rtc");
                    if (!go2rtc_integration_register_all_streams()) {
                        log_warn("Failed to register all streams with go2rtc");
                        // Continue anyway
                    }
                } else {
                    log_info("go2rtc integration module not initialized, skipping stream registration");
                }

                return true;
            }
            log_info("Waiting for go2rtc service to be ready (%d retries left)...", retries);
            retries--;
        }

        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service started but is not responding to API requests");

            // Check if the process is still running
            if (go2rtc_process_is_running()) {
                log_warn("go2rtc process is running but not responding, checking port");

                // Check if the port is in use via /proc/net/tcp (no shell needed)
                {
                    bool port_in_use = false;
                    char hex_port[8];
                    snprintf(hex_port, sizeof(hex_port), ":%04X", g_api_port);
                    /* Check both IPv4 and IPv6 TCP tables */
                    const char *tcp_files[] = {"/proc/net/tcp", "/proc/net/tcp6", NULL};
                    for (int ti = 0; tcp_files[ti] && !port_in_use; ti++) {
                        FILE *tcp_fp = fopen(tcp_files[ti], "r");
                        if (!tcp_fp) continue;
                        char tcp_line[256];
                        fgets(tcp_line, sizeof(tcp_line), tcp_fp); /* skip header */
                        while (fgets(tcp_line, sizeof(tcp_line), tcp_fp)) {
                            if (strstr(tcp_line, hex_port)) {
                                port_in_use = true;
                                log_warn("Port %d is in use (found in %s)", g_api_port, tcp_files[ti]);
                                break;
                            }
                        }
                        fclose(tcp_fp);
                    }
                    if (!port_in_use) {
                        log_error("go2rtc process is running but not listening on port %d", g_api_port);
                    }
                }

                // Try to get the process log
                char log_path[MAX_PATH_LENGTH];

                // Extract directory from g_config.log_file
                if (g_config.log_file[0] != '\0') {
                    char log_dir[MAX_PATH_LENGTH];
                    safe_strcpy(log_dir, g_config.log_file, sizeof(log_dir), 0);

                    // Find the last slash to get the directory
                    char *last_slash = strrchr(log_dir, '/');
                    if (last_slash) {
                        // Truncate at the last slash to get just the directory
                        *(last_slash + 1) = '\0';
                        // Create the go2rtc log path in the same directory as the main log file
                        snprintf(log_path, sizeof(log_path), "%sgo2rtc.log", log_dir);
                    } else if (g_config_dir) {
                        // No directory in the path, fall back to g_config_dir
                        snprintf(log_path, sizeof(log_path), "%s/go2rtc.log", g_config_dir);
                    } else {
                        // No directory in path and no g_config_dir
                        log_warn("No valid log directory found");
                        goto skip_log_check;
                    }
                } else if (g_config_dir) {
                    // If g_config.log_file is empty, fall back to g_config_dir
                    snprintf(log_path, sizeof(log_path), "%s/go2rtc.log", g_config_dir);
                } else {
                    // No valid log path available
                    log_warn("Config directory not available, cannot check go2rtc log");
                    goto skip_log_check;
                }

                log_warn("Checking go2rtc log file: %s", log_path);
                FILE *fp = fopen(log_path, "r");
                if (fp) {
                    char log_line[1024];
                    int lines = 0;

                    // Skip to the end minus 10 lines
                    fseek(fp, 0, SEEK_END);
                    long pos = ftell(fp);

                    // Read the last few lines
                    while (pos > 0 && lines < 10) {
                        pos--;
                        fseek(fp, pos, SEEK_SET);
                        int c = fgetc(fp);
                        if (c == '\n' && pos > 0) {
                            lines++;
                        }
                    }

                    log_warn("Last few lines of go2rtc log:");
                    while (fgets(log_line, sizeof(log_line), fp)) {
                        // Remove newline
                        size_t len = strlen(log_line);
                        if (len > 0 && log_line[len-1] == '\n') {
                            log_line[len-1] = '\0';
                        }
                        log_warn("  %s", log_line);
                    }

                    fclose(fp);
                } else {
                    log_warn("Could not open go2rtc log file: %s", log_path);
                }

                skip_log_check:
            } else {
                log_error("go2rtc process is not running");
            }

            return false;
        }
    } else {
        log_error("Failed to start go2rtc service");
    }

    return result;
}

bool go2rtc_stream_stop_service(void) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }

    // Stop all go2rtc processes, even if we didn't start them
    bool result = go2rtc_process_stop();

    if (result) {
        log_info("All go2rtc processes stopped successfully");
    } else {
        log_warn("Some go2rtc processes may still be running");
    }

    return result;
}

void go2rtc_stream_invalidate_ready_cache(void) {
    g_ready_cache_valid = false;
}

void go2rtc_stream_cleanup(void) {
    if (!g_initialized) {
        return;
    }

    // Clean up API client
    go2rtc_api_cleanup();

    // Clean up process manager
    go2rtc_process_cleanup();

    // Free config directory
    if (g_config_dir) {
        free(g_config_dir);
        g_config_dir = NULL;
    }

    g_initialized = false;
    g_api_port = 0;

    // Invalidate the readiness cache
    g_ready_cache_valid = false;

    log_info("go2rtc stream integration cleaned up");
}
