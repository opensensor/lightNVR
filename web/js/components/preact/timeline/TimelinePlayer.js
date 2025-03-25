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
  const lastTimeUpdateRef = useRef(null);
  const hlsPlayerRef = useRef(null);
  const lastDragTimeRef = useRef(null);

// Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Store previous values to check for changes
      const prevSegmentIndex = currentSegmentIndex;
      const prevIsPlaying = isPlaying;
      const prevCurrentTime = timelineState.prevCurrentTime;
      // Update the previous currentTime for the next state change
      timelineState.prevCurrentTime = state.currentTime;

      // Update local state
      setCurrentSegmentIndex(state.currentSegmentIndex);
      setIsPlaying(state.isPlaying);
      setSegments(state.timelineSegments || []);
      setPlaybackSpeed(state.playbackSpeed);

      // Check if we have valid segments and segment index
      const hasValidSegment = state.timelineSegments &&
          state.timelineSegments.length > 0 &&
          state.currentSegmentIndex >= 0 &&
          state.currentSegmentIndex < state.timelineSegments.length;

      if (!hasValidSegment) {
        console.warn('Invalid segment data or index');
        return;
      }

      const currentSegment = state.timelineSegments[state.currentSegmentIndex];
      if (!currentSegment) {
        console.warn(`Segment at index ${state.currentSegmentIndex} is undefined`);
        return;
      }

      // Calculate relative time within the segment
      const relativeTime = state.currentTime !== null &&
      state.currentTime >= currentSegment.start_timestamp
          ? state.currentTime - currentSegment.start_timestamp
          : 0;

      // Determine if a video reload is needed
      let needsReload = false;

      // Case 1: forceReload flag is set - always reload video
      if (state.forceReload) {
        console.log('forceReload flag detected, reloading video');
        needsReload = true;
        // Reset the flag after detecting it
        timelineState.forceReload = false;
      }
      // Case 2: Segment index changed - always reload video
      else if (state.currentSegmentIndex !== prevSegmentIndex) {
        console.log(`Segment index changed from ${prevSegmentIndex} to ${state.currentSegmentIndex}`);
        needsReload = true;
      }
      // Case 3: Play state changed to playing - reload if not already playing
      else if (state.isPlaying && !prevIsPlaying) {
        console.log(`Play state changed to playing, ensuring video playback`);
        needsReload = true;
      }
      // Case 4: Current time changed significantly - handle as a seek or drag operation
      else if (state.currentTime !== prevCurrentTime) {
        console.log(`Current time changed from ${prevCurrentTime} to ${state.currentTime}`);

        // Check if this is a drag operation (multiple time changes in quick succession)
        const now = Date.now();
        const isDrag = lastDragTimeRef.current && (now - lastDragTimeRef.current < 500);
        lastDragTimeRef.current = now;

        // Always reload the video when the current time changes due to a user interaction
        // This fixes the bug where adjusting the segment needle doesn't reload the video
        // until the current segment finishes playing
        needsReload = true;
      }

      // Case 4: External component changed segment or time but didn't trigger any of the above
      // This is a new check that ensures proper loading when other components modify the state
      // We can detect this by checking if the video source doesn't match the expected segment
      if (!needsReload && videoRef.current) {
        const expectedVideoSrc = getVideoSourceForSegment(currentSegment);
        const currentVideoSrc = videoRef.current.src;

        if (currentVideoSrc !== expectedVideoSrc) {
          console.log('Video source mismatch detected, reloading video');
          needsReload = true;
        }
      }

      // If we determined a reload is needed, play the video directly
      if (needsReload) {
        playVideoDirectly(state.currentSegmentIndex, relativeTime);
      }

      // Update playback speed if it changed
      if (state.playbackSpeed !== playbackSpeed && videoRef.current) {
        console.log(`Updating playback speed from ${playbackSpeed} to ${state.playbackSpeed}`);
        videoRef.current.playbackRate = state.playbackSpeed;
        setPlaybackSpeed(state.playbackSpeed);
      }
    });

    // Cleanup subscription when component unmounts
    return () => unsubscribe();
  }, []);

// This function would need to be implemented to get the expected video source for a segment
  function getVideoSourceForSegment(segment) {
    // Implementation depends on how your video sources are constructed
    // Example implementation:
    return segment.video_url || `${segment.base_url}/${segment.id}`;
  }

  // Set up video event listeners
  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    console.log('Setting up video event listeners');

    const handleTimeUpdate = () => {
      // Check if we have a valid segment index and segments array
      if (currentSegmentIndex < 0 || !segments || segments.length === 0 || currentSegmentIndex >= segments.length) {
        return;
      }

      const segment = segments[currentSegmentIndex];
      if (!segment) {
        console.error('Invalid segment at index', currentSegmentIndex);
        return;
      }

      // Calculate current timestamp based on video currentTime
      const currentTime = segment.start_timestamp + video.currentTime;

      // Store the previous currentTime before updating
      const prevCurrentTime = timelineState.currentTime;

      // Update timeline state
      timelineState.setState({ 
        currentTime,
        prevCurrentTime
      });

      // Debug log for video progress
      if (Math.floor(video.currentTime) % 5 === 0 && video.currentTime > 0) { // Log every 5 seconds
        console.log(`Video progress: ${video.currentTime.toFixed(2)}s / ${video.duration.toFixed(2)}s`);
      }
    };

    const handleEnded = () => {
      console.log('Video ended event listener triggered');
      console.log('Current segment index:', currentSegmentIndex);
      console.log('Total segments:', segments.length);

      // Check if we have a valid segment index and segments array
      if (currentSegmentIndex < 0 || !segments || segments.length === 0) {
        console.log('No valid segments, pausing playback');
        pausePlayback();
        return;
      }

      // Try to play the next segment
      if (currentSegmentIndex < segments.length - 1) {
        const nextIndex = currentSegmentIndex + 1;
        console.log(`Auto-playing next segment ${nextIndex} from ended event listener`);

        // Get the next segment
        const nextSegment = segments[nextIndex];
        if (!nextSegment) {
          console.warn(`Next segment at index ${nextIndex} is undefined`);
          pausePlayback();
          return;
        }

        console.log('Next segment:', nextSegment);

        // Update state - IMPORTANT: This must happen before loading the video
        // to ensure the state is updated before any callbacks are triggered
        timelineState.setState({
          currentSegmentIndex: nextIndex,
          currentTime: nextSegment.start_timestamp,
          isPlaying: true
        });

        try {
          // Force load and play the next segment's video
          console.log('Loading next segment video from ended event listener:', nextSegment);

          // Pause the current video first
          video.pause();

          // Clear any existing source and events
          video.removeAttribute('src');
          video.load();

          // Set the new source with a unique timestamp to prevent caching
          const recordingUrl = `/api/recordings/play/${nextSegment.id}?t=${Date.now()}`;
          console.log('Setting video source to:', recordingUrl);
          video.src = recordingUrl;

          // Wait for metadata to load before setting currentTime and playbackRate
          video.onloadedmetadata = () => {
            console.log('Video metadata loaded for next segment from ended event listener');
            video.currentTime = 0;

            // Set playback speed and verify it was set correctly
            const oldRate = video.playbackRate;
            video.playbackRate = playbackSpeed;
            console.log(`Setting playback rate from ${oldRate}x to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);

            // Force the playback rate again after a short delay
            setTimeout(() => {
              video.playbackRate = playbackSpeed;
              console.log(`Re-setting playback rate to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);
            }, 100);

            // Play the video
            video.play().catch(error => {
              console.error('Error playing next video:', error);
              showStatusMessage('Error playing next video: ' + error.message, 'error');
            });
          };

          // Add error handler for loading
          video.onerror = (e) => {
            console.error('Error loading next video:', e);
            showStatusMessage('Error loading next video, skipping to next segment', 'warning');

            // Try to skip to the next segment if this one fails
            if (currentSegmentIndex < segments.length - 1) {
              console.log(`Skipping to next segment ${currentSegmentIndex + 1} due to error`);

              // Update state to next segment
              timelineState.setState({
                currentSegmentIndex: currentSegmentIndex + 1,
                currentTime: segments[currentSegmentIndex + 1].start_timestamp,
                isPlaying: true
              });
            } else {
              // No more segments, pause playback
              pausePlayback();
            }
          };
        } catch (error) {
          console.error('Exception while setting up next video:', error);
          showStatusMessage('Error setting up next video: ' + error.message, 'error');
        }
      } else {
        // End of all segments
        console.log('Reached end of all segments');
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

    // Add additional event listeners for debugging
    video.addEventListener('play', () => console.log('Video play event'));
    video.addEventListener('pause', () => console.log('Video pause event'));
    video.addEventListener('seeking', () => console.log('Video seeking event'));
    video.addEventListener('seeked', () => console.log('Video seeked event'));
    video.addEventListener('loadedmetadata', () => console.log('Video loadedmetadata event, duration:', video.duration));

    // Start playback tracking
    startPlaybackTracking();

    return () => {
      // Remove event listeners
      video.removeEventListener('timeupdate', handleTimeUpdate);
      video.removeEventListener('ended', handleEnded);
      video.removeEventListener('error', handleError);

      // Remove additional event listeners
      video.removeEventListener('play', () => console.log('Video play event'));
      video.removeEventListener('pause', () => console.log('Video pause event'));
      video.removeEventListener('seeking', () => console.log('Video seeking event'));
      video.removeEventListener('seeked', () => console.log('Video seeked event'));
      video.removeEventListener('loadedmetadata', () => console.log('Video loadedmetadata event'));

      // Clear tracking interval
      if (playbackIntervalRef.current) {
        clearInterval(playbackIntervalRef.current);
      }
    };
  }, [videoRef.current, currentSegmentIndex, segments]);

  // Play video directly without updating state (to avoid infinite recursion)
  const playVideoDirectly = (index, seekTime = 0) => {
    console.log(`playVideoDirectly(${index}, ${seekTime})`);

    // Check if segments array is valid and not empty
    if (!segments || segments.length === 0) {
      console.warn('No segments available');
      return;
    }

    if (index < 0 || index >= segments.length) {
      console.warn(`Invalid segment index: ${index}, segments length: ${segments.length}`);
      return;
    }

    const segment = segments[index];
    if (!segment) {
      console.warn(`Segment at index ${index} is undefined`);
      return;
    }

    console.log('Playing segment directly:', segment);

    // Get video element
    const video = videoRef.current;
    if (!video) {
      console.error('Video element ref is null');

      // Try to get the video element directly
      const directVideo = document.querySelector('#video-player video');
      if (directVideo) {
        console.log('Found video element directly:', directVideo);

        // Use direct MP4 playback with a timestamp to prevent caching
        const recordingUrl = `/api/recordings/play/${segment.id}?t=${Date.now()}`;
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

    try {
      // Force video reload by creating a new video element
      // This is a more aggressive approach to ensure the video reloads
      const videoContainer = video.parentElement;
      if (videoContainer) {
        console.log('Forcing complete video element reload');
        
        // Remove the old video element
        videoContainer.removeChild(video);
        
        // Create a new video element with the same attributes
        const newVideo = document.createElement('video');
        newVideo.className = video.className;
        newVideo.controls = true;
        newVideo.playsInline = true;
        newVideo.onended = onEnded;
        
        // Add the new video element to the container
        videoContainer.appendChild(newVideo);
        
        // Update the ref to point to the new video element
        videoRef.current = newVideo;
        
        // Use direct MP4 playback with a timestamp to prevent caching
        const recordingUrl = `/api/recordings/play/${segment.id}?t=${Date.now()}`;
        console.log('Setting video source to:', recordingUrl);
        
        // Set the new source
        newVideo.src = recordingUrl;
        
        // Wait for metadata to load before setting currentTime and playbackRate
        newVideo.onloadedmetadata = () => {
          console.log('Video metadata loaded for new video element');
          newVideo.currentTime = seekTime;
          
          // Set playback speed
          newVideo.playbackRate = playbackSpeed;
          console.log(`Setting playback rate to ${playbackSpeed}x`);
          
          // Play the video
          newVideo.play().catch(error => {
            console.error('Error playing video:', error);
            showStatusMessage('Error playing video: ' + error.message, 'error');
          });
        };
        
        // Set up event listeners for the new video
        newVideo.addEventListener('timeupdate', handleTimeUpdate);
        newVideo.addEventListener('ended', handleEnded);
        newVideo.addEventListener('error', handleError);
        
        return;
      }
      
      // Fallback to the original approach if we can't replace the video element
      console.log('Falling back to standard video reload approach');
      
      // Use direct MP4 playback with a timestamp to prevent caching
      const recordingUrl = `/api/recordings/play/${segment.id}?t=${Date.now()}`;
      console.log('Setting video source to:', recordingUrl);

      // Pause the current video first
      video.pause();

      // Clear any existing source and events
      video.removeAttribute('src');
      video.load();

      // Set the new source
      video.src = recordingUrl;

      // Wait for metadata to load before setting currentTime and playbackRate
      video.onloadedmetadata = () => {
        console.log('Video metadata loaded for playVideoDirectly');
        video.currentTime = seekTime;

        // Set playback speed and verify it was set correctly
        const oldRate = video.playbackRate;
        video.playbackRate = playbackSpeed;
        console.log(`Setting playback rate from ${oldRate}x to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);

        // Force the playback rate again after a short delay
        setTimeout(() => {
          video.playbackRate = playbackSpeed;
          console.log(`Re-setting playback rate to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);
        }, 100);

        // Play the video
        video.play().catch(error => {
          console.error('Error playing video:', error);
          showStatusMessage('Error playing video: ' + error.message, 'error');
        });
      };

      // Add error handler for loading
      video.onerror = (e) => {
        console.error('Error loading video:', e);
        showStatusMessage('Error loading video, skipping to next segment', 'warning');

        // Try to skip to the next segment if this one fails
        if (currentSegmentIndex < segments.length - 1) {
          console.log(`Skipping to next segment ${currentSegmentIndex + 1} due to error`);

          // Update state to next segment
          timelineState.setState({
            currentSegmentIndex: currentSegmentIndex + 1,
            currentTime: segments[currentSegmentIndex + 1].start_timestamp,
            isPlaying: true
          });
        } else {
          // No more segments, pause playback
          pausePlayback();
        }
      };
    } catch (error) {
      console.error('Exception while setting up video:', error);
      showStatusMessage('Error setting up video: ' + error.message, 'error');
    }
  };

  // Play a specific segment (updates state and plays video)
  const playSegment = (index, seekTime = 0) => {
    console.log(`TimelinePlayer.playSegment(${index}, ${seekTime})`);

    // Check if segments array is valid and not empty
    if (!segments || segments.length === 0) {
      console.warn('No segments available');
      return;
    }

    if (index < 0 || index >= segments.length) {
      console.warn(`Invalid segment index: ${index}, segments length: ${segments.length}`);
      return;
    }

    const segment = segments[index];
    if (!segment) {
      console.warn(`Segment at index ${index} is undefined`);
      return;
    }

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
      // Check if we're playing and have a valid video element
      if (!isPlaying || !videoRef.current) {
        return;
      }

      // Check if we have a valid segment index and segments array
      if (currentSegmentIndex < 0 || !segments || segments.length === 0 || currentSegmentIndex >= segments.length) {
        return;
      }

      const segment = segments[currentSegmentIndex];
      if (!segment) {
        console.error('Invalid segment at index', currentSegmentIndex);
        return;
      }

      const video = videoRef.current;

      // Calculate current timestamp based on video currentTime
      const currentTime = segment.start_timestamp + video.currentTime;

      // Store the previous currentTime before updating
      const prevCurrentTime = timelineState.currentTime;

      // Update timeline state
      timelineState.setState({ 
        currentTime,
        prevCurrentTime
      });

      // Debug log for video progress
      if (Math.floor(video.currentTime) % 5 === 0 && video.currentTime > 0) { // Log every 5 seconds
        const segmentDuration = segment.end_timestamp - segment.start_timestamp;
        console.log(`Video progress: ${video.currentTime.toFixed(2)}s / ${segmentDuration.toFixed(2)}s, playback rate: ${video.playbackRate}x`);
      }

      // Track if the video has stalled near the end
      // Store the current time and position in a ref to detect if playback has stalled
      if (!lastTimeUpdateRef.current) {
        lastTimeUpdateRef.current = {
          time: Date.now(),
          position: video.currentTime
        };
      } else {
        const now = Date.now();
        const timeDiff = now - lastTimeUpdateRef.current.time;
        const positionDiff = Math.abs(video.currentTime - lastTimeUpdateRef.current.position);

        // Calculate segment duration and remaining time
        const segmentDuration = segment.end_timestamp - segment.start_timestamp;
        const remainingTime = (segmentDuration - video.currentTime) / video.playbackRate;

        // Update the last time update if the position has changed
        if (positionDiff > 0.01) {
          lastTimeUpdateRef.current = {
            time: now,
            position: video.currentTime
          };
        }
            // If the video has stalled for more than 300ms and we're not paused or seeking
            // and either:
            // 1. We're near the end (within 2 seconds) OR
            // 2. We've been playing for at least 2 seconds and there's been no progress for 1 second
        // Then advance to the next segment
        else if (video.currentTime > 0 && !video.paused && !video.seeking &&
            ((remainingTime < 2 && timeDiff > 300) ||
                (video.currentTime > 2 && timeDiff > 1000))) {

          console.log(`Video stalled ${remainingTime < 2 ? 'near end of' : 'during'} segment ${currentSegmentIndex}, currentTime: ${video.currentTime.toFixed(2)}s, duration: ${segmentDuration}s, remaining: ${remainingTime.toFixed(2)}s, stalled for: ${timeDiff}ms`);

          // Check if there's a next segment
          if (currentSegmentIndex < segments.length - 1) {
            console.log(`Auto-playing next segment ${currentSegmentIndex + 1} from tracking (stalled near end)`);

            // Get the next segment
            const nextSegment = segments[currentSegmentIndex + 1];
            if (!nextSegment) {
              console.warn(`Next segment at index ${currentSegmentIndex + 1} is undefined`);
              return;
            }

            // Update state
            timelineState.setState({
              currentSegmentIndex: currentSegmentIndex + 1,
              currentTime: nextSegment.start_timestamp,
              isPlaying: true
            });

            try {
              // Force load and play the next segment's video
              console.log('Loading next segment video from tracking (stalled near end):', nextSegment);

              // Pause the current video first
              video.pause();

              // Clear any existing source and events
              video.removeAttribute('src');
              video.load();

              // Set the new source with a unique timestamp to prevent caching
              const recordingUrl = `/api/recordings/play/${nextSegment.id}?t=${Date.now()}`;
              console.log('Setting video source to:', recordingUrl);
              video.src = recordingUrl;

              // Wait for metadata to load before setting currentTime and playbackRate
              video.onloadedmetadata = () => {
                console.log('Video metadata loaded for next segment');
                video.currentTime = 0;

                // Set playback speed and verify it was set correctly
                const oldRate = video.playbackRate;
                video.playbackRate = playbackSpeed;
                console.log(`Setting playback rate from ${oldRate}x to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);

                // Force the playback rate again after a short delay
                setTimeout(() => {
                  video.playbackRate = playbackSpeed;
                  console.log(`Re-setting playback rate to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);
                }, 100);

                // Play the video
                video.play().catch(error => {
                  console.error('Error playing next video:', error);
                  showStatusMessage('Error playing next video: ' + error.message, 'error');
                });
              };

              // Add error handler for loading
              video.onerror = (e) => {
                console.error('Error loading next video:', e);
                showStatusMessage('Error loading next video, skipping to next segment', 'warning');

                // Try to skip to the next segment if this one fails
                if (currentSegmentIndex < segments.length - 1) {
                  console.log(`Skipping to next segment ${currentSegmentIndex + 1} due to error`);

                  // Update state to next segment
                  timelineState.setState({
                    currentSegmentIndex: currentSegmentIndex + 1,
                    currentTime: segments[currentSegmentIndex + 1].start_timestamp,
                    isPlaying: true
                  });
                } else {
                  // No more segments, pause playback
                  pausePlayback();
                }
              };
            } catch (error) {
              console.error('Exception while setting up next video:', error);
              showStatusMessage('Error setting up next video: ' + error.message, 'error');
            }
          }
        }
      }
    }, 100);
  };

  // Define the onEnded handler directly
  const onEnded = () => {
    console.log('Video onended attribute triggered');
    console.log('Current segment index:', currentSegmentIndex);
    console.log('Total segments:', segments.length);

    // Check if we have a valid segment index and segments array
    if (currentSegmentIndex < 0 || !segments || segments.length === 0) {
      console.log('No valid segments, pausing playback');
      pausePlayback();
      return;
    }

    // Try to play the next segment
    if (currentSegmentIndex < segments.length - 1) {
      const nextIndex = currentSegmentIndex + 1;
      console.log(`Auto-playing next segment ${nextIndex} from onended attribute`);

      // Get the next segment
      const nextSegment = segments[nextIndex];
      if (!nextSegment) {
        console.warn(`Next segment at index ${nextIndex} is undefined`);
        pausePlayback();
        return;
      }

      console.log('Next segment:', nextSegment);

      // Update state - IMPORTANT: This must happen before loading the video
      // to ensure the state is updated before any callbacks are triggered
      timelineState.setState({
        currentSegmentIndex: nextIndex,
        currentTime: nextSegment.start_timestamp,
        isPlaying: true
      });

      // Force load and play the next segment's video
      const video = videoRef.current;
      if (video) {
        try {
          // Force load and play the next segment's video
          console.log('Loading next segment video from onended attribute:', nextSegment);

          // Pause the current video first
          video.pause();

          // Clear any existing source and events
          video.removeAttribute('src');
          video.load();

          // Set the new source with a unique timestamp to prevent caching
          const recordingUrl = `/api/recordings/play/${nextSegment.id}?t=${Date.now()}`;
          console.log('Setting video source to:', recordingUrl);
          video.src = recordingUrl;

          // Wait for metadata to load before setting currentTime and playbackRate
          video.onloadedmetadata = () => {
            console.log('Video metadata loaded for next segment from onended attribute');
            video.currentTime = 0;

            // Set playback speed and verify it was set correctly
            const oldRate = video.playbackRate;
            video.playbackRate = playbackSpeed;
            console.log(`Setting playback rate from ${oldRate}x to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);

            // Force the playback rate again after a short delay
            setTimeout(() => {
              video.playbackRate = playbackSpeed;
              console.log(`Re-setting playback rate to ${playbackSpeed}x, actual rate: ${video.playbackRate}x`);
            }, 100);

            // Play the video
            video.play().catch(error => {
              console.error('Error playing next video:', error);
              showStatusMessage('Error playing next video: ' + error.message, 'error');
            });
          };

          // Add error handler for loading
          video.onerror = (e) => {
            console.error('Error loading next video:', e);
            showStatusMessage('Error loading next video, skipping to next segment', 'warning');

            // Try to skip to the next segment if this one fails
            if (currentSegmentIndex < segments.length - 1) {
              console.log(`Skipping to next segment ${currentSegmentIndex + 1} due to error`);

              // Update state to next segment
              timelineState.setState({
                currentSegmentIndex: currentSegmentIndex + 1,
                currentTime: segments[currentSegmentIndex + 1].start_timestamp,
                isPlaying: true
              });
            } else {
              // No more segments, pause playback
              pausePlayback();
            }
          };
        } catch (error) {
          console.error('Exception while setting up next video:', error);
          showStatusMessage('Error setting up next video: ' + error.message, 'error');
        }
      }
    } else {
      // End of all segments
      console.log('Reached end of all segments from onended attribute');
      pausePlayback();
    }
  };

  return html`
    <div class="timeline-player-container mb-2" id="video-player">
      <div class="relative w-full bg-black rounded-lg overflow-hidden shadow-md" style="aspect-ratio: 16/9;">
        <video
            ref=${videoRef}
            class="w-full h-full"
            controls
            autoplay=${false}
            muted=${false}
            playsInline
            onended=${onEnded}
        ></video>
      </div>
    </div>

    <!-- Playback speed controls -->
    <${SpeedControls} />
  `;
}
