#define _POSIX_C_SOURCE 200809L

#include "telemetry/providers/linux_hardware.h"

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/sysmacros.h>

#include "database/db_storage_targets.h"

#define DEVICE_MAP_SCAN_MAX 128U

static system_health_capability_t capability_from_errno(int error_number) {
    if (error_number == EACCES || error_number == EPERM)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (error_number == ENOENT || error_number == ENOTDIR)
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static system_health_capability_t merge_capability(
    system_health_capability_t left, system_health_capability_t right) {
    if (left == SYSTEM_HEALTH_CAPABILITY_AVAILABLE ||
        right == SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (left == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED ||
        right == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (left == SYSTEM_HEALTH_CAPABILITY_ERROR ||
        right == SYSTEM_HEALTH_CAPABILITY_ERROR)
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}

static bool internal_name_valid(const char *name) {
    if (!name || !name[0] || strcmp(name, ".") == 0 ||
        strcmp(name, "..") == 0 || strlen(name) >=
            LINUX_HARDWARE_INTERNAL_NAME_LENGTH) return false;
    for (const unsigned char *cursor = (const unsigned char *)name;
         *cursor; ++cursor) {
        if (!isalnum(*cursor) && *cursor != '_' && *cursor != '-' &&
            *cursor != '.') return false;
    }
    return true;
}

static int compare_names(const void *left, const void *right) {
    return strcmp((const char *)left, (const char *)right);
}

static bool read_device_number(const char *sys_root, const char *name,
                               uint64_t *device,
                               system_health_capability_t *capability) {
    char path[1200];
    int written = snprintf(path, sizeof(path), "%s/class/block/%s/dev",
                           sys_root, name);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return false;
    }
    FILE *file = fopen(path, "r");
    if (!file) {
        *capability = capability_from_errno(errno);
        return false;
    }
    unsigned int major_number = 0U, minor_number = 0U;
    char trailing = '\0';
    int matched = fscanf(file, "%u:%u %c", &major_number, &minor_number,
                         &trailing);
    fclose(file);
    if (matched != 2) {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return false;
    }
    *device = (uint64_t)makedev(major_number, minor_number);
    *capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return true;
}

static void physical_name(const char *name,
                          char output[LINUX_HARDWARE_INTERNAL_NAME_LENGTH]) {
    size_t length = strnlen(name, LINUX_HARDWARE_INTERNAL_NAME_LENGTH);
    if (length >= LINUX_HARDWARE_INTERNAL_NAME_LENGTH) {
        output[0] = '\0';
        return;
    }
    memcpy(output, name, length + 1U);
    if (strncmp(name, "nvme", 4) == 0 && isdigit((unsigned char)name[4])) {
        char *cursor = output + 4;
        while (isdigit((unsigned char)*cursor)) cursor++;
        if (*cursor == 'n' && isdigit((unsigned char)cursor[1])) *cursor = '\0';
        return;
    }
    if (strncmp(name, "mmcblk", 6) == 0 &&
        isdigit((unsigned char)name[6])) {
        char *cursor = output + 6;
        while (isdigit((unsigned char)*cursor)) cursor++;
        if (*cursor == 'p' && isdigit((unsigned char)cursor[1])) *cursor = '\0';
        return;
    }
    if (name[0] == 's' && name[1] == 'd' &&
        isalpha((unsigned char)name[2])) {
        char *cursor = output + 2;
        while (isalpha((unsigned char)*cursor)) cursor++;
        if (isdigit((unsigned char)*cursor)) *cursor = '\0';
    }
}

static uint64_t scoped_hash(const char *scope, const char *name) {
    uint64_t hash = UINT64_C(1469598103934665603);
    const char *parts[] = {scope, "|linux-block|", name};
    for (size_t part = 0; part < sizeof(parts) / sizeof(parts[0]); ++part) {
        for (const unsigned char *cursor =
                 (const unsigned char *)parts[part]; *cursor; ++cursor) {
            hash ^= *cursor;
            hash *= UINT64_C(1099511628211);
        }
    }
    return hash;
}

static bool target_preferred(const storage_target_t *candidate,
                             const linux_device_map_entry_t *current) {
    if (!current->target_mapped) return true;
    if (candidate->is_default != current->target_is_default)
        return candidate->is_default;
    const char *current_uuid = strchr(current->public_id, ':');
    return current_uuid && strcmp(candidate->uuid, current_uuid + 1) < 0;
}

const linux_device_map_entry_t *linux_device_map_find(
    const linux_device_map_t *map, const char *physical_name_value) {
    if (!map || !physical_name_value) return NULL;
    for (size_t index = 0; index < map->count; ++index)
        if (strcmp(map->entries[index].sysfs_name, physical_name_value) == 0)
            return &map->entries[index];
    return NULL;
}

int linux_device_map_build(const char *sys_root, const char *installation_scope,
                           linux_device_map_t *map) {
    if (!sys_root || sys_root[0] != '/' || !installation_scope ||
        !installation_scope[0] || !map) return -1;
    memset(map, 0, sizeof(*map));
    char class_path[1200];
    int written = snprintf(class_path, sizeof(class_path), "%s/class/block",
                           sys_root);
    if (written < 0 || (size_t)written >= sizeof(class_path)) {
        map->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    DIR *directory = opendir(class_path);
    if (!directory) {
        map->capability = capability_from_errno(errno);
        return 0;
    }
    char names[DEVICE_MAP_SCAN_MAX][LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    size_t name_count = 0U;
    struct dirent *entry;
    while ((entry = readdir(directory)) != NULL) {
        if (!internal_name_valid(entry->d_name)) continue;
        if (name_count >= DEVICE_MAP_SCAN_MAX) {
            map->dropped++;
            break;
        }
        size_t length = strlen(entry->d_name);
        memcpy(names[name_count], entry->d_name, length + 1U);
        name_count++;
    }
    closedir(directory);
    qsort(names, name_count, sizeof(names[0]), compare_names);

    int target_total = db_storage_target_count();
    storage_target_t *targets = NULL;
    int target_count = 0;
    if (target_total > 0 && target_total <= STORAGE_TARGET_MAX_COUNT) {
        targets = calloc((size_t)target_total, sizeof(*targets));
        if (targets) {
            target_count = db_storage_target_list(targets, target_total);
            if (target_count < 0) target_count = 0;
        }
    }

    system_health_capability_t scan_capability =
        SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    for (size_t name_index = 0; name_index < name_count; ++name_index) {
        uint64_t device = 0U;
        system_health_capability_t device_capability;
        if (!read_device_number(sys_root, names[name_index], &device,
                                &device_capability)) {
            scan_capability = merge_capability(scan_capability,
                                               device_capability);
            continue;
        }
        scan_capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        char physical[LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
        physical_name(names[name_index], physical);
        size_t map_index = 0U;
        while (map_index < map->count &&
               strcmp(map->entries[map_index].sysfs_name, physical) != 0)
            map_index++;
        if (map_index == map->count) {
            if (map->count >= LINUX_HARDWARE_DEVICE_MAP_MAX) {
                map->dropped++;
                continue;
            }
            linux_device_map_entry_t *mapped = &map->entries[map->count++];
            snprintf(mapped->sysfs_name, sizeof(mapped->sysfs_name), "%s",
                     physical);
            mapped->filesystem_device = device;
        }
        linux_device_map_entry_t *mapped = &map->entries[map_index];
        for (int target_index = 0; target_index < target_count; ++target_index) {
            const storage_target_t *target = &targets[target_index];
            if (target->filesystem_device != device ||
                !target_preferred(target, mapped)) continue;
            snprintf(mapped->public_id, sizeof(mapped->public_id),
                     "target:%s", target->uuid);
            mapped->target_mapped = true;
            mapped->target_is_default = target->is_default;
            mapped->filesystem_device = device;
        }
    }
    free(targets);
    for (size_t index = 0; index < map->count; ++index) {
        linux_device_map_entry_t *mapped = &map->entries[index];
        if (!mapped->target_mapped)
            snprintf(mapped->public_id, sizeof(mapped->public_id),
                     "device.%016llx", (unsigned long long)scoped_hash(
                         installation_scope, mapped->sysfs_name));
    }
    map->capability = map->count > 0U
        ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE : scan_capability;
    return 0;
}
