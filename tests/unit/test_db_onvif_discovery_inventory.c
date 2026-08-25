#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_onvif_discovery_inventory.h"
#include "database/db_streams.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_onvif_inventory.db"

static onvif_device_info_t device(const char *endpoint, const char *ip,
                                  const char *serial, const char *mac) {
    onvif_device_info_t result;
    memset(&result, 0, sizeof(result));
    safe_strcpy(result.endpoint, endpoint, sizeof(result.endpoint), 0);
    safe_strcpy(result.device_service, endpoint,
                sizeof(result.device_service), 0);
    safe_strcpy(result.ip_address, ip, sizeof(result.ip_address), 0);
    safe_strcpy(result.serial_number, serial ? serial : "",
                sizeof(result.serial_number), 0);
    safe_strcpy(result.mac_address, mac ? mac : "",
                sizeof(result.mac_address), 0);
    safe_strcpy(result.manufacturer, "FixtureCam",
                sizeof(result.manufacturer), 0);
    result.online = true;
    return result;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "DELETE FROM onvif_discovery_addresses;"
            "DELETE FROM onvif_discovery_inventory;"
            "DELETE FROM streams;", NULL, NULL, NULL));
}

void tearDown(void) {}

void test_scan_persists_first_and_last_seen(void) {
    onvif_device_info_t observed = device(
        "http://192.0.2.10/onvif/device_service", "192.0.2.10", "", "");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &observed, 1, 100));
    TEST_ASSERT_TRUE(observed.inventory_uuid[0] != '\0');
    TEST_ASSERT_EQUAL_INT64(100, observed.first_seen_at);
    TEST_ASSERT_EQUAL_STRING("unclaimed", observed.claim_state);

    onvif_device_info_t repeated = device(
        "http://192.0.2.10/onvif/device_service", "192.0.2.10", "", "");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &repeated, 1, 200));
    TEST_ASSERT_EQUAL_STRING(observed.inventory_uuid, repeated.inventory_uuid);

    onvif_device_info_t inventory[4];
    TEST_ASSERT_EQUAL_INT(1, db_onvif_inventory_list(inventory, 4));
    TEST_ASSERT_EQUAL_INT64(100, inventory[0].first_seen_at);
    TEST_ASSERT_EQUAL_INT64(200, inventory[0].last_seen_at);
    TEST_ASSERT_TRUE(inventory[0].online);
}

void test_serial_identity_survives_dhcp_address_change(void) {
    onvif_device_info_t first = device(
        "http://192.0.2.20/onvif/device_service", "192.0.2.20",
        "SERIAL-20", "00:11:22:33:44:55");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &first, 1, 300));
    char uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(uuid, first.inventory_uuid, sizeof(uuid), 0);

    onvif_device_info_t moved = device(
        "http://192.0.2.99/onvif/device_service", "192.0.2.99",
        "SERIAL-20", "00:11:22:33:44:55");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &moved, 1, 400));
    TEST_ASSERT_EQUAL_STRING(uuid, moved.inventory_uuid);

    onvif_device_info_t inventory[4];
    TEST_ASSERT_EQUAL_INT(1, db_onvif_inventory_list(inventory, 4));
    TEST_ASSERT_EQUAL_STRING("192.0.2.99", inventory[0].ip_address);
    char addresses[ONVIF_DISCOVERY_ADDRESS_MAX][MAX_URL_LENGTH];
    int address_count = db_onvif_inventory_list_addresses(
        uuid, addresses, ONVIF_DISCOVERY_ADDRESS_MAX);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(4, address_count);
}

void test_conflicting_serial_at_same_endpoint_is_not_silently_merged(void) {
    onvif_device_info_t first = device(
        "http://192.0.2.30/onvif/device_service", "192.0.2.30",
        "SERIAL-A", "");
    onvif_device_info_t replacement = device(
        "http://192.0.2.30/onvif/device_service", "192.0.2.30",
        "SERIAL-B", "");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &first, 1, 500));
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan(
               "192.0.2.0/24", &replacement, 1, 600));
    TEST_ASSERT_NOT_EQUAL(0,
                          strcmp(first.inventory_uuid,
                                 replacement.inventory_uuid));

    onvif_device_info_t inventory[4];
    TEST_ASSERT_EQUAL_INT(2, db_onvif_inventory_list(inventory, 4));
    TEST_ASSERT_TRUE(inventory[0].duplicate_suspected);
    TEST_ASSERT_TRUE(inventory[1].duplicate_suspected);
}

void test_repeated_empty_scan_marks_only_same_scope_offline(void) {
    onvif_device_info_t first = device(
        "http://192.0.2.40/onvif/device_service", "192.0.2.40", "A", "");
    onvif_device_info_t other = device(
        "http://198.51.100.40/onvif/device_service", "198.51.100.40", "B", "");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &first, 1, 700));
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan(
               "198.51.100.0/24", &other, 1, 700));
    TEST_ASSERT_EQUAL_INT(
        0, db_onvif_inventory_record_scan("192.0.2.0/24", NULL, 0, 800));

    onvif_device_info_t inventory[4];
    TEST_ASSERT_EQUAL_INT(2, db_onvif_inventory_list(inventory, 4));
    bool first_offline = false;
    bool other_online = false;
    for (int index = 0; index < 2; index++) {
        if (strcmp(inventory[index].serial_number, "A") == 0) {
            first_offline = !inventory[index].online;
        } else if (strcmp(inventory[index].serial_number, "B") == 0) {
            other_online = inventory[index].online;
        }
    }
    TEST_ASSERT_TRUE(first_offline);
    TEST_ASSERT_TRUE(other_online);
}

void test_claim_is_explicit_and_stream_deletion_unclaims(void) {
    onvif_device_info_t observed = device(
        "http://192.0.2.50/onvif/device_service", "192.0.2.50", "C", "");
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan("192.0.2.0/24", &observed, 1, 900));

    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, "claimed-camera", sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://192.0.2.50/live", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 1280;
    stream.height = 720;
    stream.fps = 15;
    stream.priority = 5;
    stream.segment_duration = 30;
    TEST_ASSERT_NOT_EQUAL_UINT64(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(
        DB_ONVIF_INVENTORY_OK,
        db_onvif_inventory_claim_stream(observed.inventory_uuid,
                                        stream.name));

    onvif_device_info_t inventory[2];
    TEST_ASSERT_EQUAL_INT(1, db_onvif_inventory_list(inventory, 2));
    TEST_ASSERT_EQUAL_STRING("claimed", inventory[0].claim_state);
    TEST_ASSERT_TRUE(inventory[0].claimed_camera_uuid[0] != '\0');

    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        get_db_handle(), "DELETE FROM streams WHERE name='claimed-camera';",
        NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(1, db_onvif_inventory_list(inventory, 2));
    TEST_ASSERT_EQUAL_STRING("unclaimed", inventory[0].claim_state);
    TEST_ASSERT_EQUAL_STRING("", inventory[0].claimed_camera_uuid);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_scan_persists_first_and_last_seen);
    RUN_TEST(test_serial_identity_survives_dhcp_address_change);
    RUN_TEST(test_conflicting_serial_at_same_endpoint_is_not_silently_merged);
    RUN_TEST(test_repeated_empty_scan_marks_only_same_scope_offline);
    RUN_TEST(test_claim_is_explicit_and_stream_deletion_unclaims);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
