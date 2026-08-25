#define _POSIX_C_SOURCE 200809L

#include "database/db_onvif_discovery_inventory.h"

#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "database/db_core.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define INVENTORY_SELECT_FIELDS \
    "i.uuid,i.endpoint,i.device_service,i.media_service,i.ptz_service," \
    "i.imaging_service,i.manufacturer,i.model,i.firmware_version," \
    "i.serial_number,i.hardware_id,i.ip_address,i.mac_address," \
    "i.first_seen_at,i.last_seen_at,i.last_scan_network,i.online," \
    "i.claim_state,COALESCE(i.claimed_camera_uuid,'')," \
    "i.duplicate_suspected "

static void copy_column(char *destination, size_t size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", size, 0);
}

static void populate(sqlite3_stmt *statement, onvif_device_info_t *device) {
    memset(device, 0, sizeof(*device));
    copy_column(device->inventory_uuid, sizeof(device->inventory_uuid),
                statement, 0);
    copy_column(device->endpoint, sizeof(device->endpoint), statement, 1);
    copy_column(device->device_service, sizeof(device->device_service),
                statement, 2);
    copy_column(device->media_service, sizeof(device->media_service),
                statement, 3);
    copy_column(device->ptz_service, sizeof(device->ptz_service), statement, 4);
    copy_column(device->imaging_service, sizeof(device->imaging_service),
                statement, 5);
    copy_column(device->manufacturer, sizeof(device->manufacturer),
                statement, 6);
    copy_column(device->model, sizeof(device->model), statement, 7);
    copy_column(device->firmware_version, sizeof(device->firmware_version),
                statement, 8);
    copy_column(device->serial_number, sizeof(device->serial_number),
                statement, 9);
    copy_column(device->hardware_id, sizeof(device->hardware_id),
                statement, 10);
    copy_column(device->ip_address, sizeof(device->ip_address), statement, 11);
    copy_column(device->mac_address, sizeof(device->mac_address), statement, 12);
    device->first_seen_at = (time_t)sqlite3_column_int64(statement, 13);
    device->last_seen_at = (time_t)sqlite3_column_int64(statement, 14);
    device->discovery_time = device->last_seen_at;
    copy_column(device->last_scan_network, sizeof(device->last_scan_network),
                statement, 15);
    device->online = sqlite3_column_int(statement, 16) != 0;
    copy_column(device->claim_state, sizeof(device->claim_state), statement, 17);
    copy_column(device->claimed_camera_uuid,
                sizeof(device->claimed_camera_uuid), statement, 18);
    device->duplicate_suspected = sqlite3_column_int(statement, 19) != 0;
}

static bool valid_text(const char *value, size_t maximum, bool required) {
    if (!value) return false;
    size_t length = strnlen(value, maximum);
    if (length == maximum || (required && length == 0)) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static const char *device_endpoint(const onvif_device_info_t *device) {
    if (device->endpoint[0]) return device->endpoint;
    return device->device_service[0] ? device->device_service : NULL;
}

static int find_uuid_locked(sqlite3 *db, const char *column,
                            const char *value,
                            char uuid[CAMERA_UUID_STRING_SIZE]) {
    if (!value || !value[0]) return 0;
    char sql[192];
    int written = snprintf(sql, sizeof(sql),
                           "SELECT uuid FROM onvif_discovery_inventory "
                           "WHERE %s=? ORDER BY last_seen_at DESC LIMIT 1;",
                           column);
    if (written < 0 || (size_t)written >= sizeof(sql)) return -1;
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, value, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int found = 0;
    if (result == SQLITE_ROW) {
        copy_column(uuid, CAMERA_UUID_STRING_SIZE, statement, 0);
        found = 1;
    } else if (result != SQLITE_DONE) {
        found = -1;
    }
    if (statement) sqlite3_finalize(statement);
    return found;
}

static int get_locked(sqlite3 *db, const char *uuid,
                      onvif_device_info_t *device) {
    const char *sql = "SELECT " INVENTORY_SELECT_FIELDS
        "FROM onvif_discovery_inventory i WHERE i.uuid=? LIMIT 1;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int found = 0;
    if (result == SQLITE_ROW) {
        populate(statement, device);
        found = 1;
    } else if (result != SQLITE_DONE) {
        found = -1;
    }
    if (statement) sqlite3_finalize(statement);
    return found;
}

static int load_stable_identity_locked(
    sqlite3 *db, const char *uuid, char serial[64], char mac[32]) {
    const char *sql = "SELECT serial_number,mac_address "
                      "FROM onvif_discovery_inventory WHERE uuid=?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result == SQLITE_ROW) {
        copy_column(serial, 64, statement, 0);
        copy_column(mac, 32, statement, 1);
        result = SQLITE_OK;
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_OK ? 0 : -1;
}

static int mark_duplicate_locked(sqlite3 *db, const char *uuid) {
    if (!uuid || !uuid[0]) return 0;
    const char *sql = "UPDATE onvif_discovery_inventory SET "
        "duplicate_suspected=1,revision=revision+1,"
        "updated_at=strftime('%s','now') WHERE uuid=?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static int store_address_locked(sqlite3 *db, const char *uuid,
                                const char *type, const char *address,
                                int64_t observed_at) {
    if (!address || !address[0]) return 0;
    const char *sql =
        "INSERT INTO onvif_discovery_addresses("
        "inventory_uuid,address_type,address,first_seen_at,last_seen_at) "
        "VALUES(?,?,?,?,?) ON CONFLICT(inventory_uuid,address_type,address) "
        "DO UPDATE SET last_seen_at=excluded.last_seen_at;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, type, -1, SQLITE_STATIC);
        sqlite3_bind_text(statement, 3, address, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 4, observed_at);
        sqlite3_bind_int64(statement, 5, observed_at);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static int update_locked(sqlite3 *db, const char *uuid,
                         const onvif_device_info_t *device,
                         const char *endpoint, const char *network,
                         int64_t observed_at, bool duplicate) {
    const char *sql =
        "UPDATE onvif_discovery_inventory SET endpoint=?,device_service=?,"
        "media_service=COALESCE(NULLIF(?,''),media_service),"
        "ptz_service=COALESCE(NULLIF(?,''),ptz_service),"
        "imaging_service=COALESCE(NULLIF(?,''),imaging_service),"
        "manufacturer=COALESCE(NULLIF(?,''),manufacturer),"
        "model=COALESCE(NULLIF(?,''),model),"
        "firmware_version=COALESCE(NULLIF(?,''),firmware_version),"
        "serial_number=COALESCE(NULLIF(?,''),serial_number),"
        "hardware_id=COALESCE(NULLIF(?,''),hardware_id),"
        "ip_address=COALESCE(NULLIF(?,''),ip_address),"
        "mac_address=COALESCE(NULLIF(?,''),mac_address),"
        "last_seen_at=?,last_scan_network=?,online=1,"
        "duplicate_suspected=MAX(duplicate_suspected,?),"
        "revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, endpoint, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2,
                          device->device_service[0]
                              ? device->device_service : endpoint,
                          -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, device->media_service, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, device->ptz_service, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, device->imaging_service, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, device->manufacturer, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, device->model, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, device->firmware_version, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, device->serial_number, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 10, device->hardware_id, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 11, device->ip_address, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 12, device->mac_address, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 13, observed_at);
        sqlite3_bind_text(statement, 14, network, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 15, duplicate ? 1 : 0);
        sqlite3_bind_text(statement, 16, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE && sqlite3_changes(db) == 1 ? 0 : -1;
}

static int insert_locked(sqlite3 *db, const char *uuid,
                         const onvif_device_info_t *device,
                         const char *endpoint, const char *network,
                         int64_t observed_at, bool duplicate) {
    const char *sql =
        "INSERT INTO onvif_discovery_inventory("
        "uuid,endpoint,device_service,media_service,ptz_service,imaging_service,"
        "manufacturer,model,firmware_version,serial_number,hardware_id,"
        "ip_address,mac_address,first_seen_at,last_seen_at,last_scan_network,"
        "online,duplicate_suspected) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,1,?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, endpoint, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3,
                          device->device_service[0]
                              ? device->device_service : endpoint,
                          -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, device->media_service, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 5, device->ptz_service, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, device->imaging_service, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 7, device->manufacturer, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 8, device->model, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, device->firmware_version, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 10, device->serial_number, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 11, device->hardware_id, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 12, device->ip_address, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 13, device->mac_address, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 14, observed_at);
        sqlite3_bind_int64(statement, 15, observed_at);
        sqlite3_bind_text(statement, 16, network, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 17, duplicate ? 1 : 0);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static int record_device_locked(sqlite3 *db, const char *network,
                                onvif_device_info_t *device,
                                int64_t observed_at) {
    const char *endpoint = device_endpoint(device);
    if (!endpoint || !valid_text(endpoint, MAX_URL_LENGTH, true) ||
        !valid_text(device->device_service, MAX_URL_LENGTH, false) ||
        !valid_text(device->serial_number, sizeof(device->serial_number), false) ||
        !valid_text(device->mac_address, sizeof(device->mac_address), false)) {
        return -1;
    }

    char serial_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    char mac_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    char endpoint_uuid[CAMERA_UUID_STRING_SIZE] = {0};
    int serial_found = find_uuid_locked(db, "serial_number",
                                        device->serial_number, serial_uuid);
    int mac_found = find_uuid_locked(db, "mac_address",
                                     device->mac_address, mac_uuid);
    int endpoint_found = find_uuid_locked(db, "endpoint", endpoint,
                                          endpoint_uuid);
    if (serial_found < 0 || mac_found < 0 || endpoint_found < 0) return -1;

    bool duplicate = false;
    char uuid[CAMERA_UUID_STRING_SIZE] = {0};
    if (serial_found) safe_strcpy(uuid, serial_uuid, sizeof(uuid), 0);
    else if (mac_found) safe_strcpy(uuid, mac_uuid, sizeof(uuid), 0);

    if (serial_found && mac_found && strcmp(serial_uuid, mac_uuid) != 0) {
        duplicate = true;
        if (mark_duplicate_locked(db, serial_uuid) != 0 ||
            mark_duplicate_locked(db, mac_uuid) != 0) return -1;
    }

    if (!uuid[0] && endpoint_found) {
        char existing_serial[64] = {0};
        char existing_mac[32] = {0};
        if (load_stable_identity_locked(db, endpoint_uuid, existing_serial,
                                        existing_mac) != 0) return -1;
        bool serial_conflict = device->serial_number[0] && existing_serial[0] &&
            strcmp(device->serial_number, existing_serial) != 0;
        bool mac_conflict = device->mac_address[0] && existing_mac[0] &&
            strcmp(device->mac_address, existing_mac) != 0;
        if (serial_conflict || mac_conflict) {
            duplicate = true;
            if (mark_duplicate_locked(db, endpoint_uuid) != 0) return -1;
        } else {
            safe_strcpy(uuid, endpoint_uuid, sizeof(uuid), 0);
        }
    }

    if (uuid[0] && endpoint_found && strcmp(uuid, endpoint_uuid) != 0) {
        duplicate = true;
        if (mark_duplicate_locked(db, uuid) != 0 ||
            mark_duplicate_locked(db, endpoint_uuid) != 0) return -1;
    }

    if (!uuid[0] && lightnvr_uuid_generate_v4(uuid) != 0) return -1;
    int result = get_locked(db, uuid, &(onvif_device_info_t){0});
    if (result < 0) return -1;
    if (result == 1) {
        if (update_locked(db, uuid, device, endpoint, network, observed_at,
                          duplicate) != 0) return -1;
    } else if (insert_locked(db, uuid, device, endpoint, network, observed_at,
                             duplicate) != 0) {
        return -1;
    }

    if (store_address_locked(db, uuid, "endpoint", endpoint, observed_at) != 0 ||
        store_address_locked(db, uuid, "service", device->device_service,
                             observed_at) != 0 ||
        store_address_locked(db, uuid, "ip", device->ip_address,
                             observed_at) != 0) return -1;
    return get_locked(db, uuid, device) == 1 ? 0 : -1;
}

int db_onvif_inventory_record_scan(
    const char *scan_network, onvif_device_info_t *devices, int count,
    int64_t observed_at) {
    const char *network = scan_network && scan_network[0]
        ? scan_network : "auto";
    if (count < 0 || count > ONVIF_DISCOVERY_INVENTORY_MAX ||
        (count > 0 && !devices) || observed_at <= 0 ||
        !valid_text(network, 64, true)) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;

    pthread_mutex_lock(mutex);
    int result = sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL);
    sqlite3_stmt *statement = NULL;
    if (result == SQLITE_OK) {
        const char *offline_sql =
            "UPDATE onvif_discovery_inventory SET online=0,"
            "revision=revision+1,updated_at=strftime('%s','now') "
            "WHERE last_scan_network=? AND online=1;";
        result = sqlite3_prepare_v2(db, offline_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, network, -1, SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
    }
    for (int index = 0; result == SQLITE_DONE && index < count; index++) {
        result = record_device_locked(db, network, &devices[index], observed_at)
            == 0 ? SQLITE_DONE : SQLITE_ERROR;
    }
    int final_result = result == SQLITE_DONE
        ? sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL)
        : sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE && final_result == SQLITE_OK ? count : -1;
}

int db_onvif_inventory_list(onvif_device_info_t *devices, int max_count) {
    if (!devices || max_count < 1 ||
        max_count > ONVIF_DISCOVERY_INVENTORY_MAX) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = "SELECT " INVENTORY_SELECT_FIELDS
        "FROM onvif_discovery_inventory i ORDER BY i.online DESC,"
        "CASE i.claim_state WHEN 'unclaimed' THEN 0 WHEN 'claimed' THEN 1 "
        "ELSE 2 END,i.last_seen_at DESC,i.uuid LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) sqlite3_bind_int(statement, 1, max_count);
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            populate(statement, &devices[count++]);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_onvif_inventory_list_addresses(
    const char *inventory_uuid, char addresses[][MAX_URL_LENGTH],
    int max_count) {
    if (!lightnvr_uuid_is_valid(inventory_uuid) || !addresses ||
        max_count < 1 || max_count > ONVIF_DISCOVERY_ADDRESS_MAX) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = "SELECT address FROM onvif_discovery_addresses "
        "WHERE inventory_uuid=? ORDER BY last_seen_at DESC,address LIMIT ?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, inventory_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 2, max_count);
    }
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            copy_column(addresses[count++], MAX_URL_LENGTH, statement, 0);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_onvif_inventory_result_t db_onvif_inventory_claim_stream(
    const char *inventory_uuid, const char *stream_name) {
    if (!lightnvr_uuid_is_valid(inventory_uuid) ||
        !valid_text(stream_name, MAX_STREAM_NAME, true)) {
        return DB_ONVIF_INVENTORY_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_ONVIF_INVENTORY_ERROR;
    const char *sql =
        "UPDATE onvif_discovery_inventory SET claim_state='claimed',"
        "claimed_camera_uuid=(SELECT camera_uuid FROM streams WHERE name=?),"
        "revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? "
        "AND EXISTS(SELECT 1 FROM streams WHERE name=?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, stream_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, inventory_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, stream_name, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE) return DB_ONVIF_INVENTORY_ERROR;
    return changed == 1 ? DB_ONVIF_INVENTORY_OK
                        : DB_ONVIF_INVENTORY_NOT_FOUND;
}
