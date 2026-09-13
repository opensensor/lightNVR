#include "web/recording_source.h"
#include "database/db_core.h"
#include "storage/storage_source.h"

void recording_source_add_status(cJSON *object, uint64_t id) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!object || !db || !mutex) return;
    bool available = storage_source_available(id);
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    const char *sql = "SELECT r.deletion_pending,COALESCE(t.target_type,'filesystem'),"
        "COALESCE(t.health_status,'unknown'),COALESCE(r.storage_target_uuid,''),"
        "1+(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id),"
        "EXISTS(SELECT 1 FROM storage_recording_copies c JOIN storage_targets a ON a.uuid=c.target_uuid "
        "WHERE c.recording_id=r.id AND a.target_type='s3'),"
        "EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id "
        "AND j.state NOT IN('completed','failed','cancelled')),"
        "COALESCE((SELECT state FROM storage_retrieval_jobs WHERE recording_id=r.id),''),"
        "COALESCE(r.storage_policy_uuid,''),COALESCE(r.storage_policy_version,0),r.retention_override_days "
        "FROM recordings r LEFT JOIN storage_targets t ON t.uuid=r.storage_target_uuid WHERE r.id=?;";
    if (sqlite3_prepare_v2(db, sql, -1, &statement, NULL) == SQLITE_OK) {
        sqlite3_bind_int64(statement, 1, (sqlite3_int64)id);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            bool pending = sqlite3_column_int(statement, 0) != 0;
            const char *type = (const char *)sqlite3_column_text(statement, 1);
            const char *retrieval = (const char *)sqlite3_column_text(statement, 7);
            const char *state = pending ? "deletion_pending" :
                (!strcmp(retrieval, "queued") || !strcmp(retrieval, "fetching")) ? "preparing" :
                !available ? "unavailable" :
                sqlite3_column_int(statement, 6) ? "archiving" :
                !strcmp(type, "s3") ? "archived" :
                sqlite3_column_int(statement, 5) ? "hot_and_archive" : "hot";
            cJSON_AddStringToObject(object, "storage_state", state);
            cJSON_AddBoolToObject(object, "deletion_pending", pending);
            cJSON_AddBoolToObject(object, "external_source", !strcmp(type, "s3"));
            cJSON_AddStringToObject(object, "storage_target_uuid", (const char *)sqlite3_column_text(statement, 3));
            cJSON_AddNumberToObject(object, "durable_copy_count", sqlite3_column_int(statement, 4));
            cJSON_AddStringToObject(object, "storage_policy_uuid", (const char *)sqlite3_column_text(statement, 8));
            cJSON_AddNumberToObject(object, "storage_policy_version", (double)sqlite3_column_int64(statement, 9));
            int retention = sqlite3_column_type(statement, 10) == SQLITE_NULL ? -1 : sqlite3_column_int(statement, 10);
            cJSON_AddStringToObject(object, "retention_mode", retention < 0 ? "inherit" : (retention == 0 ? "indefinite" : "finite"));
        }
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
}
