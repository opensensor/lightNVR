/**
 * WebRTCVideoCell Component
 * A self-contained component for displaying a WebRTC video stream
 */

import { h } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
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

  // Refs
  const videoRef = useRef(null);
  const cellRef = useRef(null);
  const peerConnectionRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const abortControllerRef = useRef(null);

  // Initialize WebRTC connection when component mounts
  useEffect(() => {
    if (!stream || !stream.name || !videoRef.current) return;

    console.log(`Initializing WebRTC connection for stream ${stream.name}`);
    setIsLoading(true);
    setError(null);

    // Create a new RTCPeerConnection
    const pc = new RTCPeerConnection({
      iceServers: [
        { urls: 'stun:stun.l.google.com:19302' },
        { urls: 'stun:stun1.l.google.com:19302' }
      ]
    });

    peerConnectionRef.current = pc;

    // Set up event handlers
    pc.ontrack = (event) => {
      console.log(`Track received for stream ${stream.name}`);
      
      if (event.track.kind === 'video') {
        const videoElement = videoRef.current;
        if (!videoElement) return;

        // Set srcObject
        videoElement.srcObject = event.streams[0];
        
        // Add event handlers
        videoElement.onloadeddata = () => {
          console.log(`Video data loaded for stream ${stream.name}`);
        };

        videoElement.onplaying = () => {
          console.log(`Video playing for stream ${stream.name}`);
          setIsLoading(false);
          setIsPlaying(true);
        };

        videoElement.onerror = (e) => {
          console.error(`Video error for stream ${stream.name}:`, e);
          setError('Video playback error');
          setIsLoading(false);
        };
      }
    };

    pc.onicecandidate = (event) => {
      if (event.candidate) {
        // Filter out empty candidates
        if (event.candidate.candidate !== "") {
          console.log(`ICE candidate for stream ${stream.name}`);
        } else {
          console.log(`Ignoring empty ICE candidate for stream ${stream.name}`);
        }
      }
    };

    pc.oniceconnectionstatechange = () => {
      console.log(`ICE connection state for stream ${stream.name}: ${pc.iceConnectionState}`);
      
      if (pc.iceConnectionState === 'failed') {
        setError('WebRTC ICE connection failed');
        setIsLoading(false);
      }
    };

    // Add transceivers
    pc.addTransceiver('video', {direction: 'recvonly'});
    pc.addTransceiver('audio', {direction: 'recvonly'});

    // Create and send offer
    pc.createOffer()
      .then(offer => pc.setLocalDescription(offer))
      .then(() => {
        // Create a new AbortController for this request
        abortControllerRef.current = new AbortController();
        
        // Format the offer
        const formattedOffer = {
          type: pc.localDescription.type,
          sdp: pc.localDescription.sdp
        };

        // Get auth token if available
        const auth = localStorage.getItem('auth');

        // Send the offer to the server
        return fetch(`/api/webrtc?src=${encodeURIComponent(stream.name)}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...(auth ? { 'Authorization': 'Basic ' + auth } : {})
          },
          body: JSON.stringify(formattedOffer),
          signal: abortControllerRef.current.signal
        });
      })
      .then(response => {
        if (!response.ok) {
          throw new Error(`Failed to send offer: ${response.status} ${response.statusText}`);
        }
        return response.text();
      })
      .then(text => {
        try {
          return JSON.parse(text);
        } catch (error) {
          console.error(`Error parsing JSON for stream ${stream.name}:`, error);
          throw new Error('Failed to parse WebRTC answer');
        }
      })
      .then(answer => pc.setRemoteDescription(new RTCSessionDescription(answer)))
      .catch(error => {
        console.error(`Error setting up WebRTC for stream ${stream.name}:`, error);
        setError(error.message || 'Failed to establish WebRTC connection');
        setIsLoading(false);
      });

    // Cleanup function
    return () => {
      console.log(`Cleaning up WebRTC connection for stream ${stream.name}`);
      
      // Abort any pending fetch requests
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
        abortControllerRef.current = null;
      }
      
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
    };
  }, [stream]);

  // Handle retry button click
  const handleRetry = () => {
    // Force a re-render to restart the WebRTC connection
    setError(null);
    setIsLoading(true);
    
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
    
    // Force a re-render by updating state
    setIsPlaying(false);
  };

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
        playsInline
        autoPlay
        muted
        disablePictureInPicture
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
          zIndex: 3
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
            textAlign: 'center'
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
