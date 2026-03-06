#ifndef LIGHTNVR_DB_RECORDING_TAGS_H
#define LIGHTNVR_DB_RECORDING_TAGS_H

#include <stdint.h>
#include <stdbool.h>

#define MAX_RECORDING_TAGS 64
#define MAX_TAG_LENGTH 128

/**
 * Add a tag to a recording.
 * Uses INSERT OR IGNORE to avoid duplicates.
 *
 * @param recording_id Recording ID
 * @param tag          Tag string (will be trimmed)
 * @return 0 on success, -1 on error
 */
int db_recording_tag_add(uint64_t recording_id, const char *tag);

/**
 * Remove a tag from a recording.
 *
 * @param recording_id Recording ID
 * @param tag          Tag string
 * @return 0 on success (or tag didn't exist), -1 on error
 */
int db_recording_tag_remove(uint64_t recording_id, const char *tag);

/**
 * Get all tags for a recording.
 *
 * @param recording_id Recording ID
 * @param tags         Output array of tag strings (caller provides buffer)
 * @param max_tags     Max number of tags to return
 * @return Number of tags found, or -1 on error
 */
int db_recording_tag_get(uint64_t recording_id, char tags[][MAX_TAG_LENGTH], int max_tags);

/**
 * Set the complete list of tags for a recording (replace all existing).
 *
 * @param recording_id Recording ID
 * @param tags         Array of tag strings
 * @param tag_count    Number of tags
 * @return 0 on success, -1 on error
 */
int db_recording_tag_set(uint64_t recording_id, const char **tags, int tag_count);

/**
 * Get all unique tags across all recordings.
 *
 * @param tags     Output array of tag strings (caller provides buffer)
 * @param max_tags Max number of tags to return
 * @return Number of unique tags found, or -1 on error
 */
int db_recording_tag_get_all_unique(char tags[][MAX_TAG_LENGTH], int max_tags);

/**
 * Add a tag to multiple recordings at once.
 *
 * @param recording_ids Array of recording IDs
 * @param count         Number of recording IDs
 * @param tag           Tag string
 * @return Number of recordings successfully tagged, or -1 on error
 */
int db_recording_tag_batch_add(const uint64_t *recording_ids, int count, const char *tag);

/**
 * Remove a tag from multiple recordings at once.
 *
 * @param recording_ids Array of recording IDs
 * @param count         Number of recording IDs
 * @param tag           Tag string
 * @return Number of recordings successfully untagged, or -1 on error
 */
int db_recording_tag_batch_remove(const uint64_t *recording_ids, int count, const char *tag);

/**
 * Get recording IDs that have a specific tag.
 *
 * @param tag            Tag to search for
 * @param recording_ids  Output array of recording IDs
 * @param max_ids        Max number of IDs to return
 * @return Number of recording IDs found, or -1 on error
 */
int db_recording_tag_get_recordings_by_tag(const char *tag, uint64_t *recording_ids, int max_ids);

#endif // LIGHTNVR_DB_RECORDING_TAGS_H

