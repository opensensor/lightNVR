import { useCallback, useEffect, useMemo, useState } from 'preact/hooks';
import { HLSVideoCell } from './HLSVideoCell.jsx';
import { MSEVideoCell } from './MSEVideoCell.jsx';
import { WebRTCVideoCell } from './WebRTCVideoCell.jsx';
import { useI18n } from '../../i18n.js';
import {
  buildPlaybackTransportPlan,
  isPlaybackFallback,
  shouldFallbackPlaybackTransport,
} from '../../utils/playback-transport.js';

const CELLS = {
  webrtc: WebRTCVideoCell,
  mse: MSEVideoCell,
  hls: HLSVideoCell,
};

const transportName = (value) => ({
  webrtc: 'WebRTC',
  mse: 'MSE',
  hls: 'HLS',
}[value] || value || 'Playback');

/** Selects and, when configured, falls back between renderers for one tile. */
export function PlaybackTransportCell({
  stream,
  offerings,
  defaultTransport = 'webrtc',
  forcedTransport = null,
  fullscreenUpgraded = false,
  ...cellProps
}) {
  const { t } = useI18n();
  const plan = useMemo(
    () => buildPlaybackTransportPlan(
      stream?.playback_transport,
      offerings,
      defaultTransport,
      forcedTransport
    ),
    [stream?.playback_transport, offerings?.webrtc, offerings?.mse,
      offerings?.hls, defaultTransport, forcedTransport]
  );
  const [transportIndex, setTransportIndex] = useState(0);

  useEffect(() => {
    setTransportIndex(0);
  }, [stream?.name, stream?.playback_transport, offerings?.webrtc,
    offerings?.mse, offerings?.hls, defaultTransport, forcedTransport]);

  const activeTransport = plan.available[transportIndex];
  const hasRuntimeFallback = transportIndex + 1 < plan.available.length;
  const sourceUnavailableMessage = t('live.cannotConnectToSource');
  const handleTransportFailure = useCallback((failure) => {
    if (hasRuntimeFallback && shouldFallbackPlaybackTransport(failure, {
      streamStatus: stream?.status || stream?.state,
      sourceUnavailableMessage,
    })) {
      setTransportIndex((current) => Math.min(current + 1, plan.available.length - 1));
    }
  }, [hasRuntimeFallback, plan.available.length, sourceUnavailableMessage,
    stream?.status, stream?.state]);

  if (!activeTransport) {
    const unavailable = plan.unavailable[0] || plan.requested[0];
    return (
      <div className="video-cell bg-black text-white flex items-center justify-center min-h-48 rounded-lg">
        <div className="max-w-sm p-5 text-center" role="alert">
          <div className="text-3xl mb-2" aria-hidden="true">⚠</div>
          <p className="font-semibold">{stream?.name}</p>
          <p className="mt-1 text-sm text-white/80">
            {t('live.playbackTransportUnavailable', { transport: transportName(unavailable) })}
          </p>
        </div>
      </div>
    );
  }

  const Cell = CELLS[activeTransport];
  const fallbackUsed = isPlaybackFallback(plan, activeTransport, transportIndex);

  return (
    <>
      <Cell
        key={`${stream?.name}:${activeTransport}`}
        stream={stream}
        {...cellProps}
        fullscreenUpgraded={activeTransport === 'webrtc' ? fullscreenUpgraded : undefined}
        onTransportFailure={hasRuntimeFallback ? handleTransportFailure : undefined}
      />
      {fallbackUsed && (
        <span
          className="absolute top-2 left-2 z-30 rounded bg-amber-600/90 px-2 py-1 text-xs font-semibold text-white pointer-events-none"
          title={t('live.playbackFallbackUsed', { transport: transportName(activeTransport) })}
        >
          {t('live.playbackFallbackUsed', { transport: transportName(activeTransport) })}
        </span>
      )}
    </>
  );
}

export default PlaybackTransportCell;
