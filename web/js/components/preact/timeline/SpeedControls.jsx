/**
 * LightNVR Speed Controls Component
 * Handles playback speed controls for the timeline
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { useI18n } from '../../../i18n.js';

/**
 * SpeedControls component
 * @returns {JSX.Element} SpeedControls component
 */
export function SpeedControls() {
  const { t } = useI18n();
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
    showStatusMessage(t('timeline.playbackSpeed', { speed }), 'info');
  };

  return (
    <div className="flex items-center gap-0.5">
      <span className="text-[10px] text-muted-foreground mr-0.5">{t('timeline.speed')}</span>
      {speeds.map(speed => (
        <button
          key={`speed-${speed}`}
          className={`px-1.5 py-0.5 text-[11px] rounded ${speed === currentSpeed
            ? 'bg-primary text-primary-foreground'
            : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}
            font-medium transition-all focus:outline-none`}
          data-speed={speed}
          onClick={() => setPlaybackSpeed(speed)}
        >
          {speed}×
        </button>
      ))}
    </div>
  );
}
