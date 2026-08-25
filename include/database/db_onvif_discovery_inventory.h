#ifndef LIGHTNVR_DB_ONVIF_DISCOVERY_INVENTORY_H
#define LIGHTNVR_DB_ONVIF_DISCOVERY_INVENTORY_H

#include <stddef.h>
#include <stdint.h>

#include "video/onvif_discovery.h"

#define ONVIF_DISCOVERY_INVENTORY_MAX 256
#define ONVIF_DISCOVERY_ADDRESS_MAX 16

typedef enum {
    DB_ONVIF_INVENTORY_OK = 0,
    DB_ONVIF_INVENTORY_NOT_FOUND = -1,
    DB_ONVIF_INVENTORY_INVALID = -2,
    DB_ONVIF_INVENTORY_CONFLICT = -3,
    DB_ONVIF_INVENTORY_ERROR = -4
} db_onvif_inventory_result_t;

/*
 * Persist one bounded scan atomically. Devices missed by a repeated scan of
 * the same scope become offline; records from other scopes are left alone.
 * The supplied array is updated with durable inventory metadata.
 */
int db_onvif_inventory_record_scan(
    const char *scan_network, onvif_device_info_t *devices, int count,
    int64_t observed_at);

int db_onvif_inventory_list(
    onvif_device_info_t *devices, int max_count);

int db_onvif_inventory_list_addresses(
    const char *inventory_uuid,
    char addresses[][MAX_URL_LENGTH], int max_count);

/* Explicitly associate a staged discovery result with an existing stream. */
db_onvif_inventory_result_t db_onvif_inventory_claim_stream(
    const char *inventory_uuid, const char *stream_name);

#endif /* LIGHTNVR_DB_ONVIF_DISCOVERY_INVENTORY_H */
