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
    const unsubscribe = timelineState.subscribe(state => {
      setIsPlaying(state.isPlaying);
      setZoomLevel(state.zoomLevel);
    });
    
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
    
    // If there's a video player and current segment index is valid, play it
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer && timelineState.currentSegmentIndex >= 0 && 
        timelineState.currentSegmentIndex < timelineState.timelineSegments.length) {
      console.log('Playing video from TimelineControls', videoPlayer);
      videoPlayer.play().catch(error => {
        console.error('Error playing video:', error);
        showStatusMessage('Error playing video: ' + error.message, 'error');
      });
      timelineState.setState({ isPlaying: true });
    } else if (timelineState.timelineSegments.length > 0) {
      console.log('Starting first segment from TimelineControls');
      // Start playing the first segment
      timelineState.setState({ 
        currentSegmentIndex: 0,
        currentTime: timelineState.timelineSegments[0].start_timestamp,
        isPlaying: true
      });
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
    <div class="timeline-controls flex justify-between items-center mb-4">
      <div class="flex items-center gap-2">
        <button 
          id="play-button" 
          class="w-12 h-12 rounded-full bg-green-600 hover:bg-green-700 text-white flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 transition-colors shadow-md"
          onClick=${togglePlayback}
          title=${isPlaying ? 'Pause' : 'Play'}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            ${isPlaying 
              ? html`<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6m-9-6h18" />`
              : html`<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />`
            }
          </svg>
        </button>
      </div>
      
      <div class="timeline-zoom-controls flex flex-col items-end">
        <div class="text-sm text-gray-600 dark:text-gray-300 mb-1">Timeline Zoom</div>
        <div class="flex items-center gap-2">
          <button 
            id="zoom-out-button" 
            class="w-8 h-8 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
            onClick=${zoomOut}
            title="Zoom Out (Show more time)"
            disabled=${zoomLevel <= 1}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>
          <button 
            id="zoom-in-button" 
            class="w-8 h-8 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
            onClick=${zoomIn}
            title="Zoom In (Show less time)"
            disabled=${zoomLevel >= 8}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  `;
}
