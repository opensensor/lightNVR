/**
 * PTZ Controls Component
 * Pan-Tilt-Zoom controls for PTZ-enabled cameras
 */

import { useState, useEffect, useCallback, useRef } from 'preact/hooks';
import type { JSX } from 'preact';
import { useI18n } from '../../i18n.ts';
import { showSuccessToast, showErrorToast } from './ToastContainer.tsx';
import {
  getPtzCapabilities,
  getPtzPresets,
  savePtzPreset,
  setPtzHome,
  ptzMove,
  ptzStop,
  ptzHome,
  ptzGotoPreset,
  deletePtzPreset
} from './ptzActions.ts';
import type { PtzCapabilities, PtzDirection, PtzError, PtzPreset } from '../../types/live.ts';

interface DirectionButtonProps {
  direction: PtzDirection;
  onPressStart: () => void;
  onPressEnd: () => void;
  disabled?: boolean;
}

export interface PTZControlsProps {
  stream?: { name: string; ptz_enabled?: boolean } | null;
  isVisible?: boolean;
  onClose?: () => unknown;
  streamName?: string;
  ptzType?: string;
}

interface Position {
  x: number;
  y: number;
}

const DEFAULT_POSITION: Position = { x: 10, y: 10 };

/**
 * Direction button component matching original upstream event bindings
 */
function DirectionButton({
  direction,
  onPressStart,
  onPressEnd,
  disabled
}: DirectionButtonProps) {
  const icons: Record<PtzDirection, JSX.Element> = {
    up: (
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round">
        <path d="m18 15-6-6-6 6" />
      </svg>
    ),
    down: (
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round">
        <path d="m6 9 6 6 6-6" />
      </svg>
    ),
    left: (
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round">
        <path d="m15 18-6-6 6-6" />
      </svg>
    ),
    right: (
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round">
        <path d="m9 18 6-6-6-6" />
      </svg>
    ),
    'zoom-in': (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
        <line x1="11" y1="8" x2="11" y2="14" />
        <line x1="8" y1="11" x2="14" y2="11" />
      </svg>
    ),
    'zoom-out': (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
        <circle cx="11" cy="11" r="8" />
        <line x1="8" y1="11" x2="14" y2="11" />
      </svg>
    )
  };

  const handleStart = (e: Event) => {
    e.stopPropagation();
    if (disabled) return;
    onPressStart();
  };

  const handleEnd = (e: Event) => {
    e.stopPropagation();
    onPressEnd();
  };

  return (
    <button
      type="button"
      className={`ptz-btn ptz-btn-${direction}`}
      onMouseDown={handleStart as any}
      onMouseUp={handleEnd as any}
      onMouseLeave={handleEnd as any}
      onTouchStart={handleStart as any}
      onTouchEnd={handleEnd as any}
      onTouchCancel={handleEnd as any}
      disabled={disabled}
      style={{
        width: '38px',
        height: '38px',
        borderRadius: '50%',
        border: '1px solid rgba(255, 255, 255, 0.3)',
        backgroundColor: 'rgba(15, 23, 42, 0.85)',
        color: '#ffffff',
        cursor: disabled ? 'not-allowed' : 'pointer',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        transition: 'all 0.15s ease-in-out',
        opacity: disabled ? 0.35 : 1,
        userSelect: 'none',
        boxShadow: '0 2px 6px rgba(0, 0, 0, 0.4)',
        backdropFilter: 'blur(4px)',
        margin: 'auto',
        touchAction: 'none'
      }}
    >
      {icons[direction]}
    </button>
  );
}

/**
 * PTZ Controls Component
 */
export function PTZControls({ stream, isVisible = true, onClose }: PTZControlsProps) {
  const { t } = useI18n();

  const [speed, setSpeed] = useState<number>(0.5);
  const [presets, setPresets] = useState<PtzPreset[]>([]);
  const [selectedPreset, setSelectedPreset] = useState<string>('');
  const [error, setError] = useState<string | null>(null);
  const [capabilities, setCapabilities] = useState<PtzCapabilities | null>(null);
  const [canManage, setCanManage] = useState<boolean>(true);
  const [presetName, setPresetName] = useState<string>('');
  const [showPresetForm, setShowPresetForm] = useState<boolean>(false);
  const [confirmSetHome, setConfirmSetHome] = useState<boolean>(false);
  const [saving, setSaving] = useState<boolean>(false);

  // =========================
  // DRAG STATE & FUNCTIONS
  // =========================
  const panelRef = useRef<HTMLDivElement | null>(null);
  const [position, setPosition] = useState<Position>(DEFAULT_POSITION);

  const dragData = useRef({
    dragging: false,
    startX: 0,
    startY: 0,
    initialX: 10,
    initialY: 10
  });

  const handleWheel = useCallback((e: WheelEvent) => {
    e.stopPropagation();
    if (e.cancelable) {
      e.preventDefault();
    }
  }, []);

  const clampPosition = useCallback((x: number, y: number): Position => {
    const panel = panelRef.current;
    if (!panel) return { x, y };

    const parent = panel.parentElement;
    if (!parent) return { x, y };

    const parentRect = parent.getBoundingClientRect();
    const panelRect = panel.getBoundingClientRect();

    const maxX = parentRect.width - panelRect.width;
    const maxY = parentRect.height - panelRect.height;

    return {
      x: Math.max(0, Math.min(x, maxX)),
      y: Math.max(0, Math.min(y, maxY))
    };
  }, []);

  // Reset position to DEFAULT_POSITION on fullscreen change and window resize for all devices
  useEffect(() => {
    const handleFullscreenChange = () => {
      setPosition(DEFAULT_POSITION);
    };

    document.addEventListener('fullscreenchange', handleFullscreenChange);
    window.addEventListener('resize', handleFullscreenChange);

    return () => {
      document.removeEventListener('fullscreenchange', handleFullscreenChange);
      window.removeEventListener('resize', handleFullscreenChange);
    };
  }, []);

  // Strict Event Boundary Isolation for panel dragging
  useEffect(() => {
    const panel = panelRef.current;
    if (!panel) return;

    const stopPropagationOnly = (e: Event) => {
      e.stopPropagation();
    };

    const handleTouchMoveBound = (e: TouchEvent) => {
      e.stopPropagation();
      if (dragData.current.dragging && e.cancelable) {
        e.preventDefault();
      }
    };

    panel.addEventListener('touchstart', stopPropagationOnly, { capture: true, passive: true });
    panel.addEventListener('touchmove', handleTouchMoveBound, { capture: true, passive: false });
    panel.addEventListener('touchend', stopPropagationOnly, { capture: true, passive: true });
    panel.addEventListener('touchcancel', stopPropagationOnly, { capture: true, passive: true });

    return () => {
      panel.removeEventListener('touchstart', stopPropagationOnly, { capture: true });
      panel.removeEventListener('touchmove', handleTouchMoveBound, { capture: true });
      panel.removeEventListener('touchend', stopPropagationOnly, { capture: true });
      panel.removeEventListener('touchcancel', stopPropagationOnly, { capture: true });
    };
  }, []);

  const startDrag = useCallback((e: MouseEvent | TouchEvent) => {
    e.stopPropagation();
    
    const panel = panelRef.current;
    if (!panel) return;

    const touchEvent = e as TouchEvent;
    const mouseEvent = e as MouseEvent;
    const clientX = touchEvent.touches ? touchEvent.touches[0].clientX : mouseEvent.clientX;
    const clientY = touchEvent.touches ? touchEvent.touches[0].clientY : mouseEvent.clientY;

    dragData.current = {
      dragging: true,
      startX: clientX,
      startY: clientY,
      initialX: position.x,
      initialY: position.y
    };

    if (e.cancelable) {
      e.preventDefault();
    }
  }, [position]);

  useEffect(() => {
    const onDrag = (e: MouseEvent | TouchEvent) => {
      if (!dragData.current.dragging) return;

      e.stopPropagation();

      const touchEvent = e as TouchEvent;
      const mouseEvent = e as MouseEvent;
      const clientX = touchEvent.touches ? touchEvent.touches[0].clientX : mouseEvent.clientX;
      const clientY = touchEvent.touches ? touchEvent.touches[0].clientY : mouseEvent.clientY;

      const deltaX = clientX - dragData.current.startX;
      const deltaY = dragData.current.startY - clientY;

      const newX = dragData.current.initialX + deltaX;
      const newY = dragData.current.initialY + deltaY;

      const clamped = clampPosition(newX, newY);
      setPosition(clamped);

      if (e.cancelable) {
        e.preventDefault();
      }
    };

    const stopDrag = (e: MouseEvent | TouchEvent) => {
      if (dragData.current.dragging) {
        e.stopPropagation();
        dragData.current.dragging = false;
      }
    };

    window.addEventListener('mousemove', onDrag, { capture: true });
    window.addEventListener('mouseup', stopDrag, { capture: true });
    window.addEventListener('touchmove', onDrag, { passive: false, capture: true });
    window.addEventListener('touchend', stopDrag, { capture: true });
    window.addEventListener('touchcancel', stopDrag, { capture: true });

    return () => {
      window.removeEventListener('mousemove', onDrag, { capture: true });
      window.removeEventListener('mouseup', stopDrag, { capture: true });
      window.removeEventListener('touchmove', onDrag, { capture: true });
      window.removeEventListener('touchend', stopDrag, { capture: true });
      window.removeEventListener('touchcancel', stopDrag, { capture: true });
    };
  }, [clampPosition]);

  // Load capabilities and presets
  useEffect(() => {
    if (!stream?.name || !stream?.ptz_enabled) return;

    let active = true;
    setCapabilities(null);
    setCanManage(true);
    setPresets([]);
    setSelectedPreset('');
    setPresetName('');
    setShowPresetForm(false);
    setConfirmSetHome(false);

    getPtzCapabilities(stream.name)
      .then(data => { if (active) setCapabilities(data); })
      .catch(err => console.error('Failed to get PTZ capabilities:', err));

    getPtzPresets(stream.name)
      .then(data => { if (active) setPresets(data.presets || []); })
      .catch(err => console.error('Failed to get PTZ presets:', err));

    return () => { active = false; };
  }, [stream?.name, stream?.ptz_enabled]);

  const handleActionFailure = useCallback((err: PtzError, message: string) => {
    if (err.status === 403) {
      setCanManage(false);
      setShowPresetForm(false);
      setConfirmSetHome(false);
    }
    showErrorToast(message);
    console.error(message, err);
  }, []);

  const handleSavePreset = useCallback(async (event: Event) => {
    event.preventDefault();
    const name = presetName.trim();
    if (!stream?.name || !name || saving) return;
    setSaving(true);
    try {
      await savePtzPreset(stream.name, name, setPresets);
      setShowPresetForm(false);
      setPresetName('');
      showSuccessToast(t('live.ptzPresetSaved'));
    } catch (err) {
      handleActionFailure(err as PtzError, t((err as PtzError).stage === 'refresh'
        ? 'live.ptzPresetRefreshFailed' : 'live.ptzPresetSaveFailed'));
    } finally {
      setSaving(false);
    }
  }, [stream?.name, presetName, saving, t, handleActionFailure]);

  const handleDeletePreset = useCallback(async () => {
    if (!stream?.name || !selectedPreset || saving) return;
    setSaving(true);
    try {
      await deletePtzPreset(stream.name, selectedPreset);
      setPresets(prev => prev.filter(p => p.token !== selectedPreset));
      setSelectedPreset('');
      showSuccessToast(t('live.ptzPresetDeleted') || 'Preset deleted');
    } catch (err) {
      handleActionFailure(err as PtzError, t('live.ptzPresetDeleteFailed') || 'Failed to delete preset');
    } finally {
      setSaving(false);
    }
  }, [stream?.name, selectedPreset, saving, t, handleActionFailure]);

  const handleSetHome = useCallback(async () => {
    if (!stream?.name || saving) return;
    setSaving(true);
    try {
      await setPtzHome(stream.name);
      setConfirmSetHome(false);
      showSuccessToast(t('live.ptzHomeSet'));
    } catch (err) {
      handleActionFailure(err as PtzError, t('live.ptzSetHomeFailed'));
    } finally {
      setSaving(false);
    }
  }, [stream?.name, saving, t, handleActionFailure]);

  // ==========================================
  // PTZ MOVE & STOP LOGIC (GUARENTEED STOP)
  // ==========================================
  const handleMoveStop = useCallback(() => {
    if (!stream?.name) return;
    
    ptzStop(stream.name)
      .catch(err => console.error('PTZ stop error:', err));
  }, [stream?.name]);

  const handleMoveStart = useCallback((pan: number, tilt: number, zoom: number) => {
    if (!stream?.name) return;
    setError(null);
    
    ptzMove(stream.name, pan * speed, tilt * speed, zoom * speed)
      .catch(err => {
        setError(t('live.ptzMoveFailed'));
        console.error('PTZ move error:', err);
      });
  }, [stream?.name, speed, t]);

  const handleHome = useCallback((e?: MouseEvent) => {
    if (e) e.stopPropagation();
    if (!stream?.name) return;
    setError(null);
    
    ptzHome(stream.name)
      .catch(err => {
        setError(t('live.ptzHomeFailed'));
        console.error('PTZ home error:', err);
      });
  }, [stream?.name, t]);

  const handlePreset = useCallback((token: string) => {
    if (!stream?.name) return;
    setError(null);
    
    ptzGotoPreset(stream.name, token)
      .catch(err => {
        setError(t('live.ptzPresetFailed'));
        console.error('PTZ preset error:', err);
      });
  }, [stream?.name, t]);

  if (!isVisible || !stream?.ptz_enabled) return null;

  return (
    <div
      ref={panelRef}
      className="ptz-controls"
    
      // Hentikan event pointer agar tidak diterima gesture kamera
      onPointerDown={(e) => e.stopPropagation()}
      onPointerMove={(e) => e.stopPropagation()}
      onPointerUp={(e) => e.stopPropagation()}
      onPointerCancel={(e) => e.stopPropagation()}
    
      // Hentikan click agar tidak memicu fullscreen kamera
      onClick={(e) => e.stopPropagation()}
      onDblClick={(e) => e.stopPropagation()}
      onContextMenu={(e) => e.stopPropagation()}
    
      // Pertahankan isolasi event mouse dan touch
      onWheel={(e) => handleWheel(e as unknown as WheelEvent)}
      onMouseDown={(e) => e.stopPropagation()}
      onTouchStart={(e) => e.stopPropagation()}
      style={{
        position: 'absolute',
        left: `${position.x}px`,
        bottom: `${position.y}px`,
        backgroundColor: 'rgba(0, 0, 0, 0.55)',
        backdropFilter: 'blur(8px)',
        WebkitBackdropFilter: 'blur(8px)',
        borderRadius: '16px',
        border: '1px solid rgba(255, 255, 255, 0.25)',
        boxShadow: '0 8px 32px 0 rgba(0, 0, 0, 0.6)',
        padding: '10px 12px',
        zIndex: 10,
        display: 'flex',
        flexDirection: 'column',
        gap: '8px',
        minWidth: '155px',
        touchAction: 'none',
        userSelect: 'none',
        pointerEvents: 'auto'
      }}
    >
      {/* DRAG HEADER */}
      <div
        onMouseDown={(e) => startDrag(e as unknown as MouseEvent)}
        onTouchStart={(e) => startDrag(e as unknown as TouchEvent)}
        style={{
          display: 'flex',
          alignItems: 'center',
          cursor: 'move',
          padding: '6px 8px',
          backgroundColor: 'rgba(0, 0, 0, 0.45)',
          borderRadius: '8px',
          border: '1px solid rgba(255, 255, 255, 0.15)',
          width: '100%',
          boxSizing: 'border-box',
          touchAction: 'none'
        }}
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.7)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{ marginRight: '6px', flexShrink: 0 }}>
          <circle cx="9" cy="5" r="1" />
          <circle cx="9" cy="12" r="1" />
          <circle cx="9" cy="19" r="1" />
          <circle cx="15" cy="5" r="1" />
          <circle cx="15" cy="12" r="1" />
          <circle cx="15" cy="19" r="1" />
        </svg>

        <span
          style={{
            color: '#ffffff',
            fontSize: '11px',
            fontWeight: '600',
            letterSpacing: '0.3px',
            textShadow: '0 1px 3px rgba(0,0,0,0.9)',
            marginRight: 'auto'
          }}
        >
          {t('live.ptzControl')}
        </span>

        {onClose && (
          <button
            type="button"
            onClick={(e) => { e.stopPropagation(); onClose(); }}
            onMouseDown={(e) => e.stopPropagation()}
            onTouchStart={(e) => e.stopPropagation()}
            aria-label={t('common.close')}
            style={{
              background: 'none',
              border: 'none',
              cursor: 'pointer',
              padding: '2px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              marginLeft: '6px',
              color: 'rgba(255, 255, 255, 0.7)',
              touchAction: 'manipulation'
            }}
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M18 6 6 18" />
              <path d="m6 6 12 12" />
            </svg>
          </button>
        )}
      </div>

      {/* Direction pad */}
      <div 
        dir="ltr" 
        style={{ 
          display: 'grid', 
          gridTemplateColumns: 'repeat(3, 38px)', 
          gridTemplateRows: 'repeat(3, 38px)',
          gap: '6px', 
          justifyContent: 'center',
          alignItems: 'center',
          touchAction: 'none'
        }}
      >
        <div></div>
        <DirectionButton
          direction="up"
          onPressStart={() => handleMoveStart(0, 1, 0)}
          onPressEnd={handleMoveStop}
        />
        <div></div>

        <DirectionButton
          direction="left"
          onPressStart={() => handleMoveStart(-1, 0, 0)}
          onPressEnd={handleMoveStop}
        />

        {/* HOME BUTTON */}
        <button
          type="button"
          onClick={handleHome as any}
          aria-label={t('live.goToHomePosition')}
          title={t('live.goToHomePosition')}
          style={{
            width: '38px',
            height: '38px',
            borderRadius: '50%',
            border: '1px solid rgba(59, 130, 246, 0.6)',
            backgroundColor: 'rgba(15, 23, 42, 0.85)',
            color: '#60a5fa',
            cursor: 'pointer',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            transition: 'all 0.15s ease',
            boxShadow: '0 2px 6px rgba(0, 0, 0, 0.4)',
            margin: 'auto',
            touchAction: 'none'
          }}
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
            <polyline points="9 22 9 12 15 12 15 22" />
          </svg>
        </button>

        <DirectionButton
          direction="right"
          onPressStart={() => handleMoveStart(1, 0, 0)}
          onPressEnd={handleMoveStop}
        />

        <div></div>
        <DirectionButton
          direction="down"
          onPressStart={() => handleMoveStart(0, -1, 0)}
          onPressEnd={handleMoveStop}
        />
        <div></div>
      </div>

      {/* Zoom controls */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          gap: '8px',
          marginTop: '2px',
          width: '100%',
          touchAction: 'none'
        }}
      >
        <DirectionButton
          direction="zoom-out"
          onPressStart={() => handleMoveStart(0, 0, -1)}
          onPressEnd={handleMoveStop}
        />

        <span
          style={{
            color: '#ffffff',
            fontSize: '11px',
            fontWeight: '600',
            textShadow: '0 1px 3px rgba(0,0,0,0.9)'
          }}
        >
          {t('live.zoom')}
        </span>

        <DirectionButton
          direction="zoom-in"
          onPressStart={() => handleMoveStart(0, 0, 1)}
          onPressEnd={handleMoveStop}
        />
      </div>

      {/* Speed slider */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: '6px',
          marginTop: '2px',
          padding: '2px 4px'
        }}
      >
        <span style={{ color: '#ffffff', fontSize: '10px', fontWeight: '600', textShadow: '0 1px 3px rgba(0,0,0,0.9)' }}>
          {t('live.speedLabel')}
        </span>

        <input
          type="range"
          aria-label={t('live.speedLabel')}
          min="0.1"
          max="1"
          step="0.1"
          value={speed}
          onChange={(e) => setSpeed(parseFloat((e.target as HTMLInputElement).value))}
          style={{ flex: 1, height: '4px', accentColor: '#3b82f6', cursor: 'pointer', touchAction: 'manipulation' }}
        />

        <span style={{ color: '#ffffff', fontSize: '10px', width: '26px', textAlign: 'right', fontWeight: '600', textShadow: '0 1px 3px rgba(0,0,0,0.9)' }}>
          {Math.round(speed * 100)}%
        </span>
      </div>

      {/* Presets */}
      {presets.length > 0 && (
        <div
          style={{
            display: 'flex',
            alignItems: 'stretch',
            gap: '4px',
            marginTop: '2px',
            width: '100%'
          }}
        >
          <select
            aria-label={t('live.goToPreset')}
            value={selectedPreset}
            onChange={(e) => {
              const token = (e.target as HTMLSelectElement).value;
              setSelectedPreset(token);
              if (token) handlePreset(token);
            }}
            style={{
              flex: 1,
              minWidth: 0,
              padding: '5px 8px',
              borderRadius: '8px',
              border: '1px solid rgba(255, 255, 255, 0.25)',
              backgroundColor: 'rgba(15, 23, 42, 0.85)',
              color: 'white',
              fontSize: '10px',
              cursor: 'pointer',
              outline: 'none',
              boxSizing: 'border-box'
            }}
          >
            <option
              value=""
              style={{ backgroundColor: '#0f172a', color: 'white' }}
            >
              {t('live.goToPreset')}
            </option>
        
            {presets.map(preset => (
              <option
                key={preset.token}
                value={preset.token}
                style={{ backgroundColor: '#0f172a', color: 'white' }}
              >
                {preset.name || t('live.presetNumber', { token: preset.token })}
              </option>
            ))}
          </select>
        
          {canManage && selectedPreset && (
            <button
              type="button"
              onClick={handleDeletePreset}
              disabled={saving}
              title={t('common.delete') || 'Delete Preset'}
              style={{
                width: '32px',
                minWidth: '32px',
                alignSelf: 'stretch',
                borderRadius: '8px',
                border: '1px solid rgba(239, 68, 68, 0.5)',
                backgroundColor: 'rgba(239, 68, 68, 0.7)',
                color: 'white',
                cursor: saving ? 'not-allowed' : 'pointer',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                flexShrink: 0,
                padding: '0',
                boxSizing: 'border-box',
                opacity: saving ? 0.5 : 1
              }}
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M3 6h18" />
                <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
                <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
              </svg>
            </button>
          )}
        </div>
      )}
      
      {canManage && capabilities?.has_presets && (
        showPresetForm ? (
          <form onSubmit={handleSavePreset} style={{ display: 'flex', flexDirection: 'column', gap: '6px', width: '100%', marginTop: '2px' }}>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '2px', width: '100%' }}>
              <label htmlFor="ptz-preset-name" style={{ color: '#ffffff', fontSize: '10px', fontWeight: '600', textShadow: '0 1px 3px rgba(0,0,0,0.9)' }}>
                {t('live.ptzPresetName')}
              </label>
              <input
                id="ptz-preset-name"
                type="text"
                value={presetName}
                onInput={(event) => setPresetName(event.currentTarget.value)}
                maxLength={64}
                required
                style={{
                  width: '100%',
                  padding: '5px 8px',
                  borderRadius: '8px',
                  border: '1px solid rgba(255, 255, 255, 0.25)',
                  backgroundColor: 'rgba(15, 23, 42, 0.85)',
                  color: 'white',
                  fontSize: '10px',
                  boxSizing: 'border-box',
                  outline: 'none'
                }}
              />
            </div>
            <div style={{ display: 'flex', gap: '4px' }}>
              <button 
                type="submit" 
                disabled={saving || !presetName.trim()} 
                aria-label={t('live.ptzSavePreset')} 
                style={{ 
                  flex: 1, 
                  fontSize: '10px', 
                  padding: '5px 8px', 
                  borderRadius: '8px', 
                  border: '1px solid rgba(59, 130, 246, 0.5)', 
                  backgroundColor: 'rgba(37, 99, 235, 0.85)', 
                  color: 'white', 
                  cursor: saving || !presetName.trim() ? 'not-allowed' : 'pointer',
                  opacity: saving || !presetName.trim() ? 0.5 : 1
                }}
              >
                {t('live.ptzSavePreset')}
              </button>
              <button 
                type="button" 
                disabled={saving} 
                onClick={() => setShowPresetForm(false)} 
                aria-label={t('common.cancel')} 
                style={{ 
                  flex: 1, 
                  fontSize: '10px', 
                  padding: '5px 8px', 
                  borderRadius: '8px', 
                  border: '1px solid rgba(255, 255, 255, 0.2)', 
                  backgroundColor: 'rgba(255, 255, 255, 0.2)', 
                  color: 'white', 
                  cursor: saving ? 'not-allowed' : 'pointer' 
                }}
              >
                {t('common.cancel')}
              </button>
            </div>
          </form>
        ) : (
          <button 
            type="button" 
            onClick={() => { setConfirmSetHome(false); setShowPresetForm(true); }} 
            aria-label={t('live.ptzSaveCurrentPosition')} 
            style={{ 
              width: '100%', 
              fontSize: '10px', 
              fontWeight: '500',
              padding: '6px 10px', 
              borderRadius: '8px', 
              border: '1px solid rgba(255, 255, 255, 0.2)', 
              backgroundColor: 'rgba(15, 23, 42, 0.75)', 
              color: 'white', 
              cursor: 'pointer',
              marginTop: '2px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '6px',
              transition: 'all 0.15s ease'
            }}
          >
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="10" />
              <path d="M12 8v8" />
              <path d="M8 12h8" />
            </svg>
            <span>{t('live.ptzSaveCurrentPosition')}</span>
          </button>
        )
      )}

      {canManage && capabilities?.has_home_position && (
        confirmSetHome ? (
          <div style={{ color: '#ffffff', fontSize: '10px' }}>
            <span>{t('live.ptzConfirmSetHome')}</span>
            <div style={{ display: 'flex', gap: '4px', marginTop: '4px' }}>
              <button type="button" disabled={saving} onClick={handleSetHome} aria-label={t('live.ptzSetHome')} style={{ flex: 1, fontSize: '10px', padding: '4px 6px', borderRadius: '6px', border: '1px solid rgba(59, 130, 246, 0.5)', backgroundColor: 'rgba(37,99,235,0.85)', color: 'white' }}>
                {t('live.ptzSetHome')}
              </button>
              <button type="button" disabled={saving} onClick={() => setConfirmSetHome(false)} aria-label={t('common.cancel')} style={{ flex: 1, fontSize: '10px', padding: '4px 6px', borderRadius: '6px', border: '1px solid rgba(255,255,255,0.2)', backgroundColor: 'rgba(255,255,255,0.2)', color: 'white' }}>
                {t('common.cancel')}
              </button>
            </div>
          </div>
        ) : (
          <button 
            type="button" 
            onClick={() => { setShowPresetForm(false); setConfirmSetHome(true); }} 
            aria-label={t('live.ptzSetCurrentPositionHome')} 
            style={{ 
              width: '100%', 
              fontSize: '10px', 
              fontWeight: '500',
              padding: '6px 10px', 
              borderRadius: '8px', 
              border: '1px solid rgba(255, 255, 255, 0.2)', 
              backgroundColor: 'rgba(15, 23, 42, 0.75)', 
              color: 'white', 
              cursor: 'pointer',
              marginTop: '2px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '6px',
              transition: 'all 0.15s ease'
            }}
          >
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0" />
              <circle cx="12" cy="10" r="3" />
            </svg>
            <span>{t('live.ptzSetCurrentPositionHome')}</span>
          </button>
        )
      )}

      {/* Error message */}
      {error && (
        <div style={{ color: '#f87171', fontSize: '10px', textAlign: 'center', textShadow: '0 1px 3px rgba(0,0,0,0.9)' }}>
          {error}
        </div>
      )}
    </div>
  );
}
