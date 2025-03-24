/**
 * LightNVR Speed Controls Component
 * Handles playback speed controls for the timeline
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect } from '../../../preact.hooks.module.js';
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
    const unsubscribe = timelineState.subscribe(state => {
      setCurrentSpeed(state.playbackSpeed);
    });
    
    return () => unsubscribe();
  }, []);

  // Set playback speed
  const setPlaybackSpeed = (speed) => {
    // Update video playback rate
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer) {
      videoPlayer.playbackRate = speed;
      console.log(`Setting video playback rate to ${speed}x`, videoPlayer);
    } else {
      console.warn('Video player element not found');
    }
    
    // Update timeline state
    timelineState.setState({ playbackSpeed: speed });
    
    // Show status message
    showStatusMessage(`Playback speed: ${speed}x`, 'info');
  };

  return html`
    <div class="mt-6 mb-8 p-4 border-2 border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-md">
      <h3 class="text-lg font-bold text-center mb-4 text-gray-800 dark:text-white">PLAYBACK SPEED CONTROLS</h3>
      
      <div class="flex flex-wrap justify-center gap-2">
        ${speeds.map(speed => html`
          <button 
            key=${`speed-${speed}`}
            class=${`speed-btn px-4 py-2 rounded-full ${speed === currentSpeed 
              ? 'bg-green-500 text-white' 
              : 'bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600'} 
              font-bold transition-all transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50`}
            data-speed=${speed}
            onClick=${() => setPlaybackSpeed(speed)}
          >
            ${speed === 1.0 ? '1× (Normal)' : `${speed}×`}
          </button>
        `)}
      </div>
      
      <div class="mt-4 text-center font-bold text-green-600 dark:text-green-400">
        Current Speed: ${currentSpeed}× ${currentSpeed === 1.0 ? '(Normal)' : ''}
      </div>
    </div>
  `;
}
