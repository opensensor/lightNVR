// @ts-nocheck
/**
 * Player Telemetry - Client-side QoE metrics collection
 * Tracks TTFF, rebuffer events, and sends to POST /api/telemetry/player
 */

const SEND_INTERVAL_MS = 30000; // Send every 30 seconds
let sessionCounter = 0;

export function createPlayerTelemetry(streamName, transport) {
    const sessionId = `${Date.now()}-${++sessionCounter}`;
    const mountTime = performance.now();
    let ttffMs = 0;
    let rebufferCount = 0;
    let rebufferDurationMs = 0;
    let rebufferStart = 0;
    let resolutionSwitches = 0;
    let webrtcRttMs = 0;
    let sendTimer = null;
    let destroyed = false;

    function send() {
        if (destroyed) return;
        const payload = {
            stream_name: streamName,
            session_id: sessionId,
            transport: transport,
            ttff_ms: ttffMs,
            rebuffer_count: rebufferCount,
            rebuffer_duration_ms: rebufferDurationMs,
            resolution_switches: resolutionSwitches,
            webrtc_rtt_ms: webrtcRttMs,
        };

        try {
            // Use sendBeacon for reliability (works during page unload)
            if (navigator.sendBeacon) {
                navigator.sendBeacon('/api/telemetry/player', JSON.stringify(payload));
            } else {
                fetch('/api/telemetry/player', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload),
                    keepalive: true,
                }).catch(() => {}); // fire-and-forget
            }
        } catch (e) {
            // Ignore errors - telemetry is best-effort
        }
    }

    // Start periodic sending
    sendTimer = setInterval(send, SEND_INTERVAL_MS);

    return {
        /** Call when first frame is displayed */
        recordFirstFrame() {
            if (ttffMs === 0) {
                ttffMs = performance.now() - mountTime;
            }
        },

        /** Call when video starts buffering (waiting event) */
        recordRebufferStart() {
            if (rebufferStart === 0) {
                rebufferStart = performance.now();
                rebufferCount++;
            }
        },

        /** Call when video resumes playing after buffer */
        recordRebufferEnd() {
            if (rebufferStart > 0) {
                rebufferDurationMs += performance.now() - rebufferStart;
                rebufferStart = 0;
            }
        },

        /** Call when resolution changes */
        recordResolutionSwitch() {
            resolutionSwitches++;
        },

        /** Call with RTT value from WebRTC stats */
        updateRtt(rttMs) {
            webrtcRttMs = rttMs;
        },

        /** Call on component unmount - sends final telemetry */
        destroy() {
            destroyed = true;
            if (sendTimer) {
                clearInterval(sendTimer);
                sendTimer = null;
            }
            send(); // Send final metrics
        }
    };
}
