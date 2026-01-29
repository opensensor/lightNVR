# ONVIF Network Override Implementation

## Summary

This implementation adds support for overriding the ONVIF discovery network via environment variable, making it easier to use ONVIF camera discovery in containerized deployments.

## Problem Statement

When running LightNVR in a Docker container, the automatic network detection skips Docker bridge interfaces (`docker*`, `veth*`, `br-*`, `lxc*`). This prevents ONVIF discovery from working in containerized environments where cameras may be on the host network or external networks accessible through Docker networking.

## Solution

Added a priority-based network selection system with support for environment variable override:

### Priority Order

1. **Explicit parameter** - Network passed directly to `discover_onvif_devices()` function
2. **Environment variable** - `LIGHTNVR_ONVIF_NETWORK` (new)
3. **Config file** - `[onvif]` section `discovery_network` setting (new)
4. **Auto-detection** - Existing behavior (skips Docker interfaces)

## Implementation Details

### 1. Configuration Structure (`include/core/config.h`)

Already had the fields defined:
```c
bool onvif_discovery_enabled;
int onvif_discovery_interval;
char onvif_discovery_network[64];
```

### 2. Default Configuration (`src/core/config.c`)

Added initialization in `load_default_config()`:
```c
config->onvif_discovery_enabled = false;
config->onvif_discovery_interval = 300;  // 5 minutes
snprintf(config->onvif_discovery_network, sizeof(config->onvif_discovery_network), "auto");
```

### 3. Config File Parsing (`src/core/config.c`)

Added `[onvif]` section handler in `config_ini_handler()`:
```c
else if (strcmp(section, "onvif") == 0) {
    if (strcmp(name, "discovery_enabled") == 0) {
        config->onvif_discovery_enabled = (strcmp(value, "true") == 0 || strcmp(value, "1") == 0);
    } else if (strcmp(name, "discovery_interval") == 0) {
        config->onvif_discovery_interval = atoi(value);
        // Clamped to 30-3600 seconds
    } else if (strcmp(name, "discovery_network") == 0) {
        strncpy(config->onvif_discovery_network, value, sizeof(config->onvif_discovery_network) - 1);
    }
}
```

### 4. Priority-Based Network Selection (`src/video/onvif_discovery.c`)

Modified `discover_onvif_devices()` to implement complete priority order:
```c
if (!network || strlen(network) == 0 || strcmp(network, "auto") == 0) {
    // Priority 1: Check environment variable
    const char *env_network = getenv("LIGHTNVR_ONVIF_NETWORK");
    if (env_network && strlen(env_network) > 0 && strcmp(env_network, "auto") != 0) {
        log_info("Using ONVIF discovery network from environment variable: %s", env_network);
        network = env_network;
    }
    // Priority 2: Check config file
    else if (g_config.onvif_discovery_network[0] != '\0' &&
             strcmp(g_config.onvif_discovery_network, "auto") != 0) {
        log_info("Using ONVIF discovery network from config file: %s", g_config.onvif_discovery_network);
        network = g_config.onvif_discovery_network;
    }
    // Priority 3: Auto-detect
    else {
        network_count = detect_local_networks(detected_networks, MAX_DETECTED_NETWORKS);
        // ... use first detected network
    }
}
```

### 5. Documentation Updates

- **config/lightnvr.ini** - Added `[onvif]` section with examples
- **docs/DOCKER.md** - Added environment variable documentation and usage examples
- **docker-compose.yml** - Added commented example for `LIGHTNVR_ONVIF_NETWORK`
- **docker-entrypoint.sh** - Added `[onvif]` section to default config template

## Usage Examples

### Docker Compose

```yaml
services:
  lightnvr:
    environment:
      - LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
```

### Docker Run

```bash
docker run -e LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24 \
  -p 8080:8080 \
  ghcr.io/opensensor/lightnvr:latest
```

### Config File

```ini
[onvif]
discovery_enabled = true
discovery_interval = 300
discovery_network = 192.168.1.0/24
```

## Finding Your Network

On the Docker host:
```bash
ip addr show
# If host IP is 192.168.1.100 with netmask 255.255.255.0
# Use: LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
```

## Testing

The implementation maintains backward compatibility:
- Existing auto-detection still works when no override is set
- Explicit function parameters still take highest priority
- Config file parsing is optional (defaults to "auto")

## Files Modified

1. `src/core/config.c` - Added ONVIF defaults and config parsing
2. `src/video/onvif_discovery.c` - Added environment variable check
3. `config/lightnvr.ini` - Added `[onvif]` section
4. `docs/DOCKER.md` - Added documentation
5. `docker-compose.yml` - Added example
6. `docker-entrypoint.sh` - Added default config template

## Benefits

- **Container-friendly** - Easy to configure via environment variables
- **Flexible** - Multiple configuration methods
- **Backward compatible** - Existing behavior unchanged
- **Well-documented** - Clear examples for users

