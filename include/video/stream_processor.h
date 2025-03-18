/**
 * @file stream_processor.h
 * @brief Stream processor component that manages distribution of packets to writers/detection
 */
#ifndef STREAM_PROCESSOR_H
#define STREAM_PROCESSOR_H

#include <pthread.h>
#include <stdbool.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include "stream_reader.h"
#include "hls_writer.h"
#include "mp4_writer.h"

// Forward declarations
typedef struct stream_processor_s* stream_processor_t;

/**
 * @brief Output type enum for stream processor
 */
typedef enum {
    OUTPUT_TYPE_NONE = 0,
    OUTPUT_TYPE_HLS,
    OUTPUT_TYPE_MP4,
    OUTPUT_TYPE_DETECTION
} output_type_t;

/**
 * @brief Output configuration structure
 */
typedef struct {
    output_type_t type;
    union {
        struct {
            char output_path[MAX_PATH_LENGTH];
            int segment_duration;
        } hls;
        struct {
            char output_path[MAX_PATH_LENGTH];
            int segment_duration;
        } mp4;
        struct {
            char model_path[MAX_PATH_LENGTH];
            float threshold;
            int interval;
            int pre_buffer;
            int post_buffer;
        } detection;
    };
} output_config_t;

/**
 * @brief Create a new stream processor
 * 
 * @param stream_name Name of the stream
 * @param reader Stream reader to use for packet reading
 * @return stream_processor_t New stream processor or NULL on failure
 */
stream_processor_t stream_processor_create(const char *stream_name, stream_reader_ctx_t *reader);

/**
 * @brief Destroy a stream processor
 * 
 * @param processor Stream processor to destroy
 */
void stream_processor_destroy(stream_processor_t processor);

/**
 * @brief Add an output to the stream processor
 * 
 * @param processor Stream processor
 * @param config Output configuration
 * @return int 0 on success, non-zero on failure
 */
int stream_processor_add_output(stream_processor_t processor, const output_config_t *config);

/**
 * @brief Remove an output from the stream processor
 * 
 * @param processor Stream processor
 * @param type Output type to remove
 * @return int 0 on success, non-zero on failure
 */
int stream_processor_remove_output(stream_processor_t processor, output_type_t type);

/**
 * @brief Start processing
 * 
 * @param processor Stream processor
 * @return int 0 on success, non-zero on failure
 */
int stream_processor_start(stream_processor_t processor);

/**
 * @brief Stop processing
 * 
 * @param processor Stream processor
 * @return int 0 on success, non-zero on failure
 */
int stream_processor_stop(stream_processor_t processor);

/**
 * @brief Check if the processor is running
 * 
 * @param processor Stream processor
 * @return bool True if running, false otherwise
 */
bool stream_processor_is_running(stream_processor_t processor);

/**
 * @brief Get the stream name
 * 
 * @param processor Stream processor
 * @return const char* Stream name
 */
const char *stream_processor_get_name(stream_processor_t processor);

/**
 * @brief Get the stream reader
 * 
 * @param processor Stream processor
 * @return stream_reader_ctx_t* Stream reader
 */
stream_reader_ctx_t *stream_processor_get_reader(stream_processor_t processor);

/**
 * @brief Get the HLS writer
 * 
 * @param processor Stream processor
 * @return hls_writer_t* HLS writer or NULL if not enabled
 */
hls_writer_t *stream_processor_get_hls_writer(stream_processor_t processor);

/**
 * @brief Get the MP4 writer
 * 
 * @param processor Stream processor
 * @return mp4_writer_t* MP4 writer or NULL if not enabled
 */
mp4_writer_t *stream_processor_get_mp4_writer(stream_processor_t processor);

#endif // STREAM_PROCESSOR_H
