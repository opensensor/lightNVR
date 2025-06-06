/**
 * HLSVideoCell Component
 * A self-contained component for displaying an HLS video stream
 */

import { h } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
import { DetectionOverlay, takeSnapshotWithDetections } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showSnapshotPreview } from './UI.jsx';
import Hls from 'hls.js';

/**
 * HLSVideoCell component
 * @param {Object} props - Component props
 * @param {Object} props.stream - Stream object
 * @param {Function} props.onToggleFullscreen - Fullscreen toggle handler
 * @param {string} props.streamId - Stream ID for stable reference
 * @returns {JSX.Element} HLSVideoCell component
 */
export function HLSVideoCell({
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
  const hlsPlayerRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const refreshTimerRef = useRef(null);

  // Initialize HLS player when component mounts
  useEffect(() => {
    if (!stream || !stream.name || !videoRef.current) return;

    console.log(`Initializing HLS player for stream ${stream.name}`);
    setIsLoading(true);
    setError(null);

    // Build the HLS stream URL with cache-busting timestamp to prevent stale data
    const timestamp = Date.now();
    const hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${timestamp}`;

    // Check if HLS.js is supported
    if (Hls.isSupported()) {
      console.log(`Using HLS.js for stream ${stream.name}`);
      const hls = new Hls({
        maxBufferLength: 30,            // Increased from 10 to 30 seconds for better buffering on low-power devices
        maxMaxBufferLength: 60,         // Increased from 20 to 60 seconds for better buffering on low-power devices
        liveSyncDurationCount: 4,       // Increased from 3 to 4 segments for better stability
        liveMaxLatencyDurationCount: 10, // Increased from 5 to 10 segments for better stability on low-power devices
        liveDurationInfinity: false,    // Don't treat live streams as infinite duration
        lowLatencyMode: false,          // Disable low latency mode for better stability on low-power devices
        enableWorker: true,
        fragLoadingTimeOut: 30000,      // Increased from 20 to 30 seconds timeout for fragment loading
        manifestLoadingTimeOut: 20000,  // Increased from 15 to 20 seconds timeout for manifest loading
        levelLoadingTimeOut: 20000,     // Increased from 15 to 20 seconds timeout for level loading
        backBufferLength: 60,           // Add back buffer length to keep more segments in memory
        startLevel: -1,                 // Auto-select quality level based on network conditions
        abrEwmaDefaultEstimate: 500000, // Start with a lower bandwidth estimate (500kbps)
        abrBandWidthFactor: 0.7,        // Be more conservative with bandwidth estimates
        abrBandWidthUpFactor: 0.5       // Be more conservative when increasing quality
      });

      hls.loadSource(hlsStreamUrl);
      hls.attachMedia(videoRef.current);

      hls.on(Hls.Events.MANIFEST_PARSED, function() {
        setIsLoading(false);
        setIsPlaying(true);

        videoRef.current.play().catch(error => {
          console.warn('Auto-play prevented:', error);
          // We'll handle this with the play button overlay
        });
      });

      hls.on(Hls.Events.ERROR, function(event, data) {
        if (data.fatal) {
          console.error('HLS error:', data);
          hls.destroy();
          setError(data.details || 'HLS playback error');
          setIsLoading(false);
          setIsPlaying(false);
        }
      });

      // Set up less frequent refresh to reduce load on low-power devices
      const refreshInterval = 60000; // 60 seconds
      const refreshTimer = setInterval(() => {
        if (hls) {
          console.log(`Refreshing HLS stream for ${stream.name}`);
          const newTimestamp = Date.now();
          const newUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${newTimestamp}`;
          hls.loadSource(newUrl);
        }
      }, refreshInterval);

      // Store hls instance and timer for cleanup
      hlsPlayerRef.current = hls;
      refreshTimerRef.current = refreshTimer;
    }
    // Check if HLS is supported natively (Safari)
    else if (videoRef.current.canPlayType('application/vnd.apple.mpegurl')) {
      console.log(`Using native HLS support for stream ${stream.name}`);
      // Native HLS support (Safari)
      videoRef.current.src = hlsStreamUrl;
      videoRef.current.addEventListener('loadedmetadata', function() {
        setIsLoading(false);
        setIsPlaying(true);
      });

      videoRef.current.addEventListener('error', function() {
        setError('HLS stream failed to load');
        setIsLoading(false);
        setIsPlaying(false);
      });
    } else {
      // Fallback for truly unsupported browsers
      console.error(`HLS not supported for stream ${stream.name} - neither HLS.js nor native support available`);
      setError('HLS not supported by your browser - please use a modern browser');
      setIsLoading(false);
    }

    // Cleanup function
    return () => {
      console.log(`Cleaning up HLS player for stream ${stream.name}`);
      
      // Clear refresh timer
      if (refreshTimerRef.current) {
        clearInterval(refreshTimerRef.current);
        refreshTimerRef.current = null;
      }
      
      // Destroy HLS instance
      if (hlsPlayerRef.current) {
        hlsPlayerRef.current.destroy();
        hlsPlayerRef.current = null;
      }
      
      // Reset video element
      if (videoRef.current) {
        videoRef.current.pause();
        videoRef.current.removeAttribute('src');
        videoRef.current.load();
      }
    };
  }, [stream]);

  // Handle retry button click
  const handleRetry = () => {
    // Force a re-render to restart the HLS player
    setError(null);
    setIsLoading(true);
    setIsPlaying(false);
    
    // Clean up existing player
    if (hlsPlayerRef.current) {
      hlsPlayerRef.current.destroy();
      hlsPlayerRef.current = null;
    }
    
    if (videoRef.current) {
      videoRef.current.pause();
      videoRef.current.removeAttribute('src');
      videoRef.current.load();
    }

    // Fetch updated stream info and reinitialize
    fetch(`/api/streams/${encodeURIComponent(stream.name)}`)
      .then(response => response.json())
      .then(updatedStream => {
        // Force a re-render by updating state
        setIsLoading(true);
      })
      .catch(error => {
        console.error(`Error fetching stream info for retry: ${error}`);
        setError('Failed to reconnect');
        setIsLoading(false);
      });
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
        autoPlay
        muted
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
          zIndex: 15
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
          zIndex: 30,
          backgroundColor: 'rgba(0, 0, 0, 0.5)',
          padding: '5px',
          borderRadius: '4px'
        }}
      >
        <div
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

      {/* Play button overlay (for browsers that block autoplay) */}
      {!isPlaying && !isLoading && !error && (
        <div
          className="play-overlay"
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            width: '100%',
            height: '100%',
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            backgroundColor: 'rgba(0, 0, 0, 0.5)',
            zIndex: 25,
            cursor: 'pointer'
          }}
          onClick={() => {
            if (videoRef.current) {
              videoRef.current.play()
                .then(() => {
                  setIsPlaying(true);
                })
                .catch(error => {
                  console.error('Play failed:', error);
                });
            }
          }}
        >
          <div className="play-button">
            <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="white" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polygon points="5 3 19 12 5 21 5 3"></polygon>
            </svg>
          </div>
        </div>
      )}
    </div>
  );
}
