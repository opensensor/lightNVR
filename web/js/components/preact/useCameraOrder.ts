/**
 * useCameraOrder - Custom hook for managing camera display order
 * with drag-and-drop support and localStorage persistence.
 *
 * The hook owns:
 *  - orderedStreams: the streams array sorted by the user-defined order
 *  - reorderMode: boolean toggle – when true, drag handles are shown
 *  - drag handlers: onDragStart / onDragOver / onDrop / onDragEnd
 *
 * All live views (WebRTC / HLS / MSE) share a single storage key so that
 * reordering on one page is reflected on all others.
 */

import { useState, useCallback, useRef } from 'preact/hooks';
import type { LiveStream } from '../../types/live.ts';

type CameraOrderMap = Record<string, number>;

const STORAGE_KEY = 'lightnvr-camera-order';

/**
 * Load persisted order from localStorage.
 * Returns an object mapping stream name → position index.
 */
function loadOrder(): CameraOrderMap {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return {};
    const arr: unknown = JSON.parse(raw);
    if (!Array.isArray(arr)) return {};
    const map: CameraOrderMap = {};
    arr.forEach((name: string, idx: number) => { map[name] = idx; });
    return map;
  } catch {
    return {};
  }
}

/**
 * Persist an ordered array of stream names to localStorage.
 */
function saveOrder(names: string[]) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(names));
  } catch { /* quota errors are non-fatal */ }
}

/**
 * Sort streams according to a saved order map.
 * Streams not in the map appear at the end in original order.
 */
function applyOrder(streams: LiveStream[], orderMap: CameraOrderMap): LiveStream[] {
  if (!streams || streams.length === 0) return streams;
  const known = Object.keys(orderMap);
  if (known.length === 0) return streams;

  return [...streams].sort((a, b) => {
    const ia = orderMap[a.name] !== undefined ? orderMap[a.name] as number : Infinity;
    const ib = orderMap[b.name] !== undefined ? orderMap[b.name] as number : Infinity;
    return ia - ib;
  });
}

/**
 * @param {Array}  streams  - the filtered streams array from LiveView
 * @param {string} _viewType - ignored (kept for API compatibility); all views share one key
 */
export function useCameraOrder(streams: LiveStream[], _viewType?: string) {
  // The persisted user-defined order (stream name → position index)
  const [orderMap, setOrderMap] = useState(() => loadOrder());

  // Whether the drag-reorder UI is active
  const [reorderMode, setReorderMode] = useState(false);

  // Refs used during a drag gesture to avoid stale closure issues
  const dragIndexRef = useRef<number | null>(null);  // index in orderedStreams being dragged
  const pointerDragRef = useRef<number | null>(null);
  const orderRef = useRef(orderMap);  // latest order map
  orderRef.current = orderMap;

  // Apply the current order map to produce the final display list
  const orderedStreams = applyOrder(streams, orderMap);

  // Sync orderMap when new streams arrive (add new entries at the end)
  // This is done inline: applyOrder already handles unknown streams.

  const toggleReorderMode = useCallback(() => {
    setReorderMode(prev => !prev);
  }, []);

  const enterReorderMode = useCallback(() => {
    setReorderMode(true);
  }, []);

  // ---- Drag handlers ----

  const handleDragStart = useCallback((index: number) => {
    dragIndexRef.current = index;
  }, []);

  const moveCameraToIndex = useCallback((index: number) => {
    const fromIndex = dragIndexRef.current;
    if (fromIndex === null || fromIndex === index) return;

    setOrderMap(prev => {
      const current = applyOrder(streams, prev);
      const names = current.map(s => s.name);
      const [moved] = names.splice(fromIndex, 1);
      names.splice(index, 0, moved!);
      const newMap: CameraOrderMap = {};
      names.forEach((name, i) => { newMap[name] = i; });
      dragIndexRef.current = index;
      orderRef.current = newMap;
      return newMap;
    });
  }, [streams]);

  const handleDragOver = useCallback((e: DragEvent, index: number) => {
    e.preventDefault();  // required to allow drop
    moveCameraToIndex(index);
  }, [moveCameraToIndex]);

  const handleDrop = useCallback((e: DragEvent) => {
    e.preventDefault();
    // Persist the current order
    const current = applyOrder(streams, orderRef.current);
    saveOrder(current.map(s => s.name));
    dragIndexRef.current = null;
  }, [streams]);

  const handleDragEnd = useCallback(() => {
    dragIndexRef.current = null;
  }, []);

  const handleReorderPointerDown = useCallback((event: PointerEvent, index: number) => {
    if (event.pointerType !== 'touch' && event.pointerType !== 'pen') return;
    dragIndexRef.current = index;
    pointerDragRef.current = event.pointerId;
    try { (event.currentTarget as Element).setPointerCapture(event.pointerId); } catch { /* optional */ }
    event.preventDefault();
  }, []);

  const handleReorderPointerMove = useCallback((event: PointerEvent) => {
    if (pointerDragRef.current !== event.pointerId) return;
    const target = document.elementFromPoint(event.clientX, event.clientY)
      ?.closest?.<HTMLElement>('[data-camera-order-index]');
    const nextIndex = Number.parseInt(target?.dataset?.cameraOrderIndex ?? '', 10);
    if (Number.isInteger(nextIndex)) moveCameraToIndex(nextIndex);
    event.preventDefault();
  }, [moveCameraToIndex]);

  const finishReorderPointer = useCallback((event: PointerEvent) => {
    if (pointerDragRef.current !== event.pointerId) return;
    const current = applyOrder(streams, orderRef.current);
    saveOrder(current.map(s => s.name));
    pointerDragRef.current = null;
    dragIndexRef.current = null;
  }, [streams]);

  /** Clear persisted order and reset to server default */
  const resetOrder = useCallback(() => {
    localStorage.removeItem(STORAGE_KEY);
    setOrderMap({});
    setReorderMode(false);
  }, []);

  /** Apply an explicit order, such as a server-backed saved Live layout. */
  const setCameraOrder = useCallback((names: string[] | null | undefined) => {
    const uniqueNames = [...new Set((names || []).filter(Boolean))];
    const newMap: CameraOrderMap = {};
    uniqueNames.forEach((name, index) => { newMap[name] = index; });
    saveOrder(uniqueNames);
    setOrderMap(newMap);
    setReorderMode(false);
  }, []);

  /** Place a camera dragged from the Navigator at a grid position. */
  const placeCameraAtIndex = useCallback((cameraUuid: string, index: number) => {
    const stream = streams.find((candidate) => candidate.camera_uuid === cameraUuid);
    if (!stream || !Number.isInteger(index) || index < 0) return;
    setOrderMap((currentMap) => {
      const names = applyOrder(streams, currentMap)
        .map((candidate) => candidate.name)
        .filter((name) => name !== stream.name);
      names.splice(Math.min(index, names.length), 0, stream.name);
      const nextMap: CameraOrderMap = {};
      names.forEach((name, orderIndex) => { nextMap[name] = orderIndex; });
      saveOrder(names);
      return nextMap;
    });
  }, [streams]);

  return {
    orderedStreams,
    reorderMode,
    toggleReorderMode,
    enterReorderMode,
    resetOrder,
    setCameraOrder,
    placeCameraAtIndex,
    handleDragStart,
    handleDragOver,
    handleDrop,
    handleDragEnd,
    handleReorderPointerDown,
    handleReorderPointerMove,
    handleReorderPointerUp: finishReorderPointer,
    handleReorderPointerCancel: finishReorderPointer,
  };
}
