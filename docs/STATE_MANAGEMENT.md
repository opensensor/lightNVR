# State Management Architecture

This document describes the state management architecture implemented to resolve issues with TCP vs UDP streams and configuration state changes.

## Overview

The state management architecture provides a centralized system for managing stream states, configurations, and protocol-specific behaviors. It addresses several key issues:

1. **TCP vs UDP Protocol Handling**: Different protocols require different handling for timestamps, buffering, and reconnection strategies.
2. **Configuration State Changes**: Changes to stream configuration (like switching protocols) need to be properly propagated to all components.
3. **Race Conditions**: Multiple components accessing and modifying stream state can lead to race conditions.
4. **Feature Toggling**: Enabling/disabling features like streaming, recording, and detection needs to be handled consistently.

## Architecture Components

### Stream State Manager

The core of the architecture is the `stream_state_manager_t` structure, which centralizes all state information for a stream:

- **Basic Information**: Stream name, configuration, and operational state
- **Feature Flags**: Which features are enabled (streaming, recording, detection)
- **Protocol State**: Protocol-specific settings and state (TCP/UDP, multicast, buffer sizes)
- **Timestamp State**: For handling timestamps in different protocols
- **Statistics**: Performance and error metrics

### State Transitions

The architecture defines clear state transitions for streams:

- **INACTIVE**: Stream is not active
- **STARTING**: Stream is in the process of starting
- **ACTIVE**: Stream is active and running
- **ERROR**: Stream encountered an error
- **RECONNECTING**: Stream is attempting to reconnect

### Thread Safety

All state access is protected by mutexes to prevent race conditions:

- Each stream state manager has its own mutex for thread-safe access
- Global state is protected by a separate mutex
- Timestamp handling is thread-safe, even for UDP streams

## Implementation Details

### Stream State Manager

The stream state manager (`stream_state_manager_t`) is implemented in:

- `include/video/stream_state.h`: Interface definition
- `src/video/stream_state.c`: Implementation

It provides functions for:

- Creating and managing stream states
- Starting and stopping streams
- Updating stream configuration
- Setting feature flags
- Handling errors and reconnections

### Adapter Layer

To maintain backward compatibility, an adapter layer is provided:

- `include/video/stream_state_adapter.h`: Interface definition
- `src/video/stream_state_adapter.c`: Implementation

This layer maps between the old API (`stream_handle_t`) and the new state management system.

### Packet Processing

Packet processing is enhanced to handle protocol-specific behaviors:

- `include/video/stream_packet_processor.h`: Interface definition
- `src/video/stream_packet_processor.c`: Implementation

The packet processor:

- Handles timestamp correction for UDP streams
- Updates stream statistics
- Manages protocol-specific packet handling
- Provides an adapter for backward compatibility

## Protocol-Specific Handling

### TCP Streams

For TCP streams, the architecture:

- Uses reliable timestamps from the stream
- Sets appropriate buffer sizes and timeouts
- Implements a standard reconnection strategy

### UDP Streams

For UDP streams, the architecture:

- Implements robust timestamp correction and tracking
- Detects and handles timestamp discontinuities
- Uses larger buffer sizes to handle jitter
- Implements a specialized reconnection strategy for UDP
- Handles multicast streams appropriately

## Configuration Changes

When configuration changes occur:

1. The change is applied to the stream state manager
2. The change is propagated to all relevant components
3. If necessary, the stream is restarted to apply the changes
4. All components use the updated configuration

## Feature Toggling

Features can be toggled independently:

- **Streaming**: Enable/disable HLS streaming
- **Recording**: Enable/disable MP4 recording
- **Detection**: Enable/disable object detection
- **Motion Detection**: Enable/disable motion detection

When a feature is toggled:

1. The feature flag is updated in the stream state manager
2. The corresponding component is started or stopped
3. The configuration is updated to reflect the change

## Benefits

This architecture provides several benefits:

1. **Centralized State**: All state information is in one place, making it easier to reason about
2. **Thread Safety**: Proper synchronization prevents race conditions
3. **Protocol-Specific Handling**: Different protocols are handled appropriately
4. **Clean Feature Toggling**: Features can be enabled/disabled independently
5. **Backward Compatibility**: Existing code continues to work through the adapter layer

## Migration

Existing code can be migrated to the new architecture gradually:

1. Use the adapter functions to interact with the new state management system
2. Replace direct calls to the old API with calls to the adapter
3. Eventually, migrate to using the new API directly

## Future Enhancements

Possible future enhancements include:

1. **Event System**: Add an event system for state changes
2. **Configuration Validation**: Add more robust configuration validation
3. **Dynamic Protocol Switching**: Allow switching protocols without restarting the stream
4. **Enhanced Statistics**: Add more detailed performance metrics
5. **Health Monitoring**: Add health checks and automatic recovery
