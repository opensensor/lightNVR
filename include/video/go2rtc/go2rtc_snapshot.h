/**
 * @file go2rtc_snapshot.h
 * @brief API for getting snapshots from go2rtc
 */

#ifndef GO2RTC_SNAPSHOT_H
#define GO2RTC_SNAPSHOT_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @brief Get a JPEG snapshot from go2rtc for a stream
 * 
 * @param stream_name Name of the stream to get snapshot from
 * @param jpeg_data Pointer to buffer that will receive the JPEG data (will be allocated)
 * @param jpeg_size Pointer to size_t that will receive the size of the JPEG data
 * @return true if successful, false otherwise
 * 
 * Note: The caller is responsible for freeing the jpeg_data buffer when done
 */
bool go2rtc_get_snapshot(const char *stream_name, unsigned char **jpeg_data, size_t *jpeg_size);

#endif /* GO2RTC_SNAPSHOT_H */

