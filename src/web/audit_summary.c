#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <stdlib.h>
#include <string.h>

#include "utils/strings.h"
#include "web/audit_summary.h"

typedef struct {
    bool used;
    audit_summary_entry_t entry;
} summary_slot_t;

static pthread_mutex_t summary_mutex = PTHREAD_MUTEX_INITIALIZER;
static summary_slot_t *slots = NULL;
static summary_slot_t *scratch = NULL; /* reused by drain; never per request */
static size_t slot_capacity = 0;
static size_t used_count = 0;

int64_t audit_summary_window_start(int64_t now) {
    if (now < 0) return 0;
    return now - (now % AUDIT_SUMMARY_WINDOW_SECONDS);
}

void audit_summary_key_init(audit_summary_key_t *key) {
    if (key) memset(key, 0, sizeof(*key));
}

static uint64_t key_hash(const audit_summary_key_t *key) {
    const unsigned char *bytes = (const unsigned char *)key;
    uint64_t hash = 1469598103934665603ULL;
    for (size_t i = 0; i < sizeof(*key); i++) {
        hash ^= bytes[i];
        hash *= 1099511628211ULL;
    }
    return hash;
}

/* Returns the matching slot (*found = true), the first free slot on the probe
 * path (*found = false), or NULL when the table has neither. */
static summary_slot_t *find_slot_locked(const audit_summary_key_t *key, bool *found) {
    *found = false;
    size_t start = (size_t)(key_hash(key) % slot_capacity);
    for (size_t probe = 0; probe < slot_capacity; probe++) {
        summary_slot_t *slot = &slots[(start + probe) % slot_capacity];
        if (!slot->used) return slot;
        if (memcmp(&slot->entry.key, key, sizeof(*key)) == 0) {
            *found = true;
            return slot;
        }
    }
    return NULL;
}

int audit_summary_init(size_t capacity) {
    if (capacity == 0) return -1;
    summary_slot_t *new_slots = calloc(capacity, sizeof(*new_slots));
    summary_slot_t *new_scratch = calloc(capacity, sizeof(*new_scratch));
    if (!new_slots || !new_scratch) {
        free(new_slots);
        free(new_scratch);
        return -1;
    }
    pthread_mutex_lock(&summary_mutex);
    free(slots);
    free(scratch);
    slots = new_slots;
    scratch = new_scratch;
    slot_capacity = capacity;
    used_count = 0;
    pthread_mutex_unlock(&summary_mutex);
    return 0;
}

void audit_summary_shutdown(void) {
    pthread_mutex_lock(&summary_mutex);
    free(slots);
    free(scratch);
    slots = NULL;
    scratch = NULL;
    slot_capacity = 0;
    used_count = 0;
    pthread_mutex_unlock(&summary_mutex);
}

size_t audit_summary_pending(void) {
    pthread_mutex_lock(&summary_mutex);
    size_t pending = used_count;
    pthread_mutex_unlock(&summary_mutex);
    return pending;
}

static void copy_sample_field(char *dst, size_t size, const char *src) {
    safe_strcpy(dst, src ? src : "", size, 0);
}

audit_summary_add_result_t audit_summary_add(const audit_summary_key_t *key,
                                             int64_t now,
                                             const audit_summary_sample_t *sample) {
    if (!key) return AUDIT_SUMMARY_UNAVAILABLE;
    pthread_mutex_lock(&summary_mutex);
    if (!slots) {
        pthread_mutex_unlock(&summary_mutex);
        return AUDIT_SUMMARY_UNAVAILABLE;
    }
    bool found = false;
    summary_slot_t *slot = find_slot_locked(key, &found);
    if (found) {
        slot->entry.count++;
        if (now > slot->entry.last_at) slot->entry.last_at = now;
        pthread_mutex_unlock(&summary_mutex);
        return AUDIT_SUMMARY_ADDED;
    }
    if (!slot) {
        pthread_mutex_unlock(&summary_mutex);
        return AUDIT_SUMMARY_FULL;
    }
    memset(slot, 0, sizeof(*slot));
    slot->used = true;
    slot->entry.key = *key;
    slot->entry.count = 1;
    slot->entry.first_at = now;
    slot->entry.last_at = now;
    if (sample) {
        copy_sample_field(slot->entry.auth_method, sizeof(slot->entry.auth_method), sample->auth_method);
        copy_sample_field(slot->entry.api_token_uuid, sizeof(slot->entry.api_token_uuid), sample->api_token_uuid);
        copy_sample_field(slot->entry.request_id, sizeof(slot->entry.request_id), sample->request_id);
        copy_sample_field(slot->entry.method, sizeof(slot->entry.method), sample->method);
        copy_sample_field(slot->entry.path, sizeof(slot->entry.path), sample->path);
        copy_sample_field(slot->entry.decision_source, sizeof(slot->entry.decision_source), sample->decision_source);
        copy_sample_field(slot->entry.explanation, sizeof(slot->entry.explanation), sample->explanation);
    }
    used_count++;
    pthread_mutex_unlock(&summary_mutex);
    return AUDIT_SUMMARY_ADDED;
}

size_t audit_summary_drain(bool all, int64_t current_window_start,
                           audit_summary_entry_t *out, size_t out_capacity) {
    pthread_mutex_lock(&summary_mutex);
    if (!slots || !out || out_capacity == 0) {
        pthread_mutex_unlock(&summary_mutex);
        return 0;
    }
    if (used_count == 0) {
        /* Nothing to take: skip the memset + rebuild below. */
        pthread_mutex_unlock(&summary_mutex);
        return 0;
    }
    size_t drained = 0;
    size_t kept = 0;
    for (size_t i = 0; i < slot_capacity; i++) {
        if (!slots[i].used) continue;
        bool take = all || slots[i].entry.key.window_start != current_window_start;
        if (take && drained < out_capacity) {
            out[drained++] = slots[i].entry;
        } else {
            scratch[kept++] = slots[i];
        }
    }
    /* Open addressing has no cheap delete: rebuild from the kept entries. */
    memset(slots, 0, slot_capacity * sizeof(*slots));
    used_count = 0;
    for (size_t i = 0; i < kept; i++) {
        bool found = false;
        summary_slot_t *slot = find_slot_locked(&scratch[i].entry.key, &found);
        *slot = scratch[i];
        used_count++;
    }
    pthread_mutex_unlock(&summary_mutex);
    return drained;
}
