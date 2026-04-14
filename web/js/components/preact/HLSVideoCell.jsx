/**
 * HLSVideoCell Component
 * A self-contained component for displaying an HLS video stream
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { DetectionOverlay, drawDetectionsOnCanvas } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showStatusMessage } from './ToastContainer.jsx';
import { PTZControls } from './PTZControls.jsx';
import { ConfirmDialog } from './UI.jsx';
import { getGo2rtcBaseUrl, isGo2rtcAvailable, isGo2rtcEnabled, isForceNativeHls } from '../../utils/settings-utils.js';
import { formatFilenameTimestamp } from '../../utils/date-utils.js';
import { forceNavigation } from '../../utils/navigation-utils.js';
import { formatUtils } from './recordings/formatUtils.js';
import { useI18n } from '../../i18n.js';
import { useQueryClient } from '../../query-client.js';
import { createPlayerTelemetry } from '../../utils/player-telemetry.js';
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
  const [showRefreshConfirm, setShowRefreshConfirm] = useState(false);

  // Privacy mode state
  // showPrivacyConfirm: whether the "pause for privacy" confirmation overlay is visible
  // privacyActive: mirrors stream.privacy_mode (initialised from server, updated on toggle)
  // isTogglingEnabled: true while API call in flight
  const [showPrivacyConfirm, setShowPrivacyConfirm] = useState(false);
  const [privacyActive, setPrivacyActive] = useState(!!stream.privacy_mode);
  const [isTogglingEnabled, setIsTogglingEnabled] = useState(false);

  // HLS source state: 'go2rtc' (go2rtc's dynamic HLS), 'native' (lightNVR FFmpeg-based HLS), or 'failed'
  // Default to native lightNVR HLS (reliable, always running when streaming enabled)
  // go2rtc mode is used only when the backend reports go2rtc is available for this stream
  const [hlsMode, setHlsMode] = useState(() => {
    return stream && stream.go2rtc_hls_available ? 'go2rtc' : 'native';
  });

  // PTZ controls state
  const [showPTZControls, setShowPTZControls] = useState(false);

  // Detection overlay visibility state (per-camera toggle, constrained by global toggle)
  const [localShowDetections, setLocalShowDetections] = useState(true);
  const showDetections = globalShowDetections && localShowDetections;

  // Refs
  const videoRef = useRef(null);
  const cellRef = useRef(null);
  const hlsPlayerRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const fatalErrorCountRef = useRef(0);  // Track consecutive fatal error recovery attempts
  const recoveringRef = useRef(false);   // True when we're in the middle of error recovery (prevents counter reset)
  const hlsUrlRef = useRef(null);        // Master manifest URL — stored for session-expiry recovery
  const prevStatusRef = useRef(stream.status); // Track previous stream status for transition detection

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

    // Effective stream name for go2rtc source — use sub-stream in grid view
    const effectiveName = useSubStream ? `${stream.name}_sub` : stream.name;

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

      // Check if force native HLS is enabled in settings.
      // When enabled, always use lightNVR's native FFmpeg-based HLS regardless of go2rtc state.
      const forceNative = await isForceNativeHls();
      if (!isMounted) return;

      if (forceNative) {
        hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8`;
        usingGo2rtc = false;
        setHlsMode('native');
        console.log(`[HLS ${stream.name}] Force native HLS enabled, using: ${hlsStreamUrl}`);
      } else {
      // Check if go2rtc is enabled in runtime settings.
      // When enabled, we NEVER fall back to ffmpeg HLS - it's go2rtc or nothing.
      const go2rtcEnabled = await isGo2rtcEnabled();
      if (!isMounted) return;

      // If go2rtc is enabled in settings, always use go2rtc mode regardless of initial hlsMode.
      // This handles the case where stream.go2rtc_hls_available was false (e.g. when toggling
      // from MSE to HLS) but go2rtc is actually running and available.
      let effectiveMode = hlsMode;
      if (go2rtcEnabled && effectiveMode === 'native') {
        effectiveMode = 'go2rtc';
        setHlsMode('go2rtc');
      }

      // Determine which HLS source to use based on effective mode
      if (effectiveMode === 'go2rtc') {
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
            console.warn('Failed to get go2rtc URL from settings, using origin proxy:', err);
            go2rtcBaseUrl = `${window.location.origin}/go2rtc`;
          }

          if (!isMounted) return;

          // Build the HLS stream URL using go2rtc's dynamic HLS endpoint
          // Using &mp4=flac for best codec compatibility (H264/H265 + AAC/PCMA/PCMU/PCM)
          hlsStreamUrl = `${go2rtcBaseUrl}/api/stream.m3u8?src=${encodeURIComponent(effectiveName)}&mp4=flac`;
          usingGo2rtc = true;
          console.log(`[HLS ${stream.name}] Using go2rtc HLS: ${hlsStreamUrl}`);
          console.log(`[HLS ${stream.name}] go2rtc base URL: ${go2rtcBaseUrl}`);

          if (!isMounted) return;
        } else if (go2rtcEnabled) {
          // go2rtc is enabled but not responding - do NOT fall back to ffmpeg HLS
          console.error(`[HLS ${stream.name}] go2rtc is enabled but not responding - no fallback to ffmpeg HLS`);
          setError(t('live.go2rtcNotRespondingRetry'));
          setIsLoading(false);
          return;
        } else {
          // go2rtc is not enabled - use native lightNVR HLS
          console.warn(`[HLS ${stream.name}] go2rtc is not available, using native lightNVR HLS`);
          hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8`;
          usingGo2rtc = false;
          setHlsMode('native');
          console.log(`[HLS ${stream.name}] Using native lightNVR HLS: ${hlsStreamUrl}`);
        }
      } else if (effectiveMode === 'native') {
        // Use lightNVR's FFmpeg-based HLS endpoint directly
        hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8`;
        usingGo2rtc = false;
        console.log(`[HLS ${stream.name}] Using native lightNVR HLS: ${hlsStreamUrl}`);
      } else {
        // Mode is 'failed' - don't attempt anything
        console.error(`[HLS ${stream.name}] All HLS modes have failed`);
        setError(t('live.hlsStreamingUnavailableBoth'));
        setIsLoading(false);
        return;
      }
      } // end of !forceNative else block

      // Store master manifest URL so network-error recovery can reload from it
      // (go2rtc sessions expire after 5 s of inactivity; reloading the master
      // manifest creates a fresh session instead of retrying the stale one).
      hlsUrlRef.current = hlsStreamUrl;

      // Check if HLS.js is supported
      if (Hls.isSupported()) {
        console.log(`Using HLS.js for stream ${stream.name} (mode: ${usingGo2rtc ? 'go2rtc' : 'native'})`);

        // Shared low-memory budget: keep only a small forward buffer and zero back-buffer.
        // backBufferLength: 0 is the single biggest win — HLS.js default keeps decoded
        // frames forever for seeking, which multiplies by stream count and crashes the tab.
        const sharedMemoryConfig = {
          maxBufferLength: 4,        // seconds of forward buffer (default: 30)
          maxMaxBufferLength: 8,     // hard ceiling (default: 600)
          backBufferLength: 0,       // discard played frames immediately (default: Infinity)
          enableWorker: true,
          lowLatencyMode: false,
          startLevel: 0,
          debug: false,
        };

        // go2rtc mode: low-latency live source, keep latency tight
        // Native mode: FFmpeg file-based segments, slightly more tolerant retry settings
        const hlsConfig = usingGo2rtc ? {
          ...sharedMemoryConfig,
          liveSyncDurationCount: 2,
          liveMaxLatencyDurationCount: 6,
          liveDurationInfinity: true,
        } : {
          ...sharedMemoryConfig,
          liveSyncDurationCount: 3,
          liveMaxLatencyDurationCount: 10,
          liveDurationInfinity: true,
          fragLoadingTimeOut: 20000,
          manifestLoadingTimeOut: 15000,
          manifestLoadingMaxRetry: 10,
          manifestLoadingRetryDelay: 1000,
          fragLoadingMaxRetry: 10,
          fragLoadingRetryDelay: 500,
        };

        const hls = new Hls(hlsConfig);

        // Store hls instance IMMEDIATELY after creation for cleanup
        hlsPlayerRef.current = hls;

        hls.loadSource(hlsStreamUrl);
        hls.attachMedia(videoRef.current);

        videoRef.current.ondblclick = (e) => onToggleFullscreen(stream.name, e, cellRef.current);

        hls.on(Hls.Events.MANIFEST_PARSED, function() {
          if (!isMounted) return;
          if (!recoveringRef.current) {
            fatalErrorCountRef.current = 0;
          }
          setIsLoading(false);
          setIsPlaying(true);

          if (videoRef.current) {
            videoRef.current.play().catch(error => {
              console.warn('Auto-play prevented:', error);
            });
          }
        });

        // Reset fatal error counter after successful fragment buffering (real recovery signal)
        hls.on(Hls.Events.FRAG_BUFFERED, function() {
          if (!isMounted) return;
          if (recoveringRef.current) {
            console.log(`[HLS ${stream.name}] Recovery successful - fragment buffered`);
            recoveringRef.current = false;
            fatalErrorCountRef.current = 0;
          }
        });

        hls.on(Hls.Events.ERROR, function(event, data) {
          if (!isMounted) return;

          // Non-fatal errors: HLS.js handles these automatically, don't intervene
          if (!data.fatal) {
            return;
          }

          // Fatal errors require intervention
          console.error(`[HLS ${stream.name}] Fatal error: ${data.type}, details: ${data.details}`);

          const MAX_RECOVERY = usingGo2rtc ? 3 : 8;
          fatalErrorCountRef.current++;
          const attemptNum = fatalErrorCountRef.current;

          if (attemptNum > MAX_RECOVERY) {
            // Exhausted recovery attempts - give up
            console.error(`[HLS ${stream.name}] Exhausted ${MAX_RECOVERY} recovery attempts`);
            recoveringRef.current = false;
            if (hlsPlayerRef.current) {
              hlsPlayerRef.current.destroy();
              hlsPlayerRef.current = null;
            }
            if (isMounted) {
              if (usingGo2rtc && !go2rtcEnabled) {
                // go2rtc was auto-detected but not force-enabled, fall back to native
                console.warn(`[HLS ${stream.name}] Falling back to native HLS`);
                fatalErrorCountRef.current = 0;
                setHlsMode('native');
              } else {
                setError(usingGo2rtc
                  ? t('live.go2rtcHlsFailedRetry')
                  : (data.details || t('live.hlsStreamUnavailable')));
                setIsLoading(false);
                setIsPlaying(false);
              }
            }
            return;
          }

          // Attempt recovery based on error type
          recoveringRef.current = true;
          const delay = usingGo2rtc ? 1000 : 2000;

          if (data.type === Hls.ErrorTypes.NETWORK_ERROR) {
            console.warn(`[HLS ${stream.name}] Network error recovery attempt ${attemptNum}/${MAX_RECOVERY}`);
            setTimeout(() => {
              if (isMounted && hlsPlayerRef.current) {
                if (usingGo2rtc && hlsUrlRef.current) {
                  // go2rtc sessions expire after 5 s of inactivity. Simply
                  // calling startLoad() retries the same (now-expired) session
                  // ID and keeps 404-ing. Re-fetching the master manifest via
                  // loadSource() creates a brand-new go2rtc session.
                  console.log(`[HLS ${stream.name}] Reloading master manifest to create fresh go2rtc session`);
                  hlsPlayerRef.current.loadSource(hlsUrlRef.current);
                  hlsPlayerRef.current.startLoad();
                } else {
                  hlsPlayerRef.current.startLoad();
                }
              }
            }, delay);
          } else if (data.type === Hls.ErrorTypes.MEDIA_ERROR) {
            console.warn(`[HLS ${stream.name}] Media error recovery attempt ${attemptNum}/${MAX_RECOVERY}`);
            setTimeout(() => {
              if (isMounted && hlsPlayerRef.current) {
                hlsPlayerRef.current.recoverMediaError();
              }
            }, delay);
          } else {
            // Unrecoverable error type - destroy immediately
            console.error(`[HLS ${stream.name}] Unrecoverable error type`);
            recoveringRef.current = false;
            if (hlsPlayerRef.current) {
              hlsPlayerRef.current.destroy();
              hlsPlayerRef.current = null;
            }
            if (isMounted) {
              if (usingGo2rtc && !go2rtcEnabled) {
                fatalErrorCountRef.current = 0;
                setHlsMode('native');
              } else {
                setError(data.details || t('live.hlsPlaybackError'));
                setIsLoading(false);
                setIsPlaying(false);
              }
            }
          }
        });
      }
      // Check if HLS is supported natively (Safari)
      else if (videoRef.current.canPlayType('application/vnd.apple.mpegurl')) {
        console.log(`Using native HLS support for stream ${stream.name}`);
        // Native HLS support (Safari)
        videoRef.current.src = hlsStreamUrl;

        videoRef.current.ondblclick = (e) => onToggleFullscreen(stream.name, e, cellRef.current);

        // Store handlers for cleanup
        nativeLoadedHandler = function() {
          if (!isMounted) return;
          setIsLoading(false);
          setIsPlaying(true);
        };

        nativeErrorHandler = function() {
          if (!isMounted) return;
          if (usingGo2rtc) {
            if (go2rtcEnabled) {
              console.error(`[HLS ${stream.name}] go2rtc HLS failed (native player) - no fallback (go2rtc enabled)`);
              setError(t('live.go2rtcHlsFailedRetry'));
              setIsLoading(false);
              setIsPlaying(false);
            } else {
              console.warn(`[HLS ${stream.name}] go2rtc HLS failed (native player), falling back to native lightNVR HLS`);
              setHlsMode('native');
            }
          } else {
            setError(t('live.hlsStreamFailedToLoad'));
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
          setError(t('live.hlsNotSupportedModernBrowser'));
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
      recoveringRef.current = false;
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
  }, [stream, retryCount, initDelay, hlsMode, t]);

  // Auto-retry when stream status transitions back to 'Running' while the
  // error overlay is visible (e.g. camera came back online after an outage).
  useEffect(() => {
    const prev = prevStatusRef.current;
    prevStatusRef.current = stream.status;
    if (error && stream.status === 'Running' && prev !== 'Running') {
      console.log(`[HLS ${stream.name}] Status changed to Running — auto-retrying after error`);
      fatalErrorCountRef.current = 0;
      recoveringRef.current = false;
      setError(null);
      setIsLoading(true);
      setIsPlaying(false);
      setRetryCount(c => c + 1);
    }
  }, [stream.status, error]);

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
    fatalErrorCountRef.current = 0;  // Reset fatal error counter on manual retry
    recoveringRef.current = false;
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

  /**
   * Pause stream for privacy — sets privacy_mode=true without touching the enabled flag.
   * The stream stays in the Live View grid under a privacy overlay.
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
   * Resume stream from privacy mode — sets privacy_mode=false and re-starts the stream.
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

  // Player telemetry (TTFF, rebuffer tracking)
  useEffect(() => {
    const video = videoRef.current;
    if (!video || !stream?.name) return;

    const telemetry = createPlayerTelemetry(stream.name, 'hls');

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
      {stream.detection_based_recording && stream.detection_model && showDetections && (
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
            }}
          />
        </div>
        {/* Pause for privacy button */}
        <button
          type="button"
          title={t('live.pauseForPrivacy')}
          onClick={() => setShowPrivacyConfirm(true)}
          style={{
            backgroundColor: 'transparent',
            border: 'none',
            padding: '5px',
            borderRadius: '4px',
            color: 'white',
            cursor: 'pointer',
            transition: 'background-color 0.2s ease',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center'
          }}
          onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)'}
          onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
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
            {showDetections ? (
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                <circle cx="12" cy="12" r="3"/>
              </svg>
            ) : (
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                <line x1="1" y1="1" x2="23" y2="23"/>
              </svg>
            )}
          </button>
        )}
        {/* PTZ control toggle button */}
        {stream.ptz_enabled && isPlaying && (
          <button
            className={`ptz-toggle-btn ${showPTZControls ? 'active' : ''}`}
            title={showPTZControls ? t('live.hidePtzControls') : t('live.showPtzControls')}
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
            title={t('live.forceRefreshStream')}
            onClick={() => stream?.record ? setShowRefreshConfirm(true) : handleRetry()}
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
          type="button"
          className="timeline-btn"
          title={t('live.viewInTimeline')}
          aria-label={t('live.viewInTimeline')}
          onClick={(event) => {
            const fromFullscreen = !!(document.fullscreenElement || document.webkitFullscreenElement);
            forceNavigation(formatUtils.getTimelineUrl(stream.name, new Date().toISOString(), fromFullscreen), event);
          }}
          style={{
            backgroundColor: 'transparent',
            border: 'none',
            padding: '5px',
            borderRadius: '4px',
            color: 'white',
            cursor: 'pointer',
            transition: 'background-color 0.2s ease'
          }}
          onMouseOver={(event) => (event.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)')}
          onMouseOut={(event) => (event.currentTarget.style.backgroundColor = 'transparent')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 640 640" fill="white" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M320 128C426 128 512 214 512 320C512 426 426 512 320 512C254.8 512 197.1 479.5 162.4 429.7C152.3 415.2 132.3 411.7 117.8 421.8C103.3 431.9 99.8 451.9 109.9 466.4C156.1 532.6 233 576 320 576C461.4 576 576 461.4 576 320C576 178.6 461.4 64 320 64C234.3 64 158.5 106.1 112 170.7L112 144C112 126.3 97.7 112 80 112C62.3 112 48 126.3 48 144L48 256C48 273.7 62.3 288 80 288L104.6 288C105.1 288 105.6 288 106.1 288L192.1 288C209.8 288 224.1 273.7 224.1 256C224.1 238.3 209.8 224 192.1 224L153.8 224C186.9 166.6 249 128 320 128zM344 216C344 202.7 333.3 192 320 192C306.7 192 296 202.7 296 216L296 320C296 326.4 298.5 332.5 303 337L375 409C384.4 418.4 399.6 418.4 408.9 409C418.2 399.6 418.3 384.4 408.9 375.1L343.9 310.1L343.9 216z"/>
          </svg>
        </button>
        <button
          className="fullscreen-btn"
          title={t('live.toggleFullscreen')}
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
        <div
          data-testid="stream-starting-placeholder"
          style={{ position: 'absolute', top: 0, left: 0, right: 0, bottom: 0, zIndex: 5, pointerEvents: 'none' }}
        >
          <LoadingIndicator message={t('live.streamStarting')} />
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
            textAlign: 'center',
            transform: 'none'
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
              {t('common.retry')}
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
      <ConfirmDialog
        isOpen={showRefreshConfirm}
        onClose={() => setShowRefreshConfirm(false)}
        onConfirm={handleRetry}
        title={t('live.forceRefreshStream')}
        message={t('live.forceRefreshWarning')}
        confirmLabel={t('common.refresh')}
      />

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

      {/* Privacy mode overlay — shown when the stream is paused for privacy */}
      {privacyActive && (
        <div style={{
          position: 'absolute', top: 0, left: 0, right: 0, bottom: 0,
          backgroundColor: 'rgba(0,0,0,0.85)', zIndex: 15,
          display: 'flex', flexDirection: 'column',
          alignItems: 'center', justifyContent: 'center', gap: '12px'
        }}>
          {/* Eye-with-slash privacy icon */}
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
    </div>
  );
}
