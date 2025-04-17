/**
 * LightNVR Speed Controls Component
 * Handles playback speed controls for the timeline
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
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
      // Set the new playback rate
      videoPlayer.playbackRate = speed;
    }

    // Update timeline state
    timelineState.setState({ playbackSpeed: speed });

    // Show status message
    showStatusMessage(`Playback speed: ${speed}x`, 'info');
  };

  return (
    <div className="mt-2 mb-4 p-2 border border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
      <div className="flex flex-col items-center">
        <div className="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">Playback Speed</div>

        <div className="flex flex-wrap justify-center gap-1">
          {speeds.map(speed => (
            <button
              key={`speed-${speed}`}
              className={`speed-btn px-2 py-1 text-sm rounded-full ${speed === currentSpeed
                ? 'bg-green-500 text-white'
                : 'bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600'}
                font-medium transition-all focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-opacity-50`}
              data-speed={speed}
              onClick={() => setPlaybackSpeed(speed)}
            >
              {speed === 1.0 ? '1× (Normal)' : `${speed}×`}
            </button>
          ))}
        </div>

        <div className="mt-1 text-xs font-medium text-green-600 dark:text-green-400">
          Current: {currentSpeed}× {currentSpeed === 1.0 ? '(Normal)' : ''}
        </div>
      </div>
    </div>
  );
}
