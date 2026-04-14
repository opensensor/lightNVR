/**
 * WebRTCVideoCell Component
 * A self-contained component for displaying a WebRTC video stream
 * with optional two-way audio (backchannel) support
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { DetectionOverlay, drawDetectionsOnCanvas } from './DetectionOverlay.jsx';
import { SnapshotButton } from './SnapshotManager.jsx';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { showStatusMessage } from './ToastContainer.jsx';
import { PTZControls } from './PTZControls.jsx';
import { ConfirmDialog } from './UI.jsx';
import { getGo2rtcBaseUrl } from '../../utils/settings-utils.js';
import { formatFilenameTimestamp } from '../../utils/date-utils.js';
import { forceNavigation } from '../../utils/navigation-utils.js';
import { formatUtils } from './recordings/formatUtils.js';
import { useI18n } from '../../i18n.js';
import { useQueryClient } from '../../query-client.js';
import { createPlayerTelemetry } from '../../utils/player-telemetry.js';
import 'webrtc-adapter';

// Retry configuration for sending WebRTC offers to go2rtc.
// Adjust these values to tune reliability vs. latency.
// Configuration for detecting lack of incoming video data.
// MAX_VIDEO_DATA_CHECKS × VIDEO_DATA_CHECK_INTERVAL_MS defines the total
// time we will wait for video frames before surfacing an error.
const MAX_VIDEO_DATA_CHECKS = 6; // 6 checks × 15,000 ms (15s) interval = 90s total
const VIDEO_DATA_CHECK_INTERVAL_MS = 15000; // 15 seconds between checks
const MIN_NO_DATA_CHECKS_BEFORE_RETRY = 2;
const MAX_NO_DATA_RECONNECT_ATTEMPTS = 3;
const MAX_OFFER_RETRIES = 4;
const BASE_RETRY_DELAY_MS = 1500; // base delay for exponential backoff: 1.5s, 3s, 6s, 12s, 24s, ...

// Connection quality classification thresholds
// Packet loss values are percentages (0-100), RTT and jitter are in seconds.
const CONNECTION_QUALITY_THRESHOLDS = {
  good: {
    maxLossPercent: 2,
    maxRttSeconds: 0.1,
    maxJitterSeconds: 0.03
  },
  fair: {
    maxLossPercent: 5,
    maxRttSeconds: 0.3,
    maxJitterSeconds: 0.1
  },
  poor: {
    maxLossPercent: 15,
    maxRttSeconds: 1
  }
};

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
  const [isLoading, setIsLoading] = useState(() => {
    // Derive initial loading state from the incoming stream, so that we
    // avoid flashing a loading indicator for streams that are already
    // playing/connected or known to be in an error state.
    if (!stream) {
      return true;
    }

    const status = stream.status || stream.state;
    if (status === 'error' || status === 'failed') {
      return false;
    }
    if (status === 'playing' || status === 'connected' || status === 'ready') {
      return false;
    }

    return true;
  });
  const [error, setError] = useState(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [connectionQuality, setConnectionQuality] = useState('unknown'); // 'unknown', 'good', 'fair', 'poor', 'bad'
  const [retryCount, setRetryCount] = useState(0); // Used to trigger WebRTC re-initialization
  const [showRefreshConfirm, setShowRefreshConfirm] = useState(false);

  // Backchannel (two-way audio) state
  const [isTalking, setIsTalking] = useState(false);
  const [microphoneError, setMicrophoneError] = useState(null);
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
  const peerConnectionRef = useRef(null);
  const detectionOverlayRef = useRef(null);
  const abortControllerRef = useRef(null);
  const connectionMonitorRef = useRef(null);
  const reconnectAttemptsRef = useRef(0);
  const connectionRefreshRequestedRef = useRef(false);  // Track if we've already requested a refresh for this connection attempt
  const noDataReconnectAttemptsRef = useRef(0); // Separate counter for ICE-connected-but-no-data retries
  const localStreamRef = useRef(null);
  const audioSenderRef = useRef(null);
  const audioContextRef = useRef(null);
  const analyserRef = useRef(null);
  const audioLevelIntervalRef = useRef(null);
  const disconnectRecoveryTimeoutRef = useRef(null);

  // Initialize WebRTC connection when component mounts
  useEffect(() => {
    if (!stream || !stream.name || !videoRef.current) return;

    console.log(`Initializing WebRTC connection for stream ${stream.name}`);
    setIsLoading(true);
    setError(null);

    // Reset the refresh flag for this new connection attempt
    connectionRefreshRequestedRef.current = false;

    // Store cleanup functions
    let connectionTimeout = null;
    let videoDataTimeout = null;
    let initDelayTimeout = null;
    let go2rtcBaseUrl = null;

    // Async function to initialize WebRTC
    const initWebRTC = async () => {
      // Get the go2rtc base URL from settings
      try {
        go2rtcBaseUrl = await getGo2rtcBaseUrl();
        console.log(`Using go2rtc base URL: ${go2rtcBaseUrl}`);
      } catch (err) {
        console.warn('Failed to get go2rtc URL from settings, using default:', err);
        go2rtcBaseUrl = `${window.location.origin}/go2rtc`;
      }

      // Fetch ICE server configuration from API (includes TURN if configured)
      let iceServers = [{ urls: 'stun:stun.l.google.com:19302' }];
      try {
        const iceResponse = await fetch('/api/ice-servers');
        if (iceResponse.ok) {
          const iceConfig = await iceResponse.json();
          if (iceConfig.ice_servers && iceConfig.ice_servers.length > 0) {
            iceServers = iceConfig.ice_servers;
            console.log(`Using ${iceServers.length} ICE servers from config`);
          }
        }
      } catch (err) {
        console.warn('Failed to fetch ICE servers config, using defaults:', err);
      }

      // Create a new RTCPeerConnection
      const pc = new RTCPeerConnection({
        iceTransportPolicy: 'all',
        bundlePolicy: 'balanced',
        rtcpMuxPolicy: 'require',
        iceCandidatePoolSize: 0,
        iceServers,
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
              if (playRetryTimeout !== null) {
                clearTimeout(playRetryTimeout);
                playRetryTimeout = null;
              }
            })
            .catch(err => {
              // AbortError is expected when srcObject changes or another play() is called
              // Don't treat it as a fatal error, just log and potentially retry
              if (err.name === 'AbortError') {
                console.log(`Video play() was interrupted for stream ${stream.name}, will retry if needed`);
                playRetryCount++;
                if (playRetryCount < maxPlayRetries) {
                  // Retry after a short delay
                  if (playRetryTimeout !== null) {
                    clearTimeout(playRetryTimeout);
                  }
                  playRetryTimeout = setTimeout(attemptPlay, 500);
                }
              } else if (err.name === 'NotAllowedError') {
                console.warn(`Autoplay blocked for stream ${stream.name}, user interaction required`);
                setError(t('live.webrtcAutoplayBlocked'));
              } else {
                console.error(`Video play() failed for stream ${stream.name}:`, err);
              }
            });
        };

        // Set a timeout to detect if no video data is received.
        // When ICE is connected but no video frames have arrived, go2rtc is
        // likely still establishing the RTSP connection to the camera.  We
        // keep the loading indicator up and periodically re-check instead of
        // immediately showing an error.
        if (videoDataTimeout) {
          clearTimeout(videoDataTimeout);
        }
        let videoDataCheckCount = 0;
        const maxVideoDataChecks = MAX_VIDEO_DATA_CHECKS;
        const videoDataCheckInterval = VIDEO_DATA_CHECK_INTERVAL_MS;

        const scheduleVideoDataCheck = () => {
          videoDataTimeout = setTimeout(() => {
            videoDataCheckCount++;

            // Video is playing or has dimensions — nothing to do
            if (videoElement && videoElement.videoWidth > 0 && !videoElement.paused) {
              return;
            }

            // Determine current ICE state (may be null if pc was cleaned up)
            const iceState = peerConnectionRef.current
              ? peerConnectionRef.current.iceConnectionState
              : 'closed';

            console.warn(
              `No video data for stream ${stream.name} after ${videoDataCheckCount * videoDataCheckInterval / 1000}s ` +
              `(check ${videoDataCheckCount}/${maxVideoDataChecks}, ICE: ${iceState}, ` +
              `readyState: ${videoElement ? videoElement.readyState : 'N/A'})`
            );

            // ICE itself failed or closed — show error immediately
            if (iceState === 'failed' || iceState === 'closed') {
              setError(t('live.webrtcConnectionLostRetry'));
              setIsLoading(false);
              return;
            }

            // ICE is still connected/checking — camera stream may still be
            // coming up.  Keep loading state and schedule another check unless
            // we've exceeded the maximum wait time.
            if (videoDataCheckCount < maxVideoDataChecks) {
              console.log(`ICE still ${iceState} for stream ${stream.name}, waiting for camera stream...`);
              // Re-attempt play() in case the element got stuck. Failures here are non-fatal;
              // the surrounding retry logic will handle recovery.
              if (videoElement && videoElement.paused) {
                videoElement.play().catch(err => {
                  console.debug(`Non-fatal error calling play() for stream ${stream.name}:`, err);
                });
              }

              // After 30 s with ICE connected but no video data, auto-retry
              // the entire WebRTC connection.  This recovers from go2rtc RTSP
              // source issues (camera offline, stale state, etc.) without
              // requiring the user to click Retry manually.  Mirrors the
              // existing ICE-failure auto-retry at oniceconnectionstatechange.
              // Uses a dedicated counter (noDataReconnectAttemptsRef) so that
              // the ICE-connected reset of reconnectAttemptsRef doesn't
              // inadvertently allow infinite no-data retries.
              if (
                videoDataCheckCount >= MIN_NO_DATA_CHECKS_BEFORE_RETRY &&
                (iceState === 'connected' || iceState === 'completed') &&
                !connectionRefreshRequestedRef.current &&
                noDataReconnectAttemptsRef.current < MAX_NO_DATA_RECONNECT_ATTEMPTS
              ) {
                connectionRefreshRequestedRef.current = true;
                noDataReconnectAttemptsRef.current++;
                console.log(
                  `Auto-reconnecting stream ${stream.name}: ICE connected but no video data ` +
                  `after ${videoDataCheckCount * videoDataCheckInterval / 1000}s ` +
                  `(attempt ${noDataReconnectAttemptsRef.current}/${MAX_NO_DATA_RECONNECT_ATTEMPTS})`
                );
                (async () => {
                  try {
                    await refreshStreamRegistration();
                    // Give go2rtc time to fully re-register the RTSP source
                    await new Promise(resolve => setTimeout(resolve, 2000));
                  } catch (err) {
                    console.error(`Error refreshing stream ${stream.name} during auto-reconnect:`, err);
                  }
                  setRetryCount(prev => prev + 1);
                })();
                return; // Stop check chain; the retry useEffect run starts fresh
              }

              scheduleVideoDataCheck();
            } else {
              // Exceeded 90 seconds — give up and show error
              console.error(`Stream ${stream.name} connected but no video data after ${maxVideoDataChecks * videoDataCheckInterval / 1000}s`);
              setError(t('live.streamConnectedNoVideoDataRetry'));
              setIsLoading(false);
            }
          }, videoDataCheckInterval);
        };

        scheduleVideoDataCheck();

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
          // Video is genuinely playing — reset the no-data retry counter so
          // future interruptions get a fresh set of auto-reconnect attempts.
          noDataReconnectAttemptsRef.current = 0;
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
          setError(t('live.failedToLoadVideo'));
          setIsLoading(false);
          // Clear retry timeout on error
          if (playRetryTimeout) {
            clearTimeout(playRetryTimeout);
            playRetryTimeout = null;
          }
        };

        videoElement.ondblclick = (e) => onToggleFullscreen(stream.name, e, cellRef.current);

        // Start initial playback attempt
        attemptPlay();
      } else if (event.track.kind === 'audio') {
        console.log(`Audio track received for stream ${stream.name}`);
      }
    };

    const handleIceConnectionStateChange = () => {
      console.log(`ICE connection state for stream ${stream.name}: ${pc.iceConnectionState}`);

      if (pc.iceConnectionState === 'connected' || pc.iceConnectionState === 'completed') {
        // Connection is established or completed — start monitoring and reset counters
        // Clear any pending disconnect-recovery timeout since we've recovered
        if (disconnectRecoveryTimeoutRef.current) {
          clearTimeout(disconnectRecoveryTimeoutRef.current);
          disconnectRecoveryTimeoutRef.current = null;
        }
        startConnectionMonitoring();
        reconnectAttemptsRef.current = 0;
        if (error) {
          console.log(`WebRTC connection restored for stream ${stream.name}`);
          setError(null);
        }
      } else if (pc.iceConnectionState === 'failed') {
        console.error(`WebRTC ICE connection failed for stream ${stream.name}`);

        // Stop connection monitoring
        if (connectionMonitorRef.current) {
          clearInterval(connectionMonitorRef.current);
          connectionMonitorRef.current = null;
        }

        // Auto-refresh and retry if we haven't already for this connection attempt
        if (!connectionRefreshRequestedRef.current && reconnectAttemptsRef.current < 3) {
          connectionRefreshRequestedRef.current = true;
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
                // Delay to allow go2rtc to fully process the refresh (longer for slow devices)
                await new Promise(resolve => setTimeout(resolve, 3000));
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
            setError(t('live.webrtcIceConnectionFailed'));
            setIsLoading(false);
          })();
        } else {
          setError(t('live.webrtcIceConnectionFailed'));
          setIsLoading(false);
        }
      } else if (pc.iceConnectionState === 'disconnected') {
        // Connection is temporarily disconnected, log but don't show error yet
        console.warn(`WebRTC ICE connection disconnected for stream ${stream.name}, attempting to recover...`);

        // Stop connection monitoring while disconnected
        if (connectionMonitorRef.current) {
          clearInterval(connectionMonitorRef.current);
          connectionMonitorRef.current = null;
        }

        // Clear any existing disconnect-recovery timeout before scheduling a new one
        if (disconnectRecoveryTimeoutRef.current) {
          clearTimeout(disconnectRecoveryTimeoutRef.current);
          disconnectRecoveryTimeoutRef.current = null;
        }

        // Set a timeout to check if the connection recovers on its own
        disconnectRecoveryTimeoutRef.current = setTimeout(() => {
          // Clear the ref since this timeout is now firing
          disconnectRecoveryTimeoutRef.current = null;
          if (peerConnectionRef.current &&
              (peerConnectionRef.current.iceConnectionState === 'disconnected' ||
               peerConnectionRef.current.iceConnectionState === 'failed')) {
            console.error(`WebRTC ICE connection could not recover for stream ${stream.name}`);
            setError(t('live.webrtcConnectionLostPleaseRetry'));
            setIsLoading(false);
          } else if (peerConnectionRef.current) {
            console.log(`WebRTC ICE connection recovered for stream ${stream.name}, current state: ${peerConnectionRef.current.iceConnectionState}`);
          }
        }, 5000); // Wait 5 seconds to see if connection recovers
      } else if (pc.iceConnectionState === 'closed') {
        // Stop monitoring when closed
        if (connectionMonitorRef.current) {
          clearInterval(connectionMonitorRef.current);
          connectionMonitorRef.current = null;
        }
      }
    };

    pc.oniceconnectionstatechange = handleIceConnectionStateChange;

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
        setError(t('live.connectionTimeoutCheckNetwork'));
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

        // Send the offer directly to go2rtc with retry logic for 404 responses.
        // On slower devices, go2rtc may not have the stream ready yet when we
        // first try to send the offer, resulting in a 404. We retry with
        // exponential backoff to give it time.
        const maxOfferRetries = MAX_OFFER_RETRIES;
        const baseRetryDelayMs = BASE_RETRY_DELAY_MS;

        const effectiveName = useSubStream ? `${stream.name}_sub` : stream.name;
        const sendOfferWithRetry = async (attempt) => {
          const response = await fetch(`${go2rtcBaseUrl}/api/webrtc?src=${encodeURIComponent(effectiveName)}`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/sdp',
            },
            body: pc.localDescription.sdp,
          });

          const bodyText = await response.text().catch(() => '');

          if (!response.ok) {
            // Retry on 404 (stream not ready) or 500 (server overloaded / race condition) with exponential backoff
            if ((response.status === 404 || response.status === 500) && attempt < maxOfferRetries) {
              const delay = baseRetryDelayMs * Math.pow(2, attempt);
              console.warn(`go2rtc returned ${response.status} for stream ${stream.name} (attempt ${attempt + 1}/${maxOfferRetries + 1}), retrying in ${delay}ms...`);
              await new Promise(resolve => setTimeout(resolve, delay));
              return sendOfferWithRetry(attempt + 1);
            }
            console.error(`go2rtc /api/webrtc error for stream ${stream.name}: status=${response.status}, body="${bodyText}"`);
            // Check whether the go2rtc error body indicates the camera source is
            // unreachable (e.g. "dial tcp <ip>:554: connect: no route to host").
            // These patterns come from go2rtc's streams/add_consumer pipeline and
            // are much more actionable than a generic "500 Internal Server Error".
            const isSourceUnreachable = /dial tcp|no route to host|connection refused|connect:/i.test(bodyText);
            if (isSourceUnreachable) {
              throw new Error(t('live.cannotConnectToSource'));
            }
            throw new Error(`Failed to send offer: ${response.status} ${response.statusText}`);
          }
          return bodyText;
        };

        return sendOfferWithRetry(0);
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
        setError(error.message || t('live.failedToEstablishWebrtcConnection'));
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
            if (
              lossPercentage < CONNECTION_QUALITY_THRESHOLDS.good.maxLossPercent &&
              currentRtt < CONNECTION_QUALITY_THRESHOLDS.good.maxRttSeconds &&
              jitter < CONNECTION_QUALITY_THRESHOLDS.good.maxJitterSeconds
            ) {
              quality = 'good';
            } else if (
              lossPercentage < CONNECTION_QUALITY_THRESHOLDS.fair.maxLossPercent &&
              currentRtt < CONNECTION_QUALITY_THRESHOLDS.fair.maxRttSeconds &&
              jitter < CONNECTION_QUALITY_THRESHOLDS.fair.maxJitterSeconds
            ) {
              quality = 'fair';
            } else if (
              lossPercentage < CONNECTION_QUALITY_THRESHOLDS.poor.maxLossPercent &&
              currentRtt < CONNECTION_QUALITY_THRESHOLDS.poor.maxRttSeconds
            ) {
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
    
    }; // End of initWebRTC async function

    // Stagger initialization to avoid overwhelming go2rtc with concurrent offers
    if (initDelay > 0) {
      initDelayTimeout = setTimeout(() => {
        initDelayTimeout = null;
        initWebRTC();
      }, initDelay);
    } else {
      initWebRTC();
    }

    // Cleanup function
    const cleanupWebRTCResources = () => {
      console.log(`Cleaning up WebRTC connection for stream ${stream.name}`);

      // Clear timeouts
      if (initDelayTimeout) {
        clearTimeout(initDelayTimeout);
        initDelayTimeout = null;
      }
      if (connectionTimeout) {
        clearTimeout(connectionTimeout);
        connectionTimeout = null;
      }
      if (videoDataTimeout) {
        clearTimeout(videoDataTimeout);
        videoDataTimeout = null;
      }
      if (disconnectRecoveryTimeoutRef.current) {
        clearTimeout(disconnectRecoveryTimeoutRef.current);
        disconnectRecoveryTimeoutRef.current = null;
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
        audioContextRef.current.close().catch((err) => {
          // Log close errors to aid diagnosis of audio cleanup issues
          console.error('Failed to close AudioContext:', err);
        });
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

    return cleanupWebRTCResources;
  }, [stream, retryCount, t]);

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
    connectionRefreshRequestedRef.current = false;
    noDataReconnectAttemptsRef.current = 0;

    // Refresh the stream's go2rtc registration before retrying
    // This helps recover from stale go2rtc state that causes WebRTC failures
    await refreshStreamRegistration();

    // Delay to allow go2rtc to fully re-register the stream (longer for slow devices)
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Increment retry count to trigger useEffect re-run
    setRetryCount(prev => prev + 1);
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
      audioContextRef.current.close().catch((err) => {
        // Log close errors to aid diagnosis of audio cleanup issues
        console.error('Failed to close AudioContext:', err);
      });
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

      if (err.name === 'NotAllowedError') {
        setMicrophoneError(t('live.microphoneAccessDenied'));
      } else if (err.name === 'NotFoundError') {
        setMicrophoneError(t('live.noMicrophoneFound'));
      } else {
        setMicrophoneError(t('live.microphoneErrorWithMessage', { message: err.message }));
      }
    }
  }, [stream, startAudioLevelMonitoring, t]);

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

  // Player telemetry (TTFF, rebuffer, WebRTC RTT tracking)
  useEffect(() => {
    const video = videoRef.current;
    if (!video || !stream?.name) return;

    const telemetry = createPlayerTelemetry(stream.name, 'webrtc');

    const onPlaying = () => telemetry.recordFirstFrame();
    const onWaiting = () => telemetry.recordRebufferStart();
    const onCanPlay = () => telemetry.recordRebufferEnd();

    video.addEventListener('playing', onPlaying);
    video.addEventListener('waiting', onWaiting);
    video.addEventListener('canplay', onCanPlay);

    // Poll WebRTC stats for RTT
    const rttInterval = setInterval(() => {
      const pc = peerConnectionRef.current;
      if (!pc) return;
      pc.getStats().then(stats => {
        stats.forEach(report => {
          if (report.type === 'candidate-pair' && report.state === 'succeeded' && report.currentRoundTripTime) {
            telemetry.updateRtt(report.currentRoundTripTime * 1000); // Convert to ms
          }
        });
      }).catch(() => {});
    }, 5000);

    return () => {
      video.removeEventListener('playing', onPlaying);
      video.removeEventListener('waiting', onWaiting);
      video.removeEventListener('canplay', onCanPlay);
      clearInterval(rttInterval);
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
        muted={!audioEnabled}
        disablePictureInPicture
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

      {/* Stream name overlay with connection quality indicator */}
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
            zIndex: 3,
            display: 'flex',
            alignItems: 'center',
            gap: '8px'
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

          {/* Connection quality indicator - only show when we have quality data and stream is playing */}
          {isPlaying && connectionQuality !== 'unknown' && (
            <div
              className={`connection-quality-indicator quality-${connectionQuality}`}
              title={t('live.connectionQuality', { quality: t(`live.connectionQuality.${connectionQuality}`) })}
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
        {/* Audio playback toggle button (for hearing camera audio) */}
        {isPlaying && (
          <button
            className={`audio-toggle-btn ${audioEnabled ? 'active' : ''}`}
            title={audioEnabled ? t('live.muteCameraAudio') : t('live.unmuteCameraAudio')}
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
              title={talkMode === 'ptt' ? t('live.switchToToggleMode') : t('live.switchToPushToTalkMode')}
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
                ? (isTalking ? t('live.releaseToStopTalking') : t('live.holdToTalk'))
                : (isTalking ? t('live.clickToStopTalking') : t('live.clickToStartTalking'))}
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
        {/* Force refresh stream button - show during connecting (isLoading) or playing */}
        {(isPlaying || isLoading) && (
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
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
        </button>
      </div>
      )}

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
            zIndex: 10,
            textAlign: 'center',
            pointerEvents: 'auto',
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
              {t('common.retry')}
            </button>
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
    </div>
  );
}
