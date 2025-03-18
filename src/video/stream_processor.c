/**
 * @file stream_processor.c
 * @brief Implementation of the stream processor component
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <stdbool.h>
#include <assert.h>

#include "core/logger.h"
#include "video/stream_processor.h"
#include "video/stream_transcoding.h"

// Maximum number of outputs per processor
#define MAX_OUTPUTS 8

/**
 * @brief Output structure
 */
typedef struct {
    output_type_t type;
    bool enabled;
    union {
        struct {
            hls_writer_t *writer;
            char output_path[MAX_PATH_LENGTH];
            int segment_duration;
        } hls;
        struct {
            mp4_writer_t *writer;
            char output_path[MAX_PATH_LENGTH];
            int segment_duration;
        } mp4;
        struct {
            char model_path[MAX_PATH_LENGTH];
            float threshold;
            int interval;
            int pre_buffer;
            int post_buffer;
            int frame_counter;
            bool recording_active;
        } detection;
    };
} output_t;

/**
 * @brief Stream processor structure
 */
struct stream_processor_s {
    char stream_name[MAX_STREAM_NAME];
    stream_reader_ctx_t *reader;
    bool running;
    bool owns_reader;
    
    // Outputs
    output_t outputs[MAX_OUTPUTS];
    int output_count;
    
    // Synchronization
    pthread_mutex_t mutex;
    
    // Statistics
    uint64_t frames_processed;
    uint64_t frames_dropped;
    uint64_t errors;
};

// Forward declarations
static int packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data);
static int process_packet_for_outputs(stream_processor_t processor, const AVPacket *pkt, const AVStream *stream);

/**
 * @brief Create a new stream processor
 */
stream_processor_t stream_processor_create(const char *stream_name, stream_reader_ctx_t *reader) {
    if (!stream_name || !reader) {
        log_error("Invalid parameters for stream_processor_create");
        return NULL;
    }
    
    stream_processor_t processor = calloc(1, sizeof(struct stream_processor_s));
    if (!processor) {
        log_error("Failed to allocate memory for stream processor");
        return NULL;
    }
    
    // Initialize processor
    strncpy(processor->stream_name, stream_name, MAX_STREAM_NAME - 1);
    processor->stream_name[MAX_STREAM_NAME - 1] = '\0';
    processor->reader = reader;
    processor->running = false;
    processor->owns_reader = false;  // We don't own the reader by default
    processor->output_count = 0;
    
    // Initialize mutex
    if (pthread_mutex_init(&processor->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex for stream processor");
        free(processor);
        return NULL;
    }
    
    log_info("Created stream processor for stream %s", stream_name);
    return processor;
}

/**
 * @brief Destroy a stream processor
 */
void stream_processor_destroy(stream_processor_t processor) {
    if (!processor) {
        return;
    }
    
    // Stop processing if still running
    if (processor->running) {
        stream_processor_stop(processor);
    }
    
    // Clean up outputs
    pthread_mutex_lock(&processor->mutex);
    for (int i = 0; i < processor->output_count; i++) {
        output_t *output = &processor->outputs[i];
        
        switch (output->type) {
            case OUTPUT_TYPE_HLS:
                if (output->hls.writer) {
                    hls_writer_close(output->hls.writer);
                    output->hls.writer = NULL;
                }
                break;
                
            case OUTPUT_TYPE_MP4:
                if (output->mp4.writer) {
                    mp4_writer_close(output->mp4.writer);
                    output->mp4.writer = NULL;
                }
                break;
                
            case OUTPUT_TYPE_DETECTION:
                // Nothing to clean up for detection
                break;
                
            default:
                break;
        }
    }
    
    // Clean up reader if we own it
    if (processor->owns_reader && processor->reader) {
        stop_stream_reader(processor->reader);
        processor->reader = NULL;
    }
    
    pthread_mutex_unlock(&processor->mutex);
    
    // Destroy mutex
    pthread_mutex_destroy(&processor->mutex);
    
    log_info("Destroyed stream processor for stream %s", processor->stream_name);
    
    // Free processor
    free(processor);
}


/**
 * @brief Add an output to the stream processor
 */
int stream_processor_add_output(stream_processor_t processor, const output_config_t *config) {
    if (!processor || !config) {
        log_error("Invalid parameters for stream_processor_add_output");
        return -1;
    }

    pthread_mutex_lock(&processor->mutex);

    // Check if we already have this output type
    for (int i = 0; i < processor->output_count; i++) {
        if (processor->outputs[i].type == config->type) {
            log_warn("Output type %d already exists for stream %s",
                    config->type, processor->stream_name);
            pthread_mutex_unlock(&processor->mutex);
            return -1;
        }
    }

    // Check if we have room for another output
    if (processor->output_count >= MAX_OUTPUTS) {
        log_error("Maximum number of outputs reached for stream %s", processor->stream_name);
        pthread_mutex_unlock(&processor->mutex);
        return -1;
    }

    // Add the output
    output_t *output = &processor->outputs[processor->output_count];
    output->type = config->type;
    output->enabled = true;

    switch (config->type) {
        case OUTPUT_TYPE_HLS:
            strncpy(output->hls.output_path, config->hls.output_path, MAX_PATH_LENGTH - 1);
            output->hls.output_path[MAX_PATH_LENGTH - 1] = '\0';
            output->hls.segment_duration = config->hls.segment_duration;
            output->hls.writer = NULL;  // Will be created when processing starts
            break;

        case OUTPUT_TYPE_MP4:
            strncpy(output->mp4.output_path, config->mp4.output_path, MAX_PATH_LENGTH - 1);
            output->mp4.output_path[MAX_PATH_LENGTH - 1] = '\0';
            output->mp4.segment_duration = config->mp4.segment_duration;
            output->mp4.writer = NULL;  // Will be created when processing starts
            break;

        case OUTPUT_TYPE_DETECTION:
            strncpy(output->detection.model_path, config->detection.model_path, MAX_PATH_LENGTH - 1);
            output->detection.model_path[MAX_PATH_LENGTH - 1] = '\0';
            output->detection.threshold = config->detection.threshold;
            output->detection.interval = config->detection.interval;
            output->detection.pre_buffer = config->detection.pre_buffer;
            output->detection.post_buffer = config->detection.post_buffer;
            output->detection.frame_counter = 0;
            output->detection.recording_active = false;
            break;

        default:
            log_error("Invalid output type %d", config->type);
            pthread_mutex_unlock(&processor->mutex);
            return -1;
    }

    processor->output_count++;

    pthread_mutex_unlock(&processor->mutex);

    log_info("Added output type %d to stream processor for stream %s",
            config->type, processor->stream_name);

    return 0;
}

/**
 * @brief Remove an output from the stream processor
 */
int stream_processor_remove_output(stream_processor_t processor, output_type_t type) {
    if (!processor) {
        log_error("Invalid parameters for stream_processor_remove_output");
        return -1;
    }
    
    pthread_mutex_lock(&processor->mutex);
    
    // Find the output
    int index = -1;
    for (int i = 0; i < processor->output_count; i++) {
        if (processor->outputs[i].type == type) {
            index = i;
            break;
        }
    }
    
    if (index == -1) {
        log_warn("Output type %d not found for stream %s", type, processor->stream_name);
        pthread_mutex_unlock(&processor->mutex);
        return -1;
    }
    
    // Clean up the output
    output_t *output = &processor->outputs[index];
    
    // CRITICAL FIX: Make a local copy of the writer before closing it
    // This prevents race conditions if another thread is using the writer
    void *writer_to_close = NULL;
    
    switch (output->type) {
        case OUTPUT_TYPE_HLS:
            if (output->hls.writer) {
                writer_to_close = output->hls.writer;
                output->hls.writer = NULL;
            }
            break;
            
        case OUTPUT_TYPE_MP4:
            if (output->mp4.writer) {
                writer_to_close = output->mp4.writer;
                output->mp4.writer = NULL;
            }
            break;
            
        case OUTPUT_TYPE_DETECTION:
            // Nothing to clean up for detection
            break;
            
        default:
            break;
    }
    
    // Remove the output by shifting the array
    if (index < processor->output_count - 1) {
        memmove(&processor->outputs[index], &processor->outputs[index + 1], 
                (processor->output_count - index - 1) * sizeof(output_t));
    }
    
    processor->output_count--;
    
    pthread_mutex_unlock(&processor->mutex);
    
    // CRITICAL FIX: Close the writer after releasing the mutex
    // This prevents deadlocks if the close operation takes a long time
    if (writer_to_close) {
        if (type == OUTPUT_TYPE_HLS) {
            hls_writer_close((hls_writer_t *)writer_to_close);
        } else if (type == OUTPUT_TYPE_MP4) {
            mp4_writer_close((mp4_writer_t *)writer_to_close);
        }
    }
    
    log_info("Removed output type %d from stream processor for stream %s", 
            type, processor->stream_name);
    
    return 0;
}

/**
 * @brief Initialize outputs for processing
 */
static int initialize_outputs(stream_processor_t processor) {
    pthread_mutex_lock(&processor->mutex);
    
    for (int i = 0; i < processor->output_count; i++) {
        output_t *output = &processor->outputs[i];
        
        switch (output->type) {
            case OUTPUT_TYPE_HLS:
                // Create HLS writer
                output->hls.writer = hls_writer_create(output->hls.output_path, 
                                                     processor->stream_name, 
                                                     output->hls.segment_duration);
                if (!output->hls.writer) {
                    log_error("Failed to create HLS writer for stream %s", processor->stream_name);
                    pthread_mutex_unlock(&processor->mutex);
                    return -1;
                }
                break;
                
            case OUTPUT_TYPE_MP4:
                // Create MP4 writer
                output->mp4.writer = mp4_writer_create(output->mp4.output_path, 
                                                     processor->stream_name);
                if (!output->mp4.writer) {
                    log_error("Failed to create MP4 writer for stream %s", processor->stream_name);
                    pthread_mutex_unlock(&processor->mutex);
                    return -1;
                }
                break;
                
            case OUTPUT_TYPE_DETECTION:
                // Reset detection state
                output->detection.frame_counter = 0;
                output->detection.recording_active = false;
                break;
                
            default:
                break;
        }
    }
    
    pthread_mutex_unlock(&processor->mutex);
    return 0;
}

/**
 * @brief Start processing
 */
int stream_processor_start(stream_processor_t processor) {
    if (!processor) {
        log_error("Invalid parameters for stream_processor_start");
        return -1;
    }
    
    pthread_mutex_lock(&processor->mutex);
    
    // Check if already running
    if (processor->running) {
        log_warn("Stream processor for stream %s is already running", processor->stream_name);
        pthread_mutex_unlock(&processor->mutex);
        return 0;
    }
    
    // Initialize outputs
    if (initialize_outputs(processor) != 0) {
        log_error("Failed to initialize outputs for stream %s", processor->stream_name);
        pthread_mutex_unlock(&processor->mutex);
        return -1;
    }
    
    // Set the packet callback on the reader
    if (set_packet_callback(processor->reader, packet_callback, processor) != 0) {
        log_error("Failed to set packet callback for stream %s", processor->stream_name);
        pthread_mutex_unlock(&processor->mutex);
        return -1;
    }
    
    // Mark as running
    processor->running = true;
    
    pthread_mutex_unlock(&processor->mutex);
    
    log_info("Started stream processor for stream %s", processor->stream_name);
    
    return 0;
}

/**
 * @brief Stop processing
 */
int stream_processor_stop(stream_processor_t processor) {
    if (!processor) {
        log_error("Invalid parameters for stream_processor_stop");
        return -1;
    }
    
    pthread_mutex_lock(&processor->mutex);
    
    // Check if already stopped
    if (!processor->running) {
        log_warn("Stream processor for stream %s is already stopped", processor->stream_name);
        pthread_mutex_unlock(&processor->mutex);
        return 0;
    }
    
    // CRITICAL FIX: Mark as not running before cleaning up resources
    // This prevents new packets from being processed during cleanup
    processor->running = false;
    
    // Clear the packet callback on the reader
    stream_reader_ctx_t *reader = processor->reader;
    
    pthread_mutex_unlock(&processor->mutex);
    
    // CRITICAL FIX: Clear the callback outside the mutex lock
    // This prevents deadlocks if the reader is also trying to acquire the mutex
    if (reader) {
        set_packet_callback(reader, NULL, NULL);
    }
    
    pthread_mutex_lock(&processor->mutex);
    
    // Clean up outputs
    for (int i = 0; i < processor->output_count; i++) {
        output_t *output = &processor->outputs[i];
        
        switch (output->type) {
            case OUTPUT_TYPE_HLS:
                if (output->hls.writer) {
                    hls_writer_close(output->hls.writer);
                    output->hls.writer = NULL;
                }
                break;
                
            case OUTPUT_TYPE_MP4:
                if (output->mp4.writer) {
                    mp4_writer_close(output->mp4.writer);
                    output->mp4.writer = NULL;
                }
                break;
                
            case OUTPUT_TYPE_DETECTION:
                // Nothing to clean up for detection
                break;
                
            default:
                break;
        }
    }
    
    pthread_mutex_unlock(&processor->mutex);
    
    log_info("Stopped stream processor for stream %s", processor->stream_name);
    
    return 0;
}

/**
 * @brief Check if the processor is running
 */
bool stream_processor_is_running(stream_processor_t processor) {
    if (!processor) {
        return false;
    }
    
    pthread_mutex_lock(&processor->mutex);
    bool running = processor->running;
    pthread_mutex_unlock(&processor->mutex);
    
    return running;
}

/**
 * @brief Get the stream name
 */
const char *stream_processor_get_name(stream_processor_t processor) {
    if (!processor) {
        return NULL;
    }
    
    return processor->stream_name;
}

/**
 * @brief Get the stream reader
 */
stream_reader_ctx_t *stream_processor_get_reader(stream_processor_t processor) {
    if (!processor) {
        return NULL;
    }
    
    return processor->reader;
}

/**
 * @brief Get the HLS writer
 */
hls_writer_t *stream_processor_get_hls_writer(stream_processor_t processor) {
    if (!processor) {
        return NULL;
    }
    
    pthread_mutex_lock(&processor->mutex);
    
    hls_writer_t *writer = NULL;
    
    for (int i = 0; i < processor->output_count; i++) {
        if (processor->outputs[i].type == OUTPUT_TYPE_HLS) {
            writer = processor->outputs[i].hls.writer;
            break;
        }
    }
    
    pthread_mutex_unlock(&processor->mutex);
    
    return writer;
}

/**
 * @brief Get the MP4 writer
 */
mp4_writer_t *stream_processor_get_mp4_writer(stream_processor_t processor) {
    if (!processor) {
        return NULL;
    }
    
    pthread_mutex_lock(&processor->mutex);
    
    mp4_writer_t *writer = NULL;
    
    for (int i = 0; i < processor->output_count; i++) {
        if (processor->outputs[i].type == OUTPUT_TYPE_MP4) {
            writer = processor->outputs[i].mp4.writer;
            break;
        }
    }
    
    pthread_mutex_unlock(&processor->mutex);
    
    return writer;
}

/**
 * @brief Packet callback function for stream reader
 */
static int packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data) {
    stream_processor_t processor = (stream_processor_t)user_data;
    
    if (!processor || !pkt || !stream) {
        return -1;
    }
    
    // Process the packet for all outputs
    return process_packet_for_outputs(processor, pkt, stream);
}

/**
 * @brief Process a packet for all outputs
 */
static int process_packet_for_outputs(stream_processor_t processor, const AVPacket *pkt, const AVStream *stream) {
    if (!processor || !pkt || !stream) {
        return -1;
    }

    pthread_mutex_lock(&processor->mutex);

    // Check if we're running
    if (!processor->running) {
        pthread_mutex_unlock(&processor->mutex);
        return 0;
    }

    // CRITICAL FIX: Make local copies of output data to minimize lock time
    int output_count = processor->output_count;
    output_t outputs[MAX_OUTPUTS];
    memcpy(outputs, processor->outputs, sizeof(output_t) * output_count);

    // Increment frames processed counter
    processor->frames_processed++;

    pthread_mutex_unlock(&processor->mutex);

    // CRITICAL FIX: Process the packet for each output using our local copies
    // This prevents holding the mutex during potentially long-running operations
    for (int i = 0; i < output_count; i++) {
        output_t *output = &outputs[i];

        if (!output->enabled) {
            continue;
        }

        switch (output->type) {
            case OUTPUT_TYPE_HLS:
                if (output->hls.writer) {
                    // CRITICAL FIX: Improve error handling for HLS packet processing
                    int ret = process_video_packet(pkt, stream, output->hls.writer, 0, processor->stream_name);
                    if (ret < 0) {
                        log_error("Failed to process packet for HLS output for stream %s (error: %d)",
                                processor->stream_name, ret);
                        pthread_mutex_lock(&processor->mutex);
                        processor->errors++;
                        pthread_mutex_unlock(&processor->mutex);
                    }
                }
                break;

            case OUTPUT_TYPE_MP4:
                if (output->mp4.writer) {
                    // CRITICAL FIX: Improve error handling for MP4 packet processing
                    int ret = process_video_packet(pkt, stream, output->mp4.writer, 1, processor->stream_name);
                    if (ret < 0) {
                        log_error("Failed to process packet for MP4 output for stream %s (error: %d)",
                                processor->stream_name, ret);
                        pthread_mutex_lock(&processor->mutex);
                        processor->errors++;
                        pthread_mutex_unlock(&processor->mutex);
                    }
                }
                break;

            case OUTPUT_TYPE_DETECTION:
                // Increment frame counter
                output->detection.frame_counter++;

                // Check if we need to run detection
                if (output->detection.frame_counter >= output->detection.interval) {
                    output->detection.frame_counter = 0;

                    // TODO: Implement detection logic
                    // This would call into the detection subsystem
                    // For now, we just log that we would run detection
                    log_debug("Would run detection for stream %s", processor->stream_name);
                }
                break;

            default:
                break;
        }
    }
    
    return 0;
}
