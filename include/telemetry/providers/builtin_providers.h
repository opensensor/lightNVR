/** @file builtin_providers.h Optional hardware-provider registration seam. */

#ifndef LIGHTNVR_BUILTIN_HEALTH_PROVIDERS_H
#define LIGHTNVR_BUILTIN_HEALTH_PROVIDERS_H

#include <stdbool.h>
#include <stddef.h>

#include "telemetry/system_health_provider.h"

typedef bool (*system_health_provider_registration_fn)(
    const system_health_provider_t *provider, void *context);

/** T17/T18 add providers here; the portable baseline intentionally has none. */
size_t system_health_register_builtin_providers(
    const char *selection, system_health_provider_registration_fn registration,
    void *context);

/** Request an immediate device pass only for the explicitly selected backend. */
bool system_health_builtin_provider_request_device_refresh(void);

#endif /* LIGHTNVR_BUILTIN_HEALTH_PROVIDERS_H */
