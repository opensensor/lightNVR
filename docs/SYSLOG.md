# Syslog Integration

LightNVR supports logging to syslog for easier system integration and centralized log management.

## Overview

When syslog is enabled, LightNVR will send log messages to the system's syslog daemon in addition to the regular log file and console output. This provides several benefits:

- **Centralized Logging**: Integrate with system-wide log management tools
- **Remote Logging**: Forward logs to remote syslog servers for centralized monitoring
- **Log Rotation**: Leverage system log rotation policies
- **Integration with Monitoring Tools**: Use tools like `journalctl`, `rsyslog`, or `syslog-ng`
- **Standardized Format**: Follow standard syslog conventions for log levels and formatting

## Configuration

Syslog can be enabled and configured in the `lightnvr.ini` configuration file under the `[general]` section:

```ini
[general]
pid_file = /var/run/lightnvr.pid
log_file = /var/log/lightnvr.log
log_level = 2  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG
syslog_enabled = true  ; Enable logging to syslog
syslog_ident = lightnvr  ; Syslog identifier (application name)
syslog_facility = LOG_USER  ; Syslog facility
```

### Configuration Options

- **`syslog_enabled`** (boolean): Enable or disable syslog logging
  - Default: `false`
  - Set to `true` to enable syslog integration

- **`syslog_ident`** (string): Syslog identifier/application name
  - Default: `"lightnvr"`
  - This identifier appears in syslog messages and can be used for filtering
  - Maximum length: 63 characters

- **`syslog_facility`** (string or integer): Syslog facility for categorizing messages
  - Default: `LOG_USER`
  - Valid values:
    - `LOG_USER` (8): User-level messages (default)
    - `LOG_DAEMON` (24): System daemon messages
    - `LOG_LOCAL0` (128) through `LOG_LOCAL7` (184): Local use facilities for custom applications
  - Can be specified as either the name (e.g., `LOG_USER`) or numeric value (e.g., `8`)

## Log Level Mapping

LightNVR log levels are mapped to syslog priorities as follows:

| LightNVR Level | Syslog Priority | Description |
|----------------|-----------------|-------------|
| ERROR (0)      | LOG_ERR (3)     | Error conditions |
| WARN (1)       | LOG_WARNING (4) | Warning conditions |
| INFO (2)       | LOG_INFO (6)    | Informational messages |
| DEBUG (3)      | LOG_DEBUG (7)   | Debug-level messages |

## Viewing Syslog Messages

### Using journalctl (systemd-based systems)

```bash
# View all LightNVR logs
journalctl -t lightnvr

# Follow logs in real-time
journalctl -t lightnvr -f

# View logs from the last hour
journalctl -t lightnvr --since "1 hour ago"

# View logs with specific priority
journalctl -t lightnvr -p err  # Only errors
journalctl -t lightnvr -p warning  # Warnings and above

# View logs for a specific time range
journalctl -t lightnvr --since "2024-01-01 00:00:00" --until "2024-01-01 23:59:59"
```

### Using traditional syslog

On systems using traditional syslog (rsyslog, syslog-ng), logs will appear in:
- `/var/log/syslog` (Debian/Ubuntu)
- `/var/log/messages` (RHEL/CentOS)

```bash
# View LightNVR logs
tail -f /var/log/syslog | grep lightnvr

# Search for specific messages
grep lightnvr /var/log/syslog
```

## Production Deployment

For production deployments, it's recommended to:

1. **Enable syslog** for centralized logging and monitoring
2. **Use a dedicated facility** (e.g., `LOG_LOCAL0`) to separate LightNVR logs from other applications
3. **Configure log rotation** through your syslog daemon
4. **Set up remote logging** if using centralized log management

### Example Production Configuration

```ini
[general]
syslog_enabled = true
syslog_ident = lightnvr
syslog_facility = LOG_LOCAL0
```

### Configuring rsyslog for Remote Logging

To forward LightNVR logs to a remote syslog server, add to `/etc/rsyslog.d/lightnvr.conf`:

```
# Forward LightNVR logs to remote server
local0.* @@remote-log-server:514
```

Then restart rsyslog:
```bash
sudo systemctl restart rsyslog
```

## Behavior

- **Dual Output**: When syslog is enabled, messages are sent to BOTH syslog AND the regular log file/console
- **No Performance Impact**: Syslog writes are asynchronous and don't block the application
- **Automatic PID**: The process ID is automatically included in syslog messages
- **Graceful Fallback**: If syslog is unavailable, messages still go to the log file and console

## Troubleshooting

### Syslog messages not appearing

1. Check that syslog is enabled in the configuration:
   ```bash
   grep syslog_enabled /etc/lightnvr/lightnvr.ini
   ```

2. Verify the syslog daemon is running:
   ```bash
   sudo systemctl status rsyslog  # or syslog-ng
   ```

3. Check syslog configuration for filtering rules that might be blocking messages

4. Verify permissions - LightNVR needs to be able to write to `/dev/log`

### Messages appearing with wrong facility

Check the `syslog_facility` setting in the configuration file and ensure it matches your syslog daemon's configuration.

## API

The syslog functionality is exposed through the following C API functions:

```c
// Enable syslog logging
int enable_syslog(const char *ident, int facility);

// Disable syslog logging
void disable_syslog(void);

// Check if syslog is enabled
int is_syslog_enabled(void);
```

These functions are automatically called during LightNVR initialization based on the configuration file settings.

