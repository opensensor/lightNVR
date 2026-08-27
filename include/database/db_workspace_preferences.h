#ifndef LIGHTNVR_DB_WORKSPACE_PREFERENCES_H
#define LIGHTNVR_DB_WORKSPACE_PREFERENCES_H

#include <stdbool.h>
#include <stdint.h>

#define WORKSPACE_KEY_LIVE_NAVIGATOR "live.navigator"
#define WORKSPACE_KEY_INVESTIGATION "investigation"

typedef struct {
    bool live_navigator_visible;
    bool investigation_visible;
} workspace_preferences_t;

/* Missing rows resolve to the product defaults. owner_user_id 0 represents
 * the local auth-disabled installation preference. */
void db_workspace_preferences_defaults(workspace_preferences_t *preferences);
int db_workspace_preferences_get(int64_t owner_user_id,
                                 workspace_preferences_t *preferences);
int db_workspace_preferences_replace(
    int64_t owner_user_id, const workspace_preferences_t *preferences);

#endif /* LIGHTNVR_DB_WORKSPACE_PREFERENCES_H */
