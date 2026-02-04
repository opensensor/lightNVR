/**
 * WebRTCVideoCell Component
 * A self-contained component for displaying a WebRTC video stream
 * with optional two-way audio (backchannel) support
 */

import { h } from 'preact';
import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { DetectionOverlay, takeSnapshotWithDetections } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showSnapshotPreview } from './UI.jsx';
import { PTZControls } from './PTZControls.jsx';
import { getGo2rtcBaseUrl } from '../../utils/settings-utils.js';
import adapter from 'webrtc-adapter';

/**
 * WebRTCVideoCell component
 * @param {Object} props - Component props
 * @param {Object} props.stream - Stream object
 * @param {Function} props.onToggleFullscreen - Fullscreen toggle handler
 * @param {string} props.streamId - Stream ID for stable reference
 * @returns {JSX.Element} WebRTCVideoCell component
 */
export function WebRTCVideoCell({
  stream,
  streamId,
  onToggleFullscreen
}) {
  // Component state
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [connectionQuality, setConnectionQuality] = useState('unknown'); // 'unknown', 'good', 'poor', 'bad'
  const [retryCount, setRetryCount] = useState(0); // Used to trigger WebRTC re-initialization

  // Backchannel (two-way audio) state
  const [isTalking, setIsTalking] = useState(false);
  const [microphoneError, setMicrophoneError] = useState(null);
  const [hasMicrophonePermission, setHasMicrophonePermission] = useState(null);
  const [audioLevel, setAudioLevel] = useState(0);
  const [talkMode, setTalkMode] = useState('ptt'); // 'ptt' (push-to-talk) or 'toggle'

  // Audio playback state (for hearing audio from camera)
  const [audioEnabled, setAudioEnabled] = useState(false);

  // Effect to directly set the muted property on the video element
  // This is necessary because React/Preact doesn't always update the muted attribute correctly
  useEffect(() => {
    if (videoRef.current) {
      videoRef.current.muted = !audioEnabled;
      console.log(`Set video muted=${!audioEnabled} for stream ${stream?.name || 'unknown'}`);

      // Debug: Log audio track info
      if (videoRef.current.srcObject) {
        const audioTracks = videoRef.current.srcObject.getAudioTracks();
        console.log(`Audio tracks for ${stream?.name}: ${audioTracks.length}`, audioTracks.map(t => ({
          id: t.id,
          label: t.label,
          enabled: t.enabled,
          muted: t.muted,
          readyState: t.readyState
        })));
      }
    }
  }, [audioEnabled, stream?.name]);

  // PTZ controls state
  const [showPTZControls, setShowPTZControls] = useState(false);

  // Refs
  const videoRef = useRef(null);
  const cellRef = useRef(null);
  const peerConnectionRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const abortControllerRef = useRef(null);
  const connectionMonitorRef = useRef(null);
  const reconnectAttemptsRef = useRef(0);
  const refreshRequestedRef = useRef(false);  // Track if we've already requested a refresh for this connection attempt
  const localStreamRef = useRef(null);
  const audioSenderRef = useRef(null);
  const audioContextRef = useRef(null);
  const analyserRef = useRef(null);
  const audioLevelIntervalRef = useRef(null);

  // Initialize WebRTC connection when component mounts
  useEffect(() => {
    if (!stream || !stream.name || !videoRef.current) return;

    console.log(`Initializing WebRTC connection for stream ${stream.name}`);
    setIsLoading(true);
    setError(null);

    // Reset the refresh flag for this new connection attempt
    refreshRequestedRef.current = false;

    // Store cleanup functions
    let connectionTimeout = null;
    let videoDataTimeout = null;
    let statsInterval = null;
    let go2rtcBaseUrl = null;

    // Async function to initialize WebRTC
    const initWebRTC = async () => {
      // Get the go2rtc base URL from settings
      try {
        go2rtcBaseUrl = await getGo2rtcBaseUrl();
        console.log(`Using go2rtc base URL: ${go2rtcBaseUrl}`);
      } catch (err) {
        console.warn('Failed to get go2rtc URL from settings, using default port 1984:', err);
        go2rtcBaseUrl = `http://${window.location.hostname}:1984`;
      }

      // Create a new RTCPeerConnection
      const pc = new RTCPeerConnection({
      iceTransportPolicy: 'all',
      bundlePolicy: 'balanced',
      rtcpMuxPolicy: 'require',
      iceCandidatePoolSize: 0,
      iceServers: [
        { urls: 'stun:stun.l.google.com:19302' },
      ]
    });

    peerConnectionRef.current = pc;

    // Set up event handlers
    pc.ontrack = (event) => {
      console.log(`Track received for stream ${stream.name}: ${event.track.kind}`, event);
      console.log(`Track muted: ${event.track.muted}, enabled: ${event.track.enabled}, readyState: ${event.track.readyState}`);

      const videoElement = videoRef.current;
      if (!videoElement) {
        console.error(`Video element not found for stream ${stream.name}`);
        return;
      }

      // Only set srcObject once to avoid interrupting pending play() calls
      // When multiple tracks arrive (video + audio), each triggers ontrack
      // Setting srcObject again interrupts the play() call from the first track
      if (event.streams && event.streams[0]) {
        if (!videoElement.srcObject || videoElement.srcObject !== event.streams[0]) {
          videoElement.srcObject = event.streams[0];
          console.log(`Set srcObject from ontrack event for stream ${stream.name}, tracks:`,
            event.streams[0].getTracks().map(t => `${t.kind}:${t.readyState}:muted=${t.muted}`));
        } else {
          console.log(`srcObject already set for stream ${stream.name}, skipping to avoid interrupting play()`);
        }
      }

      if (event.track.kind === 'video') {
        console.log(`Video track received for stream ${stream.name}`);

        // Track retry attempts for play()
        let playRetryCount = 0;
        const maxPlayRetries = 3;
        let playRetryTimeout = null;

        // Function to attempt play with retry logic
        const attemptPlay = () => {
          if (!videoElement || videoElement.paused === false) {
            // Already playing or element gone
            return;
          }

          console.log(`Attempting to play video for stream ${stream.name} (attempt ${playRetryCount + 1})`);
          videoElement.play()
            .then(() => {
              console.log(`Video play() succeeded for stream ${stream.name}`);
              playRetryCount = 0; // Reset on success
            })
            .catch(err => {
              // AbortError is expected when srcObject changes or another play() is called
              // Don't treat it as a fatal error, just log and potentially retry
              if (err.name === 'AbortError') {
                console.log(`Video play() was interrupted for stream ${stream.name}, will retry if needed`);
                playRetryCount++;
                if (playRetryCount < maxPlayRetries) {
                  // Retry after a short delay
                  playRetryTimeout = setTimeout(attemptPlay, 500);
                }
              } else if (err.name === 'NotAllowedError') {
                console.warn(`Autoplay blocked for stream ${stream.name}, user interaction required`);
                setError('Click to play video (autoplay blocked)');
              } else {
                console.error(`Video play() failed for stream ${stream.name}:`, err);
              }
            });
        };

        // Set a timeout to detect if no video data is received
        // This handles the case where go2rtc hasn't connected to the source camera yet
        if (videoDataTimeout) {
          clearTimeout(videoDataTimeout);
        }
        videoDataTimeout = setTimeout(() => {
          // Check if video is actually playing by checking if we have video dimensions
          if (videoElement && (!videoElement.videoWidth || videoElement.videoWidth === 0)) {
            console.warn(`No video data received for stream ${stream.name} within 30 seconds, may need retry`);
            // Check if video is not playing (paused or no data)
            if (videoElement.paused || videoElement.readyState < 2) {
              console.error(`Stream ${stream.name} connected but no video data - source may not be ready`);
              setError('Stream connected but no video data. Click Retry to reconnect.');
              setIsLoading(false);
            }
          }
        }, 30000); // 30 second timeout for video data

        // Add event handlers
        videoElement.onloadedmetadata = () => {
          console.log(`Video metadata loaded for stream ${stream.name}`);
          // Clear the video data timeout since we got metadata
          if (videoDataTimeout) {
            clearTimeout(videoDataTimeout);
            videoDataTimeout = null;
          }
        };

        videoElement.onloadeddata = () => {
          console.log(`Video data loaded for stream ${stream.name}`);
        };

        videoElement.onplaying = () => {
          console.log(`Video playing for stream ${stream.name}`);
          setIsLoading(false);
          setIsPlaying(true);
          // Clear any error (e.g., if video starts playing after timeout error was shown)
          setError(null);
          // Clear timeouts since video is playing
          if (videoDataTimeout) {
            clearTimeout(videoDataTimeout);
            videoDataTimeout = null;
          }
          if (playRetryTimeout) {
            clearTimeout(playRetryTimeout);
            playRetryTimeout = null;
          }
        };

        videoElement.onwaiting = () => {
          console.log(`Video waiting for data for stream ${stream.name}`);
          // If video is waiting and paused, try to play again after a short delay
          // This handles cases where the video gets stuck in waiting state
          if (videoElement.paused && playRetryCount < maxPlayRetries) {
            console.log(`Video paused while waiting for stream ${stream.name}, scheduling retry`);
            if (playRetryTimeout) {
              clearTimeout(playRetryTimeout);
            }
            playRetryTimeout = setTimeout(attemptPlay, 1000);
          }
        };

        videoElement.onstalled = () => {
          console.warn(`Video stalled for stream ${stream.name}`);
          // Try to recover from stalled state
          if (videoElement.paused && playRetryCount < maxPlayRetries) {
            console.log(`Attempting to recover from stalled state for stream ${stream.name}`);
            if (playRetryTimeout) {
              clearTimeout(playRetryTimeout);
            }
            playRetryTimeout = setTimeout(attemptPlay, 1000);
          }
        };

        videoElement.onerror = (event) => {
          console.error(`Error loading video for stream ${stream.name}:`, event);
          if (videoElement.error) {
            console.error(`Video error code: ${videoElement.error.code}, message: ${videoElement.error.message}`);
          }
          setError('Failed to load video');
          setIsLoading(false);
          // Clear retry timeout on error
          if (playRetryTimeout) {
            clearTimeout(playRetryTimeout);
            playRetryTimeout = null;
          }
        };

        // Start initial playback attempt
        attemptPlay();
      } else if (event.track.kind === 'audio') {
        console.log(`Audio track received for stream ${stream.name}`);
      }
    };

    pc.oniceconnectionstatechange = () => {
      console.log(`ICE connection state for stream ${stream.name}: ${pc.iceConnectionState}`);

      if (pc.iceConnectionState === 'failed') {
        console.error(`WebRTC ICE connection failed for stream ${stream.name}`);

        // Auto-refresh and retry if we haven't already for this connection attempt
        if (!refreshRequestedRef.current && reconnectAttemptsRef.current < 3) {
          refreshRequestedRef.current = true;
          reconnectAttemptsRef.current++;
          console.log(`Auto-refreshing go2rtc registration for stream ${stream.name} (attempt ${reconnectAttemptsRef.current}/3)`);

          // Trigger a refresh and retry automatically
          (async () => {
            try {
              const response = await fetch(`/api/streams/${encodeURIComponent(stream.name)}/refresh`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
              });
              if (response.ok) {
                console.log(`Successfully refreshed go2rtc registration for ${stream.name}, retrying connection...`);
                // Small delay to allow go2rtc to process the refresh
                await new Promise(resolve => setTimeout(resolve, 1000));
                // Trigger a retry by incrementing retryCount
                setRetryCount(prev => prev + 1);
                return;
              } else {
                console.warn(`Failed to refresh stream ${stream.name}: ${response.status}`);
              }
            } catch (err) {
              console.error(`Error refreshing stream ${stream.name}:`, err);
            }
            // If refresh failed, show the error
            setError('WebRTC ICE connection failed');
            setIsLoading(false);
          })();
        } else {
          setError('WebRTC ICE connection failed');
          setIsLoading(false);
        }
      } else if (pc.iceConnectionState === 'disconnected') {
        // Connection is temporarily disconnected, log but don't show error yet
        console.warn(`WebRTC ICE connection disconnected for stream ${stream.name}, attempting to recover...`);

        // Set a timeout to check if the connection recovers on its own
        setTimeout(() => {
          if (peerConnectionRef.current &&
              (peerConnectionRef.current.iceConnectionState === 'disconnected' ||
               peerConnectionRef.current.iceConnectionState === 'failed')) {
            console.error(`WebRTC ICE connection could not recover for stream ${stream.name}`);
            setError('WebRTC connection lost. Please retry.');
            setIsLoading(false);
          } else if (peerConnectionRef.current) {
            console.log(`WebRTC ICE connection recovered for stream ${stream.name}, current state: ${peerConnectionRef.current.iceConnectionState}`);
          }
        }, 5000); // Wait 5 seconds to see if connection recovers
      } else if (pc.iceConnectionState === 'connected' || pc.iceConnectionState === 'completed') {
        // Connection is established or completed, clear any previous error
        // Reset refresh flag for next potential failure
        refreshRequestedRef.current = false;
        if (error) {
          console.log(`WebRTC connection restored for stream ${stream.name}`);
          setError(null);
        }
      }
    };

    // Handle ICE gathering state changes
    pc.onicegatheringstatechange = () => {
      console.log(`ICE gathering state for stream ${stream.name}: ${pc.iceGatheringState}`);
    };

    // Handle ICE candidates - critical for NAT traversal
    pc.onicecandidate = (event) => {
      if (event.candidate) {
        console.log(`ICE candidate for stream ${stream.name}:`, event.candidate.candidate);

        // Send the ICE candidate to the server
        // Note: go2rtc typically handles ICE candidates in the SDP exchange,
        // but we log them here for debugging purposes
        // If trickle ICE is needed, uncomment the code below:
        /*
        fetch(`/api/webrtc/ice?src=${encodeURIComponent(stream.name)}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...(auth ? { 'Authorization': 'Basic ' + auth } : {})
          },
          body: JSON.stringify(event.candidate)
        }).catch(err => console.warn('Failed to send ICE candidate:', err));
        */
      } else {
        console.log(`ICE gathering complete for stream ${stream.name}`);
      }
    };

    // Add transceivers for video and audio
    // Add video transceiver
    pc.addTransceiver('video', {direction: 'recvonly'});

    // Add audio transceiver for backchannel support if enabled
    // Use sendrecv to allow both receiving audio from camera and sending audio to camera
    if (stream.backchannel_enabled) {
      console.log(`Adding audio transceiver with sendrecv for backchannel on stream ${stream.name}`);
      const audioTransceiver = pc.addTransceiver('audio', {direction: 'sendrecv'});
      // Store reference to the audio sender for later use
      audioSenderRef.current = audioTransceiver.sender;
    } else {
      // Just receive audio from the camera (if available)
      pc.addTransceiver('audio', {direction: 'recvonly'});
    }

    // Note: srcObject will be set in the ontrack event handler when we receive the remote stream

    // Connect directly to go2rtc for WebRTC
    // go2rtcBaseUrl is set at the start of initWebRTC from settings

    // Set a timeout for the entire connection process
    connectionTimeout = setTimeout(() => {
      if (peerConnectionRef.current &&
          peerConnectionRef.current.iceConnectionState !== 'connected' &&
          peerConnectionRef.current.iceConnectionState !== 'completed') {
        console.error(`WebRTC connection timeout for stream ${stream.name}, ICE state: ${peerConnectionRef.current.iceConnectionState}`);
        setError('Connection timeout. Check network/firewall settings.');
        setIsLoading(false);
      }
    }, 30000); // 30 second timeout

    // Create and send offer
    pc.createOffer()
      .then(offer => {
        console.log(`Created offer for stream ${stream.name}`);
        // For debugging, log a short preview of the SDP
        if (offer && offer.sdp) {
          const preview = offer.sdp.substring(0, 120).replace(/\n/g, '\\n');
          console.log(`SDP offer preview for ${stream.name}: ${preview}...`);
        }
        return pc.setLocalDescription(offer);
      })
      .then(() => {
        console.log(`Set local description for stream ${stream.name}, waiting for ICE gathering...`);

        // Create a new AbortController for this request
        abortControllerRef.current = new AbortController();

        console.log(`Sending offer directly to go2rtc for stream ${stream.name}`);

        // Send the offer directly to go2rtc
        // go2rtc expects Content-Type: application/sdp with raw SDP
        return fetch(`${go2rtcBaseUrl}/api/webrtc?src=${encodeURIComponent(stream.name)}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/sdp',
          },
          body: pc.localDescription.sdp,
        });
      })
      .then(async (response) => {
        const bodyText = await response.text().catch(() => '');
        if (!response.ok) {
          console.error(`go2rtc /api/webrtc error for stream ${stream.name}: status=${response.status}, body="${bodyText}"`);
          throw new Error(`Failed to send offer: ${response.status} ${response.statusText}`);
        }
        return bodyText;
      })
      .then(sdpAnswer => {
        console.log(`Received SDP answer from go2rtc for stream ${stream.name}`);
        // Debug: Check if audio is in the SDP answer
        const hasAudio = sdpAnswer.includes('m=audio');
        console.log(`SDP answer contains audio: ${hasAudio} for stream ${stream.name}`);
        if (hasAudio) {
          // Find the audio section and log a snippet
          const audioIndex = sdpAnswer.indexOf('m=audio');
          console.log(`SDP audio section preview: ${sdpAnswer.substring(audioIndex, audioIndex + 200)}...`);
        }
        // go2rtc returns raw SDP, wrap it in RTCSessionDescription
        const answer = {
          type: 'answer',
          sdp: sdpAnswer
        };
        return pc.setRemoteDescription(new RTCSessionDescription(answer));
      })
      .then(() => {
        console.log(`Set remote description for stream ${stream.name}, ICE state: ${pc.iceConnectionState}`);
      })
      .catch(error => {
        console.error(`Error setting up WebRTC for stream ${stream.name}:`, error);
        setError(error.message || 'Failed to establish WebRTC connection');
        clearTimeout(connectionTimeout);
      });

    // Set up connection quality monitoring
    const startConnectionMonitoring = () => {
      // Clear any existing monitor
      if (connectionMonitorRef.current) {
        clearInterval(connectionMonitorRef.current);
      }
      
      // Start a new monitor
      connectionMonitorRef.current = setInterval(() => {
        if (!peerConnectionRef.current) return;
        
        // Get connection stats
        peerConnectionRef.current.getStats().then(stats => {
          let packetsLost = 0;
          let packetsReceived = 0;
          let currentRtt = 0;
          let jitter = 0;
          
          stats.forEach(report => {
            if (report.type === 'inbound-rtp' && report.kind === 'video') {
              packetsLost = report.packetsLost || 0;
              packetsReceived = report.packetsReceived || 0;
              jitter = report.jitter || 0;
            }
            
            if (report.type === 'candidate-pair' && report.state === 'succeeded') {
              currentRtt = report.currentRoundTripTime || 0;
            }
          });
          
          // Calculate packet loss percentage
          const totalPackets = packetsReceived + packetsLost;
          const lossPercentage = totalPackets > 0 ? (packetsLost / totalPackets) * 100 : 0;
          
          // Determine connection quality
          let quality = 'unknown';
          
          if (packetsReceived > 0) {
            if (lossPercentage < 2 && currentRtt < 0.1 && jitter < 0.03) {
              quality = 'good';
            } else if (lossPercentage < 5 && currentRtt < 0.3 && jitter < 0.1) {
              quality = 'fair';
            } else if (lossPercentage < 15 && currentRtt < 1) {
              quality = 'poor';
            } else {
              quality = 'bad';
            }
          }
          
          // Update connection quality state if changed
          if (quality !== connectionQuality) {
            console.log(`WebRTC connection quality for stream ${stream.name} changed to ${quality}`);
            console.log(`Stats: loss=${lossPercentage.toFixed(2)}%, rtt=${(currentRtt * 1000).toFixed(0)}ms, jitter=${(jitter * 1000).toFixed(0)}ms`);
            setConnectionQuality(quality);
          }
        }).catch(err => {
          console.warn(`Error getting WebRTC stats for stream ${stream.name}:`, err);
        });
      }, 10000); // Check every 10 seconds
    };
    
    // Start monitoring once we have a connection
    if (peerConnectionRef.current && peerConnectionRef.current.iceConnectionState === 'connected') {
      startConnectionMonitoring();
    }
    
    // Listen for connection state changes to start/stop monitoring
    const originalOnIceConnectionStateChange = pc.oniceconnectionstatechange;
    pc.oniceconnectionstatechange = () => {
      // Call the original handler
      if (originalOnIceConnectionStateChange) {
        originalOnIceConnectionStateChange();
      }
      
      // Start monitoring when connected
      if (pc.iceConnectionState === 'connected' || pc.iceConnectionState === 'completed') {
        startConnectionMonitoring();
        // Reset reconnect attempts counter when we get a good connection
        reconnectAttemptsRef.current = 0;
      }
      
      // Stop monitoring when disconnected or failed
      if (pc.iceConnectionState === 'disconnected' || pc.iceConnectionState === 'failed' || pc.iceConnectionState === 'closed') {
        if (connectionMonitorRef.current) {
          clearInterval(connectionMonitorRef.current);
          connectionMonitorRef.current = null;
        }
      }
    };
    }; // End of initWebRTC async function

    // Call the async init function
    initWebRTC();

    // Cleanup function
    return () => {
      console.log(`Cleaning up WebRTC connection for stream ${stream.name}`);

      // Clear timeouts
      if (connectionTimeout) {
        clearTimeout(connectionTimeout);
        connectionTimeout = null;
      }
      if (videoDataTimeout) {
        clearTimeout(videoDataTimeout);
        videoDataTimeout = null;
      }

      // Stop connection monitoring
      if (connectionMonitorRef.current) {
        clearInterval(connectionMonitorRef.current);
        connectionMonitorRef.current = null;
      }

      // Abort any pending fetch requests
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
        abortControllerRef.current = null;
      }

      // Clean up local microphone stream
      if (localStreamRef.current) {
        localStreamRef.current.getTracks().forEach(track => track.stop());
        localStreamRef.current = null;
      }

      // Clean up audio level monitoring
      if (audioLevelIntervalRef.current) {
        clearInterval(audioLevelIntervalRef.current);
        audioLevelIntervalRef.current = null;
      }
      if (audioContextRef.current) {
        audioContextRef.current.close().catch(() => {});
        audioContextRef.current = null;
      }
      analyserRef.current = null;

      // Clean up video element
      if (videoRef.current && videoRef.current.srcObject) {
        const tracks = videoRef.current.srcObject.getTracks();
        tracks.forEach(track => track.stop());
        videoRef.current.srcObject = null;
      }

      // Close peer connection
      if (peerConnectionRef.current) {
        peerConnectionRef.current.close();
        peerConnectionRef.current = null;
      }

      // Reset audio sender ref
      audioSenderRef.current = null;
    };
  }, [stream, retryCount]);

  /**
   * Refresh the stream's go2rtc registration
   * This is useful when WebRTC connections fail due to stale go2rtc state
   * @returns {Promise<boolean>} true if refresh was successful
   */
  const refreshStreamRegistration = async () => {
    if (!stream?.name) {
      console.warn('Cannot refresh stream: no stream name');
      return false;
    }

    try {
      console.log(`Refreshing go2rtc registration for stream ${stream.name}`);
      const response = await fetch(`/api/streams/${encodeURIComponent(stream.name)}/refresh`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        const data = await response.json();
        console.log(`Successfully refreshed go2rtc registration for stream ${stream.name}:`, data);
        return true;
      } else {
        const errorText = await response.text();
        console.warn(`Failed to refresh stream ${stream.name}: ${response.status} - ${errorText}`);
        return false;
      }
    } catch (err) {
      console.error(`Error refreshing stream ${stream.name}:`, err);
      return false;
    }
  };

  // Handle retry button click
  const handleRetry = async () => {
    console.log(`Retry requested for stream ${stream?.name}`);

    // Clean up existing connection
    if (peerConnectionRef.current) {
      peerConnectionRef.current.close();
      peerConnectionRef.current = null;
    }

    if (videoRef.current && videoRef.current.srcObject) {
      const tracks = videoRef.current.srcObject.getTracks();
      tracks.forEach(track => track.stop());
      videoRef.current.srcObject = null;
    }

    // Reset state
    setError(null);
    setIsLoading(true);
    setIsPlaying(false);

    // Reset auto-retry counters on manual retry (user gets fresh attempts)
    reconnectAttemptsRef.current = 0;
    refreshRequestedRef.current = false;

    // Refresh the stream's go2rtc registration before retrying
    // This helps recover from stale go2rtc state that causes WebRTC failures
    await refreshStreamRegistration();

    // Small delay to allow go2rtc to re-register the stream
    await new Promise(resolve => setTimeout(resolve, 500));

    // Increment retry count to trigger useEffect re-run
    setRetryCount(prev => prev + 1);
  };

  // Start audio level monitoring
  const startAudioLevelMonitoring = useCallback((localStream) => {
    try {
      // Create audio context and analyser for level monitoring
      const audioContext = new (window.AudioContext || window.webkitAudioContext)();
      const analyser = audioContext.createAnalyser();
      analyser.fftSize = 256;

      const source = audioContext.createMediaStreamSource(localStream);
      source.connect(analyser);

      audioContextRef.current = audioContext;
      analyserRef.current = analyser;

      // Start monitoring audio levels
      const dataArray = new Uint8Array(analyser.frequencyBinCount);
      audioLevelIntervalRef.current = setInterval(() => {
        if (analyserRef.current) {
          analyserRef.current.getByteFrequencyData(dataArray);
          // Calculate average level
          const average = dataArray.reduce((a, b) => a + b, 0) / dataArray.length;
          // Normalize to 0-100
          setAudioLevel(Math.min(100, Math.round((average / 128) * 100)));
        }
      }, 50);
    } catch (err) {
      console.warn('Failed to start audio level monitoring:', err);
    }
  }, []);

  // Stop audio level monitoring
  const stopAudioLevelMonitoring = useCallback(() => {
    if (audioLevelIntervalRef.current) {
      clearInterval(audioLevelIntervalRef.current);
      audioLevelIntervalRef.current = null;
    }
    if (audioContextRef.current) {
      audioContextRef.current.close().catch(() => {});
      audioContextRef.current = null;
    }
    analyserRef.current = null;
    setAudioLevel(0);
  }, []);

  // Start push-to-talk (acquire microphone and send audio)
  const startTalking = useCallback(async () => {
    if (!stream.backchannel_enabled || !audioSenderRef.current) {
      console.warn('Backchannel not enabled or audio sender not available');
      return;
    }

    try {
      setMicrophoneError(null);

      // Request microphone access
      console.log(`Requesting microphone access for backchannel on stream ${stream.name}`);
      const localStream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
          autoGainControl: true
        }
      });

      localStreamRef.current = localStream;
      setHasMicrophonePermission(true);

      // Start audio level monitoring
      startAudioLevelMonitoring(localStream);

      // Get the audio track and replace the sender's track
      const audioTrack = localStream.getAudioTracks()[0];
      if (audioTrack && audioSenderRef.current) {
        await audioSenderRef.current.replaceTrack(audioTrack);
        console.log(`Started sending audio for backchannel on stream ${stream.name}`);
        setIsTalking(true);
      }
    } catch (err) {
      console.error(`Failed to start backchannel audio for stream ${stream.name}:`, err);
      setHasMicrophonePermission(false);

      if (err.name === 'NotAllowedError') {
        setMicrophoneError('Microphone access denied. Please allow microphone access in your browser settings.');
      } else if (err.name === 'NotFoundError') {
        setMicrophoneError('No microphone found. Please connect a microphone.');
      } else {
        setMicrophoneError(`Microphone error: ${err.message}`);
      }
    }
  }, [stream, startAudioLevelMonitoring]);

  // Stop push-to-talk (stop sending audio)
  const stopTalking = useCallback(async () => {
    if (!stream.backchannel_enabled) return;

    try {
      // Stop audio level monitoring
      stopAudioLevelMonitoring();

      // Stop the local audio track
      if (localStreamRef.current) {
        localStreamRef.current.getTracks().forEach(track => track.stop());
        localStreamRef.current = null;
      }

      // Replace the sender's track with null to stop sending
      if (audioSenderRef.current) {
        await audioSenderRef.current.replaceTrack(null);
        console.log(`Stopped sending audio for backchannel on stream ${stream.name}`);
      }

      setIsTalking(false);
    } catch (err) {
      console.error(`Failed to stop backchannel audio for stream ${stream.name}:`, err);
    }
  }, [stream, stopAudioLevelMonitoring]);

  // Toggle talk mode handler
  const handleTalkToggle = useCallback(() => {
    if (talkMode === 'toggle') {
      if (isTalking) {
        stopTalking();
      } else {
        startTalking();
      }
    }
  }, [talkMode, isTalking, startTalking, stopTalking]);

  return (
    <div
      className="video-cell"
      data-stream-name={stream.name}
      data-stream-id={streamId}
      ref={cellRef}
      style={{
        position: 'relative',
        pointerEvents: 'auto',
        zIndex: 1
      }}
    >
      {/* Video element */}
      <video
        id={`video-${streamId.replace(/\s+/g, '-')}`}
        className="video-element"
        ref={videoRef}
        autoPlay
        muted={!audioEnabled}
        disablePictureInPicture
        playsInline
        style={{ width: '100%', height: '100%', objectFit: 'contain' }}
      />

      {/* Detection overlay component */}
      {stream.detection_based_recording && stream.detection_model && (
        <DetectionOverlay
          ref={detectionOverlayRef}
          streamName={stream.name}
          videoRef={videoRef}
          enabled={isPlaying}
          detectionModel={stream.detection_model}
        />
      )}

      {/* Stream name overlay with connection quality indicator */}
      <div
        className="stream-name-overlay"
        style={{
          position: 'absolute',
          top: '10px',
          left: '10px',
          padding: '5px 10px',
          backgroundColor: 'rgba(0, 0, 0, 0.5)',
          color: 'white',
          borderRadius: '4px',
          fontSize: '14px',
          zIndex: 3,
          display: 'flex',
          alignItems: 'center',
          gap: '8px'
        }}
      >
        {stream.name}
        
        {/* Connection quality indicator - only show when we have quality data and stream is playing */}
        {isPlaying && connectionQuality !== 'unknown' && (
          <div 
            className={`connection-quality-indicator quality-${connectionQuality}`}
            title={`Connection Quality: ${connectionQuality.charAt(0).toUpperCase() + connectionQuality.slice(1)}`}
            style={{
              width: '10px',
              height: '10px',
              borderRadius: '50%',
              backgroundColor: 
                connectionQuality === 'good' ? '#10B981' :  // Green
                connectionQuality === 'fair' ? '#FBBF24' :  // Yellow
                connectionQuality === 'poor' ? '#F97316' :  // Orange
                connectionQuality === 'bad' ? '#EF4444' :   // Red
                '#6B7280',                                  // Gray (unknown)
              boxShadow: '0 0 4px rgba(0, 0, 0, 0.3)'
            }}
          />
        )}
      </div>

      {/* Stream controls */}
      <div
        className="stream-controls"
        style={{
          position: 'absolute',
          bottom: '10px',
          right: '10px',
          display: 'flex',
          gap: '10px',
          zIndex: 5,
          backgroundColor: 'rgba(0, 0, 0, 0.5)',
          padding: '5px',
          borderRadius: '4px'
        }}
      >
        <div
          style={{
            backgroundColor: 'transparent',
            padding: '5px',
            borderRadius: '4px'
          }}
          onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)'}
          onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
        >
          <SnapshotButton
            streamId={streamId}
            streamName={stream.name}
            onSnapshot={() => {
              if (videoRef.current) {
                let canvasRef = null;

                // Try to get canvas ref from detection overlay if available
                if (detectionOverlayRef.current && typeof detectionOverlayRef.current.getCanvasRef === 'function') {
                  canvasRef = detectionOverlayRef.current.getCanvasRef();
                }

                // Take snapshot with or without detections
                if (canvasRef) {
                  const snapshot = takeSnapshotWithDetections(videoRef, canvasRef, stream.name);
                  if (snapshot) {
                    showSnapshotPreview(snapshot.canvas.toDataURL('image/jpeg', 0.95), `Snapshot: ${stream.name}`);
                  }
                } else {
                  // Take a simple snapshot without detections
                  const videoElement = videoRef.current;
                  const canvas = document.createElement('canvas');
                  canvas.width = videoElement.videoWidth;
                  canvas.height = videoElement.videoHeight;

                  if (canvas.width > 0 && canvas.height > 0) {
                    const ctx = canvas.getContext('2d');
                    ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

                    showSnapshotPreview(canvas.toDataURL('image/jpeg', 0.95), `Snapshot: ${stream.name}`);
                  }
                }
              }
            }}
          />
        </div>
        {/* Audio playback toggle button (for hearing camera audio) */}
        {isPlaying && (
          <button
            className={`audio-toggle-btn ${audioEnabled ? 'active' : ''}`}
            title={audioEnabled ? 'Mute camera audio' : 'Unmute camera audio'}
            onClick={() => setAudioEnabled(!audioEnabled)}
            style={{
              backgroundColor: audioEnabled ? 'rgba(34, 197, 94, 0.8)' : 'transparent',
              border: 'none',
              padding: '5px',
              borderRadius: '4px',
              color: 'white',
              cursor: 'pointer',
              transition: 'background-color 0.2s ease'
            }}
            onMouseOver={(e) => !audioEnabled && (e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)')}
            onMouseOut={(e) => !audioEnabled && (e.currentTarget.style.backgroundColor = 'transparent')}
          >
            {/* Speaker icon - different icon based on muted state */}
            {audioEnabled ? (
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
                <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
                <path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
              </svg>
            ) : (
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
                <line x1="23" y1="9" x2="17" y2="15"></line>
                <line x1="17" y1="9" x2="23" y2="15"></line>
              </svg>
            )}
          </button>
        )}
        {/* Two-way audio controls for backchannel */}
        {stream.backchannel_enabled && isPlaying && (
          <div style={{ display: 'flex', alignItems: 'center', gap: '4px', position: 'relative' }}>
            {/* Mode toggle button */}
            <button
              className="talk-mode-btn"
              title={talkMode === 'ptt' ? 'Switch to Toggle Mode' : 'Switch to Push-to-Talk'}
              onClick={() => setTalkMode(talkMode === 'ptt' ? 'toggle' : 'ptt')}
              style={{
                backgroundColor: 'transparent',
                border: 'none',
                padding: '3px',
                borderRadius: '4px',
                color: 'white',
                cursor: 'pointer',
                fontSize: '10px',
                opacity: 0.7
              }}
            >
              {talkMode === 'ptt' ? 'PTT' : 'TOG'}
            </button>
            {/* Main microphone button */}
            <button
              className={`ptt-btn ${isTalking ? 'talking' : ''}`}
              title={talkMode === 'ptt'
                ? (isTalking ? 'Release to stop talking' : 'Hold to talk')
                : (isTalking ? 'Click to stop talking' : 'Click to start talking')}
              onMouseDown={talkMode === 'ptt' ? startTalking : undefined}
              onMouseUp={talkMode === 'ptt' ? stopTalking : undefined}
              onMouseLeave={talkMode === 'ptt' ? stopTalking : undefined}
              onTouchStart={talkMode === 'ptt' ? (e) => { e.preventDefault(); startTalking(); } : undefined}
              onTouchEnd={talkMode === 'ptt' ? (e) => { e.preventDefault(); stopTalking(); } : undefined}
              onClick={talkMode === 'toggle' ? handleTalkToggle : undefined}
              style={{
                backgroundColor: isTalking ? 'rgba(239, 68, 68, 0.8)' : 'transparent',
                border: 'none',
                padding: '5px',
                borderRadius: '4px',
                color: 'white',
                cursor: 'pointer',
                transition: 'background-color 0.2s ease',
                position: 'relative'
              }}
              onMouseOver={(e) => !isTalking && (e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)')}
              onMouseOut={(e) => !isTalking && (e.currentTarget.style.backgroundColor = 'transparent')}
            >
              {/* Microphone icon */}
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill={isTalking ? 'white' : 'none'} stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"></path>
                <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
                <line x1="12" x2="12" y1="19" y2="22"></line>
              </svg>
              {/* Audio level indicator */}
              {isTalking && audioLevel > 0 && (
                <div
                  style={{
                    position: 'absolute',
                    bottom: '-4px',
                    left: '50%',
                    transform: 'translateX(-50%)',
                    width: '20px',
                    height: '3px',
                    backgroundColor: 'rgba(0, 0, 0, 0.3)',
                    borderRadius: '2px',
                    overflow: 'hidden'
                  }}
                >
                  <div
                    style={{
                      width: `${audioLevel}%`,
                      height: '100%',
                      backgroundColor: audioLevel > 70 ? '#22c55e' : audioLevel > 30 ? '#eab308' : '#ef4444',
                      transition: 'width 0.05s ease-out'
                    }}
                  />
                </div>
              )}
            </button>
          </div>
        )}
        {/* PTZ control toggle button */}
        {stream.ptz_enabled && isPlaying && (
          <button
            className={`ptz-toggle-btn ${showPTZControls ? 'active' : ''}`}
            title={showPTZControls ? 'Hide PTZ Controls' : 'Show PTZ Controls'}
            onClick={() => setShowPTZControls(!showPTZControls)}
            style={{
              backgroundColor: showPTZControls ? 'rgba(59, 130, 246, 0.8)' : 'transparent',
              border: 'none',
              padding: '5px',
              borderRadius: '4px',
              color: 'white',
              cursor: 'pointer',
              transition: 'background-color 0.2s ease'
            }}
            onMouseOver={(e) => !showPTZControls && (e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)')}
            onMouseOut={(e) => !showPTZControls && (e.currentTarget.style.backgroundColor = 'transparent')}
          >
            {/* PTZ/Joystick icon */}
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="3"/>
              <path d="M12 2v4M12 18v4M2 12h4M18 12h4"/>
              <path d="M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
            </svg>
          </button>
        )}
        <button
          className="fullscreen-btn"
          title="Toggle Fullscreen"
          data-id={streamId}
          data-name={stream.name}
          onClick={(e) => onToggleFullscreen(stream.name, e, cellRef.current)}
          style={{
            backgroundColor: 'transparent',
            border: 'none',
            padding: '5px',
            borderRadius: '4px',
            color: 'white',
            cursor: 'pointer'
          }}
          onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)'}
          onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
        </button>
      </div>

      {/* PTZ Controls overlay */}
      <PTZControls
        stream={stream}
        isVisible={showPTZControls}
        onClose={() => setShowPTZControls(false)}
      />

      {/* Microphone error indicator */}
      {microphoneError && (
        <div
          style={{
            position: 'absolute',
            bottom: '60px',
            right: '10px',
            backgroundColor: 'rgba(239, 68, 68, 0.9)',
            color: 'white',
            padding: '8px 12px',
            borderRadius: '4px',
            fontSize: '12px',
            maxWidth: '200px',
            zIndex: 6
          }}
        >
          {microphoneError}
        </div>
      )}

      {/* Loading indicator */}
      {isLoading && (
        <div style={{ position: 'absolute', top: 0, left: 0, right: 0, bottom: 0, zIndex: 5, pointerEvents: 'none' }}>
          <LoadingIndicator message="Connecting..." />
        </div>
      )}

      {/* Error indicator */}
      {error && (
        <div
          className="error-indicator"
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            width: '100%',
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'center',
            alignItems: 'center',
            backgroundColor: 'rgba(0, 0, 0, 0.7)',
            color: 'white',
            zIndex: 10,
            textAlign: 'center',
            pointerEvents: 'auto'
          }}
        >
          <div
            className="error-content"
            style={{
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'center',
              alignItems: 'center',
              width: '80%',
              maxWidth: '300px',
              padding: '20px',
              borderRadius: '8px',
              backgroundColor: 'rgba(0, 0, 0, 0.5)'
            }}
          >
            <div
              className="error-icon"
              style={{
                fontSize: '28px',
                marginBottom: '15px',
                fontWeight: 'bold',
                width: '40px',
                height: '40px',
                lineHeight: '40px',
                borderRadius: '50%',
                backgroundColor: 'rgba(220, 38, 38, 0.8)',
                textAlign: 'center'
              }}
            >
              !
            </div>
            <p style={{
              marginBottom: '20px',
              textAlign: 'center',
              width: '100%',
              fontSize: '14px',
              lineHeight: '1.4'
            }}>
              {error}
            </p>
            <button
              className="retry-button"
              onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();
                console.log(`Retry button clicked for stream ${stream?.name}`);
                handleRetry();
              }}
              style={{
                padding: '8px 20px',
                backgroundColor: '#2563eb',
                color: 'white',
                borderRadius: '4px',
                border: 'none',
                cursor: 'pointer',
                fontWeight: 'bold',
                fontSize: '14px',
                boxShadow: '0 2px 4px rgba(0, 0, 0, 0.2)',
                transition: 'background-color 0.2s ease',
                pointerEvents: 'auto'
              }}
              onMouseOver={(e) => e.currentTarget.style.backgroundColor = '#1d4ed8'}
              onMouseOut={(e) => e.currentTarget.style.backgroundColor = '#2563eb'}
            >
              Retry
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
