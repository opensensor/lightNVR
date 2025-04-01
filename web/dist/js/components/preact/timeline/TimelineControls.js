/**
 * LightNVR Timeline Controls Component
 * Handles play/pause and zoom controls for the timeline
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect } from '../../../preact.hooks.module.js';
import { timelineState } from './TimelinePage.js';
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
    console.log('TimelineControls: Setting up subscription to timelineState');
    
    const unsubscribe = timelineState.subscribe(state => {
      console.log('TimelineControls: Received state update:', state);
      console.log('TimelineControls: Is playing:', state.isPlaying);
      console.log('TimelineControls: Zoom level:', state.zoomLevel);
      console.log('TimelineControls: Segments count:', state.timelineSegments?.length || 0);
      
      setIsPlaying(state.isPlaying);
      setZoomLevel(state.zoomLevel);
    });
    
    // Log initial state
    console.log('TimelineControls: Initial timelineState:', timelineState);
    
    return () => unsubscribe();
  }, []);

  // Toggle playback (play/pause)
  const togglePlayback = () => {
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
    
    // Find the earliest segment in the timeline
    let earliestSegmentIndex = 0;
    let earliestTimestamp = Number.MAX_SAFE_INTEGER;
    
    timelineState.timelineSegments.forEach((segment, index) => {
      if (segment.start_timestamp < earliestTimestamp) {
        earliestTimestamp = segment.start_timestamp;
        earliestSegmentIndex = index;
      }
    });
    
    console.log(`Starting from earliest segment (index ${earliestSegmentIndex})`);
    
    // Start playing from the earliest segment
    timelineState.setState({ 
      currentSegmentIndex: earliestSegmentIndex,
      currentTime: timelineState.timelineSegments[earliestSegmentIndex].start_timestamp,
      isPlaying: true,
      forceReload: true // Force reload to ensure video player updates
    });
    
    // Force load the earliest segment's video
    const segment = timelineState.timelineSegments[earliestSegmentIndex];
    const videoPlayer = document.querySelector('#video-player video');
    
    if (videoPlayer) {
      console.log('Loading earliest segment video:', segment);
      
      // Pause any current playback
      videoPlayer.pause();
      
      // Clear the source and reload
      videoPlayer.removeAttribute('src');
      videoPlayer.load();
      
      // Set the new source with a timestamp to prevent caching
      videoPlayer.src = `/api/recordings/play/${segment.id}?t=${Date.now()}`;
      
      // Set the current time and play
      videoPlayer.onloadedmetadata = () => {
        videoPlayer.currentTime = 0;
        videoPlayer.play().catch(error => {
          console.error('Error playing video:', error);
          showStatusMessage('Error playing video: ' + error.message, 'error');
        });
      };
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
          title=${isPlaying ? 'Pause' : 'Play from earliest recording'}
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
        <span class="text-xs text-gray-600 dark:text-gray-300">Play from earliest recording</span>
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
