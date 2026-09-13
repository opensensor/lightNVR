#ifndef LIGHTNVR_STORAGE_S3_H
#define LIGHTNVR_STORAGE_S3_H

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>
#include "database/db_storage_targets.h"

/* All transport work is bounded and performed outside database/capture locks.
 * Errors are normalized and never contain URLs, credentials or response bodies. */
typedef bool (*storage_transfer_cancel_fn)(void *context);
typedef struct {
    storage_transfer_cancel_fn cancelled;
    void *context;
    uint64_t bandwidth_bps;
    const char *job_uuid; /* Optional durable multipart journal owner. */
} storage_transfer_control_t;

enum { STORAGE_S3_OK = 0, STORAGE_S3_ERROR = -1,
       STORAGE_S3_MISSING = -2, STORAGE_S3_CANCELLED = -3,
       STORAGE_S3_CONFLICT = -4 };

bool storage_s3_validate(const storage_target_t *target, char *error, size_t length);
int storage_s3_probe(storage_target_t *target, bool write_test);
int storage_s3_read_range(const storage_target_t *target, const char *key,
                          uint64_t offset, size_t length, uint64_t total_size,
                          void *buffer, const storage_transfer_control_t *control,
                          char error[STORAGE_TARGET_ERROR_MAX]);
int storage_s3_stat(const storage_target_t *target, const char *key,
                    uint64_t *size, char error[STORAGE_TARGET_ERROR_MAX]);
int storage_s3_upload(const storage_target_t *target, const char *key,
                      const char *path, const char *checksum,
                      const storage_transfer_control_t *control,
                      char error[STORAGE_TARGET_ERROR_MAX]);
int storage_s3_download(const storage_target_t *target, const char *key,
                        const char *path, uint64_t expected_size,
                        const char *expected_checksum,
                        const storage_transfer_control_t *control,
                        char error[STORAGE_TARGET_ERROR_MAX]);
int storage_s3_verify(const storage_target_t *target, const char *key,
                      uint64_t size, const char *checksum,
                      const storage_transfer_control_t *control,
                      char error[STORAGE_TARGET_ERROR_MAX]);
int storage_s3_abort_upload(const storage_target_t *target, const char *key,
                            const char *upload_id, char error[STORAGE_TARGET_ERROR_MAX]);
int storage_s3_delete(const storage_target_t *target, const char *key,
                      char error[STORAGE_TARGET_ERROR_MAX]);

#endif
