/**
 * MSEVideoCell Component
 * A self-contained component for displaying an MSE (Media Source Extensions) video stream
 * using WebSocket connection to go2rtc for low-latency streaming
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { DetectionOverlay, drawDetectionsOnCanvas } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showStatusMessage } from './ToastContainer.jsx';
import { PTZControls } from './PTZControls.jsx';
import { getGo2rtcWebSocketUrl } from '../../utils/settings-utils.js';
import { formatFilenameTimestamp } from '../../utils/date-utils.js';
import { forceNavigation } from '../../utils/navigation-utils.js';
import { formatUtils } from './recordings/formatUtils.js';
import { useI18n } from '../../i18n.js';
import { useQueryClient } from '../../query-client.js';
import { createPlayerTelemetry } from '../../utils/player-telemetry.js';

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
  useSubStream = false,
  onToggleFullscreen,
  initDelay = 0,
  showLabels = true,
  showControls = true,
  globalShowDetections = true
}) {
  const { t } = useI18n();
  const queryClient = useQueryClient();

  // Component state
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [retryCount, setRetryCount] = useState(0);

  // Auto-retry tracking (separate from manual retryCount to avoid infinite loops)
  const autoRetryCountRef = useRef(0);
  const autoRetryTimeoutRef = useRef(null);
  const MAX_AUTO_RETRIES = 3; // auto-retry up to 3 times before showing error

  // Track previous stream status so we can detect transitions to 'Running'
  const prevStatusRef = useRef(stream.status);

  // PTZ controls state
  const [showPTZControls, setShowPTZControls] = useState(false);

  // Privacy mode state
  const [showPrivacyConfirm, setShowPrivacyConfirm] = useState(false);
  const [privacyActive, setPrivacyActive] = useState(!!stream.privacy_mode);
  const [isTogglingEnabled, setIsTogglingEnabled] = useState(false);

  // Detection overlay visibility state (per-camera toggle, constrained by global toggle)
  const [localShowDetections, setLocalShowDetections] = useState(true);
  const showDetections = globalShowDetections && localShowDetections;

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
      const effectiveName = useSubStream ? `${stream.name}_sub` : stream.name;
      const wsUrl = `${go2rtcWsUrl}/api/ws?src=${encodeURIComponent(effectiveName)}`;

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

      videoRef.current.ondblclick = (e) => onToggleFullscreen(stream.name, e, cellRef.current);

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
      ws.addEventListener('error', () => handleWsError());

    } catch (err) {
      console.error(`[MSE ${stream.name}] Init error:`, err);
      setError(err.message || t('live.failedToInitializeMseStream'));
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
        setError(msg.value || t('live.streamError'));
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
      // Reset auto-retry counter on successful stream setup
      autoRetryCountRef.current = 0;
    } catch (err) {
      console.error(`[MSE ${stream.name}] SourceBuffer error:`, err);
      setError(t('live.failedToCreateMediaBuffer'));
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
   * Handle WebSocket errors with auto-retry before surfacing to user.
   * Initial load races (e.g., go2rtc not yet ready for a stream) are common
   * when multiple streams start simultaneously; a few retries resolve them.
   */
  const handleWsError = () => {
    autoRetryCountRef.current += 1;
    if (autoRetryCountRef.current <= MAX_AUTO_RETRIES) {
      // Exponential back-off: 2s, 4s, 8s
      const delay = Math.min(2000 * Math.pow(2, autoRetryCountRef.current - 1), 8000);
      console.warn(`[MSE ${stream.name}] WS error, auto-retry ${autoRetryCountRef.current}/${MAX_AUTO_RETRIES} in ${delay}ms`);
      autoRetryTimeoutRef.current = setTimeout(() => {
        autoRetryTimeoutRef.current = null;
        if (videoRef.current) {
          cleanup();
          initMSE();
        }
      }, delay);
    } else {
      setError(t('live.webSocketConnectionError'));
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
    if (autoRetryTimeoutRef.current) {
      clearTimeout(autoRetryTimeoutRef.current);
      autoRetryTimeoutRef.current = null;
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
    autoRetryCountRef.current = 0; // Reset auto-retry on manual retry
    setRetryCount(prev => prev + 1);
    setError(null);
    setIsLoading(true);
  };

  /**
   * Pause stream for privacy — sets privacy_mode=true without touching the enabled flag.
   */
  const handlePauseForPrivacy = async () => {
    setIsTogglingEnabled(true);
    try {
      const res = await fetch(`/api/streams/${encodeURIComponent(stream.name)}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ set_privacy_mode: true }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setPrivacyActive(true);
      setShowPrivacyConfirm(false);
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    } catch (err) {
      showStatusMessage(`${t('live.pauseForPrivacy')}: ${err.message}`, 'error', 5000);
      setShowPrivacyConfirm(false);
    } finally {
      setIsTogglingEnabled(false);
    }
  };

  /**
   * Resume stream from privacy mode.
   */
  const handleResumeFromPrivacy = async () => {
    setIsTogglingEnabled(true);
    try {
      const res = await fetch(`/api/streams/${encodeURIComponent(stream.name)}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ set_privacy_mode: false }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setPrivacyActive(false);
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    } catch (err) {
      showStatusMessage(`${t('live.resumeStream')}: ${err.message}`, 'error', 5000);
    } finally {
      setIsTogglingEnabled(false);
    }
  };

  /**
   * Handle snapshot button click
   */
  const handleSnapshot = () => {
    if (!videoRef.current) return;

    const videoElement = videoRef.current;

    // Ensure valid video dimensions for native resolution capture
    if (!videoElement.videoWidth || !videoElement.videoHeight) {
      showStatusMessage(t('live.cannotTakeSnapshotVideoNotLoaded'), 'error');
      return;
    }

    // Create canvas at native video resolution
    const canvas = document.createElement('canvas');
    canvas.width = videoElement.videoWidth;
    canvas.height = videoElement.videoHeight;
    const ctx = canvas.getContext('2d');

    // Draw video frame at native resolution
    ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

    // Draw detections at native resolution if available (fixes boundary shift)
    if (detectionOverlayRef.current && typeof detectionOverlayRef.current.getDetections === 'function') {
      const detections = detectionOverlayRef.current.getDetections();
      if (detections && detections.length > 0) {
        drawDetectionsOnCanvas(ctx, canvas.width, canvas.height, detections);
      }
    }

    // Auto-download for rapid-fire capability (also works in fullscreen)
    const timestamp = formatFilenameTimestamp();
    const fileName = `snapshot-${stream.name.replace(/\s+/g, '-')}-${timestamp}.jpg`;

    canvas.toBlob((blob) => {
      if (!blob) {
        showStatusMessage(t('timeline.failedToCreateSnapshot'), 'error');
        return;
      }

      const blobUrl = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = blobUrl;
      link.download = fileName;
      document.body.appendChild(link);
      link.click();

      setTimeout(() => {
        if (document.body.contains(link)) {
          document.body.removeChild(link);
        }
        URL.revokeObjectURL(blobUrl);
      }, 1000);

      showStatusMessage(t('live.snapshotSaved', { fileName }), 'success', 2000);
    }, 'image/jpeg', 0.95);
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

  // Auto-retry when stream status transitions back to 'Running' while the
  // error overlay is visible (e.g. camera came back online after an outage).
  useEffect(() => {
    const prev = prevStatusRef.current;
    prevStatusRef.current = stream.status;
    if (error && stream.status === 'Running' && prev !== 'Running') {
      console.log(`[MSE ${stream.name}] Status changed to Running — auto-retrying after error`);
      autoRetryCountRef.current = 0;
      setError(null);
      setIsLoading(true);
      setRetryCount(c => c + 1);
    }
  }, [stream.status, error]);

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

  // Player telemetry (TTFF, rebuffer tracking)
  useEffect(() => {
    const video = videoRef.current;
    if (!video || !stream?.name) return;

    const telemetry = createPlayerTelemetry(stream.name, 'mse');

    const onPlaying = () => telemetry.recordFirstFrame();
    const onWaiting = () => telemetry.recordRebufferStart();
    const onCanPlay = () => telemetry.recordRebufferEnd();

    video.addEventListener('playing', onPlaying);
    video.addEventListener('waiting', onWaiting);
    video.addEventListener('canplay', onCanPlay);

    return () => {
      video.removeEventListener('playing', onPlaying);
      video.removeEventListener('waiting', onWaiting);
      video.removeEventListener('canplay', onCanPlay);
      telemetry.destroy();
    };
  }, [stream?.name, retryCount]);

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
      {stream.detection_based_recording && stream.detection_model && showDetections && (
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
            pointerEvents: 'none',
            display: 'flex',
            alignItems: 'center',
            gap: '6px'
          }}
        >
          <span
            style={{
              width: '8px',
              height: '8px',
              borderRadius: '50%',
              flexShrink: 0,
              backgroundColor:
                stream.status === 'Running'      ? 'rgba(34, 197, 94, 0.9)'  :
                stream.status === 'Starting'     ? 'rgba(234, 179, 8, 0.9)'  :
                stream.status === 'Reconnecting' ? 'rgba(234, 179, 8, 0.9)'  :
                stream.status === 'Error'        ? 'rgba(239, 68, 68, 0.9)'  :
                stream.status === 'Stopping'     ? 'rgba(234, 179, 8, 0.9)'  :
                'rgba(148, 163, 184, 0.9)'
            }}
            title={`${t('streams.streamStatus')}: ${
              stream.status === 'Running'      ? t('streams.running')      :
              stream.status === 'Starting'     ? t('streams.starting')     :
              stream.status === 'Reconnecting' ? t('streams.reconnecting') :
              stream.status === 'Error'        ? t('streams.error')        :
              stream.status === 'Stopping'     ? t('streams.stopping')     :
              stream.status === 'Stopped'      ? t('streams.stopped')      :
              (stream.status || t('common.unknown'))
            }`}
          />
          {stream.name}
        </div>
      )}

      {/* Loading indicator */}
      {isLoading && !error && (
        <div
          data-testid="stream-starting-placeholder"
          style={{ position: 'absolute', top: 0, left: 0, right: 0, bottom: 0, zIndex: 5, pointerEvents: 'none' }}
        >
          <LoadingIndicator message={t('live.streamStarting')} />
        </div>
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
          <SnapshotButton streamId={streamId} streamName={stream.name} onSnapshot={handleSnapshot} />

          {/* Pause for privacy button */}
          <button
            type="button"
            title={t('live.pauseForPrivacy')}
            onClick={() => setShowPrivacyConfirm(true)}
            style={{
              padding: '8px 12px',
              backgroundColor: 'rgba(0, 0, 0, 0.7)',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              boxShadow: '0 2px 4px rgba(0, 0, 0, 0.3)',
              transition: 'background-color 0.2s ease',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center'
            }}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M18.36 6.64A9 9 0 1 1 5.64 17.36"/>
              <line x1="12" y1="2" x2="12" y2="12"/>
            </svg>
          </button>

          {/* Detection overlay toggle button */}
          {stream.detection_based_recording && stream.detection_model && isPlaying && (
            <button
              className={`detection-toggle-btn ${showDetections ? 'active' : ''}`}
            title={showDetections ? t('live.hideDetections') : t('live.showDetections')}
              onClick={() => setLocalShowDetections(!localShowDetections)}
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
            >
              {showDetections ? (
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                  <circle cx="12" cy="12" r="3"/>
                </svg>
              ) : (
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                  <line x1="1" y1="1" x2="23" y2="23"/>
                </svg>
              )}
            </button>
          )}

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
              title={t('live.togglePtzControls')}
            >
              PTZ
            </button>
          )}

          <button
            type="button"
            className="timeline-btn"
            onClick={(event) => {
              const fromFullscreen = !!(document.fullscreenElement || document.webkitFullscreenElement);
              forceNavigation(formatUtils.getTimelineUrl(stream.name, new Date().toISOString(), fromFullscreen), event);
            }}
            style={{
              padding: '8px 12px',
              backgroundColor: 'rgba(0, 0, 0, 0.7)',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              boxShadow: '0 2px 4px rgba(0, 0, 0, 0.3)',
              transition: 'background-color 0.2s ease',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center'
            }}
            title={t('live.viewInTimeline')}
            aria-label={t('live.viewInTimeline')}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 640 640" fill="white" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M320 128C426 128 512 214 512 320C512 426 426 512 320 512C254.8 512 197.1 479.5 162.4 429.7C152.3 415.2 132.3 411.7 117.8 421.8C103.3 431.9 99.8 451.9 109.9 466.4C156.1 532.6 233 576 320 576C461.4 576 576 461.4 576 320C576 178.6 461.4 64 320 64C234.3 64 158.5 106.1 112 170.7L112 144C112 126.3 97.7 112 80 112C62.3 112 48 126.3 48 144L48 256C48 273.7 62.3 288 80 288L104.6 288C105.1 288 105.6 288 106.1 288L192.1 288C209.8 288 224.1 273.7 224.1 256C224.1 238.3 209.8 224 192.1 224L153.8 224C186.9 166.6 249 128 320 128zM344 216C344 202.7 333.3 192 320 192C306.7 192 296 202.7 296 216L296 320C296 326.4 298.5 332.5 303 337L375 409C384.4 418.4 399.6 418.4 408.9 409C418.2 399.6 418.3 384.4 408.9 375.1L343.9 310.1L343.9 216z"/>
            </svg>
          </button>

          {/* Fullscreen button */}
          <button
            className="fullscreen-btn"
            onClick={(e) => onToggleFullscreen(stream.name, e, cellRef.current)}
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
            title={t('live.toggleFullscreen')}
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

      {/* Inline pause-for-privacy confirmation overlay */}
      {showPrivacyConfirm && (
        <div style={{
          position: 'absolute', top: 0, left: 0, right: 0, bottom: 0,
          backgroundColor: 'rgba(0,0,0,0.75)', zIndex: 20,
          display: 'flex', flexDirection: 'column',
          alignItems: 'center', justifyContent: 'center', gap: '12px',
          padding: '16px', textAlign: 'center'
        }}>
          <p style={{ color: 'white', fontSize: '14px', maxWidth: '240px', lineHeight: '1.4' }}>
            {t('live.pauseForPrivacyConfirm')}
          </p>
          <div style={{ display: 'flex', gap: '8px' }}>
            <button
              onClick={handlePauseForPrivacy}
              disabled={isTogglingEnabled}
              style={{
                padding: '6px 16px', backgroundColor: '#7c3aed', color: 'white',
                border: 'none', borderRadius: '4px', cursor: 'pointer', fontWeight: 'bold', fontSize: '13px'
              }}
            >
              {t('live.pauseForPrivacy')}
            </button>
            <button
              onClick={() => setShowPrivacyConfirm(false)}
              style={{
                padding: '6px 16px', backgroundColor: 'rgba(255,255,255,0.2)', color: 'white',
                border: '1px solid rgba(255,255,255,0.4)', borderRadius: '4px', cursor: 'pointer', fontSize: '13px'
              }}
            >
              {t('common.cancel')}
            </button>
          </div>
        </div>
      )}

      {/* Privacy mode overlay */}
      {privacyActive && (
        <div style={{
          position: 'absolute', top: 0, left: 0, right: 0, bottom: 0,
          backgroundColor: 'rgba(0,0,0,0.85)', zIndex: 15,
          display: 'flex', flexDirection: 'column',
          alignItems: 'center', justifyContent: 'center', gap: '12px'
        }}>
          <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.5)" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
            <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94"/>
            <path d="M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19"/>
            <line x1="1" y1="1" x2="23" y2="23"/>
          </svg>
          <p style={{ color: 'rgba(255,255,255,0.7)', fontSize: '14px' }}>{t('live.streamPausedForPrivacy')}</p>
          <button
            onClick={handleResumeFromPrivacy}
            disabled={isTogglingEnabled}
            style={{
              padding: '6px 16px', backgroundColor: '#16a34a', color: 'white',
              border: 'none', borderRadius: '4px', cursor: 'pointer', fontWeight: 'bold', fontSize: '13px'
            }}
          >
            {t('live.resumeStream')}
          </button>
        </div>
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
          title={t('live.mseLowLatencyStreaming')}
        >
          MSE
        </div>
      )}
    </div>
  );
}

