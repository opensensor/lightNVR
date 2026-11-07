/* SPDX-License-Identifier: GPL-3.0-or-later */
#ifndef LIGHTNVR_DB_AUTHORIZATION_SNAPSHOT_H
#define LIGHTNVR_DB_AUTHORIZATION_SNAPSHOT_H

#include "core/authorization.h"

/* Request/thread-owned coherent SQLite policy observation and bounded runtime
 * samples. No global writer lock is held. Not atomic across external devices. */
typedef struct authorization_snapshot authorization_snapshot_t;

/* 0 success, -2 authenticated identity no longer valid, -1 internal failure.
 * On failure *out is NULL. No password hashes, API keys or TOTP secrets loaded. */
int authorization_snapshot_create(const user_t *principal,
                                 authorization_snapshot_t **out);
void authorization_snapshot_free(authorization_snapshot_t *snapshot);
/* Existing identity/self-token routes exclude scoped credentials, independent
 * of camera grants. No identity or credential values leave the snapshot. */
bool authorization_snapshot_allows_non_scoped_identity(
    const authorization_snapshot_t *snapshot);
/* Setup state from the same SQLite snapshot: 1 complete, 0 open, -1 failure. */
int authorization_snapshot_setup_complete(const authorization_snapshot_t *snapshot);
int authorization_snapshot_camera_count(const authorization_snapshot_t *snapshot);
/* camera_index -1 selects a global action. resource is raw/runtime/live.
 * The same index always refers to the same camera across resource variants. */
int authorization_snapshot_evaluate(authorization_snapshot_t *snapshot,
    authorization_action_t action, int camera_index, const char *resource,
    authorization_evaluation_t *evaluation);

#endif
