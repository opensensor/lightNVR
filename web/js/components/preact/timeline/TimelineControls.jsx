/**
 * LightNVR Timeline Controls Component
 * Handles play/pause and zoom controls for the timeline
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { TagIcon, TagsOverlay } from '../recordings/TagsOverlay.jsx';
import {
  formatPlaybackTimeLabel,
  getTimelineDayLengthHours,
  MIN_TIMELINE_VIEW_HOURS,
  resolveActiveSegmentIndex,
  resolvePlaybackStreamName,
  timestampToTimelineOffset,
  zoomTimelineRange
} from './timelineUtils.js';
import { nowMilliseconds } from '../../../utils/date-utils.js';

/**
 * TimelineControls component
 * @returns {JSX.Element} TimelineControls component
 */
export function TimelineControls() {
  const [isPlaying, setIsPlaying] = useState(false);
  const [canZoomIn, setCanZoomIn] = useState(true);
  const [canZoomOut, setCanZoomOut] = useState(true);
  const [timeDisplayText, setTimeDisplayText] = useState('00:00:00');
  const [activeSegmentIndex, setActiveSegmentIndex] = useState(-1);
  const [segmentCount, setSegmentCount] = useState(0);
  const [currentRecordingId, setCurrentRecordingId] = useState(null);
  const [isProtected, setIsProtected] = useState(false);
  const [recordingTags, setRecordingTags] = useState([]);
  const [showTagsOverlay, setShowTagsOverlay] = useState(false);

  useEffect(() => {
    const syncControlsState = (state) => {
      setIsPlaying(state.isPlaying);
      setSegmentCount(state.timelineSegments?.length || 0);
      setActiveSegmentIndex(resolveActiveSegmentIndex(
        state.timelineSegments,
        state.currentSegmentIndex,
        state.currentTime
      ));
      setCurrentRecordingId(state.currentRecordingId ?? null);
      setIsProtected(!!state.currentRecordingProtected);
      setRecordingTags(Array.isArray(state.currentRecordingTags) ? state.currentRecordingTags : []);
      const dayLengthHours = getTimelineDayLengthHours(state.selectedDate);
      const range = (state.timelineEndHour ?? dayLengthHours) - (state.timelineStartHour ?? 0);
      setCanZoomIn(range > MIN_TIMELINE_VIEW_HOURS);
      setCanZoomOut(range < dayLengthHours);

      const streamName = resolvePlaybackStreamName(
        state.timelineSegments,
        state.currentSegmentIndex,
        state.currentTime
      );
      const nextTimeDisplayText = formatPlaybackTimeLabel(state.currentTime, streamName);
      setTimeDisplayText(nextTimeDisplayText || '00:00:00');
    };

    syncControlsState(timelineState);

    const unsubscribe = timelineState.subscribe(syncControlsState);
    return () => unsubscribe();
  }, []);

  useEffect(() => {
    setShowTagsOverlay(false);
  }, [currentRecordingId]);

  // Toggle playback (play/pause)
  const togglePlayback = () => {
    console.log('TimelineControls: togglePlayback called');
    console.log('TimelineControls: Current state before toggle:', {
      isPlaying,
      currentTime: timelineState.currentTime,
      currentSegmentIndex: timelineState.currentSegmentIndex,
      segmentsCount: timelineState.timelineSegments ? timelineState.timelineSegments.length : 0
    });

    if (isPlaying) {
      pausePlayback();
    } else {
      resumePlayback();
    }
  };

  // Pause playback
  const pausePlayback = () => {
    timelineState.setState({ isPlaying: false });

    // If there's a video player, pause it
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer) {
      videoPlayer.pause();
    }
  };

  // Resume playback
  const resumePlayback = () => {
    // If no segments, show message and return
    if (!timelineState.timelineSegments || timelineState.timelineSegments.length === 0) {
      showStatusMessage('No recordings to play', 'warning');
      return;
    }

    console.log('TimelineControls: resumePlayback called');
    console.log('TimelineControls: Current state:', {
      segments: timelineState.timelineSegments.length,
      currentSegmentIndex: timelineState.currentSegmentIndex,
      currentTime: timelineState.currentTime,
      selectedDate: timelineState.selectedDate
    });

    // SIMPLIFIED APPROACH: Directly use the current time to find the appropriate segment
    let segmentToPlay = null;
    let segmentIndex = -1;
    let relativeTime = 0;

    // If we have a current time, find the segment that contains it
    if (timelineState.currentTime !== null) {
      console.log('TimelineControls: Using current time to find segment:', timelineState.currentTime);

      // First try to find a segment that contains the current time
      const containingIndex = findContainingSegmentIndex(
        timelineState.timelineSegments,
        timelineState.currentTime
      );
      if (containingIndex !== -1) {
        segmentToPlay = timelineState.timelineSegments[containingIndex];
        segmentIndex = containingIndex;
        relativeTime = timelineState.currentTime - segmentToPlay.start_timestamp;
        console.log(`TimelineControls: Found segment ${containingIndex} containing current time, relative time: ${relativeTime}s`);
      }

      // If no exact match, find the closest segment
      if (!segmentToPlay) {
        const closestIndex = findNearestSegmentIndex(
          timelineState.timelineSegments,
          timelineState.currentTime
        );

        segmentToPlay = timelineState.timelineSegments[closestIndex];
        segmentIndex = closestIndex;

        // Calculate the appropriate relative time based on the cursor position
        if (timelineState.currentTime < segmentToPlay.start_timestamp) {
          // If cursor is before the segment, start at the beginning
          relativeTime = 0;
        } else if (timelineState.currentTime > segmentToPlay.end_timestamp) {
          // If cursor is after the segment, start at the end
          relativeTime = segmentToPlay.end_timestamp - segmentToPlay.start_timestamp;
        } else {
          // If cursor is within the segment's time range but not exactly matching,
          // calculate the relative position within the segment
          relativeTime = timelineState.currentTime - segmentToPlay.start_timestamp;
        }

        console.log(`TimelineControls: Using closest segment ${closestIndex}, relative time: ${relativeTime}s`);
      }
    }
    // If no current time but we have a valid segment index, use that
    else if (timelineState.currentSegmentIndex >= 0 &&
             timelineState.currentSegmentIndex < timelineState.timelineSegments.length) {
      segmentIndex = timelineState.currentSegmentIndex;
      segmentToPlay = timelineState.timelineSegments[segmentIndex];
      relativeTime = 0;
      console.log(`TimelineControls: Using current segment index ${segmentIndex}`);
    }
    // Fall back to the first segment
    else {
      segmentIndex = 0;
      segmentToPlay = timelineState.timelineSegments[0];
      relativeTime = 0;
      console.log('TimelineControls: Falling back to first segment');
    }

    // DIRECT APPROACH: Manually load and play the video
    console.log(`TimelineControls: Playing segment ${segmentIndex} (ID: ${segmentToPlay.id}) at time ${relativeTime}s`);

    // First update the state
    timelineState.currentSegmentIndex = segmentIndex;

    // Check if we should preserve the cursor position
    if (timelineState.preserveCursorPosition) {
      // If the cursor position is locked, don't change the current time at all
      console.log(`TimelineControls: Cursor position is locked, keeping current time ${timelineState.currentTime}`);
    } else {
      // Make sure we preserve the current time if it's within the segment
      // This ensures the cursor stays where the user positioned it
      if (timelineState.currentTime !== null &&
          timelineState.currentTime >= segmentToPlay.start_timestamp &&
          timelineState.currentTime <= segmentToPlay.end_timestamp) {
        // Keep the current time as is - don't reset it
        console.log(`TimelineControls: Keeping current time ${timelineState.currentTime} within segment`);
      } else {
        // Otherwise, set the time based on the calculated relative time
        timelineState.currentTime = segmentToPlay.start_timestamp + relativeTime;
        console.log(`TimelineControls: Setting current time to ${timelineState.currentTime} (segment start + ${relativeTime}s)`);
      }
    }

    timelineState.isPlaying = true;
    timelineState.directVideoControl = true; // Set flag to prevent TimelinePlayer interference

    // Notify listeners
    timelineState.setState({});

    // Keep the directVideoControl flag active longer to ensure no interference
    const resetDirectControl = () => {
      console.log('TimelineControls: Resetting directVideoControl flag');
      timelineState.directVideoControl = false;
      timelineState.setState({});
    };

    // Reset the flag after a longer delay
    setTimeout(resetDirectControl, 3000);

    // Now directly control the video element
    const videoElement = document.querySelector('#video-player video');
    if (videoElement) {
      // Pause any current playback
      videoElement.pause();

      // Set up event listener for when metadata is loaded
      const handleMetadataLoaded = () => {
        console.log(`TimelineControls: Video metadata loaded, setting time to ${relativeTime}s`);

        try {
          // Log the video duration
          console.log('TimelineControls: Video metadata', {
            duration: videoElement.duration,
            width: videoElement.videoWidth,
            height: videoElement.videoHeight,
            segment: segmentToPlay.id,
            segmentDuration: segmentToPlay.end_timestamp - segmentToPlay.start_timestamp
          });

          // Set the current time based on the cursor position
          let timeToSet = relativeTime;

          // Check if we should preserve the cursor position
          if (timelineState.preserveCursorPosition && timelineState.currentTime !== null) {
            // Calculate the relative time within the segment
            if (timelineState.currentTime >= segmentToPlay.start_timestamp &&
                timelineState.currentTime <= segmentToPlay.end_timestamp) {
              // If the cursor is within this segment, use its position
              timeToSet = timelineState.currentTime - segmentToPlay.start_timestamp;
              console.log(`TimelineControls: Using locked cursor position for playback: ${timeToSet}s`);
            } else {
              // If the cursor is outside this segment but we want to preserve its position,
              // use the beginning or end of the segment based on which is closer
              const distanceToStart = Math.abs(timelineState.currentTime - segmentToPlay.start_timestamp);
              const distanceToEnd = Math.abs(timelineState.currentTime - segmentToPlay.end_timestamp);

              if (distanceToStart <= distanceToEnd) {
                timeToSet = 0; // Use start of segment
                console.log(`TimelineControls: Cursor outside segment, using start of segment`);
              } else {
                timeToSet = segmentToPlay.end_timestamp - segmentToPlay.start_timestamp; // Use end of segment
                console.log(`TimelineControls: Cursor outside segment, using end of segment`);
              }
            }
          } else if (timelineState.currentTime !== null &&
                     timelineState.currentTime >= segmentToPlay.start_timestamp &&
                     timelineState.currentTime <= segmentToPlay.end_timestamp) {
            // If not preserving cursor position but the current time is within this segment,
            // still use the relative position from the cursor
            timeToSet = timelineState.currentTime - segmentToPlay.start_timestamp;
            console.log(`TimelineControls: Using cursor position for playback: ${timeToSet}s`);
          }

          // Ensure the time is within valid bounds
          let validTime = Math.max(0, Math.min(timeToSet, videoElement.duration || 0));

          // If we're very close to the start of the segment but not exactly at the start,
          // add a small offset to prevent snapping to the start
          if (validTime > 0 && validTime < 1.0) {
            // Ensure we're at least 1 second into the segment
            validTime = 1.0;
            console.log(`TimelineControls: Adjusting video time to ${validTime}s to prevent snapping to segment start`);
          }

          console.log(`TimelineControls: Setting video time to ${validTime}s`);
          videoElement.currentTime = validTime;

          // Start playback with a small delay to avoid conflicts
          setTimeout(() => {
            if (timelineState.isPlaying) {
              console.log('TimelineControls: Starting video playback');
              videoElement.play().then(() => {
                console.log('TimelineControls: Video playback started successfully');

                // Set up multiple checks to ensure playback continues
                const checkPlayback = (attempt = 1) => {
                  if (attempt > 5) return; // Limit to 5 attempts

                  setTimeout(() => {
                    if (videoElement.paused && timelineState.isPlaying) {
                      console.log(`TimelineControls: Video paused unexpectedly (attempt ${attempt}), trying to resume`);
                      videoElement.play().catch(e => {
                        if (e.name === 'AbortError') return;
                        console.error(`Error resuming video (attempt ${attempt}):`, e);
                      });

                      // Try again after a delay
                      checkPlayback(attempt + 1);
                    }
                  }, 500 * attempt); // Increasing delays between attempts
                };

                // Start the playback checks
                checkPlayback();

                // Set up a check to ensure the video plays for the full segment duration
                const segmentDuration = segmentToPlay.end_timestamp - segmentToPlay.start_timestamp;
                console.log(`TimelineControls: Segment duration: ${segmentDuration}s, video duration: ${videoElement.duration}s`);

                // If the video duration is significantly shorter than the segment duration,
                // we need to ensure the video plays for the full segment duration
                if (videoElement.duration < segmentDuration - 1) { // Allow 1 second tolerance
                  console.log('TimelineControls: Video duration is shorter than segment duration, will monitor playback');

                  // Monitor playback to ensure it continues for the full segment duration
                  const monitorInterval = setInterval(() => {
                    if (!timelineState.isPlaying || !videoElement) {
                      clearInterval(monitorInterval);
                      return;
                    }

                    // If we're near the end of the video but not the end of the segment,
                    // reset to the beginning and continue playing
                    if (videoElement.currentTime > videoElement.duration - 0.5 &&
                        relativeTime + videoElement.currentTime < segmentDuration) {
                      console.log('TimelineControls: Reached end of video but not end of segment, restarting video');
                      videoElement.currentTime = 0;
                      videoElement.play().catch(e => {
                        if (e.name === 'AbortError') return;
                        console.error('Error restarting video:', e);
                      });
                    }
                  }, 500);
                }
              }).catch(e => {
                if (e.name === 'AbortError') {
                  console.log('TimelineControls: play() interrupted, ignoring AbortError');
                  return;
                }
                console.error('Error playing video:', e);
                showStatusMessage('Error playing video: ' + e.message, 'error');
              });
            }
          }, 100);
        } catch (error) {
          console.error('TimelineControls: Error in handleMetadataLoaded:', error);
        } finally {
          // Remove the event listener
          videoElement.removeEventListener('loadedmetadata', handleMetadataLoaded);
        }
      };

      // Add the event listener
      videoElement.addEventListener('loadedmetadata', handleMetadataLoaded);

      // Set the new source
      console.log(`TimelineControls: Loading video from segment ${segmentToPlay.id}`);
      videoElement.src = `/api/recordings/play/${segmentToPlay.id}?t=${nowMilliseconds()}`;
      videoElement.load();
    } else {
      console.error('TimelineControls: No video element found');
      showStatusMessage('Error: Video player not found', 'error');
    }
  };

  // ── Helper: get the center hour for zoom (cursor position or range midpoint) ──
  const getCenter = () => {
    const s = timelineState.timelineStartHour ?? 0;
    const dayLengthHours = getTimelineDayLengthHours(timelineState.selectedDate);
    const e = timelineState.timelineEndHour ?? dayLengthHours;
    if (timelineState.currentTime !== null) {
      const h = timestampToTimelineOffset(timelineState.currentTime, timelineState.selectedDate);
      // Only use cursor if it's inside the current view
      if (h !== null && h >= s && h <= e) return h;
    }
    return (s + e) / 2;
  };

  // Zoom in — halve the visible range, centered on cursor
  const zoomIn = () => {
    const s = timelineState.timelineStartHour ?? 0;
    const dayLengthHours = getTimelineDayLengthHours(timelineState.selectedDate);
    const e = timelineState.timelineEndHour ?? dayLengthHours;
    const range = e - s;
    if (range <= MIN_TIMELINE_VIEW_HOURS) return;
    const center = getCenter();
    const nextRange = zoomTimelineRange(s, e, 0.5, center, dayLengthHours);
    timelineState.setState({
      timelineStartHour: nextRange.startHour,
      timelineEndHour: nextRange.endHour
    });
  };

  // Zoom out — double the visible range, centered on cursor, capped at 0-24
  const zoomOut = () => {
    const s = timelineState.timelineStartHour ?? 0;
    const dayLengthHours = getTimelineDayLengthHours(timelineState.selectedDate);
    const e = timelineState.timelineEndHour ?? dayLengthHours;
    const range = e - s;
    if (range >= dayLengthHours) return;
    const center = getCenter();
    const nextRange = zoomTimelineRange(s, e, 2, center, dayLengthHours);
    timelineState.setState({
      timelineStartHour: nextRange.startHour,
      timelineEndHour: nextRange.endHour
    });
  };

  // Fit — reset to the auto-fit range computed on data load
  const fitToSegments = () => {
    const fs = timelineState.autoFitStartHour ?? 0;
    const fe = timelineState.autoFitEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
    timelineState.setState({ timelineStartHour: fs, timelineEndHour: fe });
  };

  const jumpToAdjacentSegment = (direction) => {
    const segments = timelineState.timelineSegments;
    if (!Array.isArray(segments) || segments.length === 0) {
      showStatusMessage('No recordings to navigate', 'warning');
      return;
    }

    const currentIndex = resolveActiveSegmentIndex(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      timelineState.currentTime
    );
    if (currentIndex === -1) {
      showStatusMessage('No active recording selected', 'warning');
      return;
    }

    const targetIndex = currentIndex + direction;
    if (targetIndex < 0 || targetIndex >= segments.length) {
      return;
    }

    const targetSegment = segments[targetIndex];
    timelineState.setState({
      currentSegmentIndex: targetIndex,
      currentTime: targetSegment.start_timestamp,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: timelineState.isPlaying,
      forceReload: true
    });
  };

  const canJumpBackward = activeSegmentIndex > 0;
  const canJumpForward = activeSegmentIndex !== -1 && activeSegmentIndex < segmentCount - 1;

  const handleToggleProtection = async () => {
    if (!currentRecordingId) return;

    const newState = !isProtected;
    try {
      const response = await fetch(`/api/recordings/${currentRecordingId}/protect`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ protected: newState }),
      });

      if (!response.ok) {
        throw new Error(`Failed to ${newState ? 'protect' : 'unprotect'} recording`);
      }

      timelineState.setState({ currentRecordingProtected: newState });
      showStatusMessage(
        newState ? 'Recording protected from automatic deletion' : 'Recording protection removed',
        'success'
      );
    } catch (error) {
      console.error('Error toggling protection:', error);
      showStatusMessage(`Error: ${error.message}`, 'error');
    }
  };

  const handleTagsChanged = (_id, newTags) => {
    timelineState.setState({ currentRecordingTags: newTags });
  };

  return (
    <div className="timeline-controls flex flex-wrap justify-between items-center gap-2 mb-1">
      <div className="flex items-center gap-1.5">
        <button
          id="play-button"
          className="w-7 h-7 rounded-full btn-success flex items-center justify-center focus:outline-none transition-colors shadow-sm"
          onClick={togglePlayback}
          title={isPlaying ? 'Pause' : 'Play from current position'}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            {isPlaying ? (
              <>
                <rect x="6" y="6" width="4" height="12" rx="1" fill="white" />
                <rect x="14" y="6" width="4" height="12" rx="1" fill="white" />
              </>
            ) : (
              <path d="M8 5.14v14l11-7-11-7z" fill="white" />
            )}
          </svg>
        </button>
        <span className="text-[11px] text-muted-foreground">Play from cursor</span>
      </div>

      {/* Current time display */}
      <div className="flex items-center gap-1">
        <button
          type="button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={() => jumpToAdjacentSegment(-1)}
          title="Previous recording"
          aria-label="Previous recording"
          disabled={!canJumpBackward}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div id="time-display"
          className="timeline-time-display bg-secondary text-foreground px-2 py-0.5 rounded font-mono text-xs tabular-nums border border-border min-w-[140px] text-center">
          {timeDisplayText}
        </div>
        {currentRecordingId && (
          <button
            type="button"
            className={`w-6 h-6 rounded border flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors ${
              isProtected
                ? 'border-yellow-500 bg-yellow-500 text-white hover:bg-yellow-600'
                : 'border-border bg-secondary text-secondary-foreground hover:bg-secondary/80'
            }`}
            onClick={handleToggleProtection}
            title={isProtected ? 'Unprotect Recording' : 'Protect Recording'}
            aria-label={isProtected ? 'Unprotect Recording' : 'Protect Recording'}
            aria-pressed={isProtected}
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 3l7 4v5c0 4.3-2.9 8.2-7 9-4.1-.8-7-4.7-7-9V7l7-4z" />
            </svg>
          </button>
        )}
        {currentRecordingId && (
          <div className="relative inline-block">
            <button
              type="button"
              className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
              onClick={() => setShowTagsOverlay(!showTagsOverlay)}
              title="Manage Recording Tags"
              aria-label={recordingTags.length > 0 ? `Manage Recording Tags (${recordingTags.length})` : 'Manage Recording Tags'}
            >
              <TagIcon className="w-3.5 h-3.5" />
              {recordingTags.length > 0 && (
                <span className="absolute -top-1 -right-1 min-w-[14px] h-[14px] px-0.5 rounded-full bg-primary text-primary-foreground text-[9px] leading-[14px] text-center">
                  {recordingTags.length}
                </span>
              )}
            </button>
            {showTagsOverlay && (
              <TagsOverlay
                recording={{ id: currentRecordingId, tags: recordingTags }}
                onClose={() => setShowTagsOverlay(false)}
                onTagsChanged={handleTagsChanged}
              />
            )}
          </div>
        )}
        <button
          type="button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={() => jumpToAdjacentSegment(1)}
          title="Next recording"
          aria-label="Next recording"
          disabled={!canJumpForward}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>
      </div>

      <div className="flex items-center gap-1">
        <button
          className="px-2 h-6 rounded text-xs bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={fitToSegments}
          title="Fit to recordings"
        >
          Fit
        </button>
        <button
          id="zoom-out-button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={zoomOut}
          title="Zoom Out (Show more time)"
          disabled={!canZoomOut}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
        <button
          id="zoom-in-button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={zoomIn}
          title="Zoom In (Show less time)"
          disabled={!canZoomIn}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>
  );
}
