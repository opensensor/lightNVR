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
import { PTZControls } from './PTZControls.jsx';
import { getGo2rtcBaseUrl, isGo2rtcAvailable } from '../../utils/settings-utils.js';
import Hls from 'hls.js';

/**
 * HLSVideoCell component
 * @param {Object} props - Component props
 * @param {Object} props.stream - Stream object
 * @param {Function} props.onToggleFullscreen - Fullscreen toggle handler
 * @param {string} props.streamId - Stream ID for stable reference
 * @param {number} props.initDelay - Delay in ms before initializing HLS (for staggered loading)
 * @returns {JSX.Element} HLSVideoCell component
 */
export function HLSVideoCell({
  stream,
  streamId,
  onToggleFullscreen,
  initDelay = 0,
  showLabels = true,
  showControls = true
}) {
  // Component state
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [retryCount, setRetryCount] = useState(0);

  // HLS source state: 'go2rtc' (go2rtc's dynamic HLS), 'native' (lightNVR FFmpeg-based HLS), or 'failed'
  // Default to native lightNVR HLS (reliable, always running when streaming enabled)
  // go2rtc mode is used only when the backend reports go2rtc is available for this stream
  const [hlsMode, setHlsMode] = useState(() => {
    return stream && stream.go2rtc_hls_available ? 'go2rtc' : 'native';
  });

  // PTZ controls state
  const [showPTZControls, setShowPTZControls] = useState(false);

  // Refs
  const videoRef = useRef(null);
  const cellRef = useRef(null);
  const hlsPlayerRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const initAttemptedRef = useRef(false);

  /**
   * Refresh the stream's go2rtc registration
   * This is useful when HLS connections fail due to stale go2rtc state
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

  // Initialize HLS player when component mounts or retry is triggered
  useEffect(() => {
    if (!stream || !stream.name) {
      console.warn(`[HLS] Skipping init - no stream data`);
      return;
    }

    console.log(`[HLS ${stream.name}] useEffect triggered, videoRef:`, !!videoRef.current, 'retryCount:', retryCount, 'initDelay:', initDelay);

    // Track if component is still mounted - using ref for stable access in callbacks
    let isMounted = true;
    let initTimeout = null;
    let delayTimeout = null;

    // Store event listener references for cleanup (native HLS case)
    let nativeLoadedHandler = null;
    let nativeErrorHandler = null;

    // Async initialization function - MUST be defined before doInit to avoid TDZ errors
    const initHls = async () => {
      let hlsStreamUrl;
      let usingGo2rtc = false;

      // Determine which HLS source to use based on current mode
      if (hlsMode === 'go2rtc') {
        // Check if go2rtc is actually available before trying to use it
        const go2rtcReady = await isGo2rtcAvailable();
        if (!isMounted) return;

        if (go2rtcReady) {
          // Get go2rtc base URL for HLS streaming
          let go2rtcBaseUrl;
          try {
            go2rtcBaseUrl = await getGo2rtcBaseUrl();
            console.log(`Using go2rtc base URL for HLS: ${go2rtcBaseUrl}`);
          } catch (err) {
            console.warn('Failed to get go2rtc URL from settings, using default port 1984:', err);
            go2rtcBaseUrl = `http://${window.location.hostname}:1984`;
          }

          if (!isMounted) return;

          // Build the HLS stream URL using go2rtc's dynamic HLS endpoint
          // Using &mp4=flac for best codec compatibility (H264/H265 + AAC/PCMA/PCMU/PCM)
          hlsStreamUrl = `${go2rtcBaseUrl}/api/stream.m3u8?src=${encodeURIComponent(stream.name)}&mp4=flac`;
          usingGo2rtc = true;
          console.log(`[HLS ${stream.name}] Using go2rtc HLS: ${hlsStreamUrl}`);

          // Pre-warm the stream by making a HEAD request to trigger go2rtc to start preparing
          try {
            await fetch(hlsStreamUrl, { method: 'HEAD' });
            await new Promise(resolve => setTimeout(resolve, 200));
          } catch (e) {
            console.log(`[HLS ${stream.name}] Pre-warm request completed (may have failed, that's OK)`);
          }

          if (!isMounted) return;
        } else {
          // go2rtc is not available - fall back to native HLS immediately
          console.warn(`[HLS ${stream.name}] go2rtc is not available, falling back to native lightNVR HLS`);
          hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8`;
          usingGo2rtc = false;
          setHlsMode('native');
          console.log(`[HLS ${stream.name}] Using native lightNVR HLS: ${hlsStreamUrl}`);
        }
      } else if (hlsMode === 'native') {
        // Use lightNVR's FFmpeg-based HLS endpoint directly
        hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8`;
        usingGo2rtc = false;
        console.log(`[HLS ${stream.name}] Using native lightNVR HLS: ${hlsStreamUrl}`);
      } else {
        // Mode is 'failed' - don't attempt anything
        console.error(`[HLS ${stream.name}] All HLS modes have failed`);
        setError('HLS streaming unavailable - both go2rtc and native HLS failed');
        setIsLoading(false);
        return;
      }

      // Check if HLS.js is supported
      if (Hls.isSupported()) {
        console.log(`Using HLS.js for stream ${stream.name}`);
        const hls = new Hls({
          // Buffer management - optimized for go2rtc's dynamic HLS
          // Larger buffers = more stability, less flickering
          maxBufferLength: 30,            // Maximum buffer length in seconds
          maxMaxBufferLength: 60,         // Maximum maximum buffer length
          backBufferLength: 30,           // Increased back buffer for smoother playback

          // Live stream settings - tuned for stability over low latency
          liveSyncDurationCount: 4,       // Slightly behind live edge for stability
          liveMaxLatencyDurationCount: 15, // More tolerance before seeking to live
          liveDurationInfinity: true,     // Treat as infinite live stream
          lowLatencyMode: false,          // Disable low latency for stability

          // Buffer health monitoring - less aggressive checking
          highBufferWatchdogPeriod: 5,    // Check buffer health every 5 seconds (was 3)

          // Loading timeouts - generous to handle network hiccups
          fragLoadingTimeOut: 30000,      // Fragment loading timeout
          manifestLoadingTimeOut: 20000,  // Manifest loading timeout
          levelLoadingTimeOut: 20000,     // Level loading timeout

          // Quality settings
          startLevel: -1,                 // Auto-select quality
          abrEwmaDefaultEstimate: 500000, // Conservative bandwidth estimate
          abrBandWidthFactor: 0.7,        // Conservative bandwidth factor
          abrBandWidthUpFactor: 0.5,      // Conservative quality increase

          // Worker and debugging
          enableWorker: true,             // Use web worker for better performance
          debug: false,                   // Disable debug logging

          // Buffer flushing - more tolerant of gaps and holes
          maxBufferHole: 1.0,             // Increased hole tolerance (was 0.5)
          maxFragLookUpTolerance: 0.5,    // Increased lookup tolerance (was 0.25)
          nudgeMaxRetry: 10,              // More retry attempts (was 5)
          nudgeOffset: 0.2,               // Larger nudge offset for recovery

          // Append error handling - more retries for better recovery
          appendErrorMaxRetry: 10,        // More retries on append errors (was 5)

          // Manifest refresh - more retries for network resilience
          manifestLoadingMaxRetry: 6,     // More retries for initial manifest
          manifestLoadingRetryDelay: 1500, // Longer delay between manifest retries
          levelLoadingMaxRetry: 6,        // Retry level loading
          levelLoadingRetryDelay: 1500,   // Delay between level retries
          fragLoadingMaxRetry: 10,        // More retries for fragments (init.mp4 may take time)
          fragLoadingRetryDelay: 1000,    // Delay between fragment retries

          // Stall recovery - let HLS.js handle stalls gracefully
          maxStarvationDelay: 4,          // Max delay before starvation recovery
          maxLoadingDelay: 4,             // Max loading delay before giving up

          // Start playback even with minimal buffer
          startFragPrefetch: true         // Prefetch next fragment for smoother playback
        });

        // Store hls instance IMMEDIATELY after creation for cleanup
        hlsPlayerRef.current = hls;

        hls.loadSource(hlsStreamUrl);
        hls.attachMedia(videoRef.current);

        hls.on(Hls.Events.MANIFEST_PARSED, function() {
          if (!isMounted) return;
          setIsLoading(false);
          setIsPlaying(true);

          if (videoRef.current) {
            videoRef.current.play().catch(error => {
              console.warn('Auto-play prevented:', error);
              // We'll handle this with the play button overlay
            });
          }
        });

        hls.on(Hls.Events.ERROR, function(event, data) {
          if (!isMounted) return;

          // Handle non-fatal errors - be very conservative to avoid flickering
          if (!data.fatal) {
            // These errors are normal during live streaming and HLS.js handles them automatically
            // Do NOT call recoverMediaError() as it causes black flicker
            const ignoredErrors = [
              'bufferStalledError',      // Normal buffering event
              'bufferNudgeOnStall',      // HLS.js internal recovery
              'bufferSeekOverHole',      // Gap in buffer, HLS.js handles it
              'fragParsingError',        // Occasional parsing issues
              'internalException',       // Internal HLS.js recovery
              'bufferAppendError',       // Temporary append issues
              'fragLoadError',           // Fragment load errors - HLS.js retries automatically
              'levelLoadError',          // Level load errors - HLS.js retries automatically
              'manifestLoadError'        // Manifest load errors - HLS.js retries automatically
            ];

            if (ignoredErrors.includes(data.details)) {
              // Silently ignore - HLS.js handles these automatically
              return;
            }

            // Log other non-fatal errors but don't try aggressive recovery
            console.log(`HLS non-fatal error: ${data.type}, details: ${data.details}`);

            // For network errors, HLS.js will retry automatically - don't intervene
            if (data.type === Hls.ErrorTypes.NETWORK_ERROR) {
              // Network errors often resolve themselves, just log them
              console.log('Non-fatal network error, HLS.js will retry automatically');
            }
            // For media errors, only log - don't call recoverMediaError() as it causes flicker
            else if (data.type === Hls.ErrorTypes.MEDIA_ERROR) {
              console.log('Non-fatal media error, monitoring...', data.details);
            }
            return;
          }

          // Fatal errors require intervention
          console.error(`HLS fatal error: ${data.type}, details: ${data.details}`);

          switch(data.type) {
            case Hls.ErrorTypes.NETWORK_ERROR:
              // For network errors, try to restart loading
              console.error('Fatal network error encountered, trying to recover');
              // Add a small delay before restarting to avoid rapid retries
              setTimeout(() => {
                if (isMounted && hlsPlayerRef.current) {
                  hlsPlayerRef.current.startLoad();
                }
              }, 1000);
              break;
            case Hls.ErrorTypes.MEDIA_ERROR:
              // For fatal media errors, we need to recover
              console.error('Fatal media error encountered, trying to recover');
              // Use a delay to avoid flicker from rapid recovery attempts
              setTimeout(() => {
                if (isMounted && hlsPlayerRef.current) {
                  hlsPlayerRef.current.recoverMediaError();
                }
              }, 500);
              break;
            default:
              // Cannot recover from other fatal errors
              console.error('Unrecoverable HLS error, destroying player');
              if (hlsPlayerRef.current) {
                hlsPlayerRef.current.destroy();
                hlsPlayerRef.current = null;
              }
              if (isMounted) {
                // If we were using go2rtc, fall back to native HLS
                if (usingGo2rtc) {
                  console.warn(`[HLS ${stream.name}] go2rtc HLS failed, falling back to native lightNVR HLS`);
                  setHlsMode('native');
                } else {
                  setError(data.details || 'HLS playback error');
                  setIsLoading(false);
                  setIsPlaying(false);
                }
              }
              break;
          }
        });
      }
      // Check if HLS is supported natively (Safari)
      else if (videoRef.current.canPlayType('application/vnd.apple.mpegurl')) {
        console.log(`Using native HLS support for stream ${stream.name}`);
        // Native HLS support (Safari)
        videoRef.current.src = hlsStreamUrl;

        // Store handlers for cleanup
        nativeLoadedHandler = function() {
          if (!isMounted) return;
          setIsLoading(false);
          setIsPlaying(true);
        };

        nativeErrorHandler = function() {
          if (!isMounted) return;
          // If we were using go2rtc, fall back to native HLS
          if (usingGo2rtc) {
            console.warn(`[HLS ${stream.name}] go2rtc HLS failed (native player), falling back to native lightNVR HLS`);
            setHlsMode('native');
          } else {
            setError('HLS stream failed to load');
            setIsLoading(false);
            setIsPlaying(false);
          }
        };

        videoRef.current.addEventListener('loadedmetadata', nativeLoadedHandler);
        videoRef.current.addEventListener('error', nativeErrorHandler);
      } else {
        // Fallback for truly unsupported browsers
        console.error(`HLS not supported for stream ${stream.name} - neither HLS.js nor native support available`);
        if (isMounted) {
          setError('HLS not supported by your browser - please use a modern browser');
          setIsLoading(false);
        }
      }
    };

    // Function to actually initialize HLS once video element is ready
    // Defined after initHls to avoid Temporal Dead Zone (TDZ) errors
    const doInit = async () => {
      if (!isMounted) return;

      // Wait for video element to be available (DOM might not be ready yet)
      if (!videoRef.current) {
        console.log(`[HLS ${stream.name}] Video element not ready, waiting...`);
        initTimeout = setTimeout(doInit, 50);
        return;
      }

      console.log(`[HLS ${stream.name}] Initializing HLS player...`);
      setIsLoading(true);
      setError(null);

      await initHls();
    };

    // Apply staggered initialization delay to avoid overwhelming go2rtc
    // Go2rtc has a 5-second HLS session keepalive, so staggering helps prevent session timeouts
    if (initDelay > 0) {
      console.log(`[HLS ${stream.name}] Waiting ${initDelay}ms before initialization...`);
      delayTimeout = setTimeout(doInit, initDelay);
    } else {
      doInit();
    }

    // Cleanup function
    return () => {
      console.log(`[HLS ${stream.name}] Cleaning up HLS player`);
      isMounted = false;

      // Clear any pending delay timeout
      if (delayTimeout) {
        clearTimeout(delayTimeout);
        delayTimeout = null;
      }

      // Clear any pending init timeout
      if (initTimeout) {
        clearTimeout(initTimeout);
        initTimeout = null;
      }

      // Destroy HLS.js instance
      if (hlsPlayerRef.current) {
        hlsPlayerRef.current.destroy();
        hlsPlayerRef.current = null;
      }

      // Remove native HLS event listeners (Safari)
      if (videoRef.current) {
        if (nativeLoadedHandler) {
          videoRef.current.removeEventListener('loadedmetadata', nativeLoadedHandler);
          nativeLoadedHandler = null;
        }
        if (nativeErrorHandler) {
          videoRef.current.removeEventListener('error', nativeErrorHandler);
          nativeErrorHandler = null;
        }

        // Reset video element
        videoRef.current.pause();
        videoRef.current.removeAttribute('src');
        videoRef.current.load();
      }
    };
  }, [stream, retryCount, initDelay, hlsMode]);

  // Handle retry button click
  const handleRetry = async () => {
    console.log(`Retry requested for stream ${stream?.name}`);

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

    // Reset state
    setError(null);
    setIsLoading(true);
    setIsPlaying(false);

    // Refresh the stream's go2rtc registration before retrying
    // This helps recover from stale go2rtc state that causes HLS failures
    await refreshStreamRegistration();

    // Small delay to allow go2rtc to re-register the stream
    await new Promise(resolve => setTimeout(resolve, 500));

    // Increment retry count to trigger useEffect re-run
    setRetryCount(prev => prev + 1);
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
      {showLabels && (
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
            zIndex: 15,
            display: 'flex',
            alignItems: 'center',
            gap: '6px'
          }}
        >
          {stream.name}
          <span style={{
            fontSize: '10px',
            padding: '1px 4px',
            borderRadius: '3px',
            backgroundColor: hlsMode === 'go2rtc' ? 'rgba(0, 150, 255, 0.7)' : 'rgba(100, 100, 100, 0.7)',
            color: 'white'
          }}>
            {hlsMode === 'go2rtc' ? 'go2rtc' : 'HLS'}
          </span>
        </div>
      )}

      {/* Stream controls */}
      {showControls && (
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
        {/* Force refresh stream button */}
        {isPlaying && (
          <button
            className="force-refresh-btn"
            title="Force Refresh Stream"
            onClick={handleRetry}
            style={{
              backgroundColor: 'transparent',
              border: 'none',
              padding: '5px',
              borderRadius: '4px',
              color: 'white',
              cursor: 'pointer',
              transition: 'background-color 0.2s ease'
            }}
            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)'}
            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
          >
            {/* Refresh/reload icon */}
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="23 4 23 10 17 10"></polyline>
              <polyline points="1 20 1 14 7 14"></polyline>
              <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
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
      )}

      {/* PTZ Controls overlay */}
      <PTZControls
        stream={stream}
        isVisible={showPTZControls}
        onClose={() => setShowPTZControls(false)}
      />

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
