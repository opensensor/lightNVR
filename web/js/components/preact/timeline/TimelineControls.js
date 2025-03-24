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
    if (timelineState.timelineSegments.length === 0) {
      showStatusMessage('No recordings to play', 'warning');
      return;
    }
    
    // If there's a video player and current segment index is valid, play it
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer && timelineState.currentSegmentIndex >= 0) {
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
          class="w-10 h-10 rounded-full bg-blue-500 hover:bg-blue-600 text-white flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
          onClick=${togglePlayback}
          title=${isPlaying ? 'Pause' : 'Play'}
        >
          <span class="icon text-lg">${isPlaying ? '⏸️' : '▶️'}</span>
        </button>
      </div>
      
      <div class="timeline-zoom-controls flex items-center gap-2">
        <button 
          id="zoom-out-button" 
          class="w-8 h-8 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
          onClick=${zoomOut}
          title="Zoom Out"
          disabled=${zoomLevel <= 1}
        >
          <span class="icon">➖</span>
        </button>
        <button 
          id="zoom-in-button" 
          class="w-8 h-8 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
          onClick=${zoomIn}
          title="Zoom In"
          disabled=${zoomLevel >= 8}
        >
          <span class="icon">➕</span>
        </button>
      </div>
    </div>
  `;
}
