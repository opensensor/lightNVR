#include "unity.h"

#include "telemetry/collectors/linux_filesystem.h"

#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>
#include <sys/sysmacros.h>

#ifndef SYSTEM_HEALTH_FILESYSTEM_FIXTURE_DIR
#define SYSTEM_HEALTH_FILESYSTEM_FIXTURE_DIR \
    "tests/fixtures/system_health/filesystem"
#endif

static int fake_stat_error;
static int fake_statvfs_error;
static mode_t fake_mode;
static dev_t fake_device;
static struct statvfs fake_filesystem;
static unsigned fake_stat_calls;
static unsigned fake_statvfs_calls;

void setUp(void) {
    fake_stat_error = 0;
    fake_statvfs_error = 0;
    fake_mode = S_IFDIR | 0700;
    fake_device = makedev(8, 17);
    memset(&fake_filesystem, 0, sizeof(fake_filesystem));
    fake_filesystem.f_frsize = 4096U;
    fake_filesystem.f_blocks = 100U;
    fake_filesystem.f_bavail = 25U;
    fake_filesystem.f_files = 50U;
    fake_filesystem.f_favail = 10U;
    fake_stat_calls = 0U;
    fake_statvfs_calls = 0U;
}

void tearDown(void) {}

static int fake_stat_path(const char *path, struct stat *info) {
    (void)path;
    fake_stat_calls++;
    if (fake_stat_error != 0) {
        errno = fake_stat_error;
        return -1;
    }
    memset(info, 0, sizeof(*info));
    info->st_mode = fake_mode;
    info->st_dev = fake_device;
    return 0;
}

static int fake_statvfs_path(const char *path, struct statvfs *info) {
    (void)path;
    fake_statvfs_calls++;
    if (fake_statvfs_error != 0) {
        errno = fake_statvfs_error;
        return -1;
    }
    *info = fake_filesystem;
    return 0;
}

static void fixture_path(char *output, size_t output_size, const char *name) {
    int length = snprintf(output, output_size, "%s/%s",
                          SYSTEM_HEALTH_FILESYSTEM_FIXTURE_DIR, name);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, length);
    TEST_ASSERT_LESS_THAN_size_t(output_size, (size_t)length);
}

static linux_filesystem_resource_t make_resource(void) {
    linux_filesystem_resource_t resource;
    memset(&resource, 0, sizeof(resource));
    snprintf(resource.logical_id, sizeof(resource.logical_id), "%s",
             "recording");
    snprintf(resource.path, sizeof(resource.path), "%s", "/private/path");
    return resource;
}

static linux_filesystem_sample_t take_sample(
    const linux_filesystem_resource_t *resource, const char *mountinfo) {
    const linux_filesystem_ops_t ops = {
        .stat_path = fake_stat_path, .statvfs_path = fake_statvfs_path
    };
    linux_filesystem_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, linux_filesystem_sample_with_ops(
        resource, mountinfo, &ops, &sample));
    return sample;
}

static void test_mountinfo_parser_handles_presence_absence_and_escapes(void) {
    char path[1024];
    bool present = false;
    fixture_path(path, sizeof(path), "mountinfo-present.txt");
    TEST_ASSERT_EQUAL_INT(0, linux_filesystem_mountinfo_contains(
        path, "/mnt/recordings", &present));
    TEST_ASSERT_TRUE(present);
    TEST_ASSERT_EQUAL_INT(0, linux_filesystem_mountinfo_contains(
        path, "/mnt/absent", &present));
    TEST_ASSERT_FALSE(present);

    fixture_path(path, sizeof(path), "mountinfo-escaped.txt");
    TEST_ASSERT_EQUAL_INT(0, linux_filesystem_mountinfo_contains(
        path, "/mnt/camera archive", &present));
    TEST_ASSERT_TRUE(present);
}

static void test_capacity_inode_readonly_and_device_key_are_independent(void) {
    linux_filesystem_resource_t resource = make_resource();
#ifdef ST_RDONLY
    fake_filesystem.f_flag = ST_RDONLY;
#endif
    linux_filesystem_sample_t sample = take_sample(&resource, NULL);
    TEST_ASSERT_TRUE(sample.mount_present.value);
    TEST_ASSERT_EQUAL_STRING("linux-block-8-17", sample.device_key);
    TEST_ASSERT_EQUAL_UINT64(409600U, sample.capacity_bytes.value);
    TEST_ASSERT_EQUAL_UINT64(102400U, sample.available_bytes.value);
    TEST_ASSERT_EQUAL_UINT64(50U, sample.capacity_inodes.value);
    TEST_ASSERT_EQUAL_UINT64(10U, sample.available_inodes.value);
    TEST_ASSERT_EQUAL_UINT64((uint64_t)fake_filesystem.f_flag,
                             sample.mount_flags.value);
#ifdef ST_RDONLY
    TEST_ASSERT_TRUE(sample.read_only.value);
#endif
    TEST_ASSERT_EQUAL_STRING("recording", sample.logical_id);
    TEST_ASSERT_NULL(strstr(sample.logical_id, "/private/path"));
}

static void test_missing_mount_does_not_probe_underlying_directory(void) {
    char path[1024];
    fixture_path(path, sizeof(path), "mountinfo-present.txt");
    linux_filesystem_resource_t resource = make_resource();
    resource.mount_required = true;
    snprintf(resource.mount_guard_path, sizeof(resource.mount_guard_path),
             "%s", "/mnt/missing");
    linux_filesystem_sample_t sample = take_sample(&resource, path);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                      sample.mount_present.capability);
    TEST_ASSERT_FALSE(sample.mount_present.value);
    TEST_ASSERT_EQUAL_UINT(0U, fake_stat_calls);
    TEST_ASSERT_EQUAL_UINT(0U, fake_statvfs_calls);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.capacity_bytes.capability);
}

static void test_disappearing_or_replaced_mount_cannot_report_capacity(void) {
    linux_filesystem_resource_t resource = make_resource();
    fake_stat_error = ENOENT;
    linux_filesystem_sample_t sample = take_sample(&resource, NULL);
    TEST_ASSERT_FALSE(sample.mount_present.value);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.capacity_bytes.capability);

    fake_stat_error = 0;
    snprintf(resource.expected_device_key,
             sizeof(resource.expected_device_key), "%s", "linux-block-8-18");
    sample = take_sample(&resource, NULL);
    TEST_ASSERT_FALSE(sample.mount_present.value);
    TEST_ASSERT_EQUAL_UINT(0U, fake_statvfs_calls);
}

static void test_permission_overflow_and_inode_less_filesystem_are_explicit(void) {
    linux_filesystem_resource_t resource = make_resource();
    fake_statvfs_error = EACCES;
    linux_filesystem_sample_t sample = take_sample(&resource, NULL);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      sample.capacity_bytes.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      sample.read_only.capability);

    fake_statvfs_error = 0;
    fake_filesystem.f_frsize = 2U;
    fake_filesystem.f_blocks = (fsblkcnt_t)UINT64_MAX;
    fake_filesystem.f_bavail = 4U;
    fake_filesystem.f_files = 0U;
    sample = take_sample(&resource, NULL);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      sample.capacity_bytes.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                      sample.available_bytes.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      sample.capacity_inodes.capability);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      sample.available_inodes.capability);
}

static void test_probe_errors_and_logical_ids_are_stable(void) {
    TEST_ASSERT_EQUAL(LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION,
                      linux_filesystem_normalize_errno(EACCES));
    TEST_ASSERT_EQUAL(LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY,
                      linux_filesystem_normalize_errno(EROFS));
    TEST_ASSERT_EQUAL(LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE,
                      linux_filesystem_normalize_errno(ENOSPC));
    TEST_ASSERT_TRUE(linux_filesystem_logical_id_valid("root"));
    TEST_ASSERT_TRUE(linux_filesystem_logical_id_valid(
        "target:550e8400-e29b-41d4-a716-446655440000"));
    TEST_ASSERT_FALSE(linux_filesystem_logical_id_valid("/srv/recordings"));
    TEST_ASSERT_FALSE(linux_filesystem_logical_id_valid("target name"));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_mountinfo_parser_handles_presence_absence_and_escapes);
    RUN_TEST(test_capacity_inode_readonly_and_device_key_are_independent);
    RUN_TEST(test_missing_mount_does_not_probe_underlying_directory);
    RUN_TEST(test_disappearing_or_replaced_mount_cannot_report_capacity);
    RUN_TEST(test_permission_overflow_and_inode_less_filesystem_are_explicit);
    RUN_TEST(test_probe_errors_and_logical_ids_are_stable);
    return UNITY_END();
}
