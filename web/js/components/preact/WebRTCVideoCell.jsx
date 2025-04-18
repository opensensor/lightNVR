/**
 * WebRTCVideoCell Component
 * A self-contained component for displaying a WebRTC video stream
 */

import { h } from 'preact';
import { useState, useEffect, useRef, useCallback, useMemo } from 'preact/hooks';
import { DetectionOverlay, takeSnapshotWithDetections } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showSnapshotPreview } from './UI.jsx';

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
  const [reconnectAttempts, setReconnectAttempts] = useState(0);
  const [lastActiveTime, setLastActiveTime] = useState(Date.now());

  // Refs
  const videoRef = useRef(null);
  const cellRef = useRef(null);
  const peerConnectionRef = useRef(null);
  const abortControllerRef = useRef(null);
  const timeoutsRef = useRef([]);
  const detectionIntervalRef = useRef(null);
  const prevStreamNameRef = useRef('');
  const lastFrameTimeRef = useRef(Date.now());
  const detectionOverlayRef = useRef(null);

  // Clear all timeouts on unmount or when needed
  const clearAllTimeouts = useCallback(() => {
    timeoutsRef.current.forEach(timeoutId => clearTimeout(timeoutId));
    timeoutsRef.current = [];
  }, []);

  // Set a timeout that will be automatically cleared on unmount
  const setManagedTimeout = useCallback((callback, delay) => {
    const timeoutId = setTimeout(callback, delay);
    timeoutsRef.current.push(timeoutId);
    return timeoutId;
  }, []);

  // Clean up WebRTC connection
  const cleanupWebRTCConnection = useCallback(() => {
    console.log(`Cleaning up WebRTC connection for stream ${stream?.name}`);

    // Don't reset error state here as we want to keep showing errors
    // until the user explicitly retries or the component is unmounted

    // Clear all timeouts
    clearAllTimeouts();

    // Abort any pending fetch requests
    if (abortControllerRef.current) {
      try {
        abortControllerRef.current.abort();
      } catch (e) {
        console.error(`Error aborting fetch for stream ${stream?.name}:`, e);
      }
      abortControllerRef.current = null;
    }

    // Detection polling is now handled by the DetectionOverlay component
    detectionIntervalRef.current = null;

    // Clean up video element
    if (videoRef.current) {
      const videoElement = videoRef.current;

      // Remove event handlers
      videoElement.onloadeddata = null;
      videoElement.onplaying = null;
      videoElement.onerror = null;
      videoElement.onstalled = null;
      videoElement.ontimeout = null;

      // Stop all tracks in the srcObject if it exists
      if (videoElement.srcObject) {
        try {
          const tracks = videoElement.srcObject.getTracks();
          tracks.forEach(track => {
            try {
              track.stop();
            } catch (trackError) {
              console.warn(`Error stopping track for stream ${stream?.name}:`, trackError);
            }
          });
        } catch (tracksError) {
          console.warn(`Error accessing tracks for stream ${stream?.name}:`, tracksError);
        }

        // Clear the srcObject
        videoElement.srcObject = null;
      }
    }

    // Close the peer connection
    if (peerConnectionRef.current) {
      try {
        // Remove all event listeners to prevent memory leaks
        const pc = peerConnectionRef.current;
        if (pc.onicecandidate) pc.onicecandidate = null;
        if (pc.oniceconnectionstatechange) pc.oniceconnectionstatechange = null;
        if (pc.onconnectionstatechange) pc.onconnectionstatechange = null;
        if (pc.ontrack) pc.ontrack = null;

        // Close the connection
        pc.close();
      } catch (closeError) {
        console.warn(`Error closing connection for stream ${stream?.name}:`, closeError);
      }

      peerConnectionRef.current = null;
    }

    console.log(`WebRTC connection cleanup completed for stream ${stream?.name}`);
  }, [clearAllTimeouts]);

  // Send WebRTC offer to server
  const sendOffer = useCallback(async (streamName, offer) => {
    try {
      // Create a new AbortController for this request
      abortControllerRef.current = new AbortController();
      const signal = abortControllerRef.current.signal;

      // Format the offer according to go2rtc expectations
      const formattedOffer = {
        type: offer.type,
        sdp: offer.sdp
      };

      console.log(`Sending WebRTC offer for stream ${streamName}`);

      // Get auth token if available
      const auth = localStorage.getItem('auth');

      // Send the offer to the server
      const response = await fetch(`/api/webrtc?src=${encodeURIComponent(streamName)}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(auth ? { 'Authorization': 'Basic ' + auth } : {})
        },
        body: JSON.stringify(formattedOffer),
        signal
      });

      if (!response.ok) {
        throw new Error(`Failed to send offer: ${response.status} ${response.statusText}`);
      }

      const text = await response.text();
      try {
        return JSON.parse(text);
      } catch (jsonError) {
        console.error(`Error parsing JSON for stream ${streamName}:`, jsonError);
        console.log(`Raw response text: ${text}`);
        throw new Error(`Failed to parse WebRTC answer: ${jsonError.message}`);
      }
    } catch (error) {
      // Check if this was an abort error, which we can safely ignore
      if (error.name === 'AbortError') {
        console.log(`WebRTC offer request for stream ${streamName} was aborted`);
        return Promise.reject(new Error('Request aborted'));
      }

      console.error(`Error sending offer for stream ${streamName}:`, error);
      throw error;
    }
  }, []);

  // Initialize WebRTC connection
  const initializeWebRTCConnection = useCallback(() => {
    if (!stream || !videoRef.current) {
      console.error(`Cannot initialize WebRTC: missing stream or DOM elements`);
      return;
    }

    console.log(`Initializing WebRTC connection for stream ${stream.name}`);
    setIsLoading(true);

    // Clean up any existing connection first
    cleanupWebRTCConnection();

    // Create a new RTCPeerConnection with ICE servers
    const pc = new RTCPeerConnection({
      iceServers: [
        { urls: 'stun:stun.l.google.com:19302' },
        { urls: 'stun:stun1.l.google.com:19302' },
        { urls: 'stun:stun2.l.google.com:19302' }
      ],
      iceTransportPolicy: 'all',
      bundlePolicy: 'balanced',
      rtcpMuxPolicy: 'require',
      sdpSemantics: 'unified-plan'
    });

    // Store the connection
    peerConnectionRef.current = pc;

    // Add event listeners
    pc.ontrack = (event) => {
      console.log(`Track received for stream ${stream.name}:`, event);

      if (event.track.kind === 'video') {
        const videoElement = videoRef.current;
        if (!videoElement) return;

        // Set srcObject
        videoElement.srcObject = event.streams[0];

        // Add event handlers for video element
        videoElement.onloadeddata = () => {
          console.log(`Video data loaded for stream ${stream.name}`);
          // Don't set isLoading=false here, wait for onPlaying
        };

        videoElement.onplaying = () => {
          console.log(`Video playing for stream ${stream.name}`);
          setIsLoading(false);
          setIsPlaying(true);
          setLastActiveTime(Date.now());
          lastFrameTimeRef.current = Date.now();

          // Detection polling is now handled by the DetectionOverlay component
        };

        videoElement.onerror = (e) => {
          console.error(`Video error for stream ${stream.name}:`, e);
          setError('Video playback error: ' + (e.message || 'Unknown error'));
          setIsLoading(false);
        };

        videoElement.onstalled = () => {
          console.warn(`Video stalled for stream ${stream.name}`);
          // We don't treat this as an error immediately, as it might recover
        };
      }
    };

    pc.onicecandidate = (event) => {
      if (event.candidate) {
        console.log(`ICE candidate for stream ${stream.name}:`, event.candidate);
        // go2rtc doesn't use a separate ICE endpoint, so we don't need to send ICE candidates
      }
    };

    pc.oniceconnectionstatechange = () => {
      console.log(`ICE connection state for stream ${stream.name}:`, pc.iceConnectionState);

      if (pc.iceConnectionState === 'failed' && peerConnectionRef.current === pc) {
        console.warn(`ICE failed for stream ${stream.name}`);
        setError('WebRTC ICE connection failed');
        setIsLoading(false);
      } else if (pc.iceConnectionState === 'disconnected') {
        console.warn(`ICE disconnected for stream ${stream.name}`);

        // Start a timer to check if it reconnects
        setManagedTimeout(() => {
          if (pc.iceConnectionState === 'disconnected' && peerConnectionRef.current === pc) {
            console.warn(`ICE still disconnected for stream ${stream.name} after timeout`);

            // Only attempt auto-reconnect if we haven't exceeded max attempts
            if (reconnectAttempts < 3) {
              console.log(`Auto-reconnecting stream ${stream.name} (attempt ${reconnectAttempts + 1})`);
              setReconnectAttempts(prev => prev + 1);
              initializeWebRTCConnection();
            } else {
              setError('WebRTC connection disconnected');
              setIsPlaying(false);
              setIsLoading(false);
            }
          }
        }, 8000); // 8 second timeout to recover (increased from 5)
      }
    };

    pc.onconnectionstatechange = () => {
      console.log(`Connection state changed for stream ${stream.name}:`, pc.connectionState);

      if (pc.connectionState === 'failed' && peerConnectionRef.current === pc) {
        console.warn(`Connection failed for stream ${stream.name}`);
        setError('WebRTC connection failed');
        setIsLoading(false);
      }
    };

    // Add transceivers to ensure we get both audio and video tracks
    pc.addTransceiver('video', {direction: 'recvonly'});
    pc.addTransceiver('audio', {direction: 'recvonly'});

    // Create an offer
    const offerOptions = {
      offerToReceiveAudio: true,
      offerToReceiveVideo: true
    };

    // Set up the connection
    pc.createOffer(offerOptions)
      .then(offer => {
        if (peerConnectionRef.current !== pc) {
          throw new Error('Connection was cleaned up during offer creation');
        }
        return pc.setLocalDescription(offer);
      })
      .then(() => {
        if (peerConnectionRef.current !== pc) {
          throw new Error('Connection was cleaned up after setting local description');
        }
        return sendOffer(stream.name, pc.localDescription);
      })
      .then(answer => {
        if (peerConnectionRef.current !== pc) {
          throw new Error('Connection was cleaned up after receiving answer');
        }
        return pc.setRemoteDescription(new RTCSessionDescription(answer));
      })
      .catch(error => {
        // Only handle error if this is still the current connection
        if (peerConnectionRef.current === pc) {
          console.error(`Error setting up WebRTC for stream ${stream.name}:`, error);
          setError(error.message || 'Failed to establish WebRTC connection');
          setIsLoading(false);
        } else {
          console.log(`WebRTC setup for stream ${stream.name} was cancelled: ${error.message}`);
        }
      });
  }, [stream.name, cleanupWebRTCConnection, sendOffer, setManagedTimeout]);

  useEffect(() => {
    if (!stream || !stream.name) return;

    // Only reinitialize if the stream name changed
    if (stream.name !== prevStreamNameRef.current) {
      console.log(`WebRTCVideoCell: Stream name changed from ${prevStreamNameRef.current} to ${stream.name}, reinitializing`);
      initializeWebRTCConnection();
    }

    // Update the previous stream name
    prevStreamNameRef.current = stream.name;

  }, [stream, isLoading]);

  // Handle retry button click
  const handleRetry = useCallback(() => {
    if (!stream) return;

    console.log(`Retrying connection for stream ${stream.name}`);
    setReconnectAttempts(0); // Reset reconnect attempts on manual retry
    setError(null); // Explicitly reset error state on retry
    setIsLoading(true); // Show loading state again
    initializeWebRTCConnection();
  }, [initializeWebRTCConnection]);

  return (
    <div
      className="video-cell"
      data-stream-name={stream.name}
      data-stream-id={streamId}
      ref={cellRef}
      style={{
        position: 'relative',
        // Ensure the video cell doesn't interfere with navigation elements
        pointerEvents: isLoading ? 'none' : 'auto',
        zIndex: 1 // Lower z-index to prevent interfering with header
      }}
    >
      {/* Add keyframes for spinner animation in the component */}
      <style>
        {`
          @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
          }
        `}
      </style>
      {/* Video element */}
      <video
        id={`video-${streamId.replace(/\s+/g, '-')}`}
        className="video-element"
        ref={videoRef}
        playsInline
        autoPlay
        muted
        disablePictureInPicture
        style={{ pointerEvents: 'none', width: '100%', height: '100%', objectFit: 'contain', zIndex: 1 }}
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

      {/* Stream name overlay */}
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
          zIndex: 3, /* Lower z-index to ensure it doesn't block navigation */
          pointerEvents: 'none'
        }}
      >
        {stream.name}
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
          zIndex: 5, /* Lower z-index but still above video and overlays */
          backgroundColor: 'rgba(0, 0, 0, 0.5)',
          padding: '5px',
          borderRadius: '4px',
          pointerEvents: 'auto' /* Keep pointer events enabled for controls */
        }}
      >
        <div
          style={{
            backgroundColor: 'transparent',
            padding: '5px',
            borderRadius: '4px',
            position: 'relative',
            zIndex: 1 /* Lower z-index to prevent blocking */
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
            cursor: 'pointer',
            position: 'relative',
            zIndex: 1 /* Lower z-index to prevent blocking */
          }}
          onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)'}
          onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
        </button>
      </div>

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
            zIndex: 5,
            pointerEvents: 'auto', /* Keep pointer events enabled for error state to allow retry button clicks */
            textAlign: 'center',
            // Ensure the error indicator is contained within the video cell
            overflow: 'hidden'
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
              onClick={handleRetry}
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
                transition: 'background-color 0.2s ease'
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
