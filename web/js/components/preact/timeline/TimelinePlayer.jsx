/**
 * LightNVR Timeline Player Component
 * Handles video playback for the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { SpeedControls } from './SpeedControls.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';

/**
 * TimelinePlayer component
 * @returns {JSX.Element} TimelinePlayer component
 */
export function TimelinePlayer() {
  // Local state
  const [currentSegmentIndex, setCurrentSegmentIndex] = useState(-1);
  const [isPlaying, setIsPlaying] = useState(false);
  const [segments, setSegments] = useState([]);
  const [playbackSpeed, setPlaybackSpeed] = useState(1.0);

  // Refs
  const videoRef = useRef(null);
  const lastTimeUpdateRef = useRef(null);
  const lastSegmentIdRef = useRef(null);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Update local state
      setCurrentSegmentIndex(state.currentSegmentIndex);
      setIsPlaying(state.isPlaying);
      setSegments(state.timelineSegments || []);
      setPlaybackSpeed(state.playbackSpeed);

      // Handle video playback
      handleVideoPlayback(state);
    });

    return () => unsubscribe();
  }, []);

  // Handle video playback based on state changes
  const handleVideoPlayback = (state) => {
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
  };

  // Load a segment
  const loadSegment = (segment, seekTime = 0, autoplay = false) => {
    const video = videoRef.current;
    if (!video) return;

    console.log(`Loading segment ${segment.id} at time ${seekTime}s, autoplay: ${autoplay}`);

    // Pause current playback
    video.pause();

    // Set new source with timestamp to prevent caching
    const recordingUrl = `/api/recordings/play/${segment.id}?t=${Date.now()}`;

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
      if (validSeekTime > 0 && validSeekTime < 1.0) {
        // Ensure we're at least 1 second into the segment
        validSeekTime = 1.0;
        console.log(`TimelinePlayer: Adjusting seek time to ${validSeekTime}s to prevent snapping to segment start`);
      }

      console.log(`TimelinePlayer: Setting video time to ${validSeekTime}s (requested: ${timeToSet}s, segment duration: ${segmentDuration}s)`);
      video.currentTime = validSeekTime;

      // Set playback speed
      video.playbackRate = playbackSpeed;

      // Play if needed
      if (autoplay) {
        video.play().catch(error => {
          console.error('Error playing video:', error);
          showStatusMessage('Error playing video: ' + error.message, 'error');
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
  };

  // Handle video ended event
  const handleEnded = () => {
    console.log('Video ended');

    // Check if we have a next segment
    if (currentSegmentIndex < segments.length - 1) {
      // Play next segment
      const nextIndex = currentSegmentIndex + 1;
      console.log(`Playing next segment ${nextIndex}`);

      // Store the current cursor position
      const wasUserControllingCursor = timelineState.userControllingCursor;

      // Set a flag to indicate we're handling the segment transition
      // This prevents other components from interfering
      timelineState.directVideoControl = true;

      // Preload the next segment's video immediately
      const nextSegment = segments[nextIndex];
      const nextVideoUrl = `/api/recordings/play/${nextSegment.id}?t=${Date.now()}`;
      
      // Create a temporary video element to preload the next segment
      const tempVideo = document.createElement('video');
      tempVideo.preload = 'auto';
      tempVideo.src = nextVideoUrl;
      tempVideo.load();
      
      console.log(`Preloading next segment ${nextIndex} (ID: ${nextSegment.id})`);

      // Update timeline state
      timelineState.setState({
        currentSegmentIndex: nextIndex,
        // Only set the current time to the start of the next segment if the user wasn't controlling the cursor
        // This preserves the cursor position if the user had manually positioned it
        currentTime: wasUserControllingCursor ? timelineState.currentTime : segments[nextIndex].start_timestamp,
        isPlaying: true,
        forceReload: true
      });

      // Get the current video element
      const video = videoRef.current;
      if (video) {
        // Set up the new source immediately to minimize delay
        video.pause();
        video.src = nextVideoUrl;
        video.load();
        
        // Set up event listener for when metadata is loaded
        const onLoadedMetadata = () => {
          console.log('Next segment metadata loaded, starting playback immediately');
          video.currentTime = 0; // Start from the beginning of the next segment
          video.play().catch(e => console.error('Error playing next segment:', e));
          video.removeEventListener('loadedmetadata', onLoadedMetadata);
        };
        
        video.addEventListener('loadedmetadata', onLoadedMetadata);
      }

      // Reset the directVideoControl flag after a delay
      setTimeout(() => {
        timelineState.directVideoControl = false;
        timelineState.setState({});
      }, 1000);
    } else {
      // End of all segments
      console.log('End of all segments');

      // Update timeline state
      timelineState.setState({
        isPlaying: false
      });
    }
  };

  // Handle video time update event
  const handleTimeUpdate = () => {
    const video = videoRef.current;
    if (!video) return;

    // Check if we have a valid segment
    if (currentSegmentIndex < 0 ||
        !segments ||
        segments.length === 0 ||
        currentSegmentIndex >= segments.length) {
      return;
    }

    const segment = segments[currentSegmentIndex];
    if (!segment) return;

    // Calculate current timestamp, handling timezone correctly
    const currentTime = segment.start_timestamp + video.currentTime;

    // Log the current time for debugging
    console.log('TimelinePlayer: Current time', {
      videoTime: video.currentTime,
      segmentStart: segment.start_timestamp,
      calculatedTime: currentTime,
      localTime: new Date(currentTime * 1000).toLocaleString()
    });

    // Directly update the time display as well
    updateTimeDisplay(currentTime);

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
  const updateTimeDisplay = (time) => {
    if (time === null) return;

    const timeDisplay = document.getElementById('time-display');
    if (!timeDisplay) return;

    // Convert timestamp to timeline hour
    const hour = timelineState.timestampToTimelineHour(time);

    // Format time
    const hours = Math.floor(hour).toString().padStart(2, '0');
    const minutes = Math.floor((hour % 1) * 60).toString().padStart(2, '0');
    const seconds = Math.floor(((hour % 1) * 60) % 1 * 60).toString().padStart(2, '0');

    // Display time only (HH:MM:SS)
    timeDisplay.textContent = `${hours}:${minutes}:${seconds}`;

    console.log('TimelinePlayer: Updated time display', {
      timestamp: time,
      hour,
      formatted: `${hours}:${minutes}:${seconds}`,
      localTime: new Date(time * 1000).toLocaleString()
    });
  };

  return (
    <>
      <div className="timeline-player-container mb-2" id="video-player">
        <div className="relative w-full bg-black rounded-lg shadow-md" style={{ aspectRatio: '16/9' }}>
          <video
              ref={videoRef}
              className="w-full h-full object-contain"
              controls
              autoPlay={false}
              muted={false}
              playsInline
              onEnded={handleEnded}
              onTimeUpdate={handleTimeUpdate}
          ></video>

          {/* Add a message for invalid segments */}
          <div
            className={`absolute inset-0 flex items-center justify-center bg-black bg-opacity-70 text-white text-center p-4 ${currentSegmentIndex >= 0 && segments.length > 0 ? 'hidden' : ''}`}
          >
            <div>
              <p className="mb-2">No valid segment selected.</p>
              <p className="text-sm">Click on a segment in the timeline or use the play button to start playback.</p>
            </div>
          </div>
        </div>
      </div>

      {/* Playback speed controls */}
      <SpeedControls />
    </>
  );
}
