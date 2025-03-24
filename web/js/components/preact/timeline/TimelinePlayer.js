/**
 * LightNVR Timeline Player Component
 * Handles video playback for the timeline
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect, useRef } from '../../../preact.hooks.module.js';
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
  const playbackIntervalRef = useRef(null);
  const hlsPlayerRef = useRef(null);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Store previous values to check for changes
      const prevSegmentIndex = currentSegmentIndex;
      const prevIsPlaying = isPlaying;
      
      // Update local state
      setCurrentSegmentIndex(state.currentSegmentIndex);
      setIsPlaying(state.isPlaying);
      setSegments(state.timelineSegments || []);
      setPlaybackSpeed(state.playbackSpeed);
      
      // If current segment index changed, play that segment
      // But only if it's a real change, not just a state update
      if (state.currentSegmentIndex !== prevSegmentIndex && state.currentSegmentIndex >= 0) {
        console.log(`Segment index changed from ${prevSegmentIndex} to ${state.currentSegmentIndex}`);
        
        const startTime = state.currentTime !== null && 
                          state.timelineSegments[state.currentSegmentIndex] &&
                          state.currentTime >= state.timelineSegments[state.currentSegmentIndex].start_timestamp
          ? state.currentTime - state.timelineSegments[state.currentSegmentIndex].start_timestamp
          : 0;
          
        // Direct call to play video without updating state again
        playVideoDirectly(state.currentSegmentIndex, startTime);
      }
      
      // Update playback speed if it changed
      if (state.playbackSpeed !== playbackSpeed && videoRef.current) {
        videoRef.current.playbackRate = state.playbackSpeed;
      }
    });
    
    return () => {
      unsubscribe();
      
      // Clean up interval
      if (playbackIntervalRef.current) {
        clearInterval(playbackIntervalRef.current);
      }
      
      // Clean up HLS player
      if (hlsPlayerRef.current) {
        hlsPlayerRef.current.destroy();
      }
    };
  }, [currentSegmentIndex, playbackSpeed]);

  // Set up video event listeners
  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;
    
    const handleTimeUpdate = () => {
      if (currentSegmentIndex < 0 || !segments[currentSegmentIndex]) return;
      
      const segment = segments[currentSegmentIndex];
      
      // Calculate current timestamp based on video currentTime
      const currentTime = segment.start_timestamp + video.currentTime;
      
      // Update timeline state
      timelineState.setState({ currentTime });
    };
    
    const handleEnded = () => {
      console.log('Video ended event triggered');
      // Try to play the next segment
      if (currentSegmentIndex < segments.length - 1) {
        playSegment(currentSegmentIndex + 1);
      } else {
        // End of all segments
        pausePlayback();
      }
    };
    
    const handleError = (e) => {
      console.error('Video error:', e);
      showStatusMessage('Video playback error', 'error');
    };
    
    // Add event listeners
    video.addEventListener('timeupdate', handleTimeUpdate);
    video.addEventListener('ended', handleEnded);
    video.addEventListener('error', handleError);
    
    // Start playback tracking
    startPlaybackTracking();
    
    return () => {
      // Remove event listeners
      video.removeEventListener('timeupdate', handleTimeUpdate);
      video.removeEventListener('ended', handleEnded);
      video.removeEventListener('error', handleError);
      
      // Clear tracking interval
      if (playbackIntervalRef.current) {
        clearInterval(playbackIntervalRef.current);
      }
    };
  }, [videoRef.current, currentSegmentIndex, segments]);

  // Play video directly without updating state (to avoid infinite recursion)
  const playVideoDirectly = (index, seekTime = 0) => {
    console.log(`playVideoDirectly(${index}, ${seekTime})`);
    
    if (index < 0 || index >= segments.length) {
      console.warn(`Invalid segment index: ${index}, segments length: ${segments.length}`);
      return;
    }
    
    const segment = segments[index];
    console.log('Playing segment directly:', segment);
    
    // Get video element
    const video = videoRef.current;
    if (!video) {
      console.error('Video element ref is null');
      
      // Try to get the video element directly
      const directVideo = document.querySelector('#video-player video');
      if (directVideo) {
        console.log('Found video element directly:', directVideo);
        
        // Use direct MP4 playback
        const recordingUrl = `/api/recordings/play/${segment.id}`;
        console.log('Setting video source to:', recordingUrl);
        
        // Set the video source
        directVideo.src = recordingUrl;
        directVideo.currentTime = seekTime;
        
        // Set playback speed
        directVideo.playbackRate = playbackSpeed;
        
        // Play the video
        directVideo.play().catch(error => {
          console.error('Error playing video:', error);
          showStatusMessage('Error playing video: ' + error.message, 'error');
        });
        
        return;
      } else {
        console.error('Could not find video element');
        return;
      }
    }
    
    // Clean up any existing HLS player
    if (hlsPlayerRef.current) {
      hlsPlayerRef.current.destroy();
      hlsPlayerRef.current = null;
    }
    
    // Use direct MP4 playback
    const recordingUrl = `/api/recordings/play/${segment.id}`;
    console.log('Setting video source to:', recordingUrl);
    
    // Set the video source
    video.src = recordingUrl;
    video.currentTime = seekTime;
    
    // Set playback speed
    video.playbackRate = playbackSpeed;
    
    // Play the video
    video.play().catch(error => {
      console.error('Error playing video:', error);
      showStatusMessage('Error playing video: ' + error.message, 'error');
    });
  };

  // Play a specific segment (updates state and plays video)
  const playSegment = (index, seekTime = 0) => {
    console.log(`TimelinePlayer.playSegment(${index}, ${seekTime})`);
    
    if (index < 0 || index >= segments.length) {
      console.warn(`Invalid segment index: ${index}, segments length: ${segments.length}`);
      return;
    }
    
    const segment = segments[index];
    console.log('Playing segment:', segment);
    
    // Update state
    timelineState.setState({ 
      currentSegmentIndex: index,
      currentTime: segment.start_timestamp + seekTime,
      isPlaying: true
    });
  };

  // Pause playback
  const pausePlayback = () => {
    const video = videoRef.current;
    if (video) {
      video.pause();
    }
    
    timelineState.setState({ isPlaying: false });
  };

  // Start tracking playback progress
  const startPlaybackTracking = () => {
    // Clear any existing interval
    if (playbackIntervalRef.current) {
      clearInterval(playbackIntervalRef.current);
    }
    
    // Update more frequently (every 100ms) for smoother cursor movement
    playbackIntervalRef.current = setInterval(() => {
      if (!isPlaying || currentSegmentIndex < 0 || !videoRef.current) {
        return;
      }
      
      const segment = segments[currentSegmentIndex];
      if (!segment) {
        console.error('Invalid segment at index', currentSegmentIndex);
        return;
      }
      
      // Calculate current timestamp based on video currentTime
      const currentTime = segment.start_timestamp + videoRef.current.currentTime;
      
      // Update timeline state
      timelineState.setState({ currentTime });
      
      // Check if we've reached the end of the segment
      if (videoRef.current.currentTime >= segment.duration) {
        console.log('End of segment reached, trying to play next segment');
        // Try to play the next segment
        if (currentSegmentIndex < segments.length - 1) {
          playSegment(currentSegmentIndex + 1);
        } else {
          // End of all segments
          pausePlayback();
        }
      }
    }, 100);
  };

  return html`
    <div class="timeline-player-container mb-4" id="video-player">
      <div class="relative w-full bg-black rounded-lg overflow-hidden shadow-lg" style="aspect-ratio: 16/9;">
        <video 
          ref=${videoRef}
          class="w-full h-full"
          controls
          autoplay=${false}
          muted=${false}
          playsInline
        ></video>
      </div>
      
      <${SpeedControls} />
    </div>
  `;
}
