/**
 * MP4 Recording Utilities
 * 
 * This file contains utility functions for the MP4 recording module.
 */

#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>
#include <sys/stat.h>
#include <stdbool.h>

#include "core/logger.h"
#include "video/mp4_recording.h"
#include "video/mp4_recording_internal.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

/**
 * Update MP4 recording metadata in the database
 * 
 * This function is called periodically to update the recording metadata
 * in the database.
 * 
 * @param stream_name Name of the stream
 */
void update_mp4_recording(const char *stream_name) {
    if (!stream_name || stream_name[0] == '\0') {
        return;
    }
    
    // Make a local copy of the stream name for thread safety
    char local_stream_name[MAX_STREAM_NAME];
    strncpy(local_stream_name, stream_name, MAX_STREAM_NAME - 1);
    local_stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // Get the MP4 writer for this stream
    mp4_writer_t *writer = get_mp4_writer_for_stream(local_stream_name);
    if (!writer) {
        return;
    }
    
    // Get the output path from the writer
    char output_path[MAX_PATH_LENGTH];
    if (writer->output_path && writer->output_path[0] != '\0') {
        strncpy(output_path, writer->output_path, MAX_PATH_LENGTH - 1);
        output_path[MAX_PATH_LENGTH - 1] = '\0';
    } else {
        return;
    }
    
    // Get the recording ID from the database
    recording_metadata_t metadata;
    memset(&metadata, 0, sizeof(metadata));
    
    // We need to find the recording by stream name and file path
    // Since we don't have a direct function to do this, we'll use the current time
    // as a filter and then check each recording
    time_t now = time(NULL);
    time_t one_day_ago = now - (24 * 60 * 60); // Look back one day
    
    // Get recordings for this stream in the last day
    int count = get_recording_metadata(one_day_ago, now, local_stream_name, &metadata, 10);
    
    // Find the recording with the matching file path
    bool found = false;
    for (int i = 0; i < count; i++) {
        if (strcmp(metadata.file_path, output_path) == 0) {
            found = true;
            break;
        }
        
        // If we didn't find it in this batch, get the next batch
        if (i == count - 1 && count == 10) {
            count = get_recording_metadata(one_day_ago, now, local_stream_name, &metadata, 10);
            i = -1; // Will be incremented to 0 in the next iteration
        }
    }
    
    // If we still didn't find it, try to get all recordings regardless of is_complete flag
    if (!found) {
        log_info("Trying to find recording with path %s regardless of is_complete flag", output_path);
        
        // We need to use a direct SQL query since our get_recording_metadata function filters by is_complete=1
        sqlite3 *db = get_db_handle();
        pthread_mutex_t *db_mutex = get_db_mutex();
        
        if (db && db_mutex) {
            pthread_mutex_lock(db_mutex);
            
            sqlite3_stmt *stmt;
            const char *sql = "SELECT id, stream_name, file_path, start_time, end_time, "
                              "size_bytes, width, height, fps, codec, is_complete "
                              "FROM recordings WHERE stream_name = ? AND file_path = ? "
                              "ORDER BY id DESC LIMIT 1";
            
            if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
                sqlite3_bind_text(stmt, 1, local_stream_name, -1, SQLITE_STATIC);
                sqlite3_bind_text(stmt, 2, output_path, -1, SQLITE_STATIC);
                
                if (sqlite3_step(stmt) == SQLITE_ROW) {
                    // Found the recording
                    metadata.id = (uint64_t)sqlite3_column_int64(stmt, 0);
                    
                    const char *stream = (const char *)sqlite3_column_text(stmt, 1);
                    if (stream) {
                        strncpy(metadata.stream_name, stream, sizeof(metadata.stream_name) - 1);
                        metadata.stream_name[sizeof(metadata.stream_name) - 1] = '\0';
                    }
                    
                    const char *path = (const char *)sqlite3_column_text(stmt, 2);
                    if (path) {
                        strncpy(metadata.file_path, path, sizeof(metadata.file_path) - 1);
                        metadata.file_path[sizeof(metadata.file_path) - 1] = '\0';
                    }
                    
                    metadata.start_time = (time_t)sqlite3_column_int64(stmt, 3);
                    
                    if (sqlite3_column_type(stmt, 4) != SQLITE_NULL) {
                        metadata.end_time = (time_t)sqlite3_column_int64(stmt, 4);
                    } else {
                        metadata.end_time = 0;
                    }
                    
                    metadata.size_bytes = (uint64_t)sqlite3_column_int64(stmt, 5);
                    metadata.width = sqlite3_column_int(stmt, 6);
                    metadata.height = sqlite3_column_int(stmt, 7);
                    metadata.fps = sqlite3_column_int(stmt, 8);
                    
                    const char *codec = (const char *)sqlite3_column_text(stmt, 9);
                    if (codec) {
                        strncpy(metadata.codec, codec, sizeof(metadata.codec) - 1);
                        metadata.codec[sizeof(metadata.codec) - 1] = '\0';
                    }
                    
                    metadata.is_complete = sqlite3_column_int(stmt, 10) != 0;
                    
                    found = true;
                    log_info("Found recording with ID %llu using direct SQL query", (unsigned long long)metadata.id);
                }
                
                sqlite3_finalize(stmt);
            }
            
            pthread_mutex_unlock(db_mutex);
        }
    }
    
    if (found) {
        // Found the recording, update its metadata
        // Get file size
        struct stat st;
        uint64_t size_bytes = 0;
        if (stat(output_path, &st) == 0) {
            size_bytes = st.st_size;
        }
        
        // Update with current time, file size, and is_complete=true
        // Make sure end_time is set to the current time
        update_recording_metadata(metadata.id, now, size_bytes, true);
        log_info("Updated recording metadata for %s (ID: %llu, Size: %llu bytes, End time: %ld)", 
                local_stream_name, (unsigned long long)metadata.id, (unsigned long long)size_bytes, (long)now);
    } else {
        log_warn("Could not find recording metadata for %s at %s", 
                local_stream_name, output_path);
        
        // Create a new recording entry if we couldn't find one
        memset(&metadata, 0, sizeof(recording_metadata_t));
        strncpy(metadata.stream_name, local_stream_name, sizeof(metadata.stream_name) - 1);
        strncpy(metadata.file_path, output_path, sizeof(metadata.file_path) - 1);
        metadata.start_time = now - 60; // Assume it started a minute ago
        metadata.end_time = now;
        
        // Get file size
        struct stat st;
        if (stat(output_path, &st) == 0) {
            metadata.size_bytes = st.st_size;
        }
        
        // Try to get stream info from the writer
        if (writer->output_ctx && writer->output_ctx->streams && writer->video_stream_idx >= 0) {
            AVStream *stream = writer->output_ctx->streams[writer->video_stream_idx];
            if (stream && stream->codecpar) {
                metadata.width = stream->codecpar->width;
                metadata.height = stream->codecpar->height;
                
                // Estimate FPS from timebase if available
                if (stream->avg_frame_rate.num > 0 && stream->avg_frame_rate.den > 0) {
                    metadata.fps = stream->avg_frame_rate.num / stream->avg_frame_rate.den;
                } else {
                    metadata.fps = 30; // Default assumption
                }
                
                // Get codec name
                const AVCodec *codec = avcodec_find_decoder(stream->codecpar->codec_id);
                if (codec) {
                    strncpy(metadata.codec, codec->name, sizeof(metadata.codec) - 1);
                } else {
                    strncpy(metadata.codec, "h264", sizeof(metadata.codec) - 1); // Default assumption
                }
            }
        }
        
        metadata.is_complete = true;
        
        // Add recording to database
        uint64_t recording_id = add_recording_metadata(&metadata);
        if (recording_id == 0) {
            log_error("Failed to add recording metadata for stream %s", local_stream_name);
        } else {
            log_info("Added new recording to database with ID: %llu for file: %s", 
                    (unsigned long long)recording_id, output_path);
        }
    }
}

// We'll use the pthread_join_with_timeout function from thread_utils.c
