#ifndef LIGHTNVR_WEB_AUDIT_SUMMARY_H
#define LIGHTNVR_WEB_AUDIT_SUMMARY_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/authorization.h"
#include "core/config.h"
#include "database/db_audit.h"

/*
 * Thread-safe, fixed-capacity accumulator for summarized authorization
 * decisions. Holds no database handle and builds no JSON: audit_log.c drains
 * entries and turns them into audit rows.
 */

#define AUDIT_SUMMARY_WINDOW_SECONDS 900
#define AUDIT_SUMMARY_DEFAULT_CAPACITY 1024
#define AUDIT_SUMMARY_METHOD_MAX 16
/* Must match http_request_t.path (MAX_PATH_LENGTH): the sample path is
 * copied straight from the request that first hits a key, and per-request
 * rows already store the full path at that width. */
#define AUDIT_SUMMARY_PATH_MAX MAX_PATH_LENGTH
#define AUDIT_SUMMARY_SOURCE_MAX 32

/* Hashing and equality are byte-wise (see key_hash()/find_slot_locked() in
 * audit_summary.c): always build keys with audit_summary_key_init() so
 * unused bytes -- including the reserved padding fillers below -- are zero.
 * Struct assignment (`entry.key = *key`) is not guaranteed by C to preserve
 * padding bytes, so any padding the compiler would otherwise insert between
 * members is made explicit here instead, sized for both this platform
 * (x86_64: int64_t/int alignment 8/4) and 32-bit ARM (armhf EABI: int64_t
 * alignment is also 8, so the layout matches). The _Static_assert below
 * requires the struct size to equal the sum of its members, so a new field
 * that introduces an implicit gap fails the build. */
typedef struct {
    int64_t principal_user_id;
    char principal_username[AUDIT_USERNAME_MAX];
    int action;
    char target_type[AUDIT_TARGET_TYPE_MAX];
    char target_uuid[AUDIT_QUERY_VALUE_MAX];
    char remote_address[AUDIT_REMOTE_ADDRESS_MAX];
    char reserved_before_window_start[4]; /* pads to window_start's 8-byte alignment */
    int64_t window_start;
} audit_summary_key_t;

#define AUDIT_SUMMARY_KEY_MEMBER_SIZE(m) sizeof(((audit_summary_key_t *)0)->m)
_Static_assert(sizeof(audit_summary_key_t) ==
                   AUDIT_SUMMARY_KEY_MEMBER_SIZE(principal_user_id) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(principal_username) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(action) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(target_type) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(target_uuid) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(remote_address) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(reserved_before_window_start) +
                       AUDIT_SUMMARY_KEY_MEMBER_SIZE(window_start),
               "audit_summary_key_t has implicit padding; "
               "add a reserved_ field for the gap");
#undef AUDIT_SUMMARY_KEY_MEMBER_SIZE

typedef struct {
    audit_summary_key_t key;
    uint64_t count;
    int64_t first_at;
    int64_t last_at;
    /* Credential metadata belongs to the first sample, not the grouping key. */
    char auth_method[AUDIT_AUTH_METHOD_MAX];
    char api_token_uuid[AUDIT_EVENT_UUID_MAX];
    char request_id[AUDIT_REQUEST_ID_MAX];
    char method[AUDIT_SUMMARY_METHOD_MAX];
    char path[AUDIT_SUMMARY_PATH_MAX];
    char decision_source[AUDIT_SUMMARY_SOURCE_MAX];
    char explanation[AUTHORIZATION_EXPLANATION_MAX];
} audit_summary_entry_t;

typedef struct {
    const char *auth_method;
    const char *api_token_uuid;
    const char *request_id;
    const char *method;
    const char *path;
    const char *decision_source;
    const char *explanation;
} audit_summary_sample_t;

typedef enum {
    AUDIT_SUMMARY_ADDED = 0,
    AUDIT_SUMMARY_FULL = 1,
    AUDIT_SUMMARY_UNAVAILABLE = 2
} audit_summary_add_result_t;

int64_t audit_summary_window_start(int64_t now);
void audit_summary_key_init(audit_summary_key_t *key);
int audit_summary_init(size_t capacity);
void audit_summary_shutdown(void);
size_t audit_summary_pending(void);
audit_summary_add_result_t audit_summary_add(const audit_summary_key_t *key,
                                             int64_t now,
                                             const audit_summary_sample_t *sample);
size_t audit_summary_drain(bool all, int64_t current_window_start,
                           audit_summary_entry_t *out, size_t out_capacity);

#endif /* LIGHTNVR_WEB_AUDIT_SUMMARY_H */
