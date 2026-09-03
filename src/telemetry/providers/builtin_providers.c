#include "telemetry/providers/builtin_providers.h"

#include <stdatomic.h>
#include <string.h>

#include "core/event_identity.h"
#include "telemetry/providers/kernel_log.h"
#include "telemetry/providers/linux_hardware.h"
#include "telemetry/providers/nvme_provider.h"
#include "telemetry/providers/smartctl_provider.h"

static linux_hardware_state_t linux_hardware_state;
static nvme_provider_state_t nvme_state;
static kernel_log_state_t kernel_state;
static smartctl_provider_state_t smartctl_state;
static atomic_bool smartctl_active;

static bool register_provider(system_health_provider_registration_fn callback,
                              void *context,
                              system_health_provider_t *provider) {
    return callback(provider, context);
}

size_t system_health_register_builtin_providers(
    const char *selection, system_health_provider_registration_fn registration,
    void *context) {
    atomic_store_explicit(&smartctl_active, false, memory_order_release);
    if (!registration || !selection ||
        (strcmp(selection, "auto") != 0 &&
         strcmp(selection, "smartctl") != 0))
        return 0U;
    bool smartctl_selected = strcmp(selection, "smartctl") == 0;

    char installation_scope[LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH] = {0};
    (void)event_identity_get_source(installation_scope,
                                    sizeof(installation_scope));
    linux_hardware_state_init(&linux_hardware_state, installation_scope);
    nvme_provider_state_init(&nvme_state, installation_scope);
    kernel_log_state_init(&kernel_state, "/dev/kmsg");
    smartctl_provider_state_init(&smartctl_state, installation_scope);
    smartctl_state.discover_target_devices = true;

    size_t registered = 0U;
    system_health_provider_t provider;
    linux_hardware_provider_init(&provider, &linux_hardware_state);
    if (register_provider(registration, context, &provider)) registered++;
    if (smartctl_selected)
        smartctl_provider_init(&provider, &smartctl_state);
    else
        nvme_provider_init(&provider, &nvme_state);
    bool device_registered = register_provider(registration, context,
                                               &provider);
    if (device_registered) registered++;
    if (smartctl_selected && device_registered)
        atomic_store_explicit(&smartctl_active, true, memory_order_release);
    kernel_log_provider_init(&provider, &kernel_state);
    if (register_provider(registration, context, &provider)) registered++;
    return registered;
}

bool system_health_builtin_provider_request_device_refresh(void) {
    if (!atomic_load_explicit(&smartctl_active, memory_order_acquire))
        return false;
    return smartctl_provider_request_refresh(&smartctl_state);
}
