# Product Requirements Document
## ONVIF Motion Detection Recording for LightNVR

### Executive Summary
Implement automated recording functionality triggered by ONVIF motion detection events in LightNVR, enabling efficient storage management and event-based video capture for network video recording systems.

### Problem Statement
LightNVR currently receives ONVIF motion detection events and displays them visually but lacks the ability to automatically trigger recordings based on these events. Users need automated recording capabilities to capture security-relevant footage without continuous recording, optimizing storage usage and simplifying event review.

### Goals & Success Metrics

**Primary Goals:**
- Enable automatic recording triggered by ONVIF motion detection events
- Implement configurable pre/post-event buffer recording
- Provide seamless integration with existing LightNVR detection framework

**Success Metrics:**
- Zero missed motion events that should trigger recording
- Recording starts within 500ms of motion detection
- Pre-buffer captures minimum 5 seconds before event
- Post-buffer continues minimum 10 seconds after motion ends
- Storage efficiency improved by 60-80% vs continuous recording

### User Stories

**As a home security user:**
- I want recordings to automatically start when motion is detected so I don't miss important events
- I want to capture footage before and after motion events for complete context
- I want to easily review motion-triggered recordings without searching through hours of footage

**As a system administrator:**
- I want to configure motion sensitivity and recording parameters per camera
- I want to optimize storage by only recording relevant events
- I want to integrate with existing ONVIF-compliant cameras without additional hardware

### Technical Requirements

#### Core Functionality

**1. ONVIF Event Processing**
- Listen for ONVIF motion detection events via simple_onvif_service
- Parse motion event metadata (timestamp, duration, region)
- Queue events for recording pipeline processing
- Support multiple simultaneous camera events

**2. Recording Engine**
- Implement circular pre-event buffer (configurable 5-30 seconds)
- Trigger recording on motion detection event
- Continue recording for configurable post-event duration (5-60 seconds)
- Handle overlapping motion events gracefully (extend recording vs new file)
- Support multiple codec formats (H.264, H.265)

**3. Buffer Management**
- Maintain rolling buffer of video frames in memory
- Configurable buffer size based on resolution and framerate
- Efficient memory management for multiple camera streams
- Disk-based fallback for resource-constrained systems

**4. File Management**
- Timestamp-based file naming: `camera_name_YYYYMMDD_HHMMSS_motion.mp4`
- Organized directory structure: `/recordings/camera_name/YYYY/MM/DD/`
- Automatic cleanup of old recordings based on retention policy
- Metadata sidecar files with motion event details

#### Configuration Interface

**Per-Camera Settings:**
```yaml
camera:
  name: "front_door"
  onvif:
    motion_detection: enabled
    sensitivity: medium  # low/medium/high
  recording:
    enabled: true
    pre_buffer_seconds: 5
    post_buffer_seconds: 10
    max_file_duration: 300  # seconds
    codec: "h264"
    quality: "high"  # low/medium/high
    retention_days: 30
```

**Global Settings:**
```yaml
recording:
  storage_path: "/var/lib/lightnvr/recordings"
  max_storage_gb: 500
  buffer_memory_mb: 512
  concurrent_recordings: 4
```

### Implementation Phases

#### Phase 1: Core Recording Pipeline (Week 1-2)
- Implement event listener for ONVIF motion events
- Create recording state machine (idle → buffering → recording → finalizing)
- Basic file writing with timestamp naming
- Single camera support

#### Phase 2: Buffer Management (Week 3)
- Implement circular pre-event buffer
- Add post-event recording extension
- Memory optimization for multiple streams
- Handle overlapping events

#### Phase 3: Configuration & Management (Week 4)
- YAML/JSON configuration parsing
- Per-camera recording settings
- Storage management and cleanup
- Basic web UI for configuration

#### Phase 4: Testing & Optimization (Week 5)
- Load testing with multiple cameras
- Edge case handling (network interruptions, storage full)
- Performance optimization
- Documentation

### Technical Architecture

```
ONVIF Camera → simple_onvif_service → Event Queue
                                           ↓
                                    Motion Detector
                                           ↓
                                    Recording Manager
                                    ↙              ↘
                            Buffer Manager    File Writer
                                    ↘              ↙
                                    Storage Manager
```

### Dependencies & Integration Points

**Existing Components:**
- simple_onvif_service (event source)
- Detection framework (event processing)
- Stream manager (video data source)

**New Components Required:**
- Recording state manager
- Circular buffer implementation
- File writer with codec support
- Storage management service

### Non-Functional Requirements

**Performance:**
- Support minimum 16 concurrent camera streams
- Maximum 500ms latency from event to recording start
- CPU usage < 5% per camera stream when idle
- Memory usage < 32MB per camera buffer

**Reliability:**
- Graceful handling of network interruptions
- Automatic recovery from storage issues
- Event persistence across service restarts
- No data loss during high event frequency

**Compatibility:**
- ONVIF Profile S compliance
- Docker container support
- OpenWRT compatibility (resource-constrained)
- Linux (Ubuntu 20.04+, Debian 11+)

### Testing Strategy

**Unit Tests:**
- Event parsing and queuing
- Buffer rotation logic
- File naming and organization
- Configuration validation

**Integration Tests:**
- End-to-end motion → recording flow
- Multiple camera scenarios
- Storage limit handling
- Network failure recovery

**Performance Tests:**
- 16+ camera stress test
- Extended duration testing (24+ hours)
- Storage write performance
- Memory leak detection

### Migration Path
For existing LightNVR users:
1. Preserve existing continuous recording settings
2. Add motion recording as opt-in feature
3. Provide migration tool for existing recordings
4. Default conservative buffer settings to minimize resource impact

### Future Enhancements
- AI-based object detection filtering
- Cloud storage integration
- Mobile app push notifications
- Advanced analytics dashboard
- Multi-zone motion detection regions
- Integration with home automation systems

### Acceptance Criteria
- [ ] Motion events trigger recording within 500ms
- [ ] Pre-buffer captures configured duration before event
- [ ] Post-buffer extends recording after motion stops
- [ ] Recordings are properly timestamped and organized
- [ ] Configuration changes apply without service restart
- [ ] System handles 16 cameras simultaneously
- [ ] Storage cleanup respects retention policies
- [ ] All existing functionality remains operational
- [ ] Documentation covers setup and configuration
- [ ] Docker image includes all dependencies