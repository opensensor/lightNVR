/**
 * LightNVR Speed Controls Component
 * Handles playback speed controls for the timeline
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';

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
    <div className="mt-2 mb-4 p-2 border border-primary rounded-lg bg-card text-card-foreground shadow-sm">
      <div className="flex flex-col items-center">
        <div className="text-sm font-semibold mb-2 text-foreground">Playback Speed</div>

        <div className="flex flex-wrap justify-center gap-1">
          {speeds.map(speed => (
            <button
              key={`speed-${speed}`}
              className={`speed-btn px-2 py-1 text-sm rounded-full ${speed === currentSpeed
                ? 'bg-primary text-primary-foreground'
                : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}
                font-medium transition-all focus:outline-none focus:ring-1 focus:ring-primary focus:ring-opacity-50`}
              data-speed={speed}
              onClick={() => setPlaybackSpeed(speed)}
            >
              {speed === 1.0 ? '1× (Normal)' : `${speed}×`}
            </button>
          ))}
        </div>

        <div className="mt-1 text-xs font-medium text-primary">
          Current: {currentSpeed}× {currentSpeed === 1.0 ? '(Normal)' : ''}
        </div>
      </div>
    </div>
  );
}
