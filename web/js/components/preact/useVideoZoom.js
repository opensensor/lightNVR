/**
 * useVideoZoom — digital (client-side) zoom for a video surface.
 *
 * Adds scroll-wheel zoom on desktop and pinch-to-zoom on touch, with
 * drag-to-pan once zoomed in (#465, #453). Purely a CSS transform on the video
 * element: no extra bandwidth, no server round-trip, and it works the same for
 * WebRTC live view and recorded timeline playback.
 *
 * Usage:
 *   const zoom = useVideoZoom();
 *   <div ref={zoom.containerRef} style={{ position: 'relative', overflow: 'hidden' }}>
 *     <video style={{ transform: zoom.transform, transformOrigin: 'center center' }} />
 *   </div>
 *
 * The hook owns no DOM of its own — the caller decides where the transform and
 * the optional reset control go.
 */

import { useCallback, useEffect, useRef, useState } from 'preact/hooks';

export const MIN_ZOOM_SCALE = 1;
export const MAX_ZOOM_SCALE = 8;

// Wheel delta → scale factor. Chosen so one notch on a typical mouse wheel is a
// noticeable but not jarring step, and trackpad inertia stays smooth.
const WHEEL_ZOOM_SENSITIVITY = 0.0015;

// Below this the transform is treated as "not zoomed": panning is released and
// the reset affordance hides, so a stray fractional scale can't leave the video
// subtly offset with no way to tell.
const ZOOM_EPSILON = 0.01;

const clamp = (value, min, max) => Math.min(max, Math.max(min, value));

/**
 * Clamp a pan offset so the scaled video never reveals empty space at its edges.
 * At scale s the video overflows its box by (s-1)/2 in each direction, which is
 * exactly how far it may be panned.
 */
function clampOffset(offset, scale, rect) {
  if (!rect || scale <= MIN_ZOOM_SCALE) return { x: 0, y: 0 };
  const maxX = (rect.width * (scale - 1)) / 2;
  const maxY = (rect.height * (scale - 1)) / 2;
  return {
    x: clamp(offset.x, -maxX, maxX),
    y: clamp(offset.y, -maxY, maxY),
  };
}

/**
 * @param {Object}  [options]
 * @param {boolean} [options.enabled=true] Set false to disable zoom entirely
 *   (e.g. while a modal owns the pointer).
 * @param {number}  [options.maxScale=MAX_ZOOM_SCALE]
 */
export function useVideoZoom({ enabled = true, maxScale = MAX_ZOOM_SCALE } = {}) {
  const containerRef = useRef(null);
  const [scale, setScale] = useState(MIN_ZOOM_SCALE);
  const [offset, setOffset] = useState({ x: 0, y: 0 });

  // Live mirrors of the transform. The wheel/pointer handlers are attached
  // imperatively (they need { passive: false } to be able to preventDefault),
  // so they read current values from refs rather than closing over state.
  const scaleRef = useRef(MIN_ZOOM_SCALE);
  const offsetRef = useRef({ x: 0, y: 0 });
  scaleRef.current = scale;
  offsetRef.current = offset;

  // Active pointers, keyed by pointerId, for pan (1 pointer) and pinch (2).
  const pointersRef = useRef(new Map());
  const panStateRef = useRef(null);
  const pinchStateRef = useRef(null);

  const isZoomed = scale > MIN_ZOOM_SCALE + ZOOM_EPSILON;

  const reset = useCallback(() => {
    setScale(MIN_ZOOM_SCALE);
    setOffset({ x: 0, y: 0 });
  }, []);

  /**
   * Zoom to `nextScale` while keeping the content under (clientX, clientY)
   * pinned in place — the behaviour that makes wheel zoom feel like a
   * magnifying glass rather than a slider.
   */
  const zoomAtPoint = useCallback((nextScaleRaw, clientX, clientY) => {
    const container = containerRef.current;
    if (!container) return;

    const rect = container.getBoundingClientRect();
    if (!rect.width || !rect.height) return;

    const currentScale = scaleRef.current;
    const nextScale = clamp(nextScaleRaw, MIN_ZOOM_SCALE, maxScale);
    if (nextScale === currentScale) return;

    // Cursor position relative to the container's centre, which is also the
    // transform origin.
    const cursorX = clientX - (rect.left + rect.width / 2);
    const cursorY = clientY - (rect.top + rect.height / 2);

    const current = offsetRef.current;
    // Solve for the offset that leaves the point under the cursor unmoved:
    //   cursor = offset + p * scale  ⇒  p = (cursor - offset) / scale
    //   offset' = cursor - p * scale'
    const ratio = nextScale / currentScale;
    const next = {
      x: cursorX - (cursorX - current.x) * ratio,
      y: cursorY - (cursorY - current.y) * ratio,
    };

    setScale(nextScale);
    setOffset(clampOffset(next, nextScale, rect));
  }, [maxScale]);

  const zoomBy = useCallback((factor) => {
    const container = containerRef.current;
    if (!container) return;
    const rect = container.getBoundingClientRect();
    zoomAtPoint(scaleRef.current * factor,
                rect.left + rect.width / 2,
                rect.top + rect.height / 2);
  }, [zoomAtPoint]);

  // ---- Wheel zoom ---------------------------------------------------------
  useEffect(() => {
    const container = containerRef.current;
    if (!container || !enabled) return undefined;

    const handleWheel = (event) => {
      const current = scaleRef.current;

      // Don't swallow the page scroll for a gesture that cannot do anything.
      // Scrolling *out* on an already-unzoomed tile is the common case in a
      // camera grid, and hijacking it would leave the page unscrollable
      // wherever the cursor happens to rest. Zooming in always wins, and once
      // zoomed both directions belong to the video until it is back at 1×.
      const zoomingOut = event.deltaY > 0;
      if (zoomingOut && current <= MIN_ZOOM_SCALE + ZOOM_EPSILON) return;

      event.preventDefault();

      const factor = Math.exp(-event.deltaY * WHEEL_ZOOM_SENSITIVITY);
      zoomAtPoint(current * factor, event.clientX, event.clientY);
    };

    container.addEventListener('wheel', handleWheel, { passive: false });
    return () => container.removeEventListener('wheel', handleWheel);
  }, [enabled, zoomAtPoint]);

  // ---- Pan (1 pointer) and pinch (2 pointers) -----------------------------
  useEffect(() => {
    const container = containerRef.current;
    if (!container || !enabled) return undefined;

    const pointers = pointersRef.current;

    const pinchDistance = () => {
      const [a, b] = Array.from(pointers.values());
      return Math.hypot(a.x - b.x, a.y - b.y);
    };
    const pinchMidpoint = () => {
      const [a, b] = Array.from(pointers.values());
      return { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 };
    };

    const handlePointerDown = (event) => {
      // Never turn a press on an overlay control into a pan — the live-view
      // tiles put snapshot/PTZ/fullscreen buttons on top of the video.
      const target = event.target instanceof Element ? event.target : null;
      if (target?.closest('button, a, input, select, textarea, label, [role="button"]')) {
        return;
      }

      pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
      try {
        container.setPointerCapture(event.pointerId);
      } catch {
        // Pointer capture is unavailable on a few older embedded browsers.
      }

      if (pointers.size === 2) {
        panStateRef.current = null;
        pinchStateRef.current = {
          startDistance: pinchDistance(),
          startScale: scaleRef.current,
        };
        return;
      }

      // Only claim single-pointer drags while zoomed in. At scale 1 there is
      // nothing to pan, and swallowing the gesture would break click-to-play,
      // the native controls, and the timeline's keyboard-nav mode switching.
      if (pointers.size === 1 && scaleRef.current > MIN_ZOOM_SCALE + ZOOM_EPSILON) {
        panStateRef.current = {
          pointerId: event.pointerId,
          startX: event.clientX,
          startY: event.clientY,
          startOffset: offsetRef.current,
        };
      }
    };

    const handlePointerMove = (event) => {
      if (!pointers.has(event.pointerId)) return;
      pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });

      const pinch = pinchStateRef.current;
      if (pinch && pointers.size === 2) {
        const distance = pinchDistance();
        if (pinch.startDistance > 0) {
          const mid = pinchMidpoint();
          zoomAtPoint(pinch.startScale * (distance / pinch.startDistance), mid.x, mid.y);
        }
        event.preventDefault();
        return;
      }

      const pan = panStateRef.current;
      if (pan && pan.pointerId === event.pointerId) {
        const rect = container.getBoundingClientRect();
        const next = {
          x: pan.startOffset.x + (event.clientX - pan.startX),
          y: pan.startOffset.y + (event.clientY - pan.startY),
        };
        setOffset(clampOffset(next, scaleRef.current, rect));
        // Stop the browser turning the drag into a scroll or text selection.
        event.preventDefault();
      }
    };

    const endPointer = (event) => {
      pointers.delete(event.pointerId);
      try {
        if (container.hasPointerCapture(event.pointerId)) {
          container.releasePointerCapture(event.pointerId);
        }
      } catch {
        // The pointer may already have been released by the browser.
      }
      if (pointers.size < 2) pinchStateRef.current = null;
      if (panStateRef.current && panStateRef.current.pointerId === event.pointerId) {
        panStateRef.current = null;
      }
    };

    container.addEventListener('pointerdown', handlePointerDown);
    container.addEventListener('pointermove', handlePointerMove, { passive: false });
    container.addEventListener('pointerup', endPointer);
    container.addEventListener('pointercancel', endPointer);

    return () => {
      container.removeEventListener('pointerdown', handlePointerDown);
      container.removeEventListener('pointermove', handlePointerMove);
      container.removeEventListener('pointerup', endPointer);
      container.removeEventListener('pointercancel', endPointer);
      pointers.clear();
      panStateRef.current = null;
      pinchStateRef.current = null;
    };
  }, [enabled, zoomAtPoint]);

  // Re-clamp the pan when the container is resized (layout change, fullscreen
  // toggle, orientation change) so a previously valid offset can't strand the
  // video off-centre.
  useEffect(() => {
    const container = containerRef.current;
    if (!container || typeof ResizeObserver === 'undefined') return undefined;

    const observer = new ResizeObserver(() => {
      if (scaleRef.current <= MIN_ZOOM_SCALE + ZOOM_EPSILON) return;
      const rect = container.getBoundingClientRect();
      setOffset((prev) => clampOffset(prev, scaleRef.current, rect));
    });
    observer.observe(container);
    return () => observer.disconnect();
  }, []);

  // Drop back to 1× if zoom is turned off mid-gesture, so the video never
  // stays stuck in a transform the user can no longer undo.
  useEffect(() => {
    if (!enabled && scaleRef.current !== MIN_ZOOM_SCALE) reset();
  }, [enabled, reset]);

  const transform = isZoomed
    ? `translate(${offset.x}px, ${offset.y}px) scale(${scale})`
    : '';

  return {
    containerRef,
    scale,
    offset,
    isZoomed,
    transform,
    touchAction: isZoomed ? 'none' : 'pan-y',
    reset,
    zoomBy,
    zoomAtPoint,
  };
}
