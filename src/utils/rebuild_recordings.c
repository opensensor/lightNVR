/**
 * @file rebuild_recordings.c
 * @brief Utility to rebuild missing recordings data from the configured recordings path
 * 
 * This utility scans the recordings directory, checks if each recording is in the database,
 * and adds missing recordings. If a recording's stream doesn't exist, it creates a
 * soft-deleted stream with the same name and a dummy URL.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include <errno.h>
#include <time.h>
#include <stdbool.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>

#include "core/config.h"
#include "core/logger.h"
#include "database/database_manager.h"
#include "database/db_streams.h"
#include "database/db_recordings.h"
#include "database/db_schema.h"
#include "database/db_schema_cache.h"

// Dummy URL for soft-deleted streams
#define DUMMY_URL "rtsp://dummy.url/stream"

// Structure to hold recording file information
typedef struct {
    char path[MAX_PATH_LENGTH];
    char stream_name[MAX_STREAM_NAME];
    time_t start_time;
    time_t end_time;
    uint64_t size_bytes;
    int width;
    int height;
    int fps;
    char codec[16];
} recording_file_info_t;

/**
 * Check if a recording exists in the database
 * 
 * @param file_path Path to the recording file
 * @return true if the recording exists in the database, false otherwise
 */
static bool recording_exists_in_db(const char *file_path) {
    recording_metadata_t *metadata;
    int count, i;
    bool exists = false;
    
    // Allocate memory for metadata (assuming a reasonable maximum)
    metadata = (recording_metadata_t *)malloc(1000 * sizeof(recording_metadata_t));
    if (!metadata) {
        log_error("Failed to allocate memory for recording metadata");
        return false;
    }
    
    // Get all recordings from the database
    count = get_recording_metadata(0, 0, NULL, metadata, 1000);
    
    // Check if the file path exists in the database
    for (i = 0; i < count; i++) {
        if (strcmp(metadata[i].file_path, file_path) == 0) {
            exists = true;
            break;
        }
    }
    
    free(metadata);
    return exists;
}

/**
 * Check if a stream exists in the database
 * 
 * @param stream_name Name of the stream
 * @param is_disabled Pointer to store whether the stream is disabled
 * @return true if the stream exists in the database, false otherwise
 */
static bool stream_exists_in_db(const char *stream_name, bool *is_disabled) {
    stream_config_t stream;
    int result;
    
    // Try to get the stream configuration
    result = get_stream_config_by_name(stream_name, &stream);
    
    if (result == 0) {
        // Stream exists
        if (is_disabled) *is_disabled = !stream.enabled;
        return true;
    }
    
    // Check if the stream exists but is disabled
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *db_mutex = get_db_mutex();
    
    if (!db) {
        log_error("Database not initialized");
        return false;
    }
    
    pthread_mutex_lock(db_mutex);
    
    sqlite3_stmt *stmt;
    const char *sql = "SELECT id FROM streams WHERE name = ? AND enabled = 0;";
    
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare statement: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(db_mutex);
        return false;
    }
    
    sqlite3_bind_text(stmt, 1, stream_name, -1, SQLITE_STATIC);
    
    bool exists = false;
    if (sqlite3_step(stmt) == SQLITE_ROW) {
        exists = true;
        if (is_disabled) *is_disabled = true;
    }
    
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    
    return exists;
}

/**
 * Create a disabled stream in the database
 * 
 * @param stream_name Name of the stream
 * @return true if the stream was created successfully, false otherwise
 */
static bool create_disabled_stream(const char *stream_name) {
    stream_config_t stream;
    uint64_t stream_id;
    bool is_disabled = false;
    
    // Check if the stream already exists but is disabled
    if (stream_exists_in_db(stream_name, &is_disabled) && is_disabled) {
        log_info("Stream %s already exists as disabled, not modifying it", stream_name);
        return true;
    }
    
    // Initialize stream configuration with default values
    memset(&stream, 0, sizeof(stream_config_t));
    strncpy(stream.name, stream_name, MAX_STREAM_NAME - 1);
    strncpy(stream.url, DUMMY_URL, MAX_URL_LENGTH - 1);
    stream.enabled = false;
    stream.streaming_enabled = false;
    stream.width = 1280;
    stream.height = 720;
    stream.fps = 30;
    strncpy(stream.codec, "h264", sizeof(stream.codec) - 1);
    stream.priority = 5;
    stream.record = false;  // Set record to false for disabled streams
    stream.segment_duration = 60;
    stream.detection_based_recording = false;
    stream.protocol = STREAM_PROTOCOL_TCP;
    stream.record_audio = false;  // Set record_audio to false for disabled streams
    
    // Add the stream to the database
    stream_id = add_stream_config(&stream);
    if (stream_id == 0) {
        log_error("Failed to add stream configuration for %s", stream_name);
        return false;
    }
    
    // The stream is already created as disabled by default
    log_info("Created disabled stream: %s", stream_name);
    return true;
}

/**
 * Extract recording information from a file path
 * 
 * @param file_path Path to the recording file
 * @param info Pointer to recording_file_info_t structure to fill
 * @return true if information was extracted successfully, false otherwise
 */
static bool extract_recording_info(const char *file_path, recording_file_info_t *info) {
    AVFormatContext *format_ctx = NULL;
    AVCodecParameters *codec_params = NULL;
    int video_stream_index = -1;
    int i;
    struct stat st;
    
    // Initialize info structure
    memset(info, 0, sizeof(recording_file_info_t));
    strncpy(info->path, file_path, MAX_PATH_LENGTH - 1);
    
    // Extract stream name from path
    // Assuming path format: /storage_path/mp4/stream_name/recording.mp4
    const char *mp4_pos = strstr(file_path, "/mp4/");
    if (!mp4_pos) {
        log_error("Invalid recording path format: %s", file_path);
        return false;
    }
    
    const char *stream_name_start = mp4_pos + 5; // Skip "/mp4/"
    const char *stream_name_end = strchr(stream_name_start, '/');
    if (!stream_name_end) {
        log_error("Invalid recording path format: %s", file_path);
        return false;
    }
    
    size_t stream_name_len = stream_name_end - stream_name_start;
    if (stream_name_len >= MAX_STREAM_NAME) {
        stream_name_len = MAX_STREAM_NAME - 1;
    }
    strncpy(info->stream_name, stream_name_start, stream_name_len);
    info->stream_name[stream_name_len] = '\0';
    
    // Get file size
    if (stat(file_path, &st) == 0) {
        info->size_bytes = st.st_size;
    } else {
        log_error("Failed to get file size: %s", file_path);
        return false;
    }
    
    // Open the file with FFmpeg
    if (avformat_open_input(&format_ctx, file_path, NULL, NULL) != 0) {
        log_error("Failed to open file: %s", file_path);
        return false;
    }
    
    // Get stream information
    if (avformat_find_stream_info(format_ctx, NULL) < 0) {
        log_error("Failed to find stream info: %s", file_path);
        avformat_close_input(&format_ctx);
        return false;
    }
    
    // Find the first video stream
    for (i = 0; i < format_ctx->nb_streams; i++) {
        if (format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_index = i;
            break;
        }
    }
    
    if (video_stream_index == -1) {
        log_error("No video stream found: %s", file_path);
        avformat_close_input(&format_ctx);
        return false;
    }
    
    // Get codec parameters
    codec_params = format_ctx->streams[video_stream_index]->codecpar;
    
    // Fill in video information
    info->width = codec_params->width;
    info->height = codec_params->height;
    
    // Get codec name
    const AVCodecDescriptor *codec_desc = avcodec_descriptor_get(codec_params->codec_id);
    if (codec_desc) {
        strncpy(info->codec, codec_desc->name, sizeof(info->codec) - 1);
    } else {
        strncpy(info->codec, "unknown", sizeof(info->codec) - 1);
    }
    
    // Calculate FPS
    AVRational frame_rate = format_ctx->streams[video_stream_index]->avg_frame_rate;
    if (frame_rate.den != 0) {
        info->fps = frame_rate.num / frame_rate.den;
    } else {
        info->fps = 30; // Default to 30 fps if not available
    }
    
    // Get start and end times
    if (format_ctx->duration != AV_NOPTS_VALUE) {
        // Duration is in AV_TIME_BASE units (microseconds)
        int64_t duration_us = format_ctx->duration;
        int64_t duration_s = duration_us / AV_TIME_BASE;
        
        // Use file modification time as the end time
        info->end_time = st.st_mtime;
        
        // Calculate start time by subtracting duration from end time
        info->start_time = info->end_time - duration_s;
        
        log_info("Using file modification time for recording: %s (start: %ld, end: %ld, duration: %ld)",
                file_path, info->start_time, info->end_time, duration_s);
    } else {
        // If duration is not available, use file modification time and assume 1 minute duration
        info->end_time = st.st_mtime;
        info->start_time = info->end_time - 30; // Assume 30 second duration
        
        log_warn("Duration not available for recording: %s, assuming 30 seconds", file_path);
    }
    
    avformat_close_input(&format_ctx);
    return true;
}

/**
 * Add a recording to the database
 * 
 * @param info Recording information
 * @return true if the recording was added successfully, false otherwise
 */
static bool add_recording_to_db(const recording_file_info_t *info) {
    recording_metadata_t metadata;
    uint64_t recording_id;
    
    // Fill in metadata
    memset(&metadata, 0, sizeof(recording_metadata_t));
    strncpy(metadata.stream_name, info->stream_name, sizeof(metadata.stream_name) - 1);
    strncpy(metadata.file_path, info->path, sizeof(metadata.file_path) - 1);
    metadata.start_time = info->start_time;
    metadata.end_time = info->end_time;
    metadata.size_bytes = info->size_bytes;
    metadata.width = info->width;
    metadata.height = info->height;
    metadata.fps = info->fps;
    strncpy(metadata.codec, info->codec, sizeof(metadata.codec) - 1);
    metadata.is_complete = true;
    
    // Add to database
    recording_id = add_recording_metadata(&metadata);
    if (recording_id == 0) {
        log_error("Failed to add recording metadata for %s", info->path);
        return false;
    }
    
    log_info("Added recording: %s (stream: %s, start: %ld, end: %ld)",
             info->path, info->stream_name, info->start_time, info->end_time);
    
    return true;
}

/**
 * Process a recording file
 * 
 * @param file_path Path to the recording file
 * @return true if the file was processed successfully, false otherwise
 */
static bool process_recording_file(const char *file_path) {
    recording_file_info_t info;
    bool is_disabled;
    
    // Check if the recording already exists in the database
    if (recording_exists_in_db(file_path)) {
        log_debug("Recording already exists in database: %s", file_path);
        return true;
    }
    
    // Extract recording information
    if (!extract_recording_info(file_path, &info)) {
        log_error("Failed to extract recording information: %s", file_path);
        return false;
    }
    
    // Check if the stream exists
    if (!stream_exists_in_db(info.stream_name, &is_disabled)) {
        // Stream doesn't exist, create a disabled stream
        if (!create_disabled_stream(info.stream_name)) {
            log_error("Failed to create disabled stream: %s", info.stream_name);
            return false;
        }
    } else if (is_disabled) {
        log_info("Stream %s already exists as disabled", info.stream_name);
    } else {
        log_info("Stream %s already exists", info.stream_name);
    }
    
    // Add the recording to the database
    if (!add_recording_to_db(&info)) {
        log_error("Failed to add recording to database: %s", file_path);
        return false;
    }
    
    return true;
}

// Forward declarations
static bool process_directory(const char *dir_path, int *processed_count, int *added_count);

/**
 * Process MP4 files in a directory (non-recursive)
 * 
 * @param dir_path Path to the directory
 * @param processed_count Pointer to counter for processed files
 * @param added_count Pointer to counter for added files
 * @return true if the directory was processed successfully, false otherwise
 */
static bool process_directory(const char *dir_path, int *processed_count, int *added_count) {
    DIR *dir;
    struct dirent *entry;
    char file_path[MAX_PATH_LENGTH];
    struct stat st;
    int initial_added_count = *added_count;
    
    dir = opendir(dir_path);
    if (!dir) {
        log_error("Failed to open directory: %s (error: %s)", dir_path, strerror(errno));
        return false;
    }
    
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Construct full path
        snprintf(file_path, sizeof(file_path), "%s/%s", dir_path, entry->d_name);
        
        // Get file/directory info
        if (stat(file_path, &st) != 0) {
            log_error("Failed to stat file: %s (error: %s)", file_path, strerror(errno));
            continue;
        }
        
        // Only process regular files
        if (S_ISREG(st.st_mode)) {
            // Check if it's an MP4 file
            const char *ext = strrchr(entry->d_name, '.');
            if (ext && strcasecmp(ext, ".mp4") == 0) {
                (*processed_count)++;
                
                // Process the recording file
                if (process_recording_file(file_path)) {
                    (*added_count)++;
                }
                
                // Print progress every 10 files
                if (*processed_count % 10 == 0) {
                    printf("Processed %d files, added %d recordings\n", *processed_count, *added_count);
                }
            }
        }
    }
    
    closedir(dir);
    
    // Print summary for this directory if files were added
    if (*added_count > initial_added_count) {
        printf("Added %d recordings from %s\n", *added_count - initial_added_count, dir_path);
    }
    
    return true;
}

/**
 * Scan a directory and its subdirectories for MP4 files
 * 
 * @param base_dir Path to the base directory
 * @param processed_count Pointer to counter for processed files
 * @param added_count Pointer to counter for added files
 * @return true if the scan was successful, false otherwise
 */
static bool scan_directory(const char *base_dir, int *processed_count, int *added_count) {
    DIR *dir;
    struct dirent *entry;
    char path[MAX_PATH_LENGTH];
    struct stat st;
    
    // Process files in the base directory
    if (!process_directory(base_dir, processed_count, added_count)) {
        return false;
    }
    
    // Find and process subdirectories
    dir = opendir(base_dir);
    if (!dir) {
        log_error("Failed to open directory: %s (error: %s)", base_dir, strerror(errno));
        return false;
    }
    
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Construct full path
        snprintf(path, sizeof(path), "%s/%s", base_dir, entry->d_name);
        
        // Get file/directory info
        if (stat(path, &st) != 0) {
            log_error("Failed to stat file: %s (error: %s)", path, strerror(errno));
            continue;
        }
        
        // Process subdirectories
        if (S_ISDIR(st.st_mode)) {
            printf("Scanning subdirectory: %s\n", path);
            process_directory(path, processed_count, added_count);
        }
    }
    
    closedir(dir);
    return true;
}

/**
 * Main function
 */
int main(int argc, char *argv[]) {
    char storage_path[MAX_PATH_LENGTH];
    char mp4_path[MAX_PATH_LENGTH];
    int processed_count = 0;
    int added_count = 0;
    
    // Initialize logging
    init_logger();
    
    // Load configuration
    config_t config;
    if (load_config(&config) != 0) {
        log_error("Failed to load configuration");
        return 1;
    }
    
    // Parse command line arguments
    if (argc > 1) {
        strncpy(storage_path, argv[1], sizeof(storage_path) - 1);
    } else {
        // Use storage path from config
        strncpy(storage_path, config.storage_path, sizeof(storage_path) - 1);
    }
    
    printf("Using storage path: %s\n", storage_path);
    
    // Construct MP4 directory path
    snprintf(mp4_path, sizeof(mp4_path), "%s/mp4", storage_path);
    
    // Initialize database with the path from config
    const char *db_path = config.db_path;
    if (init_database(db_path) != 0) {
        log_error("Failed to initialize database");
        return 1;
    }
    
    // Initialize schema cache
    init_schema_cache();
    
    // Run schema migrations
    if (run_schema_migrations() != 0) {
        log_error("Failed to run schema migrations");
        shutdown_database();
        return 1;
    }
    
    printf("Scanning for recordings in %s\n", mp4_path);
    
    // Scan the MP4 directory
    if (!scan_directory(mp4_path, &processed_count, &added_count)) {
        log_error("Failed to scan directory: %s", mp4_path);
        shutdown_database();
        return 1;
    }
    
    printf("Scan complete. Processed %d files, added %d recordings to the database.\n",
           processed_count, added_count);
    
    // Shutdown database
    shutdown_database();
    
    return 0;
}
