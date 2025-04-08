/**
 * LightNVR Speed Controls Component
 * Handles playback speed controls for the timeline
 */


import { html } from '../../../html-helper.js';
import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.js';
import { showStatusMessage } from '../UI.js';

/**
 * SpeedControls component
 * @returns {JSX.Element} SpeedControls component
 */
export function SpeedControls() {
  // Local state
  const [currentSpeed, setCurrentSpeed] = useState(1.0);
  
  // Available speeds
  const speeds = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0];

  // Subscribe to timeline state changes
  useEffect(() => {
    console.log('SpeedControls: Setting up subscription to timelineState');
    
    const unsubscribe = timelineState.subscribe(state => {
      console.log('SpeedControls: Received state update:', state);
      console.log('SpeedControls: Playback speed:', state.playbackSpeed);
      
      setCurrentSpeed(state.playbackSpeed);
    });
    
    // Log initial state
    console.log('SpeedControls: Initial timelineState:', timelineState);
    
    return () => unsubscribe();
  }, []);

  // Set playback speed
  const setPlaybackSpeed = (speed) => {
    // Update video playback rate
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer) {
      // Store the current playback rate for debugging
      const oldRate = videoPlayer.playbackRate;
      
      // Set the new playback rate
      videoPlayer.playbackRate = speed;
      
      console.log(`Setting video playback rate from ${oldRate}x to ${speed}x`, videoPlayer);
      console.log(`Actual playback rate after setting: ${videoPlayer.playbackRate}x`);
      
      // Force the playback rate again after a short delay
      setTimeout(() => {
        videoPlayer.playbackRate = speed;
        console.log(`Re-setting playback rate to ${speed}x, actual rate: ${videoPlayer.playbackRate}x`);
      }, 100);
    } else {
      console.warn('Video player element not found');
    }
    
    // Update timeline state
    timelineState.setState({ playbackSpeed: speed });
    
    // Show status message
    showStatusMessage(`Playback speed: ${speed}x`, 'info');
  };

  return html`
    <div class="mt-2 mb-4 p-2 border border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
      <div class="flex flex-col items-center">
        <div class="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">Playback Speed</div>
        
        <div class="flex flex-wrap justify-center gap-1">
          ${speeds.map(speed => html`
            <button 
              key=${`speed-${speed}`}
              class=${`speed-btn px-2 py-1 text-sm rounded-full ${speed === currentSpeed 
                ? 'bg-green-500 text-white' 
                : 'bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600'} 
                font-medium transition-all focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-opacity-50`}
              data-speed=${speed}
              onClick=${() => setPlaybackSpeed(speed)}
            >
              ${speed === 1.0 ? '1× (Normal)' : `${speed}×`}
            </button>
          `)}
        </div>
        
        <div class="mt-1 text-xs font-medium text-green-600 dark:text-green-400">
          Current: ${currentSpeed}× ${currentSpeed === 1.0 ? '(Normal)' : ''}
        </div>
      </div>
    </div>
  `;
}
