/**
 * LightNVR Timeline Controls Component
 * Handles play/pause and zoom controls for the timeline
 */


import { html } from '../../../html-helper.js';
import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { showStatusMessage } from '../UI.js';

/**
 * TimelineControls component
 * @returns {JSX.Element} TimelineControls component
 */
export function TimelineControls() {
  // Local state
  const [isPlaying, setIsPlaying] = useState(false);
  const [zoomLevel, setZoomLevel] = useState(1);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      setIsPlaying(state.isPlaying);
      setZoomLevel(state.zoomLevel);
    });

    return () => unsubscribe();
  }, []);

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
      for (let i = 0; i < timelineState.timelineSegments.length; i++) {
        const segment = timelineState.timelineSegments[i];
        if (timelineState.currentTime >= segment.start_timestamp &&
            timelineState.currentTime <= segment.end_timestamp) {
          segmentToPlay = segment;
          segmentIndex = i;
          relativeTime = timelineState.currentTime - segment.start_timestamp;
          console.log(`TimelineControls: Found segment ${i} containing current time, relative time: ${relativeTime}s`);
          break;
        }
      }

      // If no exact match, find the closest segment
      if (!segmentToPlay) {
        let closestIndex = 0;
        let minDistance = Infinity;

        for (let i = 0; i < timelineState.timelineSegments.length; i++) {
          const segment = timelineState.timelineSegments[i];
          const midpoint = (segment.start_timestamp + segment.end_timestamp) / 2;
          const distance = Math.abs(timelineState.currentTime - midpoint);

          if (distance < minDistance) {
            minDistance = distance;
            closestIndex = i;
          }
        }

        segmentToPlay = timelineState.timelineSegments[closestIndex];
        segmentIndex = closestIndex;
        // If the current time is after the segment, start at the beginning
        if (timelineState.currentTime < segmentToPlay.start_timestamp) {
          relativeTime = 0;
        } else {
          // Otherwise, start at the end
          relativeTime = 0;
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
    timelineState.currentTime = segmentToPlay.start_timestamp + relativeTime;
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

          // Set the current time
          const validTime = Math.max(0, Math.min(relativeTime, videoElement.duration || 0));
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
                        console.error('Error restarting video:', e);
                      });
                    }
                  }, 500);
                }
              }).catch(e => {
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
      videoElement.src = `/api/recordings/play/${segmentToPlay.id}?t=${Date.now()}`;
      videoElement.load();
    } else {
      console.error('TimelineControls: No video element found');
      showStatusMessage('Error: Video player not found', 'error');
    }
  };

  // Play a specific segment
  const playSegment = (index) => {
    // This function will be implemented in the TimelinePlayer component
    // Here we just update the state
    timelineState.setState({
      currentSegmentIndex: index,
      isPlaying: true
    });
  };

  // Zoom in on timeline
  const zoomIn = () => {
    if (zoomLevel < 8) {
      const newZoomLevel = zoomLevel * 2;
      timelineState.setState({ zoomLevel: newZoomLevel });
      showStatusMessage(`Zoomed in: ${24 / newZoomLevel} hours view`, 'info');
    }
  };

  // Zoom out on timeline
  const zoomOut = () => {
    if (zoomLevel > 1) {
      const newZoomLevel = zoomLevel / 2;
      timelineState.setState({ zoomLevel: newZoomLevel });
      showStatusMessage(`Zoomed out: ${24 / newZoomLevel} hours view`, 'info');
    }
  };

  return html`
    <div class="timeline-controls flex justify-between items-center mb-2">
      <div class="flex items-center">
        <button
          id="play-button"
          class="w-10 h-10 rounded-full bg-green-600 hover:bg-green-700 text-white flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-offset-1 transition-colors shadow-sm mr-2"
          onClick=${togglePlayback}
          title=${isPlaying ? 'Pause' : 'Play from current position'}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            ${isPlaying
              ? html`
                <!-- Pause icon - two vertical bars -->
                <rect x="6" y="6" width="4" height="12" rx="1" fill="white" />
                <rect x="14" y="6" width="4" height="12" rx="1" fill="white" />
              `
              : html`
                <!-- Play icon - triangle -->
                <path d="M8 5.14v14l11-7-11-7z" fill="white" />
              `
            }
          </svg>
        </button>
        <span class="text-xs text-gray-600 dark:text-gray-300">Play from current position</span>
      </div>

      <div class="flex items-center gap-1">
        <span class="text-xs text-gray-600 dark:text-gray-300 mr-1">Zoom:</span>
        <button
          id="zoom-out-button"
          class="w-6 h-6 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors"
          onClick=${zoomOut}
          title="Zoom Out (Show more time)"
          disabled=${zoomLevel <= 1}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
        <button
          id="zoom-in-button"
          class="w-6 h-6 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors"
          onClick=${zoomIn}
          title="Zoom In (Show less time)"
          disabled=${zoomLevel >= 8}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>
  `;
}
