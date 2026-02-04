/**
 * ONVIF Motion Detection Recording Implementation
 * 
 * This module implements automated recording triggered by ONVIF motion detection events.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>

#include "video/onvif_motion_recording.h"
#include "video/streams.h"
#include "video/stream_manager.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// Global event queue
static motion_event_queue_t event_queue;

// Recording contexts for all streams
static motion_recording_context_t recording_contexts[MAX_STREAMS];
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;

// Event processing thread
static pthread_t event_processor_thread;
static bool event_processor_running = false;
static bool event_processor_thread_created = false;

// Forward declarations
static void* event_processor_thread_func(void *arg);
static int start_motion_recording_internal(motion_recording_context_t *ctx);
static int stop_motion_recording_internal(motion_recording_context_t *ctx);
static void update_recording_state(motion_recording_context_t *ctx, time_t current_time);
static motion_recording_context_t* get_recording_context(const char *stream_name);
static motion_recording_context_t* create_recording_context(const char *stream_name);
static int flush_packet_callback(const AVPacket *packet, void *user_data);

/**
 * Initialize the event queue
 */
static int init_event_queue(void) {
    memset(&event_queue, 0, sizeof(motion_event_queue_t));
    event_queue.head = 0;
    event_queue.tail = 0;
    event_queue.count = 0;
    
    if (pthread_mutex_init(&event_queue.mutex, NULL) != 0) {
        log_error("Failed to initialize event queue mutex");
        return -1;
    }
    
    if (pthread_cond_init(&event_queue.cond, NULL) != 0) {
        log_error("Failed to initialize event queue condition variable");
        pthread_mutex_destroy(&event_queue.mutex);
        return -1;
    }
    
    log_info("Motion event queue initialized");
    return 0;
}

/**
 * Cleanup the event queue
 */
static void cleanup_event_queue(void) {
    pthread_mutex_lock(&event_queue.mutex);
    event_queue.count = 0;
    event_queue.head = 0;
    event_queue.tail = 0;
    pthread_cond_broadcast(&event_queue.cond);
    pthread_mutex_unlock(&event_queue.mutex);
    
    pthread_mutex_destroy(&event_queue.mutex);
    pthread_cond_destroy(&event_queue.cond);
    
    log_info("Motion event queue cleaned up");
}

/**
 * Push an event to the queue
 */
static int push_event(const motion_event_t *event) {
    if (!event) {
        return -1;
    }
    
    pthread_mutex_lock(&event_queue.mutex);
    
    if (event_queue.count >= MAX_MOTION_EVENT_QUEUE) {
        log_warn("Motion event queue full, dropping oldest event");
        // Remove oldest event
        event_queue.head = (event_queue.head + 1) % MAX_MOTION_EVENT_QUEUE;
        event_queue.count--;
    }
    
    // Add new event
    memcpy(&event_queue.events[event_queue.tail], event, sizeof(motion_event_t));
    event_queue.tail = (event_queue.tail + 1) % MAX_MOTION_EVENT_QUEUE;
    event_queue.count++;
    
    // Signal waiting thread
    pthread_cond_signal(&event_queue.cond);
    
    pthread_mutex_unlock(&event_queue.mutex);
    
    return 0;
}

/**
 * Pop an event from the queue
 */
static int pop_event(motion_event_t *event) {
    if (!event) {
        return -1;
    }
    
    pthread_mutex_lock(&event_queue.mutex);
    
    // Wait for events if queue is empty
    while (event_queue.count == 0 && event_processor_running) {
        pthread_cond_wait(&event_queue.cond, &event_queue.mutex);
    }
    
    if (event_queue.count == 0) {
        pthread_mutex_unlock(&event_queue.mutex);
        return -1;
    }
    
    // Get event
    memcpy(event, &event_queue.events[event_queue.head], sizeof(motion_event_t));
    event_queue.head = (event_queue.head + 1) % MAX_MOTION_EVENT_QUEUE;
    event_queue.count--;
    
    pthread_mutex_unlock(&event_queue.mutex);
    
    return 0;
}

/**
 * Get or create a recording context for a stream
 */
static motion_recording_context_t* get_recording_context(const char *stream_name) {
    if (!stream_name) {
        return NULL;
    }
    
    pthread_mutex_lock(&contexts_mutex);
    
    // Look for existing context
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i].active && 
            strcmp(recording_contexts[i].stream_name, stream_name) == 0) {
            pthread_mutex_unlock(&contexts_mutex);
            return &recording_contexts[i];
        }
    }
    
    pthread_mutex_unlock(&contexts_mutex);
    return NULL;
}

/**
 * Create a new recording context
 */
static motion_recording_context_t* create_recording_context(const char *stream_name) {
    if (!stream_name) {
        return NULL;
    }
    
    pthread_mutex_lock(&contexts_mutex);
    
    // Find free slot
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!recording_contexts[i].active) {
            memset(&recording_contexts[i], 0, sizeof(motion_recording_context_t));
            strncpy(recording_contexts[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            recording_contexts[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            recording_contexts[i].state = RECORDING_STATE_IDLE;
            recording_contexts[i].active = true;
            
            // Initialize mutex
            pthread_mutex_init(&recording_contexts[i].mutex, NULL);

            // Set default configuration
            recording_contexts[i].pre_buffer_seconds = 5;
            recording_contexts[i].post_buffer_seconds = 10;
            recording_contexts[i].max_file_duration = 300;
            recording_contexts[i].enabled = false;
            recording_contexts[i].buffer_enabled = false;
            recording_contexts[i].buffer = NULL;
            recording_contexts[i].buffer_flushed = false;

            pthread_mutex_unlock(&contexts_mutex);
            log_info("Created motion recording context for stream: %s", stream_name);
            return &recording_contexts[i];
        }
    }
    
    pthread_mutex_unlock(&contexts_mutex);
    log_error("No free slots for motion recording context");
    return NULL;
}

/**
 * Generate recording file path with timestamp
 */
static int generate_recording_path(const char *stream_name, char *path, size_t path_size) {
    if (!stream_name || !path) {
        return -1;
    }

    // Get storage path from config
    extern config_t* get_streaming_config(void);
    config_t *config = get_streaming_config();
    if (!config) {
        log_error("Failed to get configuration");
        return -1;
    }
    
    // Create timestamp string
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    char timestamp[32];
    strftime(timestamp, sizeof(timestamp), "%Y%m%d_%H%M%S", tm_info);
    
    // Create directory structure: /recordings/camera_name/YYYY/MM/DD/
    char year[8], month[4], day[4];
    strftime(year, sizeof(year), "%Y", tm_info);
    strftime(month, sizeof(month), "%m", tm_info);
    strftime(day, sizeof(day), "%d", tm_info);
    
    char dir_path[MAX_PATH_LENGTH];
    snprintf(dir_path, sizeof(dir_path), "%s/%s/%s/%s/%s",
             config->storage_path, stream_name, year, month, day);
    
    // Create directories if they don't exist
    char temp_path[MAX_PATH_LENGTH];
    snprintf(temp_path, sizeof(temp_path), "%s/%s", config->storage_path, stream_name);
    mkdir(temp_path, 0755);
    
    snprintf(temp_path, sizeof(temp_path), "%s/%s/%s", config->storage_path, stream_name, year);
    mkdir(temp_path, 0755);
    
    snprintf(temp_path, sizeof(temp_path), "%s/%s/%s/%s", config->storage_path, stream_name, year, month);
    mkdir(temp_path, 0755);
    
    mkdir(dir_path, 0755);
    
    // Generate full file path
    snprintf(path, path_size, "%s/%s_%s_motion.mp4", dir_path, stream_name, timestamp);
    
    log_info("Generated recording path: %s", path);
    return 0;
}

/**
 * Callback function for flushing buffer packets to recording
 */
static int flush_packet_callback(const AVPacket *packet, void *user_data) {
    if (!packet || !user_data) {
        return -1;
    }

    const char *stream_name = (const char *)user_data;

    // TODO: Write packet to the recording file
    // This will be implemented when we integrate with the MP4 writer
    // For now, just log that we would write it
    log_debug("Would flush packet for stream: %s (size: %d, pts: %lld)",
              stream_name, packet->size, (long long)packet->pts);

    return 0;
}

/**
 * Start recording for a motion event
 */
static int start_motion_recording_internal(motion_recording_context_t *ctx) {
    if (!ctx) {
        return -1;
    }

    pthread_mutex_lock(&ctx->mutex);

    if (ctx->state == RECORDING_STATE_RECORDING) {
        // Already recording, just update timestamp
        ctx->last_motion_time = time(NULL);
        pthread_mutex_unlock(&ctx->mutex);
        return 0;
    }

    // Generate recording file path
    if (generate_recording_path(ctx->stream_name, ctx->current_file_path,
                                sizeof(ctx->current_file_path)) != 0) {
        log_error("Failed to generate recording path for stream: %s", ctx->stream_name);
        pthread_mutex_unlock(&ctx->mutex);
        return -1;
    }

    // Flush pre-event buffer if enabled and not already flushed
    if (ctx->buffer_enabled && ctx->buffer && !ctx->buffer_flushed) {
        int packet_count = 0;
        size_t memory_usage = 0;
        int duration = 0;

        if (packet_buffer_get_stats(ctx->buffer, &packet_count, &memory_usage, &duration) == 0) {
            log_info("Flushing pre-event buffer for stream: %s (%d packets, %d seconds)",
                     ctx->stream_name, packet_count, duration);

            // Flush buffer to recording
            int flushed = packet_buffer_flush(ctx->buffer, flush_packet_callback, (void *)ctx->stream_name);
            if (flushed > 0) {
                ctx->total_buffer_flushes++;
                ctx->buffer_flushed = true;
                log_info("Flushed %d packets from pre-event buffer for stream: %s", flushed, ctx->stream_name);
            }
        }
    }

    // Start MP4 recording using existing infrastructure with trigger_type='motion'
    int result = start_mp4_recording_with_trigger(ctx->stream_name, "motion");
    if (result != 0) {
        log_error("Failed to start MP4 recording for stream: %s", ctx->stream_name);
        pthread_mutex_unlock(&ctx->mutex);
        return -1;
    }

    // Update state
    ctx->state = RECORDING_STATE_RECORDING;
    ctx->recording_start_time = time(NULL);
    ctx->last_motion_time = ctx->recording_start_time;
    ctx->state_change_time = ctx->recording_start_time;
    ctx->total_recordings++;

    log_info("Started motion recording for stream: %s, file: %s",
             ctx->stream_name, ctx->current_file_path);

    pthread_mutex_unlock(&ctx->mutex);
    return 0;
}

/**
 * Stop recording for a motion event
 */
static int stop_motion_recording_internal(motion_recording_context_t *ctx) {
    if (!ctx) {
        return -1;
    }

    pthread_mutex_lock(&ctx->mutex);

    if (ctx->state == RECORDING_STATE_IDLE) {
        pthread_mutex_unlock(&ctx->mutex);
        return 0;
    }

    // Stop MP4 recording
    extern int stop_mp4_recording(const char *stream_name);
    int result = stop_mp4_recording(ctx->stream_name);
    if (result != 0) {
        log_warn("Failed to stop MP4 recording for stream: %s", ctx->stream_name);
    }

    // Update state - go back to buffering if buffer is enabled, otherwise idle
    if (ctx->buffer_enabled && ctx->buffer) {
        ctx->state = RECORDING_STATE_BUFFERING;
        ctx->buffer_flushed = false; // Reset for next recording
        log_info("Stopped motion recording for stream: %s, returning to buffering state", ctx->stream_name);
    } else {
        ctx->state = RECORDING_STATE_IDLE;
        log_info("Stopped motion recording for stream: %s", ctx->stream_name);
    }

    ctx->state_change_time = time(NULL);
    ctx->current_file_path[0] = '\0';

    pthread_mutex_unlock(&ctx->mutex);
    return 0;
}

/**
 * Update recording state based on time and motion events
 */
static void update_recording_state(motion_recording_context_t *ctx, time_t current_time) {
    if (!ctx) {
        return;
    }

    pthread_mutex_lock(&ctx->mutex);

    if (!ctx->enabled) {
        pthread_mutex_unlock(&ctx->mutex);
        return;
    }

    switch (ctx->state) {
        case RECORDING_STATE_IDLE:
            // Nothing to do in idle state
            break;

        case RECORDING_STATE_BUFFERING:
            // Just buffering, waiting for motion
            // Buffer is being filled by feed_packet_to_event_buffer()
            break;

        case RECORDING_STATE_RECORDING:
            // Check if motion has ended and we should enter finalizing state
            if (current_time - ctx->last_motion_time > 2) { // 2 second grace period
                // Motion has ended, enter finalizing state for post-buffer
                ctx->state = RECORDING_STATE_FINALIZING;
                ctx->state_change_time = current_time;
                log_info("Motion ended for stream: %s, entering finalizing state (post-buffer: %ds)",
                         ctx->stream_name, ctx->post_buffer_seconds);
            }

            // Check if we've exceeded max file duration
            if (ctx->max_file_duration > 0 &&
                current_time - ctx->recording_start_time > ctx->max_file_duration) {
                log_info("Max file duration reached for stream: %s, rotating file", ctx->stream_name);
                pthread_mutex_unlock(&ctx->mutex);
                stop_motion_recording_internal(ctx);
                start_motion_recording_internal(ctx);
                return;
            }
            break;

        case RECORDING_STATE_FINALIZING:
            // Post-buffer active, check if we should stop
            if (current_time - ctx->state_change_time > ctx->post_buffer_seconds) {
                log_info("Post-buffer timeout for stream: %s, stopping recording", ctx->stream_name);
                pthread_mutex_unlock(&ctx->mutex);
                stop_motion_recording_internal(ctx);
                return;
            }
            break;
    }

    pthread_mutex_unlock(&ctx->mutex);
}

/**
 * Event processor thread function
 */
static void* event_processor_thread_func(void *arg) {
    log_info("Motion event processor thread started");

    while (event_processor_running) {
        motion_event_t event;

        // Get next event from queue
        if (pop_event(&event) != 0) {
            // Queue is empty or we're shutting down
            if (!event_processor_running) {
                break;
            }
            continue;
        }

        // Get or create recording context
        motion_recording_context_t *ctx = get_recording_context(event.stream_name);
        if (!ctx) {
            log_warn("No recording context for stream: %s, skipping event", event.stream_name);
            continue;
        }

        // Update statistics
        pthread_mutex_lock(&ctx->mutex);
        ctx->total_motion_events++;
        pthread_mutex_unlock(&ctx->mutex);

        // Process event based on motion state
        if (event.active) {
            // Motion detected - start or continue recording
            if (ctx->enabled) {
                pthread_mutex_lock(&ctx->mutex);
                recording_state_t current_state = ctx->state;
                pthread_mutex_unlock(&ctx->mutex);

                if (current_state == RECORDING_STATE_RECORDING) {
                    // Already recording - this is an overlapping event
                    // Just update the last motion time to extend the recording
                    pthread_mutex_lock(&ctx->mutex);
                    ctx->last_motion_time = event.timestamp;
                    // If we were in FINALIZING, go back to RECORDING
                    if (ctx->state == RECORDING_STATE_FINALIZING) {
                        ctx->state = RECORDING_STATE_RECORDING;
                        log_info("Overlapping motion detected for stream: %s, extending recording", ctx->stream_name);
                    }
                    pthread_mutex_unlock(&ctx->mutex);
                } else if (current_state == RECORDING_STATE_FINALIZING) {
                    // Motion detected during post-buffer - restart recording
                    pthread_mutex_lock(&ctx->mutex);
                    ctx->state = RECORDING_STATE_RECORDING;
                    ctx->last_motion_time = event.timestamp;
                    pthread_mutex_unlock(&ctx->mutex);
                    log_info("Motion detected during post-buffer for stream: %s, continuing recording", ctx->stream_name);
                } else {
                    // Start new recording
                    start_motion_recording_internal(ctx);
                }
            }
        } else {
            // Motion ended - update last motion time
            pthread_mutex_lock(&ctx->mutex);
            ctx->last_motion_time = event.timestamp;
            pthread_mutex_unlock(&ctx->mutex);
        }

        // Update recording state
        update_recording_state(ctx, time(NULL));
    }

    log_info("Motion event processor thread stopped");
    return NULL;
}

/**
 * Load motion recording configurations from database and apply them
 */
static void load_motion_configs_from_database(void) {
    log_info("Loading motion recording configurations from database");

    // Allocate arrays for configurations and stream names
    motion_recording_config_t *configs = malloc(MAX_STREAMS * sizeof(motion_recording_config_t));
    char (*stream_names)[256] = malloc(MAX_STREAMS * 256);

    if (!configs || !stream_names) {
        log_error("Failed to allocate memory for loading motion configs");
        if (configs) free(configs);
        if (stream_names) free(stream_names);
        return;
    }

    // Load all configurations from database
    int count = load_all_motion_configs(configs, stream_names, MAX_STREAMS);
    if (count < 0) {
        log_warn("Failed to load motion recording configurations from database");
        free(configs);
        free(stream_names);
        return;
    }

    if (count == 0) {
        log_info("No motion recording configurations found in database");
        free(configs);
        free(stream_names);
        return;
    }

    log_info("Loaded %d motion recording configurations from database", count);

    // Apply each configuration
    for (int i = 0; i < count; i++) {
        if (configs[i].enabled) {
            log_info("Enabling motion recording for stream: %s (pre:%ds, post:%ds)",
                    stream_names[i], configs[i].pre_buffer_seconds, configs[i].post_buffer_seconds);

            if (enable_motion_recording(stream_names[i], &configs[i]) != 0) {
                log_error("Failed to enable motion recording for stream: %s", stream_names[i]);
            }
        } else {
            log_debug("Motion recording disabled for stream: %s", stream_names[i]);
        }
    }

    free(configs);
    free(stream_names);
}

/**
 * Initialize the ONVIF motion recording system
 */
int init_onvif_motion_recording(void) {
    log_info("Initializing ONVIF motion recording system");

    // Initialize packet buffer pool (50MB default limit)
    if (init_packet_buffer_pool(50) != 0) {
        log_error("Failed to initialize packet buffer pool");
        return -1;
    }

    // Initialize recording contexts
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        memset(&recording_contexts[i], 0, sizeof(motion_recording_context_t));
        recording_contexts[i].active = false;
    }
    pthread_mutex_unlock(&contexts_mutex);

    // Initialize event queue
    if (init_event_queue() != 0) {
        log_error("Failed to initialize motion event queue");
        cleanup_packet_buffer_pool();
        return -1;
    }

    // Start event processor thread
    event_processor_running = true;
    if (pthread_create(&event_processor_thread, NULL, event_processor_thread_func, NULL) != 0) {
        log_error("Failed to create motion event processor thread");
        event_processor_running = false;
        cleanup_event_queue();
        cleanup_packet_buffer_pool();
        return -1;
    }
    event_processor_thread_created = true;

    // Load configurations from database and apply them
    load_motion_configs_from_database();

    log_info("ONVIF motion recording system initialized successfully");
    return 0;
}

/**
 * Cleanup the ONVIF motion recording system
 */
void cleanup_onvif_motion_recording(void) {
    log_info("Cleaning up ONVIF motion recording system");

    // Stop event processor thread only if it was created
    if (event_processor_thread_created) {
        event_processor_running = false;
        pthread_cond_broadcast(&event_queue.cond);
        pthread_join(event_processor_thread, NULL);
        event_processor_thread_created = false;
        log_info("Event processor thread stopped");
    }

    // Stop all active recordings (without holding contexts_mutex to avoid deadlocks)
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_lock(&contexts_mutex);
        bool is_active = recording_contexts[i].active;
        packet_buffer_t *buffer = recording_contexts[i].buffer;
        pthread_mutex_unlock(&contexts_mutex);

        if (is_active) {
            // Stop recording without holding contexts_mutex
            stop_motion_recording_internal(&recording_contexts[i]);

            // Destroy buffer if it exists (without holding contexts_mutex)
            if (buffer) {
                destroy_packet_buffer(buffer);
            }
        }
    }

    // Now destroy all mutexes and mark contexts as inactive
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (recording_contexts[i].active) {
            recording_contexts[i].buffer = NULL;
            pthread_mutex_destroy(&recording_contexts[i].mutex);
            recording_contexts[i].active = false;
        }
    }
    pthread_mutex_unlock(&contexts_mutex);

    // Cleanup event queue
    cleanup_event_queue();

    // Cleanup packet buffer pool
    cleanup_packet_buffer_pool();

    log_info("ONVIF motion recording system cleaned up");
}

/**
 * Enable motion recording for a stream
 */
int enable_motion_recording(const char *stream_name, const motion_recording_config_t *config) {
    if (!stream_name || !config) {
        log_error("Invalid parameters for enable_motion_recording");
        return -1;
    }

    // Get or create recording context
    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        ctx = create_recording_context(stream_name);
        if (!ctx) {
            log_error("Failed to create recording context for stream: %s", stream_name);
            return -1;
        }
    }

    // Update configuration
    pthread_mutex_lock(&ctx->mutex);
    ctx->enabled = config->enabled;
    ctx->pre_buffer_seconds = config->pre_buffer_seconds;
    ctx->post_buffer_seconds = config->post_buffer_seconds;
    ctx->max_file_duration = config->max_file_duration;

    // Create or update buffer if pre-buffering is enabled
    if (config->pre_buffer_seconds > 0) {
        if (!ctx->buffer) {
            // Create new buffer
            ctx->buffer = create_packet_buffer(stream_name, config->pre_buffer_seconds, BUFFER_MODE_MEMORY);
            if (ctx->buffer) {
                ctx->buffer_enabled = true;
                ctx->state = RECORDING_STATE_BUFFERING;
                log_info("Created pre-event buffer for stream: %s (%ds)", stream_name, config->pre_buffer_seconds);
            } else {
                log_warn("Failed to create pre-event buffer for stream: %s", stream_name);
                ctx->buffer_enabled = false;
            }
        }
    } else {
        // Destroy buffer if it exists
        if (ctx->buffer) {
            destroy_packet_buffer(ctx->buffer);
            ctx->buffer = NULL;
            ctx->buffer_enabled = false;
            ctx->state = RECORDING_STATE_IDLE;
        }
    }

    pthread_mutex_unlock(&ctx->mutex);

    // Save configuration to database
    if (save_motion_config(stream_name, config) != 0) {
        log_warn("Failed to save motion recording config to database for stream: %s", stream_name);
        // Don't fail the operation, just log the warning
    }

    log_info("Enabled motion recording for stream: %s (pre: %ds, post: %ds, max: %ds, buffer: %s)",
             stream_name, config->pre_buffer_seconds, config->post_buffer_seconds,
             config->max_file_duration, ctx->buffer_enabled ? "yes" : "no");

    return 0;
}

/**
 * Disable motion recording for a stream
 */
int disable_motion_recording(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        return 0; // Already disabled or doesn't exist
    }

    // Stop any active recording
    stop_motion_recording_internal(ctx);

    // Disable recording
    pthread_mutex_lock(&ctx->mutex);
    ctx->enabled = false;
    pthread_mutex_unlock(&ctx->mutex);

    log_info("Disabled motion recording for stream: %s", stream_name);
    return 0;
}

/**
 * Process a motion event
 */
int process_motion_event(const char *stream_name, bool motion_detected, time_t timestamp) {
    if (!stream_name) {
        return -1;
    }

    // Create motion event
    motion_event_t event;
    memset(&event, 0, sizeof(motion_event_t));
    strncpy(event.stream_name, stream_name, MAX_STREAM_NAME - 1);
    event.stream_name[MAX_STREAM_NAME - 1] = '\0';
    event.timestamp = timestamp;
    event.active = motion_detected;
    event.confidence = 1.0f;
    strncpy(event.event_type, "motion", sizeof(event.event_type) - 1);

    // Push to event queue
    if (push_event(&event) != 0) {
        log_error("Failed to push motion event to queue for stream: %s", stream_name);
        return -1;
    }

    log_debug("Queued motion event for stream: %s (active: %d)", stream_name, motion_detected);
    return 0;
}

/**
 * Get recording state for a stream
 */
recording_state_t get_motion_recording_state(const char *stream_name) {
    if (!stream_name) {
        return RECORDING_STATE_IDLE;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        return RECORDING_STATE_IDLE;
    }

    pthread_mutex_lock(&ctx->mutex);
    recording_state_t state = ctx->state;
    pthread_mutex_unlock(&ctx->mutex);

    return state;
}

/**
 * Get recording statistics for a stream
 */
int get_motion_recording_stats(const char *stream_name, uint64_t *total_recordings, uint64_t *total_events) {
    if (!stream_name || !total_recordings || !total_events) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        *total_recordings = 0;
        *total_events = 0;
        return -1;
    }

    pthread_mutex_lock(&ctx->mutex);
    *total_recordings = ctx->total_recordings;
    *total_events = ctx->total_motion_events;
    pthread_mutex_unlock(&ctx->mutex);

    return 0;
}

/**
 * Update motion recording configuration for a stream
 */
int update_motion_recording_config(const char *stream_name, const motion_recording_config_t *config) {
    if (!stream_name || !config) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        log_error("No recording context found for stream: %s", stream_name);
        return -1;
    }

    pthread_mutex_lock(&ctx->mutex);
    ctx->enabled = config->enabled;
    ctx->pre_buffer_seconds = config->pre_buffer_seconds;
    ctx->post_buffer_seconds = config->post_buffer_seconds;
    ctx->max_file_duration = config->max_file_duration;
    pthread_mutex_unlock(&ctx->mutex);

    // Save configuration to database
    if (update_motion_config(stream_name, config) != 0) {
        log_warn("Failed to save motion recording config to database for stream: %s", stream_name);
        // Don't fail the operation, just log the warning
    }

    log_info("Updated motion recording config for stream: %s", stream_name);
    return 0;
}

/**
 * Check if motion recording is enabled for a stream
 */
bool is_motion_recording_enabled(const char *stream_name) {
    if (!stream_name) {
        return false;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        return false;
    }

    pthread_mutex_lock(&ctx->mutex);
    bool enabled = ctx->enabled;
    pthread_mutex_unlock(&ctx->mutex);

    return enabled;
}

/**
 * Get current recording file path for a stream
 */
int get_current_motion_recording_path(const char *stream_name, char *path, size_t path_size) {
    if (!stream_name || !path || path_size == 0) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        return -1;
    }

    pthread_mutex_lock(&ctx->mutex);
    if (ctx->current_file_path[0] != '\0') {
        strncpy(path, ctx->current_file_path, path_size - 1);
        path[path_size - 1] = '\0';
        pthread_mutex_unlock(&ctx->mutex);
        return 0;
    }
    pthread_mutex_unlock(&ctx->mutex);

    return -1;
}

/**
 * Force stop recording for a stream
 */
int force_stop_motion_recording(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx) {
        return 0;
    }

    return stop_motion_recording_internal(ctx);
}

/**
 * Feed a video packet to the event recording buffer
 */
int feed_packet_to_event_buffer(const char *stream_name, const AVPacket *packet) {
    if (!stream_name || !packet) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx || !ctx->enabled || !ctx->buffer_enabled || !ctx->buffer) {
        return 0; // Not an error, just not buffering
    }

    // Add packet to buffer
    int result = packet_buffer_add_packet(ctx->buffer, packet, time(NULL));

    // Update state to BUFFERING if we're in IDLE
    if (result == 0 && ctx->state == RECORDING_STATE_IDLE) {
        pthread_mutex_lock(&ctx->mutex);
        ctx->state = RECORDING_STATE_BUFFERING;
        pthread_mutex_unlock(&ctx->mutex);
    }

    return result;
}

/**
 * Get buffer statistics for a stream
 */
int get_event_buffer_stats(const char *stream_name, int *packet_count, size_t *memory_usage, int *duration) {
    if (!stream_name) {
        return -1;
    }

    motion_recording_context_t *ctx = get_recording_context(stream_name);
    if (!ctx || !ctx->buffer) {
        return -1;
    }

    return packet_buffer_get_stats(ctx->buffer, packet_count, memory_usage, duration);
}
