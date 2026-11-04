import { useEffect, useRef, useState } from 'preact/hooks';
import { useI18n } from '../../i18n.ts';
import { formatFilenameTimestamp } from '../../utils/date-utils.ts';
import { showStatusMessage } from './ToastContainer.tsx';
import {
  LONG_PRESS_DELAY_MS,
  TILE_CHROME_TIMEOUT_MS,
  shouldIgnoreTileGestureTarget,
} from './mobileLiveGestures.ts';
import type { RefObject } from 'preact';
import type { Translate } from '../../types/live.ts';

export interface UseMobileTileGesturesOptions {
  streamName: string;
  cellRef: RefObject<HTMLElement | null>;
  videoRef: RefObject<HTMLVideoElement | null>;
  audioEnabled: boolean;
  onToggleAudio?: () => unknown;
  onRequestReorder?: () => unknown;
  disabled?: boolean;
}

function isTouchPointer(event?: Pick<PointerEvent, 'pointerType'> | null) {
  return event?.pointerType === 'touch' || event?.pointerType === 'pen';
}

function isCoarsePointer() {
  return typeof window !== 'undefined'
    && typeof window.matchMedia === 'function'
    && window.matchMedia('(hover: none) and (pointer: coarse)').matches;
}

function downloadCurrentFrame(video: HTMLVideoElement | null, streamName: string, t: Translate) {
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
}: UseMobileTileGesturesOptions) {
  const { t } = useI18n();
  const [chromeVisible, setChromeVisible] = useState(true);
  const [menuOpen, setMenuOpen] = useState(false);
  const chromeTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const longPressTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pointerRef = useRef<{ id: number; x: number; y: number } | null>(null);
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
    const handleOtherTileAudio = (event: Event) => {
      if ((event as CustomEvent<{ streamName?: string } | undefined>).detail?.streamName !== streamName && audioEnabledRef.current) onToggleAudio?.();
    };
    window.addEventListener('lightnvr:tile-audio-enabled', handleOtherTileAudio);
    return () => window.removeEventListener('lightnvr:tile-audio-enabled', handleOtherTileAudio);
  }, [onToggleAudio, streamName]);

  const onPointerDown = (event: PointerEvent) => {
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

  const onPointerMove = (event: PointerEvent) => {
    const pointer = pointerRef.current;
    if (!pointer || pointer.id !== event.pointerId) return;
    if (Math.hypot(event.clientX - pointer.x, event.clientY - pointer.y) > 10) clearLongPress();
  };

  const onPointerEnd = () => clearLongPress();

  const onClick = (event: MouseEvent) => {
    revealChrome();
    if (!suppressClickRef.current) return false;
    suppressClickRef.current = false;
    event.preventDefault();
    event.stopPropagation();
    return true;
  };

  // Browsers dispatch contextmenu as a PointerEvent; Preact types it as a MouseEvent.
  const onContextMenu = (event: MouseEvent) => {
    if (isTouchPointer(event as PointerEvent) || suppressClickRef.current) event.preventDefault();
  };

  const closeMenu = () => {
    setMenuOpen(false);
  };

  const toggleAudio = () => {
    onToggleAudio?.();
    closeMenu();
  };

  const takeSnapshot = () => {
    const existingButton = cellRef.current?.querySelector<HTMLElement>('.snapshot-btn');
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

export interface MobileTileContextMenuProps {
  gestures: ReturnType<typeof useMobileTileGestures>;
  audioEnabled: boolean;
  audioAvailable?: boolean;
  canReorder?: boolean;
}

export function MobileTileContextMenu({
  gestures,
  audioEnabled,
  audioAvailable = true,
  canReorder = gestures.canReorder,
}: MobileTileContextMenuProps) {
  const { t } = useI18n();
  const menuRef = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    if (gestures.menuOpen) menuRef.current?.querySelector('button')?.focus();
  }, [gestures.menuOpen]);
  if (!gestures.menuOpen) return null;

  const action = (handler: () => unknown) => (event: MouseEvent) => {
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
          tabIndex={-1}
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
