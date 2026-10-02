#ifndef LIGHTNVR_DB_UI_PREFERENCES_H
#define LIGHTNVR_DB_UI_PREFERENCES_H

#include <stdint.h>

#include "core/config.h"

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    int64_t created_at;
} user_favorite_t;

int db_user_ui_mode_get(int64_t user_id, char *mode, unsigned int mode_size);
int db_user_ui_mode_set(int64_t user_id, const char *mode);
int db_user_favorites_list(int64_t user_id, user_favorite_t *favorites,
                           int max_count);
int db_user_favorite_add(int64_t user_id, const char *camera_uuid,
                         int64_t *created_at);
int db_user_favorite_delete(int64_t user_id, const char *camera_uuid);

#endif
