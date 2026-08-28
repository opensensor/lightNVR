import { useEffect, useRef, useState } from 'preact/hooks';
import { useI18n } from '../../i18n.js';
import { formatFilenameTimestamp } from '../../utils/date-utils.js';
import { showStatusMessage } from './ToastContainer.jsx';
import {
  LONG_PRESS_DELAY_MS,
  TILE_CHROME_TIMEOUT_MS,
  shouldIgnoreTileGestureTarget,
} from './mobileLiveGestures.js';

function isTouchPointer(event) {
  return event?.pointerType === 'touch' || event?.pointerType === 'pen';
}

function isCoarsePointer() {
  return typeof window !== 'undefined'
    && typeof window.matchMedia === 'function'
    && window.matchMedia('(hover: none) and (pointer: coarse)').matches;
}

function downloadCurrentFrame(video, streamName, t) {
  if (!video?.videoWidth || !video?.videoHeight) {
    showStatusMessage(t('live.cannotTakeSnapshotVideoNotLoaded'), 'error');
    return;
  }

  const canvas = document.createElement('canvas');
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  canvas.getContext('2d')?.drawImage(video, 0, 0, canvas.width, canvas.height);
  canvas.toBlob((blob) => {
    if (!blob) {
      showStatusMessage(t('timeline.failedToCreateSnapshot'), 'error');
      return;
    }

    const fileName = `snapshot-${streamName.replace(/\s+/g, '-')}-${formatFilenameTimestamp()}.jpg`;
    const blobUrl = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = blobUrl;
    link.download = fileName;
    document.body.appendChild(link);
    link.click();
    setTimeout(() => {
      link.remove();
      URL.revokeObjectURL(blobUrl);
    }, 1000);
    showStatusMessage(t('live.snapshotSaved', { fileName }), 'success', 2000);
  }, 'image/jpeg', 0.95);
}

export function useMobileTileGestures({
  streamName,
  cellRef,
  videoRef,
  audioEnabled,
  onToggleAudio,
  onRequestReorder,
  disabled = false,
}) {
  const { t } = useI18n();
  const [chromeVisible, setChromeVisible] = useState(true);
  const [menuOpen, setMenuOpen] = useState(false);
  const chromeTimerRef = useRef(null);
  const longPressTimerRef = useRef(null);
  const pointerRef = useRef(null);
  const suppressClickRef = useRef(false);
  const audioEnabledRef = useRef(audioEnabled);
  audioEnabledRef.current = audioEnabled;

  const clearChromeTimer = () => {
    if (chromeTimerRef.current) clearTimeout(chromeTimerRef.current);
    chromeTimerRef.current = null;
  };

  const scheduleChromeHide = () => {
    clearChromeTimer();
    if (!isCoarsePointer() || menuOpen) return;
    chromeTimerRef.current = setTimeout(() => setChromeVisible(false), TILE_CHROME_TIMEOUT_MS);
  };

  const revealChrome = () => {
    setChromeVisible(true);
    scheduleChromeHide();
  };

  const clearLongPress = () => {
    if (longPressTimerRef.current) clearTimeout(longPressTimerRef.current);
    longPressTimerRef.current = null;
    pointerRef.current = null;
  };

  useEffect(() => {
    return () => {
      clearChromeTimer();
      clearLongPress();
    };
  }, []);

  useEffect(() => {
    if (menuOpen) {
      clearChromeTimer();
      setChromeVisible(true);
    } else {
      scheduleChromeHide();
    }
    return clearChromeTimer;
  }, [menuOpen]);

  useEffect(() => {
    const handleOtherTileAudio = (event) => {
      if (event.detail?.streamName !== streamName && audioEnabledRef.current) onToggleAudio?.();
    };
    window.addEventListener('lightnvr:tile-audio-enabled', handleOtherTileAudio);
    return () => window.removeEventListener('lightnvr:tile-audio-enabled', handleOtherTileAudio);
  }, [onToggleAudio, streamName]);

  const onPointerDown = (event) => {
    revealChrome();
    if (!isTouchPointer(event)) return;
    if (event.isPrimary === false) {
      clearLongPress();
      return;
    }
    if (disabled || shouldIgnoreTileGestureTarget(event.target)) return;

    clearLongPress();
    pointerRef.current = {
      id: event.pointerId,
      x: event.clientX,
      y: event.clientY,
    };
    longPressTimerRef.current = setTimeout(() => {
      suppressClickRef.current = true;
      setMenuOpen(true);
      setChromeVisible(true);
      if (typeof navigator !== 'undefined' && typeof navigator.vibrate === 'function') {
        navigator.vibrate(10);
      }
      longPressTimerRef.current = null;
    }, LONG_PRESS_DELAY_MS);
  };

  const onPointerMove = (event) => {
    const pointer = pointerRef.current;
    if (!pointer || pointer.id !== event.pointerId) return;
    if (Math.hypot(event.clientX - pointer.x, event.clientY - pointer.y) > 10) clearLongPress();
  };

  const onPointerEnd = () => clearLongPress();

  const onClick = (event) => {
    revealChrome();
    if (!suppressClickRef.current) return false;
    suppressClickRef.current = false;
    event.preventDefault();
    event.stopPropagation();
    return true;
  };

  const onContextMenu = (event) => {
    if (isTouchPointer(event) || suppressClickRef.current) event.preventDefault();
  };

  const closeMenu = () => {
    setMenuOpen(false);
  };

  const toggleAudio = () => {
    onToggleAudio?.();
    closeMenu();
  };

  const takeSnapshot = () => {
    const existingButton = cellRef.current?.querySelector('.snapshot-btn');
    if (existingButton) existingButton.click();
    else downloadCurrentFrame(videoRef.current, streamName, t);
    closeMenu();
  };

  const openRecordings = () => {
    window.location.assign(`/recordings.html?stream=${encodeURIComponent(streamName)}`);
  };

  const requestReorder = () => {
    onRequestReorder?.();
    closeMenu();
  };

  return {
    canReorder: Boolean(onRequestReorder),
    chromeVisible,
    menuOpen,
    onPointerDown,
    onPointerMove,
    onPointerUp: onPointerEnd,
    onPointerCancel: onPointerEnd,
    onContextMenu,
    onClick,
    closeMenu,
    toggleAudio,
    takeSnapshot,
    openRecordings,
    requestReorder,
  };
}

export function MobileTileContextMenu({
  gestures,
  audioEnabled,
  audioAvailable = true,
  canReorder = gestures.canReorder,
}) {
  const { t } = useI18n();
  const menuRef = useRef(null);
  useEffect(() => {
    if (gestures.menuOpen) menuRef.current?.querySelector('button')?.focus();
  }, [gestures.menuOpen]);
  if (!gestures.menuOpen) return null;

  const action = (handler) => (event) => {
    event.preventDefault();
    event.stopPropagation();
    handler();
  };

  return (
    <div
      className="mobile-tile-menu-backdrop"
      onClick={action(gestures.closeMenu)}
      role="presentation"
    >
      <div
        ref={menuRef}
        className="mobile-tile-menu"
        role="menu"
        aria-label={t('live.tileActions')}
        onClick={(event) => event.stopPropagation()}
        onKeyDown={(event) => {
          if (event.key === 'Escape') gestures.closeMenu();
        }}
      >
        {audioAvailable && (
          <button type="button" role="menuitem" onClick={action(gestures.toggleAudio)}>
            {t(audioEnabled ? 'live.muteCameraAudio' : 'live.unmuteCameraAudio')}
          </button>
        )}
        <button type="button" role="menuitem" onClick={action(gestures.takeSnapshot)}>
          {t('live.takeSnapshot')}
        </button>
        <button type="button" role="menuitem" onClick={action(gestures.openRecordings)}>
          {t('live.openStreamRecordings')}
        </button>
        {canReorder && (
          <button type="button" role="menuitem" onClick={action(gestures.requestReorder)}>
            {t('live.enterReorderMode')}
          </button>
        )}
      </div>
    </div>
  );
}
