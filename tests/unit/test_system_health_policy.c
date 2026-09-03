#include "unity.h"

#include "telemetry/system_health_policy.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void setUp(void) {}
void tearDown(void) {}

static system_health_policy_settings_t defaults(void) {
    system_health_policy_settings_t settings;
    system_health_policy_settings_defaults(&settings);
    return settings;
}

static void test_scalar_defaults_match_ops03(void) {
    system_health_policy_settings_t settings = defaults();
    TEST_ASSERT_TRUE(settings.enabled);
    TEST_ASSERT_EQUAL_STRING("balanced", settings.profile);
    TEST_ASSERT_EQUAL_UINT32(10U, settings.fast_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(60U, settings.normal_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(300U, settings.slow_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(900U, settings.device_interval_seconds);
    TEST_ASSERT_TRUE(settings.write_probe_enabled);
    TEST_ASSERT_EQUAL_STRING("auto", settings.hardware_provider);
    TEST_ASSERT_EQUAL_UINT32(60U, settings.presence_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(90U, settings.incident_retention_days);
}

static void test_cadences_profiles_and_provider_are_rejected_not_clamped(void) {
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    system_health_policy_settings_t settings = defaults();
    settings.fast_interval_seconds = SYSTEM_HEALTH_FAST_INTERVAL_MIN - 1U;
    TEST_ASSERT_EQUAL_INT(-1,
        system_health_policy_validate_settings(&settings, error));
    TEST_ASSERT_NOT_NULL(strstr(error, "fast interval"));

    settings = defaults();
    settings.normal_interval_seconds = 30U;
    settings.fast_interval_seconds = 40U;
    TEST_ASSERT_EQUAL_INT(-1,
        system_health_policy_validate_settings(&settings, error));
    TEST_ASSERT_NOT_NULL(strstr(error, "ordered"));

    settings = defaults();
    snprintf(settings.profile, sizeof(settings.profile), "%s", "custom");
    TEST_ASSERT_EQUAL_INT(-1,
        system_health_policy_validate_settings(&settings, error));
    settings = defaults();
    snprintf(settings.hardware_provider, sizeof(settings.hardware_provider),
             "%s", "/bin/x");
    TEST_ASSERT_EQUAL_INT(-1,
        system_health_policy_validate_settings(&settings, error));
}

static void test_smartctl_requires_explicit_valid_provider_selection(void) {
    system_health_policy_settings_t settings;
    system_health_policy_settings_defaults(&settings);
    snprintf(settings.hardware_provider, sizeof(settings.hardware_provider),
             "%s", "smartctl");
    TEST_ASSERT_EQUAL_INT(0,
        system_health_policy_validate_settings(&settings, NULL));
}

static void test_balanced_conservative_and_disabled_profiles_are_complete(void) {
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    system_health_policy_t balanced;
    system_health_policy_settings_t settings = defaults();
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, NULL, &balanced, NULL, 0U, error));
    const system_health_condition_policy_t *memory =
        &balanced.conditions[SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW];
    TEST_ASSERT_TRUE(memory->enabled);
    TEST_ASSERT_TRUE(memory->critical_threshold < memory->warning_threshold);
    TEST_ASSERT_TRUE(memory->warning_threshold < memory->recovery_threshold);
    TEST_ASSERT_EQUAL_UINT32(120U, memory->warning_for_seconds);

    snprintf(settings.profile, sizeof(settings.profile), "%s", "conservative");
    system_health_policy_t conservative;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, NULL, &conservative, NULL, 0U, error));
    TEST_ASSERT_TRUE(conservative.conditions[
        SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW].warning_threshold >
        memory->warning_threshold);

    snprintf(settings.profile, sizeof(settings.profile), "%s", "disabled");
    system_health_policy_t disabled;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, NULL, &disabled, NULL, 0U, error));
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        TEST_ASSERT_FALSE(disabled.conditions[index].enabled);
    }
}

static void test_complete_overrides_are_validated_and_canonicalized(void) {
    static const char input[] =
        "{\"conditions\":["
        "{\"code\":\"process.fd_exhaustion\",\"unit\":\"ratio\","
        "\"warning\":0.7,\"critical\":0.85,\"recovery\":0.6,"
        "\"warning_for_seconds\":90,\"critical_for_seconds\":30,"
        "\"recovery_for_seconds\":120},"
        "{\"profile\":\"disabled\",\"code\":\"cpu.saturation\"}],"
        "\"version\":1}";
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    char canonical[SYSTEM_HEALTH_OVERRIDE_JSON_MAX + 1U];
    system_health_policy_t policy;
    system_health_policy_settings_t settings = defaults();
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, input, &policy, canonical, sizeof(canonical), error));
    TEST_ASSERT_FALSE(policy.conditions[
        SYSTEM_HEALTH_CONDITION_CPU_SATURATION].enabled);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_PROFILE_CUSTOM, policy.conditions[
        SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION].profile);
    const char *cpu = strstr(canonical, "cpu.saturation");
    const char *fds = strstr(canonical, "process.fd_exhaustion");
    TEST_ASSERT_NOT_NULL(cpu);
    TEST_ASSERT_NOT_NULL(fds);
    TEST_ASSERT_TRUE(cpu < fds);

    char second[SYSTEM_HEALTH_OVERRIDE_JSON_MAX + 1U];
    system_health_policy_t rebuilt;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, canonical, &rebuilt, second, sizeof(second), error));
    TEST_ASSERT_EQUAL_STRING(canonical, second);
}

static void test_bad_overrides_reject_unknown_partial_units_order_and_secrets(void) {
    static const char *const invalid[] = {
        "{\"version\":1,\"conditions\":[{\"code\":\"bogus\",\"profile\":\"disabled\"}]}",
        "{\"version\":1,\"conditions\":[{\"code\":\"memory.available_low\",\"warning\":0.2}]}",
        "{\"version\":1,\"conditions\":[{\"code\":\"memory.available_low\",\"unit\":\"bytes\",\"warning\":0.15,\"critical\":0.08,\"recovery\":0.2,\"warning_for_seconds\":1,\"critical_for_seconds\":1,\"recovery_for_seconds\":1}]}",
        "{\"version\":1,\"conditions\":[{\"code\":\"memory.available_low\",\"unit\":\"ratio\",\"warning\":0.08,\"critical\":0.15,\"recovery\":0.2,\"warning_for_seconds\":1,\"critical_for_seconds\":1,\"recovery_for_seconds\":1}]}",
        "{\"version\":1,\"conditions\":[{\"code\":\"memory.available_low\",\"unit\":\"ratio\",\"warning\":0.15,\"critical\":0.08,\"recovery\":0.2,\"warning_for_seconds\":0,\"critical_for_seconds\":1,\"recovery_for_seconds\":1}]}",
        "{\"version\":1,\"path\":\"/etc/shadow\",\"conditions\":[]}",
        "{\"version\":1,\"conditions\":[],\"password\":\"secret\"}",
        "{\"version\":1,\"conditions\":[{\"code\":\"cpu.saturation\",\"profile\":\"disabled\"},{\"code\":\"cpu.saturation\",\"profile\":\"balanced\"}]}"
    };
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    system_health_policy_settings_t settings = defaults();
    system_health_policy_t policy;
    for (size_t index = 0; index < sizeof(invalid) / sizeof(invalid[0]); ++index) {
        TEST_ASSERT_EQUAL_INT(-1, system_health_policy_build(
            &settings, invalid[index], &policy, NULL, 0U, error));
        TEST_ASSERT_NOT_EQUAL('\0', error[0]);
    }
}

static void test_failed_build_leaves_atomic_snapshot_unchanged(void) {
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    system_health_policy_settings_t settings = defaults();
    system_health_policy_t candidate;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, NULL, &candidate, NULL, 0U, error));
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_replace(&candidate, error));

    system_health_policy_t before;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_snapshot(&before));
    settings.fast_interval_seconds = 1U;
    TEST_ASSERT_EQUAL_INT(-1, system_health_policy_build(
        &settings, NULL, &candidate, NULL, 0U, error));
    system_health_policy_t after;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_snapshot(&after));
    TEST_ASSERT_EQUAL_UINT64(before.generation, after.generation);
    TEST_ASSERT_EQUAL_UINT32(before.settings.fast_interval_seconds,
                             after.settings.fast_interval_seconds);

    before.conditions[SYSTEM_HEALTH_CONDITION_CPU_SATURATION].enabled = false;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_snapshot(&after));
    TEST_ASSERT_TRUE(after.conditions[
        SYSTEM_HEALTH_CONDITION_CPU_SATURATION].enabled);
}

static void test_effective_defaults_are_visible_in_serialized_policy(void) {
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    char json[32768];
    system_health_policy_settings_t settings = defaults();
    system_health_policy_t policy;
    TEST_ASSERT_EQUAL_INT(0, system_health_policy_build(
        &settings, NULL, &policy, NULL, 0U, error));
    TEST_ASSERT_EQUAL_INT(0,
        system_health_policy_serialize(&policy, json, sizeof(json)));
    TEST_ASSERT_NOT_NULL(strstr(json, "memory.available_low"));
    TEST_ASSERT_NOT_NULL(strstr(json, "\"warning\":0.15"));
    TEST_ASSERT_NULL(strstr(json, "path"));
    TEST_ASSERT_NULL(strstr(json, "password"));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_scalar_defaults_match_ops03);
    RUN_TEST(test_cadences_profiles_and_provider_are_rejected_not_clamped);
    RUN_TEST(test_smartctl_requires_explicit_valid_provider_selection);
    RUN_TEST(test_balanced_conservative_and_disabled_profiles_are_complete);
    RUN_TEST(test_complete_overrides_are_validated_and_canonicalized);
    RUN_TEST(test_bad_overrides_reject_unknown_partial_units_order_and_secrets);
    RUN_TEST(test_failed_build_leaves_atomic_snapshot_unchanged);
    RUN_TEST(test_effective_defaults_are_visible_in_serialized_policy);
    return UNITY_END();
}
