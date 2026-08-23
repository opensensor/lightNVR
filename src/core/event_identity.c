#define _POSIX_C_SOURCE 200809L

#include "core/event_identity.h"

#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "core/logger.h"
#include "database/db_system_settings.h"
#include "utils/uuid.h"

#define EVENT_INSTALLATION_UUID_KEY "event_installation_uuid"

static pthread_mutex_t identity_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool identity_initialized = false;
static char installation_source[EVENT_SOURCE_MAX];

int event_identity_init(void) {
    pthread_mutex_lock(&identity_mutex);
    if (identity_initialized) {
        pthread_mutex_unlock(&identity_mutex);
        return 0;
    }

    char uuid[LIGHTNVR_UUID_STRING_SIZE] = {0};
    int read_result = db_get_system_setting(
        EVENT_INSTALLATION_UUID_KEY, uuid, sizeof(uuid));
    if (read_result != 0 || !lightnvr_uuid_is_valid(uuid)) {
        if (read_result == 0) {
            log_warn("Replacing invalid persisted event installation UUID");
        }
        if (lightnvr_uuid_generate_v4(uuid) != 0 ||
            db_set_system_setting(EVENT_INSTALLATION_UUID_KEY, uuid) != 0) {
            log_error("Failed to create persistent event installation identity");
            pthread_mutex_unlock(&identity_mutex);
            return -1;
        }
    }

    int written = snprintf(installation_source, sizeof(installation_source),
                           "urn:lightnvr:%s", uuid);
    if (written < 0 || (size_t)written >= sizeof(installation_source)) {
        installation_source[0] = '\0';
        pthread_mutex_unlock(&identity_mutex);
        return -1;
    }
    identity_initialized = true;
    pthread_mutex_unlock(&identity_mutex);
    return 0;
}

int event_identity_get_source(char *output, size_t output_size) {
    if (!output || output_size == 0) return -1;
    pthread_mutex_lock(&identity_mutex);
    if (!identity_initialized || installation_source[0] == '\0' ||
        strlen(installation_source) >= output_size) {
        output[0] = '\0';
        pthread_mutex_unlock(&identity_mutex);
        return -1;
    }
    snprintf(output, output_size, "%s", installation_source);
    pthread_mutex_unlock(&identity_mutex);
    return 0;
}

void event_identity_shutdown(void) {
    pthread_mutex_lock(&identity_mutex);
    memset(installation_source, 0, sizeof(installation_source));
    identity_initialized = false;
    pthread_mutex_unlock(&identity_mutex);
}
