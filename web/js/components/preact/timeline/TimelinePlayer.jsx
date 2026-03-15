/**
 * LightNVR Timeline Player Component
 * Handles video playback for the timeline
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { formatPlaybackTimeLabel, resolvePlaybackStreamName } from './timelineUtils.js';
import { SpeedControls } from './SpeedControls.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog } from '../UI.jsx';
import { formatFilenameTimestamp, formatLocalDateTime, toUnixSeconds } from '../../../utils/date-utils.js';
import { useI18n } from '../../../i18n.js';

// Timeout for cleaning up preloaded temporary video elements (in milliseconds).
const PRELOAD_CLEANUP_TIMEOUT_MS = 15000;
const DETECTION_TIME_WINDOW_SECONDS = 2; // Time window (seconds) for filtering visible detections around current playback time
const DETECTION_SCALE_BASE = 400; // Baseline display dimension (px) for detection overlay scaling

/**
 * TimelinePlayer component
 * @returns {JSX.Element} TimelinePlayer component
 */
export function TimelinePlayer({ videoElementRef = null }) {
  const { t } = useI18n();
  // Local state
  const [currentSegmentIndex, setCurrentSegmentIndex] = useState(-1);
  const [segments, setSegments] = useState([]);
  const [playbackSpeed, setPlaybackSpeed] = useState(1.0);
  const [detections, setDetections] = useState([]);
  const [detectionOverlayEnabled, setDetectionOverlayEnabled] = useState(false);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [segmentRecordingData, setSegmentRecordingData] = useState(null);

  // Refs
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const videoContainerRef = useRef(null);
  const lastTimeUpdateRef = useRef(null);
  const lastSegmentIdRef = useRef(null);
  const lastDetectionSegmentIdRef = useRef(null);
  const preloadedVideoCleanupRef = useRef(null);
  const initialPausedSyncEventLoggedRef = useRef(false);
  const lastInitialPausedSegmentIdRef = useRef(null);

  const setVideoRefs = useCallback((node) => {
    videoRef.current = node;

    if (!videoElementRef) {
      return;
    }

    if (typeof videoElementRef === 'function') {
      videoElementRef(node);
      return;
    }

    videoElementRef.current = node;
  }, [videoElementRef]);

  const releaseDirectVideoControl = useCallback(() => {
    if (!timelineState.directVideoControl) {
      return;
    }

    timelineState.directVideoControl = false;
    timelineState.setState({});
  }, []);

  const cleanupPreloadedVideo = useCallback(() => {
    if (typeof preloadedVideoCleanupRef.current !== 'function') {
      return;
    }

    preloadedVideoCleanupRef.current();
    preloadedVideoCleanupRef.current = null;
  }, []);

  // Handle video playback based on state changes
  const handleVideoPlayback = useCallback((state) => {
    const video = videoRef.current;
    if (!video) return;

    // Check if we have valid segments and segment index
    if (!state.timelineSegments ||
        state.timelineSegments.length === 0 ||
        state.currentSegmentIndex < 0 ||
        state.currentSegmentIndex >= state.timelineSegments.length) {
      return;
    }

    // Get current segment
    const segment = state.timelineSegments[state.currentSegmentIndex];
    if (!segment) return;

    // Check if we need to load a new segment
    const segmentChanged = lastSegmentIdRef.current !== segment.id;

    // IMPORTANT: Only reload if the segment has actually changed
    // This prevents constant reloading
    const needsReload = segmentChanged;

    // Calculate relative time within the segment
    let relativeTime = 0;

    if (state.currentTime !== null) {
      if (state.currentTime >= segment.start_timestamp && state.currentTime <= segment.end_timestamp) {
        // If current time is within this segment, calculate the relative position
        relativeTime = state.currentTime - segment.start_timestamp;
        console.log(`Current time ${state.currentTime} is within segment ${segment.id}, relative time: ${relativeTime}s`);
      } else if (state.currentTime < segment.start_timestamp) {
        // If current time is before this segment, start at the beginning
        relativeTime = 0;
        console.log(`Current time ${state.currentTime} is before segment ${segment.id}, starting at beginning`);
      } else {
        // If current time is after this segment, start at the end
        relativeTime = segment.end_timestamp - segment.start_timestamp;
        console.log(`Current time ${state.currentTime} is after segment ${segment.id}, starting at end`);
      }
    }

    // Only update the video if:
    // 1. We need to load a new segment, OR
    // 2. The user is dragging the cursor (indicated by a significant time change)
    const timeChanged = state.prevCurrentTime !== null &&
                        Math.abs(state.currentTime - state.prevCurrentTime) > 1;

    // Update last segment ID
    if (segmentChanged) {
      console.log(`Segment changed from ${lastSegmentIdRef.current} to ${segment.id}`);
      lastSegmentIdRef.current = segment.id;
    }

    // Handle playback
    if (needsReload) {
      // Load new segment
      console.log(`Loading new segment ${segment.id} (segmentChanged: ${segmentChanged})`);
      loadSegment(segment, relativeTime, state.isPlaying);
    } else if (timeChanged) {
      // User is dragging the cursor, just update the current time
      console.log(`Seeking to ${relativeTime}s within current segment`);
      video.currentTime = relativeTime;
    } else if (state.isPlaying && video.paused) {
      // Resume playback if needed
      video.play().catch(error => {
        if (error.name === 'AbortError') {
          // play() was interrupted by a new load or pause — expected when clicking around the timeline
          console.log('Video play() interrupted, ignoring AbortError');
          return;
        }
        console.error('Error playing video:', error);
      });
    } else if (!state.isPlaying && !video.paused) {
      // Pause playback if needed
      video.pause();
    }

    // Update playback speed if needed
    if (video.playbackRate !== state.playbackSpeed) {
      video.playbackRate = state.playbackSpeed;
    }
  }, [
    cleanupPreloadedVideo,
    releaseDirectVideoControl,
    playbackSpeed,
    detectionOverlayEnabled,
  ]);

  // Subscribe to timeline state changes
  useEffect(() => {
    const listener = state => {
      // Update local state
      setCurrentSegmentIndex(state.currentSegmentIndex);
      setSegments(state.timelineSegments || []);
      setPlaybackSpeed(state.playbackSpeed);

      // Handle video playback
      handleVideoPlayback(state);
    };

    const unsubscribe = timelineState.subscribe(listener);

    // Initialize with current state immediately so we don't miss
    // segments that were already loaded before this component mounted
    listener(timelineState);

    return () => unsubscribe();
  }, [handleVideoPlayback]);

  useEffect(() => cleanupPreloadedVideo, [cleanupPreloadedVideo]);

  // Load a segment
  const loadSegment = useCallback((segment, seekTime = 0, autoplay = false) => {
    const video = videoRef.current;
    if (!video) return;

    console.log(`Loading segment ${segment.id} at time ${seekTime}s, autoplay: ${autoplay}`);

    // Pause current playback
    video.pause();

    // Set new source using a deterministic version to allow caching of identical segments
    const segmentVersion = `${segment.id}-${segment.start_timestamp}-${segment.end_timestamp}`;
    const recordingUrl = `/api/recordings/play/${segment.id}?v=${encodeURIComponent(segmentVersion)}`;

    // Set up event listeners for the new video
    const onLoadedMetadata = () => {
      console.log('Video metadata loaded');

      // Check if the current time is within this segment
      // If so, use the relative position from the cursor
      let timeToSet = seekTime;

      // Check if we should preserve the cursor position
      if (timelineState.preserveCursorPosition && timelineState.currentTime !== null) {
        // Calculate the relative time within the segment
        if (timelineState.currentTime >= segment.start_timestamp &&
            timelineState.currentTime <= segment.end_timestamp) {
          // If the cursor is within this segment, use its position
          timeToSet = timelineState.currentTime - segment.start_timestamp;
          console.log(`TimelinePlayer: Using locked cursor position for playback: ${timeToSet}s`);
        } else {
          // If the cursor is outside this segment but we want to preserve its position,
          // use the beginning or end of the segment based on which is closer
          const distanceToStart = Math.abs(timelineState.currentTime - segment.start_timestamp);
          const distanceToEnd = Math.abs(timelineState.currentTime - segment.end_timestamp);

          if (distanceToStart <= distanceToEnd) {
            timeToSet = 0; // Use start of segment
            console.log(`TimelinePlayer: Cursor outside segment, using start of segment`);
          } else {
            timeToSet = segment.end_timestamp - segment.start_timestamp; // Use end of segment
            console.log(`TimelinePlayer: Cursor outside segment, using end of segment`);
          }
        }
      } else if (timelineState.currentTime !== null &&
                 timelineState.currentTime >= segment.start_timestamp &&
                 timelineState.currentTime <= segment.end_timestamp) {
        // If not preserving cursor position but the current time is within this segment,
        // still use the relative position from the cursor
        timeToSet = timelineState.currentTime - segment.start_timestamp;
        console.log(`TimelinePlayer: Using cursor position for playback: ${timeToSet}s`);
      }

      // Set current time, ensuring it's within valid bounds
      const segmentDuration = segment.end_timestamp - segment.start_timestamp;

      // Add a small buffer for positions near the beginning of the segment
      // This prevents the cursor from snapping to the start
      let validSeekTime = Math.min(Math.max(0, timeToSet), segmentDuration);

      // If we're very close to the start of the segment but not exactly at the start,
      // add a small offset to prevent snapping to the start
      if (validSeekTime > 0 && validSeekTime < 0.1) {
        // Ensure we're at least 0.1 seconds into the segment
        validSeekTime = 0.1;
        console.log(`TimelinePlayer: Adjusting seek time to ${validSeekTime}s to prevent snapping to segment start`);
      }

      console.log(`TimelinePlayer: Setting video time to ${validSeekTime}s (requested: ${timeToSet}s, segment duration: ${segmentDuration}s)`);
      video.currentTime = validSeekTime;

      // Set playback speed
      video.playbackRate = playbackSpeed;

      // Play if needed
      if (autoplay) {
        video.play().catch(error => {
          if (error.name === 'AbortError') {
            console.log('Video play() interrupted during segment load, ignoring AbortError');
            return;
          }
          console.error('Error playing video:', error);
          showStatusMessage(t('timeline.errorPlayingVideo', { message: error.message }), 'error');
        });
      }

      // Remove event listener
      video.removeEventListener('loadedmetadata', onLoadedMetadata);
    };

    // Add event listener for metadata loaded
    video.addEventListener('loadedmetadata', onLoadedMetadata);

    // Set new source
    video.src = recordingUrl;
    video.load();
  }, [videoRef, playbackSpeed]);

  // Handle video ended event
  const handleEnded = () => {
    if (currentSegmentIndex < segments.length - 1) {
      const nextIndex = currentSegmentIndex + 1;
      const nextSegment = segments[nextIndex];

      // Warm the browser cache for the next segment in the background using a
      // temporary video element.  The actual loading and seeking is handled by
      // handleVideoPlayback → loadSegment once we update the state below.
      cleanupPreloadedVideo();
      const nextVideoUrl = `/api/recordings/play/${nextSegment.id}`;
      const tempVideo = document.createElement('video');
      tempVideo.preload = 'auto';
      tempVideo.src = nextVideoUrl;

      let tempCleanupTimeoutId = null;
      const cleanupTempVideo = () => {
        tempVideo.removeEventListener('loadeddata', cleanupTempVideo);
        tempVideo.removeEventListener('error', cleanupTempVideo);
        try { tempVideo.pause(); } catch (e) { /* ignore */ }
        tempVideo.removeAttribute('src');
        try { tempVideo.load(); } catch (e) { /* ignore */ }
        if (tempCleanupTimeoutId !== null) {
          clearTimeout(tempCleanupTimeoutId);
          tempCleanupTimeoutId = null;
        }
        if (preloadedVideoCleanupRef.current === cleanupTempVideo) {
          preloadedVideoCleanupRef.current = null;
        }
      };
      preloadedVideoCleanupRef.current = cleanupTempVideo;
      tempVideo.addEventListener('loadeddata', cleanupTempVideo);
      tempVideo.addEventListener('error', cleanupTempVideo);
      tempCleanupTimeoutId = setTimeout(cleanupTempVideo, PRELOAD_CLEANUP_TIMEOUT_MS);
      tempVideo.load();

      // Update state — handleVideoPlayback detects the new segment ID and calls
      // loadSegment(nextSegment, 0, true), which handles the actual video work.
      // Always start the next segment from the beginning so the seek position is
      // unambiguous (avoids the race where the old cursor position was used).
      timelineState.setState({
        currentSegmentIndex: nextIndex,
        currentTime: nextSegment.start_timestamp,
        prevCurrentTime: timelineState.currentTime,
        isPlaying: true,
      });
    } else {
      // End of all segments
      timelineState.setState({ isPlaying: false });
    }
  };

  // Handle native video play event (user pressed play inside the browser video controls)
  const handlePlay = () => {
    if (!timelineState.isPlaying) {
      timelineState.setState({ isPlaying: true });
    }
  };

  // Handle native video pause event (user pressed pause inside the browser video controls)
  const handlePause = () => {
    // Only sync if our own code didn't trigger the pause (e.g. during segment load)
    if (timelineState.isPlaying && !timelineState.directVideoControl) {
      timelineState.setState({ isPlaying: false });
    }
  };

  // Handle video time update event
  const handleTimeUpdate = () => {
    const video = videoRef.current;
    if (!video) return;

    // Use global timeline state instead of local component state to avoid stale closure
    // issues during segment transitions. When the video fires an early timeupdate (e.g.
    // with video.currentTime === 0 immediately after src/load), the Preact re-render with
    // the new currentSegmentIndex may not have happened yet, so the local closure value
    // would still point to the previous segment and produce a wrong currentTime.
    const globalSegmentIndex = timelineState.currentSegmentIndex;
    const globalSegments = timelineState.timelineSegments;

    if (globalSegmentIndex < 0 ||
        !globalSegments ||
        globalSegments.length === 0 ||
        globalSegmentIndex >= globalSegments.length) {
      return;
    }

    const segment = globalSegments[globalSegmentIndex];
    if (!segment) return;

    const desiredTime = timelineState.currentTime;
    const isInitialPausedSyncEvent =
      !timelineState.isPlaying &&
      video.currentTime === 0 &&
      desiredTime !== null &&
      desiredTime > segment.start_timestamp &&
      desiredTime <= segment.end_timestamp;

    if (lastInitialPausedSegmentIdRef.current !== segment.id) {
      lastInitialPausedSegmentIdRef.current = segment.id;
      initialPausedSyncEventLoggedRef.current = false;
    }

    if (isInitialPausedSyncEvent) {
      if (!initialPausedSyncEventLoggedRef.current) {
        initialPausedSyncEventLoggedRef.current = true;
        console.log(
          'TimelinePlayer: Ignoring initial 0s timeupdate before seek position is applied ' +
          `(segmentId=${segment.id}, segmentStart=${segment.start_timestamp}, ` +
          `desiredTime=${desiredTime}, videoTime=${video.currentTime})`
        );
      }
      return;
    }

    // Calculate current timestamp, handling timezone correctly
    const currentTime = segment.start_timestamp + video.currentTime;

    // Log the current time for debugging
    console.log('TimelinePlayer: Current time', {
      videoTime: video.currentTime,
      segmentStart: segment.start_timestamp,
      calculatedTime: currentTime,
      localTime: formatLocalDateTime(currentTime)
    });

    // Directly update the time display as well
    updateTimeDisplay(currentTime, segment);

    // Check if the user is controlling the cursor
    // If so, don't update the timeline state to avoid overriding the user's position
    if (timelineState.userControllingCursor) {
      console.log('TimelinePlayer: User is controlling cursor, not updating timeline state');
      return;
    }

    // Check if cursor position is locked
    // If so, don't update the timeline state to preserve the cursor position
    if (timelineState.cursorPositionLocked) {
      console.log('TimelinePlayer: Cursor position is locked, not updating timeline state');
      return;
    }

    // Check if directVideoControl flag is set
    // If so, don't update the timeline state to avoid conflicts with TimelineControls
    if (timelineState.directVideoControl) {
      console.log('TimelinePlayer: Direct video control active, not updating timeline state');
      return;
    }

    // Update timeline state with the current time
    timelineState.setState({
      currentTime: currentTime,
      prevCurrentTime: lastTimeUpdateRef.current
    });

    // Update last time update
    lastTimeUpdateRef.current = currentTime;
  };

  // Add a direct time display update function
  const updateTimeDisplay = (time, segment = null) => {
    const timeDisplay = document.getElementById('time-display');
    if (!timeDisplay) return;
    const streamName = segment?.stream
      || segmentRecordingData?.stream
      || resolvePlaybackStreamName(segments, currentSegmentIndex, time);
    const formatted = formatPlaybackTimeLabel(time, streamName);
    timeDisplay.textContent = formatted || '00:00:00';

    console.log('TimelinePlayer: Updated time display', {
      timestamp: time,
      formatted,
      stream: streamName || null,
      localTime: formatLocalDateTime(time)
    });
  };

  // Fetch recording data and detections when segment changes
  useEffect(() => {
    if (currentSegmentIndex < 0 || !segments || segments.length === 0 ||
        currentSegmentIndex >= segments.length) {
      setDetections([]);
      setSegmentRecordingData(null);
      lastDetectionSegmentIdRef.current = null;
      timelineState.setState({
        currentRecordingId: null,
        currentRecordingProtected: false,
        currentRecordingTags: []
      });
      return;
    }

    const segment = segments[currentSegmentIndex];
    if (!segment || !segment.id) return;

    // Don't refetch if same segment
    if (lastDetectionSegmentIdRef.current === segment.id) return;

    setSegmentRecordingData(null);
    timelineState.setState({
      currentRecordingId: segment.id,
      currentRecordingProtected: false,
      currentRecordingTags: []
    });
    lastDetectionSegmentIdRef.current = segment.id;

    // Fetch recording info to get stream name, timestamps, and protection status
    fetch(`/api/recordings/${segment.id}`)
      .then(async res => {
        if (res.ok) {
          return res.json();
        }

        // Log details for non-OK HTTP responses to aid debugging.
        let errorText = '';
        try {
          errorText = await res.text();
        } catch (e) {
          // Ignore secondary errors while attempting to read the body.
        }
        console.warn(
          'Failed to load recording info for timeline segment:',
          segment.id,
          'status:',
          res.status,
          res.statusText,
          errorText || '(no response body)'
        );
        return null;
      })
      .then(data => {
        if (!data) return;

        // Store recording data for action buttons
        setSegmentRecordingData(data);
        timelineState.setState({
          currentRecordingId: segment.id,
          currentRecordingProtected: !!data.protected
        });
        updateTimeDisplay(timelineState.currentTime ?? segment.start_timestamp, {
          stream: data.stream || segment.stream
        });

        // Fetch recording tags
        fetch(`/api/recordings/${segment.id}/tags`)
          .then(res => {
            if (!res.ok) {
              console.warn('Failed to load recording tags:', segment.id, 'status:', res.status, res.statusText);
              throw new Error(`Failed to load recording tags: ${res.status} ${res.statusText}`);
            }
            return res.json();
          })
          .then(tagData => timelineState.setState({ currentRecordingTags: tagData?.tags || [] }))
          .catch(() => timelineState.setState({ currentRecordingTags: [] }));

        if (!data.stream || !data.start_time || !data.end_time) return;

        const startTime = toUnixSeconds(data.start_time, { assumeUtc: true });
        const endTime = toUnixSeconds(data.end_time, { assumeUtc: true });
        if (startTime <= 0 || endTime <= 0) return;

        return fetch(`/api/detection/results/${encodeURIComponent(data.stream)}?start=${startTime}&end=${endTime}`);
      })
      .then(async res => {
        if (!res) {
          return null;
        }
        if (res.ok) {
          return res.json();
        }

        // Log details for non-OK HTTP responses when loading detections.
        let errorText = '';
        try {
          errorText = await res.text();
        } catch (e) {
          // Ignore secondary errors while attempting to read the body.
        }
        console.warn(
          'Failed to load detections HTTP response for timeline segment:',
          'status:',
          res.status,
          res.statusText,
          errorText || '(no response body)'
        );
        return null;
      })
      .then(data => {
        if (data && data.detections) {
          setDetections(data.detections);
        } else {
          setDetections([]);
        }
      })
      .catch(err => {
        console.warn('Failed to load detections for timeline segment:', err);
        setDetections([]);
      });
  }, [currentSegmentIndex, segments]);

  // Draw detection overlays on canvas
  const drawTimelineDetections = useCallback(() => {
    if (!canvasRef.current || !videoRef.current || !detectionOverlayEnabled) return;

    const video = videoRef.current;
    const canvas = canvasRef.current;

    const videoWidth = video.videoWidth;
    const videoHeight = video.videoHeight;
    if (!videoWidth || !videoHeight) return;

    const displayWidth = video.clientWidth;
    const displayHeight = video.clientHeight;
    canvas.width = displayWidth;
    canvas.height = displayHeight;

    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    if (!detections || detections.length === 0) return;

    // Get current segment to compute current timestamp
    if (currentSegmentIndex < 0 || currentSegmentIndex >= segments.length) return;
    const segment = segments[currentSegmentIndex];
    if (!segment) return;

    const currentTimestamp = segment.start_timestamp + video.currentTime;

    // Calculate letterbox offsets
    const videoAspect = videoWidth / videoHeight;
    const displayAspect = displayWidth / displayHeight;
    let drawWidth, drawHeight, offsetX = 0, offsetY = 0;

    if (videoAspect > displayAspect) {
      drawWidth = displayWidth;
      drawHeight = displayWidth / videoAspect;
      offsetY = (displayHeight - drawHeight) / 2;
    } else {
      drawHeight = displayHeight;
      drawWidth = displayHeight * videoAspect;
      offsetX = (displayWidth - drawWidth) / 2;
    }

    // Filter detections within a 2-second window of current time
    const timeWindow = DETECTION_TIME_WINDOW_SECONDS;
    const visibleDetections = detections.filter(d =>
      d.timestamp && Math.abs(d.timestamp - currentTimestamp) <= timeWindow
    );

    const scale = Math.max(1, Math.min(drawWidth, drawHeight) / DETECTION_SCALE_BASE);

    visibleDetections.forEach(detection => {
      const x = (detection.x * drawWidth) + offsetX;
      const y = (detection.y * drawHeight) + offsetY;
      const width = detection.width * drawWidth;
      const height = detection.height * drawHeight;

      ctx.strokeStyle = 'rgba(0, 255, 0, 0.8)';
      ctx.lineWidth = Math.max(2, 3 * scale);
      ctx.strokeRect(x, y, width, height);

      const fontSize = Math.round(Math.max(12, 14 * scale));
      ctx.font = `${fontSize}px Arial`;
      const labelText = `${detection.label} (${Math.round(detection.confidence * 100)}%)`;
      const labelWidth = ctx.measureText(labelText).width + 10;
      const labelHeight = fontSize + 8;
      ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
      ctx.fillRect(x, y - labelHeight, labelWidth, labelHeight);
      ctx.fillStyle = 'white';
      ctx.fillText(labelText, x + 5, y - 5);
    });

    // Continue animation if playing
    if (!video.paused && !video.ended) {
      requestAnimationFrame(drawTimelineDetections);
    }
  }, [detectionOverlayEnabled, detections, currentSegmentIndex, segments]);

  // Redraw detections on play/seek/timeupdate
  useEffect(() => {
    if (!detectionOverlayEnabled || !videoRef.current) return;
    const video = videoRef.current;

    const onEvent = () => drawTimelineDetections();
    video.addEventListener('play', onEvent);
    video.addEventListener('seeked', onEvent);
    video.addEventListener('timeupdate', onEvent);

    return () => {
      video.removeEventListener('play', onEvent);
      video.removeEventListener('seeked', onEvent);
      video.removeEventListener('timeupdate', onEvent);
    };
  }, [detectionOverlayEnabled, drawTimelineDetections]);

  // Track fullscreen state and redraw overlays when the player size changes.
  useEffect(() => {
    if (!videoRef.current || !videoContainerRef.current) return;

    const container = videoContainerRef.current;

    const handleFullscreenChange = () => {
      const fullscreenElement = document.fullscreenElement || document.webkitFullscreenElement;
      setIsFullscreen(fullscreenElement === container);
      setTimeout(() => {
        if (detectionOverlayEnabled) {
          drawTimelineDetections();
        }
      }, 100);
    };

    const resizeObserver = typeof ResizeObserver !== 'undefined'
      ? new ResizeObserver(() => {
        if (detectionOverlayEnabled) {
          drawTimelineDetections();
        }
      })
      : null;

    if (resizeObserver) {
      resizeObserver.observe(container);
      resizeObserver.observe(videoRef.current);
    }

    document.addEventListener('fullscreenchange', handleFullscreenChange);
    document.addEventListener('webkitfullscreenchange', handleFullscreenChange);
    handleFullscreenChange();

    return () => {
      document.removeEventListener('fullscreenchange', handleFullscreenChange);
      document.removeEventListener('webkitfullscreenchange', handleFullscreenChange);
      resizeObserver?.disconnect();
    };
  }, [detectionOverlayEnabled, drawTimelineDetections]);

  const handleToggleFullscreen = useCallback(async () => {
    const container = videoContainerRef.current;
    if (!container) return;

    try {
      const fullscreenElement = document.fullscreenElement || document.webkitFullscreenElement;
      if (fullscreenElement === container) {
        if (document.exitFullscreen) {
          await document.exitFullscreen();
        } else if (document.webkitExitFullscreen) {
          document.webkitExitFullscreen();
        }
        return;
      }

      if (container.requestFullscreen) {
        await container.requestFullscreen();
      } else if (container.webkitRequestFullscreen) {
        container.webkitRequestFullscreen();
      } else if (container.msRequestFullscreen) {
        container.msRequestFullscreen();
      } else {
        showStatusMessage(t('timeline.fullscreenNotSupported'), 'warning');
      }
    } catch (error) {
      console.error('Error toggling fullscreen:', error);
      showStatusMessage(t('timeline.couldNotToggleFullscreen', { message: error.message }), 'error');
    }
  }, []);

  // Get current segment ID
  const currentSegmentId = (currentSegmentIndex >= 0 && segments.length > 0 && currentSegmentIndex < segments.length)
    ? segments[currentSegmentIndex].id : null;

  // Delete the current segment's recording
  const handleDeleteRecording = useCallback(async () => {
    if (!currentSegmentId) return;
    try {
      const response = await fetch(`/api/recordings/${currentSegmentId}`, { method: 'DELETE' });
      if (!response.ok) throw new Error(t('timeline.failedToDeleteRecording'));
      showStatusMessage(t('timeline.recordingDeletedSuccessfully'), 'success');
      setShowDeleteConfirm(false);
      setSegmentRecordingData(null);
      lastDetectionSegmentIdRef.current = null;

      // Tell TimelinePage to reload segments and advance to the next recording
      window.dispatchEvent(new CustomEvent('timeline-recording-deleted', {
        detail: { id: currentSegmentId }
      }));
    } catch (error) {
      console.error('Error deleting recording:', error);
      showStatusMessage(t('recordings.errorMessage', { message: error.message }), 'error');
      setShowDeleteConfirm(false);
    }
  }, [currentSegmentId]);

  // Take a snapshot of the current video frame
  const handleSnapshot = useCallback(() => {
    if (!videoRef.current) return;
    const video = videoRef.current;
    if (!video.videoWidth || !video.videoHeight) {
      showStatusMessage(t('timeline.cannotTakeSnapshotVideoNotLoaded'), 'error');
      return;
    }
    const canvas = document.createElement('canvas');
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

    const streamName = segmentRecordingData?.stream || 'timeline';
    const timestamp = formatFilenameTimestamp();
    const fileName = `snapshot-${streamName.replace(/\s+/g, '-')}-${timestamp}.jpg`;

    canvas.toBlob((blob) => {
      if (!blob) {
        showStatusMessage(t('timeline.failedToCreateSnapshot'), 'error');
        return;
      }
      const blobUrl = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = blobUrl;
      link.download = fileName;
      document.body.appendChild(link);
      link.click();

      const cleanupDownload = () => {
        if (document.body.contains(link)) document.body.removeChild(link);
        URL.revokeObjectURL(blobUrl);
      };

      if (typeof window !== 'undefined' && typeof window.requestAnimationFrame === 'function') {
        window.requestAnimationFrame(() => {
          cleanupDownload();
        });
      } else {
        setTimeout(() => {
          cleanupDownload();
        }, 0);
      }

      showStatusMessage(`Snapshot saved: ${fileName}`, 'success', 2000);
    }, 'image/jpeg', 0.95);
  }, [segmentRecordingData, t]);

  return (
    <>
      <div className="timeline-player-container mb-1" id="video-player">
        <div
          ref={videoContainerRef}
          data-testid="timeline-video-container"
          className="relative w-full bg-black rounded-lg shadow-md"
          style={isFullscreen ? { width: '100vw', height: '100vh' } : { aspectRatio: '16/9' }}
        >
          <video
              ref={setVideoRefs}
              className="w-full h-full object-contain"
              controls
              controlsList="nofullscreen"
              autoPlay={false}
              muted={false}
              playsInline
              onPlay={handlePlay}
              onPause={handlePause}
              onEnded={handleEnded}
              onTimeUpdate={handleTimeUpdate}
          ></video>

          {/* Detection overlay canvas */}
          {detectionOverlayEnabled && (
            <canvas
              ref={canvasRef}
              className="absolute top-0 left-0 w-full h-full pointer-events-none"
              style={{ zIndex: 2 }}
            />
          )}

          {/* Add a message for invalid segments */}
          <div
            className={`absolute inset-0 flex items-center justify-center bg-black bg-opacity-70 text-white text-center p-4 ${currentSegmentIndex >= 0 && segments.length > 0 ? 'hidden' : ''}`}
          >
            <div>
              <p className="mb-2">{t('timeline.noValidSegmentSelected')}</p>
              <p className="text-sm">{t('timeline.selectSegmentOrPlay')}</p>
            </div>
          </div>
        </div>
      </div>

      {/* Compact toolbar: detections toggle | action buttons | speed */}
      {/* data-keyboard-nav-preserve: clicking here must not exit 'fine' keyboard mode */}
      <div className="flex items-center flex-wrap gap-x-3 gap-y-1 mb-1" data-keyboard-nav-preserve>
        {/* Detection toggle */}
        <label className="flex items-center gap-1.5 cursor-pointer">
          <input
            type="checkbox"
            id="timeline-detection-overlay"
            className="w-3.5 h-3.5 accent-primary"
            checked={detectionOverlayEnabled}
            onChange={(e) => setDetectionOverlayEnabled(e.target.checked)}
          />
          <span className="text-[11px] text-foreground">
            {t('recordings.detections')}{detections.length > 0 ? ` (${detections.length})` : ''}
          </span>
        </label>

        <button
          type="button"
          className="px-2 py-1 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors flex items-center text-[11px]"
          onClick={handleToggleFullscreen}
          title={isFullscreen ? t('timeline.exitFullscreen') : t('timeline.enterFullscreen')}
        >
          {isFullscreen ? t('timeline.exitFullscreen') : t('timeline.fullscreen')}
        </button>

        {/* Action buttons — only when a segment is selected */}
        {currentSegmentId && (
          <div className="flex items-center gap-1">
            <button
              className="px-2 py-1 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors flex items-center text-[11px]"
              onClick={handleSnapshot}
              title={t('timeline.takeSnapshot')}
            >
              📷 {t('timeline.snapshot')}
            </button>
            <a
              className="px-2 py-1 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors flex items-center text-[11px]"
              href={`/api/recordings/download/${currentSegmentId}`}
              download
            >
              ↓ {t('recordings.download')}
            </a>
            <button
              className="px-2 py-1 bg-red-600 text-white rounded hover:bg-red-700 transition-colors flex items-center text-[11px]"
              onClick={() => setShowDeleteConfirm(true)}
              title={t('timeline.deleteRecording')}
            >
              🗑 {t('common.delete')}
            </button>
          </div>
        )}

        {/* Speed controls — pushed right */}
        <div className="ml-auto">
          <SpeedControls />
        </div>
      </div>

      {/* Delete confirmation */}
      <ConfirmDialog
        isOpen={showDeleteConfirm}
        onClose={() => setShowDeleteConfirm(false)}
        onConfirm={handleDeleteRecording}
        title={t('timeline.deleteRecording')}
        message={t('timeline.deleteRecordingConfirmation')}
        confirmLabel={t('common.delete')}
      />
    </>
  );
}
