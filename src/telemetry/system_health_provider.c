#include "telemetry/system_health_provider.h"

#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#define PROVIDER_CAPACITY SYSTEM_HEALTH_MAX_DEVICES

static system_health_provider_t providers[PROVIDER_CAPACITY];
static size_t provider_count;

void system_health_provider_registry_reset(void) {
    memset(providers, 0, sizeof(providers));
    provider_count = 0U;
}

bool system_health_provider_registry_register(
    const system_health_provider_t *provider) {
    if (!provider || !provider->name[0] || !provider->collect ||
        !memchr(provider->name, '\0', sizeof(provider->name)) ||
        provider_count >= PROVIDER_CAPACITY) {
        return false;
    }
    for (size_t index = 0; index < provider_count; ++index) {
        if (strcmp(providers[index].name, provider->name) == 0) return false;
    }
    providers[provider_count++] = *provider;
    return true;
}

int system_health_provider_registry_collect(
    const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink, size_t *resources_dropped) {
    int failures = 0;
    size_t dropped = 0U;
    for (size_t index = 0; index < provider_count; ++index) {
        system_health_provider_t *provider = &providers[index];
        if (provider->discover) {
            system_health_provider_inventory_t inventory;
            memset(&inventory, 0, sizeof(inventory));
            if (provider->discover(provider->state, context, &inventory) != 0)
                failures++;
            dropped += inventory.dropped;
        }
        if (provider->collect(provider->state, context, sink) != 0) failures++;
    }
    if (resources_dropped) *resources_dropped = dropped;
    return failures == 0 ? 0 : -1;
}

void system_health_provider_registry_destroy(void) {
    for (size_t index = 0; index < provider_count; ++index) {
        if (providers[index].destroy)
            providers[index].destroy(providers[index].state);
    }
    system_health_provider_registry_reset();
}

size_t system_health_provider_registry_count(void) { return provider_count; }
