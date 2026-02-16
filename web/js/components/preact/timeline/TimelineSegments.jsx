/**
 * LightNVR Timeline Segments Component
 * Displays recording segments on the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';

/**
 * TimelineSegments component
 * @param {Object} props Component props
 * @param {Array} props.segments Array of timeline segments
 * @returns {JSX.Element} TimelineSegments component
 */
export function TimelineSegments({ segments: propSegments }) {
  // Local state
  const [segments, setSegments] = useState(propSegments || []);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [currentSegmentIndex, setCurrentSegmentIndex] = useState(-1);

  // Update segments when props change
  useEffect(() => {
    if (propSegments && propSegments.length > 0) {
      setSegments(propSegments);
    }
  }, [propSegments]);

  // Refs
  const containerRef = useRef(null);
  const isDragging = useRef(false);
  const lastSegmentsRef = useRef([]);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Update segments when they change
      if (state.timelineSegments) {
        const changed = state.timelineSegments.length !== lastSegmentsRef.current.length
          || state.forceReload
          || state.timelineSegments !== lastSegmentsRef.current;
        if (changed) {
          setSegments(state.timelineSegments);
          lastSegmentsRef.current = state.timelineSegments;
        }
      }

      setStartHour(state.timelineStartHour ?? 0);
      setEndHour(state.timelineEndHour ?? 24);
      setCurrentSegmentIndex(state.currentSegmentIndex ?? -1);
    });

    // Hydrate from global state on mount
    if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
      setSegments(timelineState.timelineSegments);
      lastSegmentsRef.current = timelineState.timelineSegments;
      setCurrentSegmentIndex(timelineState.currentSegmentIndex ?? 0);
      if (timelineState.timelineStartHour !== undefined) setStartHour(timelineState.timelineStartHour);
      if (timelineState.timelineEndHour !== undefined)   setEndHour(timelineState.timelineEndHour);
    }

    return () => unsubscribe();
  }, []);

  // Set up drag handling
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const handleMouseDown = (e) => {
      // Handle clicks on the container, clickable area, or directly on segments
      if (e.target === container || e.target.classList.contains('timeline-clickable-area') || e.target.classList.contains('timeline-segment')) {
        isDragging.current = true;
        handleTimelineClick(e);

        // Add event listeners for drag
        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
      }
    };

    const handleMouseMove = (e) => {
      if (!isDragging.current) return;
      handleTimelineClick(e);
    };

    const handleMouseUp = () => {
      isDragging.current = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    container.addEventListener('mousedown', handleMouseDown);

    return () => {
      container.removeEventListener('mousedown', handleMouseDown);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [startHour, endHour, segments]);

  // Handle click on timeline for seeking
  const handleTimelineClick = (event) => {
    const container = containerRef.current;
    if (!container) return;

    const rect = container.getBoundingClientRect();
    const clickPercent = (event.clientX - rect.left) / rect.width;
    const clickHour = startHour + clickPercent * (endHour - startHour);

    // Convert fractional hour → timestamp using the shared utility
    const clickTimestamp = timelineState.timelineHourToTimestamp(clickHour, timelineState.selectedDate);

    // Move cursor to click position (don't auto-play)
    timelineState.setState({
      currentTime: clickTimestamp,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: false
    });

    // Find segment that contains this timestamp
    let foundSegment = false;
    for (let i = 0; i < segments.length; i++) {
      const segment = segments[i];
      const st = segment.start_timestamp;
      const et = segment.end_timestamp;

      if (clickTimestamp >= st && clickTimestamp <= et) {
        timelineState.setState({ currentSegmentIndex: i });

        // Direct click on a segment bar → start playback at that point
        if (event.target.classList.contains('timeline-segment')) {
          playSegment(i, clickTimestamp - st);
        }
        foundSegment = true;
        break;
      }
    }

    if (!foundSegment) {
      timelineState.setState({ currentSegmentIndex: -1 });
    }
  };

  // Play a specific segment
  const playSegment = (index, relativeTime = null) => {
    if (index < 0 || index >= segments.length) return;

    const segment = segments[index];
    const startTimestamp = segment.start_timestamp;
    const absoluteTime = relativeTime !== null ? startTimestamp + relativeTime : startTimestamp;

    // First, pause any current playback and reset the segment index
    timelineState.setState({
      isPlaying: false,
      currentSegmentIndex: -1
    });

    // Force a synchronous DOM update
    document.body.offsetHeight;

    // Now set the new segment index and start playing
    setTimeout(() => {
      timelineState.setState({
        currentSegmentIndex: index,
        currentTime: absoluteTime,
        isPlaying: true,
        forceReload: true
      });

      // Force the video player to reload
      setTimeout(() => {
        const videoElement = document.querySelector('#video-player video');
        if (videoElement) {
          // Pause any current playback
          videoElement.pause();

          // Clear the source and reload
          videoElement.removeAttribute('src');
          videoElement.load();

          // Set the new source
          videoElement.src = `/api/recordings/play/${segment.id}?t=${Date.now()}`;

          // Set the current time and play
          videoElement.onloadedmetadata = () => {
            const seekTime = relativeTime !== null ? relativeTime : 0;
            videoElement.currentTime = seekTime;
            videoElement.play().catch(e => {
              if (e.name === 'AbortError') return;
              console.error('Error playing video:', e);
            });
          };
        }
      }, 50);
    }, 50);
  };

  // ── Merge adjacent segments (gap ≤ 1 s) and render ──
  const renderSegments = () => {
    if (!segments || segments.length === 0) {
      return (
        <div className="absolute inset-0 flex items-center justify-center text-muted-foreground text-sm">
          No segments to display
        </div>
      );
    }

    const hourRange = endHour - startHour;
    if (hourRange <= 0) return null;

    // Sort + merge adjacent segments
    const sorted = [...segments].sort((a, b) => a.start_timestamp - b.start_timestamp);
    const merged = [];
    let cur = { ...sorted[0] };

    for (let i = 1; i < sorted.length; i++) {
      const seg = sorted[i];
      if (seg.start_timestamp - cur.end_timestamp <= 1) {
        // extend current merged segment
        cur.end_timestamp = Math.max(cur.end_timestamp, seg.end_timestamp);
        if (seg.has_detection) cur.has_detection = true;
      } else {
        merged.push(cur);
        cur = { ...seg };
      }
    }
    merged.push(cur);

    // Render each merged segment as a positioned bar
    const rendered = [];
    merged.forEach((seg, i) => {
      const sh = timelineState.timestampToTimelineHour(seg.start_timestamp);
      const eh = timelineState.timestampToTimelineHour(seg.end_timestamp);

      // Clip to visible range
      if (eh <= startHour || sh >= endHour) return;
      const vStart = Math.max(sh, startHour);
      const vEnd   = Math.min(eh, endHour);

      const leftPct  = ((vStart - startHour) / hourRange) * 100;
      const widthPct = ((vEnd - vStart) / hourRange) * 100;

      // Tooltip
      const t0 = new Date(seg.start_timestamp * 1000).toLocaleTimeString();
      const t1 = new Date(seg.end_timestamp * 1000).toLocaleTimeString();
      const dur = Math.round(seg.end_timestamp - seg.start_timestamp);
      const durLabel = dur >= 3600
        ? `${Math.floor(dur / 3600)}h ${Math.floor((dur % 3600) / 60)}m`
        : dur >= 60
          ? `${Math.floor(dur / 60)}m ${dur % 60}s`
          : `${dur}s`;

      rendered.push(
        <div
          key={`seg-${i}`}
          className={`timeline-segment ${seg.has_detection ? 'has-detection' : ''}`}
          style={{
            left: `${leftPct}%`,
            width: `${Math.max(widthPct, 0.15)}%`,   // min width so tiny segments stay visible
          }}
          title={`${t0} – ${t1}  (${durLabel})`}
        />
      );
    });

    return rendered;
  };

  return (
    <div
      className="timeline-segments relative w-full h-16"
      ref={containerRef}
    >
      {renderSegments()}
    </div>
  );
}
