/**
 * MSEVideoCell Component
 * A self-contained component for displaying an MSE (Media Source Extensions) video stream
 * using WebSocket connection to go2rtc for low-latency streaming
 */

import { h } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
import { DetectionOverlay, takeSnapshotWithDetections } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showSnapshotPreview } from './UI.jsx';
import { PTZControls } from './PTZControls.jsx';
import { getGo2rtcWebSocketUrl } from '../../utils/settings-utils.js';

/**
 * MSEVideoCell component
 * @param {Object} props - Component props
 * @param {Object} props.stream - Stream object
 * @param {Function} props.onToggleFullscreen - Fullscreen toggle handler
 * @param {string} props.streamId - Stream ID for stable reference
 * @param {number} props.initDelay - Delay in ms before initializing MSE (for staggered loading)
 * @returns {JSX.Element} MSEVideoCell component
 */
export function MSEVideoCell({
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

  // PTZ controls state
  const [showPTZControls, setShowPTZControls] = useState(false);

  // Refs
  const videoRef = useRef(null);
  const cellRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const wsRef = useRef(null);
  const mediaSourceRef = useRef(null);
  const sourceBufferRef = useRef(null);
  const bufferRef = useRef(null);
  const bufferLenRef = useRef(0);
  const reconnectTimeoutRef = useRef(null);
  const initTimeoutRef = useRef(null);
  const dataHandlerRef = useRef(null);

  // Constants from go2rtc's video-rtc.js
  const RECONNECT_TIMEOUT = 15000;
  const CODECS = [
    'avc1.640029',      // H.264 high 4.1
    'avc1.64002A',      // H.264 high 4.2
    'avc1.640033',      // H.264 high 5.1
    'hvc1.1.6.L153.B0', // H.265 main 5.1
    'mp4a.40.2',        // AAC LC
    'mp4a.40.5',        // AAC HE
    'flac',             // FLAC
    'opus',             // OPUS
  ];

  /**
   * Get supported codecs string for MSE negotiation
   * @param {Function} isSupported - MediaSource.isTypeSupported function
   * @returns {string} Comma-separated list of supported codecs
   */
  const getCodecs = (isSupported) => {
    return CODECS
      .filter(codec => isSupported(`video/mp4; codecs="${codec}"`))
      .join(',');
  };

  /**
   * Initialize MSE and WebSocket connection
   * Follows go2rtc's video-rtc.js onmse() pattern closely.
   */
  const initMSE = async () => {
    if (!videoRef.current) {
      initTimeoutRef.current = setTimeout(initMSE, 50);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      // Use direct WebSocket URL to go2rtc (bypasses lightNVR's HTTP-only proxy)
      const go2rtcWsUrl = await getGo2rtcWebSocketUrl();
      const wsUrl = `${go2rtcWsUrl}/api/ws?src=${encodeURIComponent(stream.name)}`;

      // Create WebSocket connection
      const ws = new WebSocket(wsUrl);
      ws.binaryType = 'arraybuffer';
      wsRef.current = ws;

      // Create MediaSource — match go2rtc's pattern exactly
      const MediaSourceClass = window.ManagedMediaSource || window.MediaSource;
      if (!MediaSourceClass) {
        throw new Error('MediaSource not supported in this browser');
      }

      const ms = new MediaSourceClass();
      mediaSourceRef.current = ms;

      // Helper: send codec negotiation once both WS and MS are ready
      const sendCodecs = () => {
        if (ws.readyState === WebSocket.OPEN && ms.readyState === 'open') {
          ws.send(JSON.stringify({
            type: 'mse',
            value: getCodecs(MediaSourceClass.isTypeSupported.bind(MediaSourceClass))
          }));
        }
      };

      // Codec negotiation: go2rtc sends inside sourceopen, but we also
      // need to handle the case where WS opens after sourceopen
      if (window.ManagedMediaSource && ms instanceof window.ManagedMediaSource) {
        ms.addEventListener('sourceopen', sendCodecs, { once: true });
        videoRef.current.disableRemotePlayback = true;
        videoRef.current.srcObject = ms;
      } else {
        ms.addEventListener('sourceopen', () => {
          URL.revokeObjectURL(videoRef.current.src);
          sendCodecs();
        }, { once: true });
        videoRef.current.src = URL.createObjectURL(ms);
        videoRef.current.srcObject = null;
      }

      // Start playback
      videoRef.current.play().catch(() => {
        if (videoRef.current && !videoRef.current.muted) {
          videoRef.current.muted = true;
          videoRef.current.play().catch(() => {});
        }
      });

      // WebSocket event handlers
      ws.addEventListener('open', () => sendCodecs());

      ws.addEventListener('message', (ev) => {
        if (typeof ev.data === 'string') {
          handleMessage(JSON.parse(ev.data), ms, MediaSourceClass);
        } else {
          handleBinaryData(ev.data);
        }
      });

      ws.addEventListener('close', () => handleClose());
      ws.addEventListener('error', () => setError('WebSocket connection error'));

    } catch (err) {
      console.error(`[MSE ${stream.name}] Init error:`, err);
      setError(err.message || 'Failed to initialize MSE stream');
      setIsLoading(false);
    }
  };

  /**
   * Handle JSON messages from WebSocket
   * Matches go2rtc's onmessage['mse'] handler pattern.
   */
  const handleMessage = (msg, ms, MediaSourceClass) => {
    if (msg.type !== 'mse') {
      if (msg.type === 'error') {
        setError(msg.value || 'Stream error');
        setIsLoading(false);
      }
      return;
    }

    try {
      const sb = ms.addSourceBuffer(msg.value);
      sb.mode = 'segments';
      sourceBufferRef.current = sb;

      const buf = new Uint8Array(2 * 1024 * 1024);
      let bufLen = 0;
      bufferRef.current = buf;
      bufferLenRef.current = 0;

      sb.addEventListener('updateend', () => {
        if (!sb.updating && bufLen > 0) {
          try {
            sb.appendBuffer(buf.slice(0, bufLen));
            bufLen = 0;
            bufferLenRef.current = 0;
          } catch (e) {
            // silently ignore — go2rtc pattern
          }
        }

        if (!sb.updating && sb.buffered && sb.buffered.length) {
          const end = sb.buffered.end(sb.buffered.length - 1);
          const start = end - 5;
          const start0 = sb.buffered.start(0);
          if (start > start0) {
            sb.remove(start0, start);
            if (ms.setLiveSeekableRange) {
              ms.setLiveSeekableRange(start, end);
            }
          }
          if (videoRef.current && videoRef.current.currentTime < start) {
            videoRef.current.currentTime = start;
          }
          if (videoRef.current) {
            const gap = end - videoRef.current.currentTime;
            videoRef.current.playbackRate = gap > 0.1 ? gap : 0.1;
          }
        }
      });

      // Wire up binary data handler using closure over sb/buf/bufLen
      dataHandlerRef.current = (data) => {
        if (sb.updating || bufLen > 0) {
          const b = new Uint8Array(data);
          buf.set(b, bufLen);
          bufLen += b.byteLength;
          bufferLenRef.current = bufLen;
        } else {
          try {
            sb.appendBuffer(data);
          } catch (e) {
            // silently ignore — go2rtc pattern
          }
        }
      };

      setIsLoading(false);
      setIsPlaying(true);
    } catch (err) {
      console.error(`[MSE ${stream.name}] SourceBuffer error:`, err);
      setError('Failed to create media buffer');
      setIsLoading(false);
    }
  };

  /**
   * Handle binary video data from WebSocket
   * Delegates to the closure-based handler set up in handleMessage.
   */
  const handleBinaryData = (data) => {
    if (dataHandlerRef.current) {
      dataHandlerRef.current(data);
    }
  };

  /**
   * Handle WebSocket close and reconnection
   */
  const handleClose = () => {
    if (reconnectTimeoutRef.current) return;

    reconnectTimeoutRef.current = setTimeout(() => {
      reconnectTimeoutRef.current = null;
      if (videoRef.current) {
        cleanup();
        initMSE();
      }
    }, RECONNECT_TIMEOUT);
  };

  /**
   * Cleanup resources
   */
  const cleanup = () => {
    // Clear timeouts
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }
    if (initTimeoutRef.current) {
      clearTimeout(initTimeoutRef.current);
      initTimeoutRef.current = null;
    }

    // Close WebSocket
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }

    // Clean up MediaSource
    if (mediaSourceRef.current) {
      try {
        if (mediaSourceRef.current.readyState === 'open') {
          mediaSourceRef.current.endOfStream();
        }
      } catch (e) {
        // silently ignore
      }
      mediaSourceRef.current = null;
    }

    // Clean up video element
    if (videoRef.current) {
      if (videoRef.current.src && videoRef.current.src.startsWith('blob:')) {
        URL.revokeObjectURL(videoRef.current.src);
      }
      videoRef.current.src = '';
      videoRef.current.srcObject = null;
    }

    sourceBufferRef.current = null;
    bufferRef.current = null;
    bufferLenRef.current = 0;
    dataHandlerRef.current = null;
  };

  /**
   * Handle retry button click
   */
  const handleRetry = () => {
    setRetryCount(prev => prev + 1);
    setError(null);
    setIsLoading(true);
  };

  /**
   * Handle snapshot button click
   */
  const handleSnapshot = () => {
    if (!videoRef.current) return;

    // If detection overlay is enabled, use it for snapshot
    if (detectionOverlayRef.current && stream.detection_based_recording && stream.detection_model) {
      takeSnapshotWithDetections(
        videoRef.current,
        detectionOverlayRef.current,
        stream.name
      ).then(blob => {
        if (blob) {
          showSnapshotPreview(blob, stream.name);
        }
      });
    } else {
      // Regular snapshot without detections
      const canvas = document.createElement('canvas');
      canvas.width = videoRef.current.videoWidth;
      canvas.height = videoRef.current.videoHeight;
      const ctx = canvas.getContext('2d');
      ctx.drawImage(videoRef.current, 0, 0);

      canvas.toBlob(blob => {
        if (blob) {
          showSnapshotPreview(blob, stream.name);
        }
      }, 'image/jpeg', 0.95);
    }
  };

  // Initialize MSE when component mounts or retry is triggered
  useEffect(() => {
    if (!stream || !stream.name) return;

    let isMounted = true;
    let delayTimeout = null;

    const doInit = () => {
      if (!isMounted) return;
      initMSE();
    };

    if (initDelay > 0) {
      delayTimeout = setTimeout(doInit, initDelay);
    } else {
      doInit();
    }

    return () => {
      isMounted = false;
      if (delayTimeout) clearTimeout(delayTimeout);
      cleanup();
    };
  }, [stream?.name, retryCount, initDelay]);

  // Video element event handlers — go2rtc closes WS on video error to trigger reconnect
  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    const handlePlay = () => {
      setIsPlaying(true);
      setIsLoading(false);
    };

    const handleError = () => {
      if (wsRef.current) wsRef.current.close(); // triggers reconnect
    };

    video.addEventListener('play', handlePlay);
    video.addEventListener('error', handleError);

    return () => {
      video.removeEventListener('play', handlePlay);
      video.removeEventListener('error', handleError);
    };
  }, [stream?.name]);

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

      {/* Stream name label */}
      {showLabels && (
        <div
          className="stream-name-label"
          style={{
            position: 'absolute',
            top: '8px',
            left: '8px',
            backgroundColor: 'rgba(0, 0, 0, 0.7)',
            color: 'white',
            padding: '4px 8px',
            borderRadius: '4px',
            fontSize: '14px',
            fontWeight: 'bold',
            zIndex: 10,
            pointerEvents: 'none'
          }}
        >
          {stream.name}
        </div>
      )}

      {/* Loading indicator */}
      {isLoading && !error && (
        <LoadingIndicator message={`Loading ${stream.name}...`} />
      )}

      {/* Error overlay */}
      {error && (
        <div
          className="error-overlay"
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            backgroundColor: 'rgba(0, 0, 0, 0.8)',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            zIndex: 20,
            padding: '20px'
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
            lineHeight: '1.4',
            color: 'white'
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
          >
            Retry
          </button>
        </div>
      )}

      {/* Control buttons overlay */}
      {showControls && isPlaying && !error && (
        <div
          className="video-controls"
          style={{
            position: 'absolute',
            bottom: '8px',
            right: '8px',
            display: 'flex',
            gap: '8px',
            zIndex: 10
          }}
        >
          {/* Snapshot button */}
          <SnapshotButton onClick={handleSnapshot} />

          {/* PTZ toggle button */}
          {stream.ptz_type && stream.ptz_type !== 'none' && (
            <button
              className="ptz-toggle-btn"
              onClick={() => setShowPTZControls(!showPTZControls)}
              style={{
                padding: '8px 12px',
                backgroundColor: showPTZControls ? '#2563eb' : 'rgba(0, 0, 0, 0.7)',
                color: 'white',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer',
                fontSize: '14px',
                fontWeight: 'bold',
                boxShadow: '0 2px 4px rgba(0, 0, 0, 0.3)',
                transition: 'background-color 0.2s ease'
              }}
              title="Toggle PTZ Controls"
            >
              PTZ
            </button>
          )}

          {/* Fullscreen button */}
          <button
            className="fullscreen-btn"
            onClick={() => onToggleFullscreen(stream.name)}
            style={{
              padding: '8px 12px',
              backgroundColor: 'rgba(0, 0, 0, 0.7)',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '14px',
              fontWeight: 'bold',
              boxShadow: '0 2px 4px rgba(0, 0, 0, 0.3)',
              transition: 'background-color 0.2s ease'
            }}
            title="Toggle Fullscreen"
          >
            ⛶
          </button>
        </div>
      )}

      {/* PTZ Controls */}
      {showPTZControls && stream.ptz_type && stream.ptz_type !== 'none' && (
        <PTZControls
          streamName={stream.name}
          ptzType={stream.ptz_type}
          onClose={() => setShowPTZControls(false)}
        />
      )}

      {/* MSE mode indicator */}
      {showLabels && isPlaying && (
        <div
          className="mode-indicator"
          style={{
            position: 'absolute',
            top: '8px',
            right: '8px',
            backgroundColor: 'rgba(16, 185, 129, 0.8)',
            color: 'white',
            padding: '4px 8px',
            borderRadius: '4px',
            fontSize: '12px',
            fontWeight: 'bold',
            zIndex: 10,
            pointerEvents: 'none'
          }}
          title="MSE (Media Source Extensions) - Low latency streaming"
        >
          MSE
        </div>
      )}
    </div>
  );
}

