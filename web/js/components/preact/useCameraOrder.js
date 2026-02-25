/**
 * useCameraOrder - Custom hook for managing camera display order
 * with drag-and-drop support and localStorage persistence.
 *
 * The hook owns:
 *  - orderedStreams: the streams array sorted by the user-defined order
 *  - reorderMode: boolean toggle – when true, drag handles are shown
 *  - drag handlers: onDragStart / onDragOver / onDrop / onDragEnd
 *
 * Order is keyed by view type ("webrtc" | "hls") and stored in
 * localStorage as an array of stream names.
 */

import { useState, useCallback, useRef } from 'preact/hooks';

const STORAGE_KEY_PREFIX = 'lightnvr-camera-order-';

/**
 * Load persisted order from localStorage.
 * Returns an object mapping stream name → position index.
 */
function loadOrder(viewType) {
  try {
    const raw = localStorage.getItem(STORAGE_KEY_PREFIX + viewType);
    if (!raw) return {};
    const arr = JSON.parse(raw);
    if (!Array.isArray(arr)) return {};
    const map = {};
    arr.forEach((name, idx) => { map[name] = idx; });
    return map;
  } catch {
    return {};
  }
}

/**
 * Persist an ordered array of stream names to localStorage.
 */
function saveOrder(viewType, names) {
  try {
    localStorage.setItem(STORAGE_KEY_PREFIX + viewType, JSON.stringify(names));
  } catch { /* quota errors are non-fatal */ }
}

/**
 * Sort streams according to a saved order map.
 * Streams not in the map appear at the end in original order.
 */
function applyOrder(streams, orderMap) {
  if (!streams || streams.length === 0) return streams;
  const known = Object.keys(orderMap);
  if (known.length === 0) return streams;

  return [...streams].sort((a, b) => {
    const ia = orderMap[a.name] !== undefined ? orderMap[a.name] : Infinity;
    const ib = orderMap[b.name] !== undefined ? orderMap[b.name] : Infinity;
    return ia - ib;
  });
}

/**
 * @param {Array}  streams  - the filtered streams array from LiveView / WebRTCView
 * @param {string} viewType - "webrtc" | "hls"
 */
export function useCameraOrder(streams, viewType) {
  // The persisted user-defined order (stream name → position index)
  const [orderMap, setOrderMap] = useState(() => loadOrder(viewType));

  // Whether the drag-reorder UI is active
  const [reorderMode, setReorderMode] = useState(false);

  // Refs used during a drag gesture to avoid stale closure issues
  const dragIndexRef = useRef(null);  // index in orderedStreams being dragged
  const orderRef = useRef(orderMap);  // latest order map
  orderRef.current = orderMap;

  // Apply the current order map to produce the final display list
  const orderedStreams = applyOrder(streams, orderMap);

  // Sync orderMap when new streams arrive (add new entries at the end)
  // This is done inline: applyOrder already handles unknown streams.

  const toggleReorderMode = useCallback(() => {
    setReorderMode(prev => !prev);
  }, []);

  // ---- Drag handlers ----

  const handleDragStart = useCallback((index) => {
    dragIndexRef.current = index;
  }, []);

  const handleDragOver = useCallback((e, index) => {
    e.preventDefault();  // required to allow drop
    const fromIndex = dragIndexRef.current;
    if (fromIndex === null || fromIndex === index) return;

    // Reorder in-place
    setOrderMap(prev => {
      // Build a names array from the current sorted order
      const current = applyOrder(streams, prev);
      const names = current.map(s => s.name);

      // Splice
      const [moved] = names.splice(fromIndex, 1);
      names.splice(index, 0, moved);

      // Rebuild map
      const newMap = {};
      names.forEach((name, i) => { newMap[name] = i; });

      // Update the drag index so further enters work correctly
      dragIndexRef.current = index;

      return newMap;
    });
  }, [streams]);

  const handleDrop = useCallback((e) => {
    e.preventDefault();
    // Persist the current order
    const current = applyOrder(streams, orderRef.current);
    saveOrder(viewType, current.map(s => s.name));
    dragIndexRef.current = null;
  }, [streams, viewType]);

  const handleDragEnd = useCallback(() => {
    dragIndexRef.current = null;
  }, []);

  /** Clear persisted order and reset to server default */
  const resetOrder = useCallback(() => {
    localStorage.removeItem(STORAGE_KEY_PREFIX + viewType);
    setOrderMap({});
    setReorderMode(false);
  }, [viewType]);

  return {
    orderedStreams,
    reorderMode,
    toggleReorderMode,
    resetOrder,
    handleDragStart,
    handleDragOver,
    handleDrop,
    handleDragEnd,
  };
}

