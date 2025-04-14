/**
 * LightNVR Timeline Player Component
 * Handles video playback for the timeline
 */

import { html } from '../../../html-helper.js';
import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.js';
import { SpeedControls } from './SpeedControls.js';
import { showStatusMessage } from '../UI.js';

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
    const relativeTime = state.currentTime !== null && 
                         state.currentTime >= segment.start_timestamp
      ? state.currentTime - segment.start_timestamp
      : 0;
    
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
      
      // Set current time
      video.currentTime = seekTime;
      
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
      
      // Update timeline state
      timelineState.setState({
        currentSegmentIndex: nextIndex,
        currentTime: segments[nextIndex].start_timestamp,
        isPlaying: true,
        forceReload: true
      });
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
    
    // Calculate current timestamp
    const currentTime = segment.start_timestamp + video.currentTime;
    
    // Directly update the time display as well
    updateTimeDisplay(currentTime);
    
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
    
    const date = new Date(time * 1000);
    
    // Format date and time
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');
    
    // Display time only (HH:MM:SS)
    timeDisplay.textContent = `${hours}:${minutes}:${seconds}`;
  };

  return html`
    <div class="timeline-player-container mb-2" id="video-player">
      <div class="relative w-full bg-black rounded-lg shadow-md" style="aspect-ratio: 16/9;">
        <video
            ref=${videoRef}
            class="w-full h-full object-contain"
            controls
            autoplay=${false}
            muted=${false}
            playsInline
            onended=${handleEnded}
            ontimeupdate=${handleTimeUpdate}
        ></video>
        
        <!-- Add a message for invalid segments -->
        <div 
          class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-70 text-white text-center p-4 ${currentSegmentIndex >= 0 && segments.length > 0 ? 'hidden' : ''}"
        >
          <div>
            <p class="mb-2">No valid segment selected.</p>
            <p class="text-sm">Click on a segment in the timeline or use the play button to start playback.</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Playback speed controls -->
    <${SpeedControls} />
  `;
}
