import { useRef, useState } from 'preact/hooks';
import {
  PULL_REFRESH_THRESHOLD_PX,
  resistedPullDistance,
  shouldIgnoreTileGestureTarget,
} from './mobileLiveGestures.js';

export function usePullToRefresh(onRefresh, { disabled = false } = {}) {
  const [distance, setDistance] = useState(0);
  const [refreshing, setRefreshing] = useState(false);
  const startRef = useRef(null);
  const distanceRef = useRef(0);

  const reset = () => {
    startRef.current = null;
    distanceRef.current = 0;
    setDistance(0);
  };

  const onTouchStart = (event) => {
    if (disabled || refreshing || event.touches?.length !== 1 || window.scrollY > 0
        || event.target?.closest?.('[data-zoom-scale]')
        || shouldIgnoreTileGestureTarget(event.target)) return;
    startRef.current = event.touches[0].clientY;
  };

  const onTouchMove = (event) => {
    if (startRef.current === null || event.touches?.length !== 1 || window.scrollY > 0) {
      reset();
      return;
    }
    const nextDistance = resistedPullDistance(event.touches[0].clientY - startRef.current);
    distanceRef.current = nextDistance;
    setDistance(nextDistance);
    if (nextDistance > 0) event.preventDefault();
  };

  const onTouchEnd = async () => {
    const shouldRefresh = distanceRef.current >= PULL_REFRESH_THRESHOLD_PX;
    reset();
    if (!shouldRefresh || refreshing) return;

    setRefreshing(true);
    try {
      await onRefresh?.();
    } finally {
      setRefreshing(false);
    }
  };

  return {
    distance,
    refreshing,
    ready: distance >= PULL_REFRESH_THRESHOLD_PX,
    bind: {
      onTouchStart,
      onTouchMove,
      onTouchEnd,
      onTouchCancel: reset,
    },
  };
}
