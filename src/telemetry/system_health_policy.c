#include "telemetry/system_health_policy.h"

#include <cjson/cJSON.h>
#include <math.h>
#include <pthread.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define POLICY_DWELL_MAX_SECONDS 604800U

static pthread_rwlock_t active_policy_lock = PTHREAD_RWLOCK_INITIALIZER;
static pthread_once_t active_policy_once = PTHREAD_ONCE_INIT;
static system_health_policy_t active_policy;

static int fail(char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH],
                const char *format, ...) {
    if (error) {
        va_list arguments;
        va_start(arguments, format);
        vsnprintf(error, SYSTEM_HEALTH_POLICY_ERROR_LENGTH, format, arguments);
        va_end(arguments);
    }
    return -1;
}

static void clear_error(char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]) {
    if (error) error[0] = '\0';
}

void system_health_policy_settings_defaults(
    system_health_policy_settings_t *settings) {
    if (!settings) return;
    memset(settings, 0, sizeof(*settings));
    settings->enabled = true;
    snprintf(settings->profile, sizeof(settings->profile), "%s", "balanced");
    settings->fast_interval_seconds = 10U;
    settings->normal_interval_seconds = 60U;
    settings->slow_interval_seconds = 300U;
    settings->device_interval_seconds = 900U;
    settings->write_probe_enabled = true;
    snprintf(settings->hardware_provider, sizeof(settings->hardware_provider),
             "%s", "auto");
    settings->presence_interval_seconds = 60U;
    settings->incident_retention_days = 90U;
}

bool system_health_policy_profile_from_name(const char *name,
                                            system_health_profile_t *profile) {
    if (!name || !profile) return false;
    if (strcmp(name, "balanced") == 0) {
        *profile = SYSTEM_HEALTH_PROFILE_BALANCED;
    } else if (strcmp(name, "conservative") == 0) {
        *profile = SYSTEM_HEALTH_PROFILE_CONSERVATIVE;
    } else if (strcmp(name, "disabled") == 0) {
        *profile = SYSTEM_HEALTH_PROFILE_DISABLED;
    } else if (strcmp(name, "custom") == 0) {
        *profile = SYSTEM_HEALTH_PROFILE_CUSTOM;
    } else {
        return false;
    }
    return true;
}

const char *system_health_policy_profile_name(system_health_profile_t profile) {
    switch (profile) {
        case SYSTEM_HEALTH_PROFILE_BALANCED: return "balanced";
        case SYSTEM_HEALTH_PROFILE_CONSERVATIVE: return "conservative";
        case SYSTEM_HEALTH_PROFILE_DISABLED: return "disabled";
        case SYSTEM_HEALTH_PROFILE_CUSTOM: return "custom";
        default: return "unknown";
    }
}

int system_health_policy_validate_settings(
    const system_health_policy_settings_t *settings,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]) {
    clear_error(error);
    if (!settings) return fail(error, "health settings are required");
    if (!memchr(settings->profile, '\0', sizeof(settings->profile)) ||
        !memchr(settings->hardware_provider, '\0',
                sizeof(settings->hardware_provider))) {
        return fail(error, "health profile and provider must be bounded strings");
    }
    system_health_profile_t profile;
    if (!system_health_policy_profile_from_name(settings->profile, &profile) ||
        profile == SYSTEM_HEALTH_PROFILE_CUSTOM) {
        return fail(error, "health profile must be balanced, conservative, or disabled");
    }
    if (strcmp(settings->hardware_provider, "auto") != 0 &&
        strcmp(settings->hardware_provider, "smartctl") != 0 &&
        strcmp(settings->hardware_provider, "disabled") != 0) {
        return fail(error,
                    "hardware provider must be auto, smartctl, or disabled");
    }
    if (settings->fast_interval_seconds < SYSTEM_HEALTH_FAST_INTERVAL_MIN ||
        settings->fast_interval_seconds > SYSTEM_HEALTH_FAST_INTERVAL_MAX) {
        return fail(error, "fast interval must be between %u and %u seconds",
                    SYSTEM_HEALTH_FAST_INTERVAL_MIN,
                    SYSTEM_HEALTH_FAST_INTERVAL_MAX);
    }
    if (settings->normal_interval_seconds < SYSTEM_HEALTH_NORMAL_INTERVAL_MIN ||
        settings->normal_interval_seconds > SYSTEM_HEALTH_NORMAL_INTERVAL_MAX) {
        return fail(error, "normal interval must be between %u and %u seconds",
                    SYSTEM_HEALTH_NORMAL_INTERVAL_MIN,
                    SYSTEM_HEALTH_NORMAL_INTERVAL_MAX);
    }
    if (settings->slow_interval_seconds < SYSTEM_HEALTH_SLOW_INTERVAL_MIN ||
        settings->slow_interval_seconds > SYSTEM_HEALTH_SLOW_INTERVAL_MAX) {
        return fail(error, "slow interval must be between %u and %u seconds",
                    SYSTEM_HEALTH_SLOW_INTERVAL_MIN,
                    SYSTEM_HEALTH_SLOW_INTERVAL_MAX);
    }
    if (settings->device_interval_seconds < SYSTEM_HEALTH_DEVICE_INTERVAL_MIN ||
        settings->device_interval_seconds > SYSTEM_HEALTH_DEVICE_INTERVAL_MAX) {
        return fail(error, "device interval must be between %u and %u seconds",
                    SYSTEM_HEALTH_DEVICE_INTERVAL_MIN,
                    SYSTEM_HEALTH_DEVICE_INTERVAL_MAX);
    }
    if (settings->normal_interval_seconds < settings->fast_interval_seconds ||
        settings->slow_interval_seconds < settings->normal_interval_seconds ||
        settings->device_interval_seconds < settings->slow_interval_seconds) {
        return fail(error, "health intervals must be ordered fast <= normal <= slow <= device");
    }
    if (settings->presence_interval_seconds <
            SYSTEM_HEALTH_PRESENCE_INTERVAL_MIN ||
        settings->presence_interval_seconds >
            SYSTEM_HEALTH_PRESENCE_INTERVAL_MAX) {
        return fail(error, "presence interval must be between %u and %u seconds",
                    SYSTEM_HEALTH_PRESENCE_INTERVAL_MIN,
                    SYSTEM_HEALTH_PRESENCE_INTERVAL_MAX);
    }
    if (settings->incident_retention_days < SYSTEM_HEALTH_RETENTION_DAYS_MIN ||
        settings->incident_retention_days > SYSTEM_HEALTH_RETENTION_DAYS_MAX) {
        return fail(error, "incident retention must be between %u and %u days",
                    SYSTEM_HEALTH_RETENTION_DAYS_MIN,
                    SYSTEM_HEALTH_RETENTION_DAYS_MAX);
    }
    return 0;
}

static void set_threshold(system_health_policy_t *policy,
                          system_health_condition_t condition,
                          system_health_threshold_direction_t direction,
                          system_health_unit_t unit, double warning,
                          double critical, double recovery,
                          uint32_t warning_for, uint32_t critical_for,
                          uint32_t recovery_for) {
    system_health_condition_policy_t *rule = &policy->conditions[condition];
    rule->direction = direction;
    rule->unit = unit;
    rule->warning_threshold = warning;
    rule->critical_threshold = critical;
    rule->recovery_threshold = recovery;
    rule->warning_for_seconds = warning_for;
    rule->critical_for_seconds = critical_for;
    rule->recovery_for_seconds = recovery_for;
}

static void fill_balanced(system_health_policy_t *policy) {
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        system_health_condition_policy_t *rule = &policy->conditions[index];
        memset(rule, 0, sizeof(*rule));
        rule->condition = (system_health_condition_t)index;
        rule->profile = SYSTEM_HEALTH_PROFILE_BALANCED;
        rule->enabled = true;
        rule->unit = SYSTEM_HEALTH_UNIT_NONE;
    }

    set_threshold(policy, SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
                  SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .15, .08, .20, 120U, 30U, 300U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_CPU_SATURATION,
                  SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .90, .95, .75, 300U, 300U, 300U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_CPU_THROTTLED,
                  SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .10, .30, .05, 300U, 300U, 300U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_IO_PRESSURE,
                  SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_SECONDS, .50, 2.0, .20, 300U, 0U, 600U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW,
                  SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .15, .05, .20, 300U, 60U, 300U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW,
                  SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .10, .05, .15, 300U, 60U, 300U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE,
                  SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .01, .05, .005, 300U, 300U, 600U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
                  SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .80, .90, .70, 300U, 60U, 300U);
    set_threshold(policy, SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION,
                  SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
                  SYSTEM_HEALTH_UNIT_RATIO, .80, .90, .70, 300U, 60U, 300U);

    policy->conditions[SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN]
        .warning_for_seconds = 30U;
    policy->conditions[SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN]
        .critical_for_seconds = 300U;
    policy->conditions[SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN]
        .recovery_for_seconds = 120U;
    policy->conditions[SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED]
        .warning_for_seconds = 600U;
    policy->conditions[SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED]
        .recovery_for_seconds = 300U;
    policy->conditions[SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED]
        .recovery_for_seconds = 300U;
    policy->conditions[SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY]
        .recovery_for_seconds = 300U;
}

static void apply_conservative(system_health_policy_t *policy) {
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        system_health_condition_policy_t *rule = &policy->conditions[index];
        rule->profile = SYSTEM_HEALTH_PROFILE_CONSERVATIVE;
        if (rule->direction == SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE) {
            rule->warning_threshold *= 1.25;
            rule->critical_threshold *= 1.25;
            rule->recovery_threshold *= 1.25;
        } else if (rule->direction == SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE &&
                   rule->unit == SYSTEM_HEALTH_UNIT_RATIO) {
            rule->warning_threshold *= .90;
            rule->critical_threshold *= .90;
            rule->recovery_threshold *= .90;
        }
        if (rule->warning_for_seconds > 30U) rule->warning_for_seconds /= 2U;
        if (rule->critical_for_seconds > 30U) rule->critical_for_seconds /= 2U;
    }
}

static void fill_for_profile(system_health_profile_t profile,
                             system_health_policy_t *policy) {
    fill_balanced(policy);
    if (profile == SYSTEM_HEALTH_PROFILE_CONSERVATIVE) {
        apply_conservative(policy);
    } else if (profile == SYSTEM_HEALTH_PROFILE_DISABLED) {
        for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
            policy->conditions[index].profile = SYSTEM_HEALTH_PROFILE_DISABLED;
            policy->conditions[index].enabled = false;
        }
    }
}

static bool valid_number(double number) {
    return isfinite(number) && number >= -1000000000000.0 &&
           number <= 1000000000000.0;
}

static int validate_condition(const system_health_condition_policy_t *rule,
                              char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]) {
    const char *code = system_health_condition_code(rule->condition);
    if (!code) return fail(error, "policy contains an unknown condition");
    if (rule->warning_for_seconds > POLICY_DWELL_MAX_SECONDS ||
        rule->critical_for_seconds > POLICY_DWELL_MAX_SECONDS ||
        rule->recovery_for_seconds > POLICY_DWELL_MAX_SECONDS) {
        return fail(error, "%s dwell exceeds seven days", code);
    }
    if (rule->direction == SYSTEM_HEALTH_THRESHOLD_NONE || !rule->enabled) {
        return 0;
    }
    if (rule->warning_for_seconds == 0U ||
        rule->recovery_for_seconds == 0U) {
        return fail(error, "%s warning and recovery dwell must be nonzero", code);
    }
    if (!valid_number(rule->warning_threshold) ||
        !valid_number(rule->critical_threshold) ||
        !valid_number(rule->recovery_threshold)) {
        return fail(error, "%s has an invalid numeric threshold", code);
    }
    if (rule->unit == SYSTEM_HEALTH_UNIT_RATIO &&
        (rule->warning_threshold < 0.0 || rule->warning_threshold > 1.0 ||
         rule->critical_threshold < 0.0 || rule->critical_threshold > 1.0 ||
         rule->recovery_threshold < 0.0 || rule->recovery_threshold > 1.0)) {
        return fail(error, "%s ratio thresholds must be between zero and one",
                    code);
    }
    if (rule->direction == SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE &&
        !(rule->critical_threshold < rule->warning_threshold &&
          rule->warning_threshold < rule->recovery_threshold)) {
        return fail(error, "%s requires critical < warning < recovery", code);
    }
    if (rule->direction == SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE &&
        !(rule->recovery_threshold < rule->warning_threshold &&
          rule->warning_threshold < rule->critical_threshold)) {
        return fail(error, "%s requires recovery < warning < critical", code);
    }
    return 0;
}

static bool object_keys_exact(const cJSON *object,
                              const char *const *allowed,
                              size_t allowed_count, size_t expected_count) {
    size_t count = 0;
    for (const cJSON *child = object->child; child; child = child->next) {
        bool matched = false;
        for (size_t index = 0; index < allowed_count; ++index) {
            if (child->string && strcmp(child->string, allowed[index]) == 0) {
                if (cJSON_GetObjectItemCaseSensitive(object, allowed[index]) !=
                    child) return false;
                matched = true;
                break;
            }
        }
        if (!matched) return false;
        count++;
    }
    return count == expected_count;
}

static bool json_u32(const cJSON *value, uint32_t *output) {
    if (!cJSON_IsNumber(value) || !isfinite(value->valuedouble) ||
        value->valuedouble < 0.0 ||
        value->valuedouble > (double)POLICY_DWELL_MAX_SECONDS) return false;
    uint32_t integer = (uint32_t)value->valuedouble;
    if ((double)integer != value->valuedouble) return false;
    *output = integer;
    return true;
}

static bool unit_from_name(const char *name, system_health_unit_t *unit) {
    if (!name || !unit) return false;
    for (int index = 0; index <= SYSTEM_HEALTH_UNIT_BOOLEAN; ++index) {
        system_health_unit_t candidate = (system_health_unit_t)index;
        if (strcmp(name, system_health_unit_name(candidate)) == 0) {
            *unit = candidate;
            return true;
        }
    }
    return false;
}

static int apply_profile_override(system_health_policy_t *policy,
                                  system_health_condition_t condition,
                                  system_health_profile_t profile) {
    system_health_policy_t defaults;
    memset(&defaults, 0, sizeof(defaults));
    fill_for_profile(profile, &defaults);
    policy->conditions[condition] = defaults.conditions[condition];
    policy->conditions[condition].overridden = true;
    return 0;
}

static int apply_override_item(
    const cJSON *item, bool seen[SYSTEM_HEALTH_CONDITION_COUNT],
    system_health_policy_t *policy,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]) {
    static const char *const profile_keys[] = {"code", "profile"};
    static const char *const custom_keys[] = {
        "code", "unit", "warning", "critical", "recovery",
        "warning_for_seconds", "critical_for_seconds",
        "recovery_for_seconds"
    };
    if (!cJSON_IsObject(item)) return fail(error, "each health override must be an object");
    const cJSON *code_json = cJSON_GetObjectItemCaseSensitive(item, "code");
    if (!cJSON_IsString(code_json) || !code_json->valuestring) {
        return fail(error, "each health override requires a condition code");
    }
    system_health_condition_t condition;
    if (!system_health_condition_from_code(code_json->valuestring, &condition)) {
        return fail(error, "unknown health condition code: %s",
                    code_json->valuestring);
    }
    if (seen[condition]) return fail(error, "duplicate health condition override: %s",
                                     code_json->valuestring);
    seen[condition] = true;

    const cJSON *profile_json = cJSON_GetObjectItemCaseSensitive(item, "profile");
    if (profile_json) {
        if (!object_keys_exact(item, profile_keys, 2U, 2U) ||
            !cJSON_IsString(profile_json) || !profile_json->valuestring) {
            return fail(error, "%s profile override must contain only code and profile",
                        code_json->valuestring);
        }
        system_health_profile_t profile;
        if (!system_health_policy_profile_from_name(profile_json->valuestring,
                                                    &profile) ||
            profile == SYSTEM_HEALTH_PROFILE_CUSTOM) {
            return fail(error, "%s has an invalid override profile",
                        code_json->valuestring);
        }
        return apply_profile_override(policy, condition, profile);
    }

    if (!object_keys_exact(item, custom_keys, 8U, 8U)) {
        return fail(error, "%s custom override must be complete and contain no extra fields",
                    code_json->valuestring);
    }
    system_health_condition_policy_t *rule = &policy->conditions[condition];
    if (rule->direction == SYSTEM_HEALTH_THRESHOLD_NONE) {
        return fail(error, "%s does not accept numeric threshold overrides",
                    code_json->valuestring);
    }
    const cJSON *unit_json = cJSON_GetObjectItemCaseSensitive(item, "unit");
    system_health_unit_t unit;
    if (!cJSON_IsString(unit_json) || !unit_json->valuestring ||
        !unit_from_name(unit_json->valuestring, &unit) || unit != rule->unit) {
        return fail(error, "%s override unit must be %s", code_json->valuestring,
                    system_health_unit_name(rule->unit));
    }
    const cJSON *warning = cJSON_GetObjectItemCaseSensitive(item, "warning");
    const cJSON *critical = cJSON_GetObjectItemCaseSensitive(item, "critical");
    const cJSON *recovery = cJSON_GetObjectItemCaseSensitive(item, "recovery");
    if (!cJSON_IsNumber(warning) || !cJSON_IsNumber(critical) ||
        !cJSON_IsNumber(recovery)) {
        return fail(error, "%s thresholds must be numbers", code_json->valuestring);
    }
    rule->warning_threshold = warning->valuedouble;
    rule->critical_threshold = critical->valuedouble;
    rule->recovery_threshold = recovery->valuedouble;
    if (!json_u32(cJSON_GetObjectItemCaseSensitive(item, "warning_for_seconds"),
                  &rule->warning_for_seconds) ||
        !json_u32(cJSON_GetObjectItemCaseSensitive(item, "critical_for_seconds"),
                  &rule->critical_for_seconds) ||
        !json_u32(cJSON_GetObjectItemCaseSensitive(item, "recovery_for_seconds"),
                  &rule->recovery_for_seconds)) {
        return fail(error, "%s dwell values must be whole seconds up to seven days",
                    code_json->valuestring);
    }
    rule->profile = SYSTEM_HEALTH_PROFILE_CUSTOM;
    rule->enabled = true;
    rule->overridden = true;
    return validate_condition(rule, error);
}

static cJSON *override_json_for_policy(const system_health_policy_t *policy) {
    cJSON *root = cJSON_CreateObject();
    cJSON *conditions = cJSON_CreateArray();
    if (!root || !conditions) {
        cJSON_Delete(root);
        cJSON_Delete(conditions);
        return NULL;
    }
    cJSON_AddNumberToObject(root, "version", 1);
    cJSON_AddItemToObject(root, "conditions", conditions);
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        const system_health_condition_policy_t *rule = &policy->conditions[index];
        if (!rule->overridden) continue;
        cJSON *item = cJSON_CreateObject();
        if (!item) {
            cJSON_Delete(root);
            return NULL;
        }
        cJSON_AddStringToObject(item, "code",
                                system_health_condition_code(rule->condition));
        if (rule->profile != SYSTEM_HEALTH_PROFILE_CUSTOM) {
            cJSON_AddStringToObject(item, "profile",
                                    system_health_policy_profile_name(rule->profile));
        } else {
            cJSON_AddStringToObject(item, "unit", system_health_unit_name(rule->unit));
            cJSON_AddNumberToObject(item, "warning", rule->warning_threshold);
            cJSON_AddNumberToObject(item, "critical", rule->critical_threshold);
            cJSON_AddNumberToObject(item, "recovery", rule->recovery_threshold);
            cJSON_AddNumberToObject(item, "warning_for_seconds",
                                    rule->warning_for_seconds);
            cJSON_AddNumberToObject(item, "critical_for_seconds",
                                    rule->critical_for_seconds);
            cJSON_AddNumberToObject(item, "recovery_for_seconds",
                                    rule->recovery_for_seconds);
        }
        cJSON_AddItemToArray(conditions, item);
    }
    return root;
}

static int print_json(cJSON *root, char *output, size_t output_size) {
    if (!output || output_size == 0U) return root ? 0 : -1;
    char *printed = cJSON_PrintUnformatted(root);
    if (!printed) return -1;
    size_t length = strlen(printed);
    if (length >= output_size) {
        free(printed);
        return -1;
    }
    memcpy(output, printed, length + 1U);
    free(printed);
    return 0;
}

int system_health_policy_build(
    const system_health_policy_settings_t *settings, const char *override_json,
    system_health_policy_t *policy, char *canonical_json,
    size_t canonical_json_size,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]) {
    clear_error(error);
    if (!policy) return fail(error, "health policy output is required");
    if (canonical_json && canonical_json_size > 0U) canonical_json[0] = '\0';
    if (system_health_policy_validate_settings(settings, error) != 0) return -1;

    system_health_profile_t profile;
    system_health_policy_profile_from_name(settings->profile, &profile);
    memset(policy, 0, sizeof(*policy));
    policy->settings = *settings;
    fill_for_profile(profile, policy);
    if (!settings->enabled) {
        for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
            policy->conditions[index].enabled = false;
        }
    }

    if (override_json && override_json[0] != '\0') {
        size_t length = strlen(override_json);
        if (length > SYSTEM_HEALTH_OVERRIDE_JSON_MAX) {
            return fail(error, "health overrides exceed %u bytes",
                        SYSTEM_HEALTH_OVERRIDE_JSON_MAX);
        }
        const char *parse_end = NULL;
        cJSON *root = cJSON_ParseWithOpts(override_json, &parse_end, true);
        static const char *const root_keys[] = {"version", "conditions"};
        if (!root || !cJSON_IsObject(root) ||
            !object_keys_exact(root, root_keys, 2U, 2U)) {
            cJSON_Delete(root);
            return fail(error, "health overrides must be a versioned conditions object");
        }
        const cJSON *version = cJSON_GetObjectItemCaseSensitive(root, "version");
        const cJSON *conditions = cJSON_GetObjectItemCaseSensitive(root, "conditions");
        if (!cJSON_IsNumber(version) || version->valuedouble != 1.0 ||
            !cJSON_IsArray(conditions)) {
            cJSON_Delete(root);
            return fail(error, "health override version must be 1 with a conditions array");
        }
        int count = cJSON_GetArraySize(conditions);
        if (count < 0 || count > SYSTEM_HEALTH_CONDITION_COUNT) {
            cJSON_Delete(root);
            return fail(error, "too many health condition overrides");
        }
        bool seen[SYSTEM_HEALTH_CONDITION_COUNT] = {false};
        for (int index = 0; index < count; ++index) {
            if (apply_override_item(cJSON_GetArrayItem(conditions, index), seen,
                                    policy, error) != 0) {
                cJSON_Delete(root);
                return -1;
            }
        }
        cJSON_Delete(root);
    }

    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        if (!settings->enabled) policy->conditions[index].enabled = false;
        if (validate_condition(&policy->conditions[index], error) != 0) return -1;
    }
    cJSON *canonical = override_json_for_policy(policy);
    if (!canonical) return fail(error, "could not allocate canonical health overrides");
    int print_result = print_json(canonical, canonical_json, canonical_json_size);
    cJSON_Delete(canonical);
    if (print_result != 0) return fail(error, "canonical health overrides do not fit output buffer");
    return 0;
}

static void initialize_active_policy(void) {
    system_health_policy_settings_t settings;
    system_health_policy_settings_defaults(&settings);
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    if (system_health_policy_build(&settings, NULL, &active_policy, NULL, 0U,
                                   error) != 0) {
        memset(&active_policy, 0, sizeof(active_policy));
    }
    active_policy.generation = 1U;
}

int system_health_policy_replace(
    const system_health_policy_t *candidate,
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH]) {
    clear_error(error);
    if (!candidate) return fail(error, "health policy candidate is required");
    if (system_health_policy_validate_settings(&candidate->settings, error) != 0)
        return -1;
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        if (candidate->conditions[index].condition !=
                (system_health_condition_t)index) {
            return fail(error, "invalid complete health policy at condition %d", index);
        }
        if (validate_condition(&candidate->conditions[index], error) != 0)
            return -1;
    }
    pthread_once(&active_policy_once, initialize_active_policy);
    pthread_rwlock_wrlock(&active_policy_lock);
    uint64_t generation = active_policy.generation == UINT64_MAX
        ? 1U : active_policy.generation + 1U;
    active_policy = *candidate;
    active_policy.generation = generation;
    pthread_rwlock_unlock(&active_policy_lock);
    return 0;
}

int system_health_policy_snapshot(system_health_policy_t *snapshot) {
    if (!snapshot) return -1;
    pthread_once(&active_policy_once, initialize_active_policy);
    pthread_rwlock_rdlock(&active_policy_lock);
    *snapshot = active_policy;
    pthread_rwlock_unlock(&active_policy_lock);
    return 0;
}

int system_health_policy_serialize(const system_health_policy_t *policy,
                                   char *json, size_t json_size) {
    if (!policy || !json || json_size == 0U) return -1;
    cJSON *root = cJSON_CreateObject();
    cJSON *conditions = cJSON_CreateArray();
    if (!root || !conditions) {
        cJSON_Delete(root);
        cJSON_Delete(conditions);
        return -1;
    }
    cJSON_AddBoolToObject(root, "enabled", policy->settings.enabled);
    cJSON_AddStringToObject(root, "profile", policy->settings.profile);
    cJSON_AddNumberToObject(root, "generation", (double)policy->generation);
    cJSON_AddItemToObject(root, "conditions", conditions);
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        const system_health_condition_policy_t *rule = &policy->conditions[index];
        cJSON *item = cJSON_CreateObject();
        if (!item) {
            cJSON_Delete(root);
            return -1;
        }
        cJSON_AddStringToObject(item, "code",
                                system_health_condition_code(rule->condition));
        cJSON_AddBoolToObject(item, "enabled", rule->enabled);
        cJSON_AddStringToObject(item, "profile",
                                system_health_policy_profile_name(rule->profile));
        cJSON_AddStringToObject(item, "unit", system_health_unit_name(rule->unit));
        if (rule->direction != SYSTEM_HEALTH_THRESHOLD_NONE) {
            cJSON_AddNumberToObject(item, "warning", rule->warning_threshold);
            cJSON_AddNumberToObject(item, "critical", rule->critical_threshold);
            cJSON_AddNumberToObject(item, "recovery", rule->recovery_threshold);
        }
        cJSON_AddNumberToObject(item, "warning_for_seconds",
                                rule->warning_for_seconds);
        cJSON_AddNumberToObject(item, "critical_for_seconds",
                                rule->critical_for_seconds);
        cJSON_AddNumberToObject(item, "recovery_for_seconds",
                                rule->recovery_for_seconds);
        cJSON_AddItemToArray(conditions, item);
    }
    int result = print_json(root, json, json_size);
    cJSON_Delete(root);
    return result;
}
