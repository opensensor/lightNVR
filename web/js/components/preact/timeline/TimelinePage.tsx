import { RecordingsNavigation } from '../recordings/RecordingsNavigation.tsx';
/**
 * LightNVR Timeline Page Component
 * Main component for the timeline view
 */

import { useState, useEffect, useRef, useCallback, useMemo } from 'preact/hooks';
import { TimelineControls } from './TimelineControls.tsx';
import { TimelineRuler } from './TimelineRuler.tsx';
import { TimelineSegments } from './TimelineSegments.tsx';
import { TimelineCursor } from './TimelineCursor.tsx';
import { TimelinePlayer } from './TimelinePlayer.tsx';
import { CalendarPicker } from './CalendarPicker.tsx';
import { BatchDownloadModal } from '../BatchDownloadModal.tsx';
import { showStatusMessage } from '../ToastContainer.tsx';
import { LoadingIndicator } from '../LoadingIndicator.tsx';
import { useQuery } from '../../../query-client.ts';
import { fetchJSON } from '../../../fetch-utils.ts';
import { useI18n } from '../../../i18n.ts';
import { isReduceMotionActive } from '../../../utils/reduceMotion.ts';
import { fetchAllStreamSummaries } from '../../../utils/stream-summaries.ts';
import {
  currentDateInputValue,
  formatDateForInput,
  formatDisplayDate,
  getLocalDayIsoRange,
  nowMilliseconds
} from '../../../utils/date-utils.ts';
import {
  advanceTimelineFling,
  buildTimelineSegmentsUrl,
  calculateTimelineFlingVelocity,
  countSegmentsForDate,
  findFirstVisibleSegmentIndex,
  findContainingSegmentIndex,
  formatTimestampAsLocalDate,
  getSteppedVideoTime,
  getAvailableDatesForSegments,
  getClippedSegmentHourRange,
  getLocalDayBounds,
  getTimelineDayLengthHours,
  localClockTimeToTimestamp,
  normalizeTimelineRange,
  panTimelineRange,
  reconcileTimelineSegments,
  resolveActiveSegmentIndex,
  timelineOffsetToTimestamp,
  timestampToTimelineOffset,
  zoomTimelineRange
} from './timelineUtils.ts';
import type { t as translate } from '../../../i18n.ts';
import type { StreamSummary } from '../../../types/core.ts';
import type {
  TimelineDetectionInterval,
  TimelineFlingSample,
  TimelineSegment,
  TimelineSegmentsByIdsResponse,
  TimelineSegmentsResponse,
  TimelineState,
} from '../../../types/timeline.ts';

type Translate = typeof translate;
type TimeoutId = ReturnType<typeof setTimeout>;
type KeyboardNavigationMode = 'fine' | 'broad';

interface TimelineHelpProps {
  idsMode: boolean;
  t: Translate;
}

/** A one-finger touch that may become a horizontal pan and fling. */
interface TimelineSinglePan {
  identifier: number;
  startX: number;
  startY: number;
  lastX: number;
  samples: TimelineFlingSample[];
  moved: boolean;
}

const RECORDINGS_RETURN_URL_KEY = 'lightnvr_recordings_return_url';

// Convert an elapsed timeline offset → Unix timestamp (seconds) for the selected local day.
function timelineHourToTimestamp(hour: number, selectedDate: string | null): number {
  return timelineOffsetToTimestamp(hour, selectedDate);
}

// Convert a Unix timestamp to an elapsed timeline offset for the selected local day.
function timestampToTimelineHour(timestamp: number | null, selectedDate: string | null = null): number | null {
  return timestampToTimelineOffset(timestamp, selectedDate);
}

function isEditableKeyboardTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) {
    return false;
  }

  const tagName = target.tagName;
  return (target as HTMLElement).isContentEditable ||
    tagName === 'INPUT' ||
    tagName === 'TEXTAREA' ||
    tagName === 'SELECT';
}

// Global timeline state for child components
const timelineState: TimelineState = {
  streams: [],
  timelineSegments: [],
  selectedStream: null,
  selectedDate: null,
  currentRecordingId: null,
  currentRecordingProtected: false,
  currentRecordingTags: [],
  isPlaying: false,
  currentSegmentIndex: -1,
  timelineStartHour: 0,
  timelineEndHour: 24,
  autoFitStartHour: 0,   // auto-fit range computed from segments
  autoFitEndHour: 24,
  currentTime: null,
  prevCurrentTime: null,
  playbackSpeed: 1.0,
  forceReload: false,
  userControllingCursor: false, // New flag to track if user is controlling cursor
  preserveCursorPosition: false, // New flag to explicitly preserve cursor position
  cursorPositionLocked: false, // New flag to lock the cursor position during playback
  // Utility functions for time conversion
  timelineHourToTimestamp,
  timestampToTimelineHour,
  listeners: new Set(),

  // Last time state was updated
  lastUpdateTime: 0,

  // Pending state updates
  pendingUpdates: {},

  // Prevent re-entrant listener notifications from overlapping state mutations.
  isNotifying: false,

  // Update state and notify listeners
  setState(newState) {
    const now = nowMilliseconds();
    const isTimeOnlyUpdate = newState.currentTime !== undefined &&
      newState.currentSegmentIndex === undefined &&
      // Playback updates need to propagate immediately to keep the cursor and
      // currently playing segment synchronized with the video element.
      newState.isPlaying !== true;

    // Batch frequent time-only updates (≤250 ms apart)
    if (isTimeOnlyUpdate && now - this.lastUpdateTime < 250) {
      return;
    }

    if (this.isNotifying) {
      this.pendingUpdates = { ...this.pendingUpdates, ...newState };
      return;
    }

    // Follow recording changes across gaps without undoing deliberate pan/zoom.
    // Ordinary media-clock updates do not recenter a view the operator has moved.
    if (newState.currentSegmentIndex !== undefined && newState.currentSegmentIndex >= 0 &&
        newState.currentSegmentIndex !== this.currentSegmentIndex &&
        newState.currentTime != null && newState.timelineStartHour === undefined &&
        newState.timelineEndHour === undefined && !this.userControllingCursor) {
      const day = newState.selectedDate ?? this.selectedDate;
      const hour = timestampToTimelineOffset(newState.currentTime, day);
      const start = this.timelineStartHour ?? 0;
      const end = this.timelineEndHour ?? getTimelineDayLengthHours(day);
      if (hour !== null && (hour < start || hour > end)) {
        const range = end - start;
        const next = normalizeTimelineRange(hour - range / 2, hour + range / 2, getTimelineDayLengthHours(day));
        newState = {...newState, timelineStartHour: next.startHour, timelineEndHour: next.endHour};
      }
    }

    Object.assign(this, newState);

    if (newState.forceReload) {
      this.forceReload = false;
    }

    this.lastUpdateTime = now;

    this.isNotifying = true;
    try {
      this.notifyListeners();
    } finally {
      this.isNotifying = false;
    }
  },

  // Subscribe to state changes
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  },

  // Notify all listeners of state change
  notifyListeners() {
    this.listeners.forEach(listener => listener(this));
  },

  // Flush any pending updates
  flushPendingUpdates() {
    if (!this.isNotifying && Object.keys(this.pendingUpdates).length > 0) {
      const pendingUpdates = this.pendingUpdates;
      this.pendingUpdates = {};
      this.setState(pendingUpdates);
    }
  }
};

/**
 * Parse URL parameters
 */
function parseUrlParams() {
  const params = new URLSearchParams(window.location.search);
  return {
    stream: params.get('stream') || '',
    cameraUuid: params.get('camera_uuid') || '',
    date: params.get('date') || currentDateInputValue(),
    time: params.get('time') || '',         // Optional HH:MM:SS to auto-seek on load
    ids: params.get('ids') || '',           // Comma-separated recording IDs for selected-recordings mode
    fullscreen: params.get('fullscreen') === '1', // Prompt user to re-enter fullscreen (from Live View)
    nav: params.get('nav') || '',           // Pre-select keyboard nav mode ('fine' | '')
  };
}

/**
 * Update URL parameters
 */
function updateUrlParams(stream: string, date: string, cameraUuid: string) {
  if (!stream && !cameraUuid) return;
  const url = new URL(window.location.href);
  if (stream) url.searchParams.set('stream', stream);
  else url.searchParams.delete('stream');
  if (cameraUuid) url.searchParams.set('camera_uuid', cameraUuid);
  else url.searchParams.delete('camera_uuid');
  url.searchParams.set('date', date);
  url.searchParams.delete('time');        // Remove one-time seek param after initial load
  url.searchParams.delete('fullscreen');  // Remove one-shot fullscreen hint
  url.searchParams.delete('nav');         // Remove one-shot nav hint
  window.history.replaceState({}, '', url);
}

/**
 * Collapsible help panel — replaces the old large static instructions block.
 */
function TimelineHelp({ idsMode, t }: TimelineHelpProps) {
  const [open, setOpen] = useState(false);
  return (
    <div className="mt-3">
      <button
        className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
        onClick={() => setOpen(!open)}
      >
        <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
            d="M13 16h-1v-4h-1m1-4h.01M12 2a10 10 0 100 20 10 10 0 000-20z" />
        </svg>
        {open ? t('timeline.hideHelp') : t('timeline.howToUseTimeline')}
      </button>
      {open && (
        <ul className="mt-1.5 ms-5 text-xs text-muted-foreground list-disc space-y-0.5">
          {!idsMode && <li>{t('timeline.help.selectStreamAndDate')}</li>}
          {idsMode && <li>{t('timeline.help.refineSelections')}</li>}
          <li>{t('timeline.help.positionCursor')}</li>
          <li>{t('timeline.help.dragPlayhead')}</li>
          <li>{t('timeline.help.clickSegment')}</li>
          <li>{t('timeline.help.arrowKeys')}</li>
          <li>{t('timeline.help.playFromCursor')}</li>
          <li>{t('timeline.help.zoomControls')}</li>
          <li>{t('timeline.help.panControls')}</li>
          <li>{t('timeline.help.actions')}</li>
        </ul>
      )}
    </div>
  );
}

/**
 * TimelinePage component
 */
export function TimelinePage() {
  const { t } = useI18n();
  // Get initial values from URL parameters
  const urlParams = parseUrlParams();

  // Detect selected-recordings mode
  const idsMode = urlParams.ids.length > 0;
  const recordingIds = idsMode ? urlParams.ids : '';

  // State
  const [optionsOpen, setOptionsOpen] = useState(false);
  const [contextOpen, setContextOpen] = useState(false);
  const [streamsList, setStreamsList] = useState<StreamSummary[]>([]);
  const [selectedStream, setSelectedStream] = useState(urlParams.stream);
  const [selectedCameraUuid, setSelectedCameraUuid] = useState(urlParams.cameraUuid);
  const [selectedDate, setSelectedDate] = useState(urlParams.date);
  const [segments, setSegments] = useState<TimelineSegment[]>([]);
  const [detectionIntervals, setDetectionIntervals] = useState<TimelineDetectionInterval[]>([]);
  const [idsLoading, setIdsLoading] = useState(idsMode);
  const [idsSegmentInfo, setIdsSegmentInfo] = useState<TimelineSegmentsByIdsResponse | null>(null);  // metadata from IDs endpoint
  const [idsTimelineSegments, setIdsTimelineSegments] = useState<TimelineSegment[]>([]);
  const [isDownloadModalOpen, setIsDownloadModalOpen] = useState(false);
  const [keyboardNavigationMode, setKeyboardNavigationMode] = useState<KeyboardNavigationMode>(
    urlParams.nav === 'fine' ? 'fine' : 'broad'
  );
  const [timelineContainer, setTimelineContainer] = useState<HTMLDivElement | null>(null);

  useEffect(() => {
    const closeDisclosure = (event: KeyboardEvent) => {
      const target = event.target;
      if (event.key !== 'Escape' || event.defaultPrevented || !(target instanceof Element) ||
          !target.closest('.timeline-page') || target.closest('[role="dialog"]')) return;
      if (optionsOpen) {
        setOptionsOpen(false);
        document.getElementById('timeline-options-toggle')?.focus();
      } else if (contextOpen) {
        setContextOpen(false);
        document.getElementById('timeline-context-toggle')?.focus();
      }
    };
    window.addEventListener('keydown', closeDisclosure);
    return () => window.removeEventListener('keydown', closeDisclosure);
  }, [optionsOpen, contextOpen]);

  // Refs
  const timelineContainerRef = useCallback((node: HTMLDivElement | null) => {
    setTimelineContainer(current => current === node ? current : node);
  }, []);
  const initialLoadRef = useRef(false);
  const flushIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const initialTimeRef = useRef(urlParams.time);  // Store initial time param for auto-seek
  const processedDataRef = useRef<TimelineSegmentsResponse | null>(null);  // Track last processed timeline data
  const selectedDateRef = useRef(urlParams.date);
  const videoElementRef = useRef<HTMLVideoElement | null>(null);
  const pointerDownTimerRef = useRef<TimeoutId | null>(null);

  useEffect(() => {
    selectedDateRef.current = selectedDate;
  }, [selectedDate]);

  const jumpToAdjacentSegment = useCallback((direction: number) => {
    const activeIndex = resolveActiveSegmentIndex(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      timelineState.currentTime
    );

    if (activeIndex === -1) {
      return false;
    }

    const targetIndex = activeIndex + direction;
    const targetSegment = timelineState.timelineSegments?.[targetIndex];
    if (!targetSegment) {
      return true;
    }

    timelineState.setState({
      currentSegmentIndex: targetIndex,
      currentTime: targetSegment.start_timestamp,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: timelineState.isPlaying,
      forceReload: true
    });
    return true;
  }, []);

  const seekCurrentVideo = useCallback((directionSeconds: number) => {
    const videoPlayer = videoElementRef.current;
    if (!(videoPlayer instanceof HTMLVideoElement)) {
      return false;
    }

    const activeIndex = resolveActiveSegmentIndex(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      timelineState.currentTime
    );
    const activeSegment = timelineState.timelineSegments?.[activeIndex];
    if (!activeSegment) {
      return false;
    }

    // Backward auto-advance: if seeking before the start of this segment, jump to the
    // end of the previous segment (mirrors how forward auto-advance works via handleEnded).
    if (directionSeconds < 0 && videoPlayer.currentTime + directionSeconds < 0) {
      const prevIndex = activeIndex - 1;
      const prevSegment = timelineState.timelineSegments?.[prevIndex];
      if (prevSegment) {
        const prevDuration = prevSegment.end_timestamp - prevSegment.start_timestamp;
        // Land 1 second before the end (magnitude of directionSeconds), clamped to [0, duration].
        const seekOffset = Math.max(prevDuration + directionSeconds, 0);
        timelineState.setState({
          currentSegmentIndex: prevIndex,
          currentTime: prevSegment.start_timestamp + seekOffset,
          prevCurrentTime: timelineState.currentTime,
          isPlaying: timelineState.isPlaying,
          forceReload: true
        });
      }
      // Return true whether or not there is a previous segment, so the browser
      // does not scroll the page on the key press.
      return true;
    }

    const fallbackDuration = Math.max(activeSegment.end_timestamp - activeSegment.start_timestamp, 0);
    const effectiveDuration = Number.isFinite(videoPlayer.duration) && videoPlayer.duration > 0
      ? videoPlayer.duration
      : fallbackDuration;

    // Forward auto-advance: if seeking past the end of this segment, jump to
    // the beginning of the next segment (mirrors backward auto-advance above).
    if (directionSeconds > 0 && videoPlayer.currentTime + directionSeconds >= effectiveDuration) {
      const nextIndex = activeIndex + 1;
      const nextSegment = timelineState.timelineSegments?.[nextIndex];
      if (nextSegment) {
        timelineState.setState({
          currentSegmentIndex: nextIndex,
          currentTime: nextSegment.start_timestamp,
          prevCurrentTime: timelineState.currentTime,
          isPlaying: timelineState.isPlaying,
          forceReload: true
        });
      }
      // Return true whether or not there is a next segment, so the browser
      // does not scroll the page on the key press.
      return true;
    }

    const nextTime = getSteppedVideoTime(videoPlayer.currentTime, directionSeconds, effectiveDuration);

    try {
      videoPlayer.currentTime = nextTime;
    } catch (error) {
      console.warn('TimelinePage: unable to seek current video from keyboard navigation', error);
      return false;
    }

    timelineState.setState({
      currentSegmentIndex: activeIndex,
      currentTime: activeSegment.start_timestamp + nextTime,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: timelineState.isPlaying
    });
    return true;
  }, []);

  useEffect(() => {
    const handlePointerDown = (event: PointerEvent) => {
      const target = event.target instanceof Element ? event.target : null;
      // Controls marked with this attribute (speed buttons, action bar, etc.) should
      // not exit 'fine' mode — the user is adjusting playback settings, not leaving
      // the video player context.
      if (target?.closest('[data-keyboard-nav-preserve]')) {
        return;
      }
      const nextMode = target?.closest('#video-player') ? 'fine' : 'broad';
      setKeyboardNavigationMode((prevMode) => prevMode === nextMode ? prevMode : nextMode);
    };

    // separate clicks from double clicks
    // (don't change keyboard navigation mode when double clicking)
    const watchPointerDown = (event: PointerEvent) => {
      if (pointerDownTimerRef.current) {
        clearTimeout(pointerDownTimerRef.current);
        pointerDownTimerRef.current = null;
        return;
      }
      pointerDownTimerRef.current = setTimeout(() => {
        pointerDownTimerRef.current = null;
        handlePointerDown(event);
      }, 200)
    };

    document.addEventListener('pointerdown', watchPointerDown, true);
    return () => {
      document.removeEventListener('pointerdown', watchPointerDown, true);
    };
  }, []);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) {
        return;
      }

      if (isEditableKeyboardTarget(event.target)) {
        return;
      }

      // Space: toggle play/pause on the active video regardless of which element
      // has focus. The native video controls also handle space, but only when the
      // video element itself is focused; this handler makes it global.
      if (event.key === ' ') {
        const videoPlayer = videoElementRef.current;
        if (videoPlayer) {
          // When the video element itself has focus the browser's native controls
          // already handle the space key — if we also toggle here the two actions
          // cancel each other out.  Prevent the default scroll but let the native
          // control do the toggling.
          if (event.target === videoPlayer) {
            event.preventDefault();
            return;
          }
          event.preventDefault();
          // Toggle through timelineState *before* touching the video element.
          // This mirrors the "state first, video second" ordering used by
          // pausePlayback() / resumePlayback() in TimelineControls and
          // eliminates the race condition that causes the repeating play/pause
          // loop in Firefox: previously, calling videoPlayer.pause() made
          // video.paused=true synchronously, but timelineState.isPlaying was
          // still true until the asynchronous 'pause' event fired.  Any
          // handleVideoPlayback call in between (e.g. triggered by a timeupdate)
          // would see isPlaying=true && video.paused=true and call play() again.
          if (timelineState.isPlaying) {
            // Set state first so every subsequent handleVideoPlayback call sees
            // isPlaying=false and does NOT try to resume playback.
            timelineState.setState({ isPlaying: false });
            // handleVideoPlayback synchronously calls video.pause() via the
            // '!isPlaying && !video.paused' branch; also call it directly as a
            // belt-and-suspenders measure for edge cases (e.g. no valid segment).
            videoPlayer.pause();
          } else if (
            timelineState.currentSegmentIndex >= 0 &&
            (timelineState.timelineSegments?.length ?? 0) > 0
          ) {
            // Set state first; handleVideoPlayback synchronously calls
            // video.play() via the 'isPlaying && video.paused' branch.
            timelineState.setState({ isPlaying: true });
          }
        }
        return;
      }

      if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') {
        return;
      }

      const direction = event.key === 'ArrowLeft' ? -1 : 1;
      const handled = keyboardNavigationMode === 'fine'
        ? seekCurrentVideo(direction * (timelineState.playbackSpeed || 1))
        : jumpToAdjacentSegment(direction);

      if (handled) {
        event.preventDefault();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [jumpToAdjacentSegment, keyboardNavigationMode, seekCurrentVideo, videoElementRef]);

  useEffect(() => {
    const container = timelineContainer;
    if (!container) {
      return undefined;
    }

    const handleWheel = (event: WheelEvent) => {
      const startHour = timelineState.timelineStartHour ?? 0;
      const endHour = timelineState.timelineEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
      const currentRange = endHour - startHour;
      if (currentRange <= 0) {
        return;
      }

      const rect = container.getBoundingClientRect();
      if (rect.width <= 0) {
        return;
      }

      if (event.ctrlKey || event.metaKey) {
        event.preventDefault();
        const pointerRatio = Math.min(Math.max((event.clientX - rect.left) / rect.width, 0), 1);
        const anchorHour = startHour + (pointerRatio * currentRange);
        const zoomFactor = event.deltaY < 0 ? 0.8 : 1.25;
        const nextRange = zoomTimelineRange(startHour, endHour, zoomFactor, anchorHour, getTimelineDayLengthHours(timelineState.selectedDate));
        timelineState.setState({
          timelineStartHour: nextRange.startHour,
          timelineEndHour: nextRange.endHour
        });
        return;
      }

      const horizontalDelta = Math.abs(event.deltaX) > Math.abs(event.deltaY)
        ? event.deltaX
        : (event.shiftKey ? event.deltaY : 0);

      if (horizontalDelta !== 0) {
        event.preventDefault();
        const deltaHours = (horizontalDelta / rect.width) * currentRange;
        const nextRange = panTimelineRange(startHour, endHour, deltaHours, getTimelineDayLengthHours(timelineState.selectedDate));
        timelineState.setState({
          timelineStartHour: nextRange.startHour,
          timelineEndHour: nextRange.endHour
        });
      }
    };

    container.addEventListener('wheel', handleWheel, { passive: false });
    return () => {
      container.removeEventListener('wheel', handleWheel);
    };
  }, [timelineContainer]);

  // Mobile timeline gestures: pinch zoom, two-finger pan, and a one-finger
  // inertial horizontal fling. A tap is left untouched so TimelineSegments can
  // still seek through its existing compatibility-mouse path. Once a touch has
  // moved horizontally beyond the intent threshold, we own it and suppress the
  // synthetic click so a fling never moves the playback cursor as a side effect.
  useEffect(() => {
    const container = timelineContainer;
    if (!container) {
      return undefined;
    }

    const PAN_INTENT_THRESHOLD_PX = 6;
    const MIN_FLING_VELOCITY_PX_PER_MS = 0.04;
    const STOP_FLING_VELOCITY_PX_PER_MS = 0.015;

    let pinchActive = false;
    let lastDistance = 0;
    let lastMidX = 0;
    let singlePan: TimelineSinglePan | null = null;
    let ignoreSingleTouchUntilRelease = false;
    let flingFrame: number | null = null;

    const cancelFling = () => {
      if (flingFrame !== null) {
        window.cancelAnimationFrame(flingFrame);
        flingFrame = null;
      }
    };

    const startFling = (velocityHoursPerMillisecond: number, velocityPixelsPerMillisecond: number) => {
      if (
        isReduceMotionActive() ||
        Math.abs(velocityPixelsPerMillisecond) < MIN_FLING_VELOCITY_PX_PER_MS
      ) {
        return;
      }

      let velocity = velocityHoursPerMillisecond;
      let previousFrameTime = performance.now();

      const animate = (frameTime: number) => {
        const elapsed = Math.max(frameTime - previousFrameTime, 0);
        previousFrameTime = frameTime;

        const startHour = timelineState.timelineStartHour ?? 0;
        const endHour = timelineState.timelineEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
        const dayLength = getTimelineDayLengthHours(timelineState.selectedDate);
        const next = advanceTimelineFling(
          startHour,
          endHour,
          velocity,
          elapsed,
          dayLength
        );
        velocity = next.velocityHoursPerMillisecond;

        timelineState.setState({
          timelineStartHour: next.startHour,
          timelineEndHour: next.endHour
        });

        const rect = container.getBoundingClientRect();
        const range = next.endHour - next.startHour;
        const currentPixelVelocity = rect.width > 0 && range > 0
          ? Math.abs((velocity * rect.width) / range)
          : 0;
        if (velocity !== 0 && currentPixelVelocity >= STOP_FLING_VELOCITY_PX_PER_MS) {
          flingFrame = window.requestAnimationFrame(animate);
        } else {
          flingFrame = null;
        }
      };

      flingFrame = window.requestAnimationFrame(animate);
    };

    const distanceAndMid = (touches: TouchList) => {
      const dx = touches[0]!.clientX - touches[1]!.clientX;
      const dy = touches[0]!.clientY - touches[1]!.clientY;
      return {
        distance: Math.hypot(dx, dy),
        midX: (touches[0]!.clientX + touches[1]!.clientX) / 2
      };
    };

    const handleTouchStart = (event: TouchEvent) => {
      cancelFling();

      if (event.touches.length === 2) {
        const { distance, midX } = distanceAndMid(event.touches);
        pinchActive = true;
        singlePan = null;
        ignoreSingleTouchUntilRelease = true;
        lastDistance = distance;
        lastMidX = midX;
        event.preventDefault();
        return;
      }

      const target = event.target instanceof Element ? event.target : null;
      if (
        event.touches.length === 1 &&
        !ignoreSingleTouchUntilRelease &&
        !target?.closest('.timeline-cursor')
      ) {
        const touch = event.touches[0]!;
        const now = performance.now();
        singlePan = {
          identifier: touch.identifier,
          startX: touch.clientX,
          startY: touch.clientY,
          lastX: touch.clientX,
          samples: [{ position: touch.clientX, time: now }],
          moved: false
        };
      }
    };

    const handleTouchMove = (event: TouchEvent) => {
      if (pinchActive && event.touches.length === 2) {
        event.preventDefault();

        const startHour = timelineState.timelineStartHour ?? 0;
        const endHour = timelineState.timelineEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
        const currentRange = endHour - startHour;
        const rect = container.getBoundingClientRect();
        if (currentRange <= 0 || rect.width <= 0) {
          return;
        }

        const { distance, midX } = distanceAndMid(event.touches);
        const dayLength = getTimelineDayLengthHours(timelineState.selectedDate);

        // Zoom: fingers apart (distance grows) -> zoom in (range shrinks).
        let nextStart = startHour;
        let nextEnd = endHour;
        if (lastDistance > 0 && Math.abs(distance - lastDistance) > 1) {
          const zoomFactor = lastDistance / distance;
          const pointerRatio = Math.min(Math.max((midX - rect.left) / rect.width, 0), 1);
          const anchorHour = startHour + (pointerRatio * currentRange);
          const zoomed = zoomTimelineRange(nextStart, nextEnd, zoomFactor, anchorHour, dayLength);
          nextStart = zoomed.startHour;
          nextEnd = zoomed.endHour;
        }

        // Pan: follow the midpoint's horizontal movement between fingers.
        const midDelta = midX - lastMidX;
        if (Math.abs(midDelta) > 0.5) {
          const range = nextEnd - nextStart;
          const deltaHours = -(midDelta / rect.width) * range;
          const panned = panTimelineRange(nextStart, nextEnd, deltaHours, dayLength);
          nextStart = panned.startHour;
          nextEnd = panned.endHour;
        }

        lastDistance = distance;
        lastMidX = midX;
        timelineState.setState({
          timelineStartHour: nextStart,
          timelineEndHour: nextEnd
        });
        return;
      }

      if (!singlePan || event.touches.length !== 1) {
        return;
      }

      const touch = Array.from(event.touches).find(
        candidate => candidate.identifier === singlePan!.identifier
      );
      if (!touch) {
        return;
      }

      const totalDeltaX = touch.clientX - singlePan.startX;
      if (!singlePan.moved) {
        const totalDeltaY = touch.clientY - singlePan.startY;
        if (
          Math.abs(totalDeltaX) < PAN_INTENT_THRESHOLD_PX ||
          Math.abs(totalDeltaX) <= Math.abs(totalDeltaY)
        ) {
          return;
        }
      }

      singlePan.moved = true;
      event.preventDefault();

      const startHour = timelineState.timelineStartHour ?? 0;
      const endHour = timelineState.timelineEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
      const currentRange = endHour - startHour;
      const rect = container.getBoundingClientRect();
      if (currentRange <= 0 || rect.width <= 0) {
        return;
      }

      const now = performance.now();
      const deltaX = touch.clientX - singlePan.lastX;
      singlePan.samples.push({ position: touch.clientX, time: now });
      // Retain one point before the velocity window so the helper can
      // interpolate its exact boundary without accumulating a long gesture.
      while (singlePan.samples.length > 2 && singlePan.samples[1]!.time < now - 120) {
        singlePan.samples.shift();
      }

      const dayLength = getTimelineDayLengthHours(timelineState.selectedDate);
      const deltaHours = -(deltaX / rect.width) * currentRange;
      const next = panTimelineRange(startHour, endHour, deltaHours, dayLength);
      singlePan.lastX = touch.clientX;
      timelineState.setState({
        timelineStartHour: next.startHour,
        timelineEndHour: next.endHour
      });
    };

    const handleTouchEnd = (event: TouchEvent) => {
      if (pinchActive) {
        pinchActive = false;
        singlePan = null;
        if (event.touches.length === 0) {
          ignoreSingleTouchUntilRelease = false;
        }
        return;
      }

      if (event.touches.length === 0) {
        ignoreSingleTouchUntilRelease = false;
      }

      if (!singlePan) {
        return;
      }

      const endedTouch = Array.from(event.changedTouches).find(
        touch => touch.identifier === singlePan!.identifier
      );
      if (!endedTouch) {
        return;
      }

      if (singlePan.moved) {
        event.preventDefault();
        const now = performance.now();
        const velocityPixelsPerMillisecond = calculateTimelineFlingVelocity(
          singlePan.samples,
          now,
          endedTouch.clientX
        );
        const rect = container.getBoundingClientRect();
        const range = (timelineState.timelineEndHour ?? 24) -
          (timelineState.timelineStartHour ?? 0);
        const velocityHoursPerMillisecond = rect.width > 0
          ? -(velocityPixelsPerMillisecond / rect.width) * range
          : 0;
        startFling(velocityHoursPerMillisecond, velocityPixelsPerMillisecond);
      }
      singlePan = null;
    };

    const handleTouchCancel = () => {
      pinchActive = false;
      singlePan = null;
      ignoreSingleTouchUntilRelease = false;
      cancelFling();
    };

    container.addEventListener('touchstart', handleTouchStart, { passive: false });
    container.addEventListener('touchmove', handleTouchMove, { passive: false });
    container.addEventListener('touchend', handleTouchEnd, { passive: false });
    container.addEventListener('touchcancel', handleTouchCancel);
    return () => {
      cancelFling();
      container.removeEventListener('touchstart', handleTouchStart);
      container.removeEventListener('touchmove', handleTouchMove);
      container.removeEventListener('touchend', handleTouchEnd);
      container.removeEventListener('touchcancel', handleTouchCancel);
    };
  }, [timelineContainer]);

  const idsAvailableDates = useMemo(() => (
    idsMode ? getAvailableDatesForSegments(idsTimelineSegments) : []
  ), [idsMode, idsTimelineSegments]);

  const idsSelectedDayIndex = idsAvailableDates.indexOf(selectedDate);
  const idsVisibleSegmentCount = useMemo(() => (
    idsMode ? countSegmentsForDate(idsTimelineSegments, selectedDate) : 0
  ), [idsMode, idsTimelineSegments, selectedDate]);
  const selectedIdsList = useMemo(() => (
    idsMode ? recordingIds.split(',').map(id => id.trim()).filter(Boolean) : []
  ), [idsMode, recordingIds]);
  const selectedRecordingMap = useMemo(() => (
    selectedIdsList.reduce<Record<string, boolean>>((acc, id) => {
      acc[String(id)] = true;
      return acc;
    }, {})
  ), [selectedIdsList]);
  const selectedRecordingsForDownload = useMemo(() => {
    if (!idsMode) return [];

    const seen = new Set<string>();
    return idsTimelineSegments.filter(segment => {
      const key = String(segment.id);
      if (!selectedRecordingMap[key] || seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    });
  }, [idsMode, idsTimelineSegments, selectedRecordingMap]);

  const loadSegmentsIntoTimeline = useCallback((rawSegments: TimelineSegment[] | null | undefined, effectiveDate: string) => {
    const segmentsCopy: TimelineSegment[] = JSON.parse(JSON.stringify(rawSegments || []));

    if (segmentsCopy.length === 0) {
      setSegments([]);
      timelineState.setState({
        timelineSegments: [],
        currentSegmentIndex: -1,
        currentTime: null,
        prevCurrentTime: null,
        isPlaying: false,
        selectedDate: effectiveDate
      });
      return false;
    }

    setSegments(segmentsCopy);

    const dayBounds = getLocalDayBounds(effectiveDate);
    const dayLengthHours = getTimelineDayLengthHours(effectiveDate);
    const visibleIndices: number[] = [];

    // Compute auto-fit range from segments for the selected day
    let earliest = dayLengthHours;
    let latest = 0;
    segmentsCopy.forEach((seg, index) => {
      const visibleRange = getClippedSegmentHourRange(seg, effectiveDate);
      if (!visibleRange) return;
      visibleIndices.push(index);
      earliest = Math.min(earliest, visibleRange.startHour);
      latest = Math.max(latest, visibleRange.endHour);
    });

    if (visibleIndices.length === 0) {
      timelineState.setState({
        timelineSegments: segmentsCopy,
        currentSegmentIndex: -1,
        currentTime: null,
        prevCurrentTime: null,
        isPlaying: false,
        selectedDate: effectiveDate,
        timelineStartHour: 0,
        timelineEndHour: dayLengthHours,
        autoFitStartHour: 0,
        autoFitEndHour: dayLengthHours
      });
      return false;
    }

    const findNearestVisibleIndex = (targetTimestamp: number) => {
      let bestIndex = -1;
      let bestDistance = Infinity;
      let bestStart = Infinity;

      visibleIndices.forEach(index => {
        const seg = segmentsCopy[index]!;
        let distance = 0;
        if (targetTimestamp < seg.start_timestamp) {
          distance = seg.start_timestamp - targetTimestamp;
        } else if (targetTimestamp > seg.end_timestamp) {
          distance = targetTimestamp - seg.end_timestamp;
        }

        if (distance < bestDistance || (distance === bestDistance && seg.start_timestamp < bestStart)) {
          bestDistance = distance;
          bestStart = seg.start_timestamp;
          bestIndex = index;
        }
      });

      return bestIndex;
    };

    // Determine initial time/segment (honour ?time= URL param)
    let initialTime: number | null = null;
    let initialSegmentIndex = -1;
    let preserveExistingPlayback = false;

    if (timelineState.currentTime !== null && timelineState.currentTime !== undefined) {
      const containingIndex = findContainingSegmentIndex(segmentsCopy, timelineState.currentTime);
      if (containingIndex !== -1 && getClippedSegmentHourRange(segmentsCopy[containingIndex], effectiveDate)) {
        initialSegmentIndex = containingIndex;
        initialTime = timelineState.currentTime;
        preserveExistingPlayback = true;
      }
    }

    if (initialSegmentIndex === -1 && initialTimeRef.current) {
      const seekTs = localClockTimeToTimestamp(initialTimeRef.current, effectiveDate);
      if (seekTs !== null) {

        const containingIndex = findContainingSegmentIndex(segmentsCopy, seekTs);
        if (containingIndex !== -1 && getClippedSegmentHourRange(segmentsCopy[containingIndex], effectiveDate)) {
          initialSegmentIndex = containingIndex;
          initialTime = seekTs;
        } else {
          const nearestIndex = findNearestVisibleIndex(seekTs);
          if (nearestIndex !== -1) {
            initialSegmentIndex = nearestIndex;
            initialTime = Math.max(segmentsCopy[nearestIndex]!.start_timestamp, dayBounds!.startTimestamp);
          }
        }
      }
      initialTimeRef.current = '';
    }

    if (initialSegmentIndex === -1) {
      initialSegmentIndex = findFirstVisibleSegmentIndex(segmentsCopy, effectiveDate);
      if (initialSegmentIndex !== -1) {
        initialTime = Math.max(segmentsCopy[initialSegmentIndex]!.start_timestamp, dayBounds!.startTimestamp);
      }
    }

    if (initialSegmentIndex === -1 || initialTime === null) {
      return false;
    }

    let fitStart = 0;
    let fitEnd = dayLengthHours;
    if (earliest < dayLengthHours && latest > 0) {
      const span = latest - earliest;
      const pad = Math.max(0.5, Math.min(1, span * 0.1));
      fitStart = Math.max(0, Math.floor((earliest - pad) * 2) / 2);
      fitEnd = Math.min(dayLengthHours, Math.ceil((latest + pad) * 2) / 2);
      if (fitEnd - fitStart < 2) {
        const center = (earliest + latest) / 2;
        fitStart = Math.max(0, Math.floor((center - 1) * 2) / 2);
        fitEnd = Math.min(dayLengthHours, fitStart + 2);
      }
    }

    // Push to global state
    timelineState.timelineSegments = segmentsCopy;
    timelineState.currentSegmentIndex = initialSegmentIndex;
    timelineState.currentTime = initialTime;
    timelineState.prevCurrentTime = initialTime;
    if (!preserveExistingPlayback) {
      timelineState.isPlaying = false;
    }
    timelineState.forceReload = !preserveExistingPlayback;
    timelineState.autoFitStartHour = fitStart;
    timelineState.autoFitEndHour = fitEnd;
    timelineState.timelineStartHour = fitStart;
    timelineState.timelineEndHour = fitEnd;
    timelineState.selectedDate = effectiveDate;
    timelineState.setState({});

    if (!preserveExistingPlayback) {
      // Safety net: retry if state didn't stick
      setTimeout(() => {
        if (!timelineState.currentTime || timelineState.currentSegmentIndex === -1) {
          timelineState.setState({
            currentSegmentIndex: initialSegmentIndex,
            currentTime: initialTime,
            prevCurrentTime: initialTime,
            selectedDate: effectiveDate
          });
        }
      }, 100);

    }

    return true;
  }, []);

  // Set up periodic flush of pending updates
  useEffect(() => {
    // Set up interval to flush pending updates every 200ms
    flushIntervalRef.current = setInterval(() => {
      timelineState.flushPendingUpdates();
    }, 200);

    // Clean up interval on unmount
    return () => {
      if (flushIntervalRef.current) {
        clearInterval(flushIntervalRef.current);
      }
    };
  }, []);

  // Load streams using preact-query
  const {
    data: streamsData,
    error: streamsError
  } = useQuery({
    queryKey: ['streams', 'timeline-summary', 'live'],
    // Historical playback needs the browser-side fisheye calibration, which
    // is intentionally included only in the playback-oriented summary shape.
    queryFn: ({ signal }) => fetchAllStreamSummaries({ surface: 'live', signal }),
    staleTime: 30000,
  });

  // Handle initial data load when streams are available
  useEffect(() => {
    if (streamsData && Array.isArray(streamsData) && streamsData.length > 0 && !initialLoadRef.current) {
      console.log('TimelinePage: Streams loaded, initializing data');
      initialLoadRef.current = true;

      // Update streamsList state
      setStreamsList(streamsData);

      // Update global state for child components
      timelineState.setState({ streams: streamsData });

      // Preserve UUID deep links even when live-view permission hides the camera
      // from this selector; the playback API authorizes the requested identity.
      const requestedStream = streamsData.find(s => selectedCameraUuid
        ? s.camera_uuid === selectedCameraUuid : s.name === selectedStream);

      if (requestedStream) {
        setSelectedStream(requestedStream.name);
      } else if (!selectedCameraUuid && !idsMode) {
        // Use first stream if URL stream doesn't exist
        const firstStream = streamsData[0]!.name;
        console.log(`TimelinePage: Using first stream: ${firstStream}`);
        setSelectedStream(firstStream);
      }
    }
  }, [streamsData]);

  // Handle streams error
  useEffect(() => {
    if (streamsError) {
      console.error('TimelinePage: Error loading streams:', streamsError);
      showStatusMessage(t('recordings.errorLoadingStreams', { message: streamsError.message }), 'error');
    }
  }, [streamsError]);

  // IDs mode: fetch segments by recording IDs instead of stream+date
  useEffect(() => {
    if (!idsMode || !recordingIds) return;

    const fetchByIds = async () => {
      setIdsLoading(true);
      try {
        const resp = await fetch(`/api/timeline/segments-by-ids?ids=${encodeURIComponent(recordingIds)}`);
        if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
        const data: TimelineSegmentsByIdsResponse = await resp.json();
        setIdsSegmentInfo(data);

        const segs = (data.segments || []).sort((a, b) => a.start_timestamp - b.start_timestamp);
        if (segs.length > 0) {
          const availableDates = getAvailableDatesForSegments(segs);
          const anchorDate = availableDates.includes(selectedDateRef.current)
            ? selectedDateRef.current
            : (availableDates[0] || formatDateForInput(segs[0]!.start_timestamp));

          setIdsTimelineSegments(segs);
          if (anchorDate === selectedDateRef.current) {
            loadSegmentsIntoTimeline(segs, anchorDate);
          }
          if (anchorDate !== selectedDateRef.current) {
            setSelectedDate(anchorDate);
          }

          // Build a pseudo-stream name from unique streams
          const uniqueStreams = [...new Set(segs.map(s => s.stream))];
          setSelectedStream(uniqueStreams.join(', '));
        } else {
          setIdsTimelineSegments([]);
          loadSegmentsIntoTimeline([], selectedDate);
          showStatusMessage(t('timeline.noRecordingsForSelectedIds'), 'info');
        }
      } catch (err) {
        console.error('Error fetching segments by IDs:', err);
        setIdsTimelineSegments([]);
        showStatusMessage(t('timeline.errorLoadingSelectedRecordings', { message: (err as Error).message }), 'error');
      } finally {
        setIdsLoading(false);
      }
    };

    fetchByIds();
  }, [idsMode, recordingIds, loadSegmentsIntoTimeline]);

  useEffect(() => {
    if (!idsMode) return;
    if (idsTimelineSegments.length === 0) return;
    if (idsAvailableDates.length === 0) return;

    const effectiveDate = idsAvailableDates.includes(selectedDate)
      ? selectedDate
      : idsAvailableDates[0]!;

    if (effectiveDate !== selectedDate) {
      setSelectedDate(effectiveDate);
      return;
    }

    loadSegmentsIntoTimeline(idsTimelineSegments, effectiveDate);
  }, [idsMode, idsTimelineSegments, idsAvailableDates, selectedDate, loadSegmentsIntoTimeline]);

  // Calculate time range for timeline data
  const getTimeRange = (date: string) => {
    return getLocalDayIsoRange(date);
  };

  // Update URL and global state when stream or date changes
  useEffect(() => {
    if (idsMode) {
      const url = new URL(window.location.href);
      url.searchParams.set('date', selectedDate);
      url.searchParams.delete('time');
      window.history.replaceState({}, '', url);
      timelineState.setState({ selectedDate });
      return;
    }

    if (selectedStream || selectedCameraUuid) {
      updateUrlParams(selectedStream, selectedDate, selectedCameraUuid);
      timelineState.setState({ selectedStream, selectedDate });
    }
  }, [idsMode, selectedStream, selectedCameraUuid, selectedDate]);

  // Get time range for current date
  const { startTime, endTime } = getTimeRange(selectedDate);

  // Construct the URL for the API call
  const timelineUrl = buildTimelineSegmentsUrl({
    streamName: selectedStream,
    cameraUuid: selectedCameraUuid,
    startTime,
    endTime,
  });

  // Fetch timeline segments using preact-query
  // NOTE: onSuccess/onError were removed in TanStack Query v5 (@preact-signals/query v2.x)
  // so we use a useEffect below to process the data instead.
  const {
    data: timelineData,
    isLoading: isLoadingTimeline,
    error: timelineError,
    refetch: refetchTimeline,
  } = useQuery<TimelineSegmentsResponse>({
    queryKey: ['timeline-segments', selectedStream, selectedCameraUuid, selectedDate],
    queryFn: ({ signal }) => fetchJSON<TimelineSegmentsResponse>(timelineUrl!, {
      signal, timeout: 30000, retries: 0,
    }),
    // Retrying timed-out work can queue more expensive queries on the server.
    retry: false,
    enabled: !!timelineUrl && !idsMode,
  });

  // Process timeline data when it arrives
  useEffect(() => {
    if (timelineError) {
      console.error('TimelinePage: Error loading timeline data:', timelineError.message);
      showStatusMessage(t('timeline.errorLoadingData', { message: timelineError.message }), 'error');
      setSegments([]);
      setDetectionIntervals([]);
      return;
    }

    if (!timelineData || timelineData === processedDataRef.current) return;
    processedDataRef.current = timelineData;

    const timelineSegments = timelineData.segments || [];
    setDetectionIntervals(timelineData.detection_intervals || []);

    if (timelineSegments.length === 0) {
      loadSegmentsIntoTimeline([], selectedDate);
      showStatusMessage(t('timeline.noRecordingsForSelectedDate'), 'warning');
      return;
    }

    loadSegmentsIntoTimeline(timelineSegments, selectedDate);
  }, [timelineData, timelineError, selectedDate, loadSegmentsIntoTimeline]);

  // Wait for the initial load, then leave 10 seconds between completed polls.
  // A fixed interval can pile up queries when a busy server responds slowly.
  useEffect(() => {
    if (idsMode || !timelineUrl || isLoadingTimeline || timelineError || !timelineData) return;

    const POLL_INTERVAL_MS = 10000;
    let fadeTimeoutId: TimeoutId | null = null;
    let cancelled = false;
    let pollTimeoutId: TimeoutId;
    const controller = new AbortController();

    const pollForNewRecordings = async () => {
      try {
        const data = await fetchJSON<TimelineSegmentsResponse>(timelineUrl, {
          signal: controller.signal, timeout: 30000, retries: 0,
        });
        if (cancelled) return;
        const polledSegments = data.segments || [];
        setDetectionIntervals(data.detection_intervals || []);

        const currentSegs = (timelineState.timelineSegments || [])
          .filter((segment) => !segment._removing);
        const {
          added: addedSegs,
          removed: removedSegs,
          updated: updatedSegs,
          authoritative,
        } = reconcileTimelineSegments(currentSegs, polledSegments);

        if (addedSegs.length === 0 && removedSegs.length === 0 && updatedSegs.length === 0) return;

        // Use polled segments as the authoritative list
        const updated = authoritative.sort((a, b) => a.start_timestamp - b.start_timestamp);
        if (removedSegs.length > 0) {
          const fading = [
            ...updated,
            ...removedSegs.map((segment) => ({ ...segment, _removing: true })),
          ].sort((a, b) => a.start_timestamp - b.start_timestamp);
          setSegments(fading);
          timelineState.setState({ timelineSegments: fading });
          clearTimeout(fadeTimeoutId as TimeoutId);
          fadeTimeoutId = setTimeout(() => {
            if (cancelled) return;
            setSegments(updated);
            timelineState.setState({ timelineSegments: updated });
          }, 100);
        } else {
          setSegments(updated);
          timelineState.setState({ timelineSegments: updated });
        }

        if (addedSegs.length > 0) {
          showStatusMessage(
            `${addedSegs.length} new recording${addedSegs.length !== 1 ? 's' : ''} added to timeline`,
            'info',
            3000
          );
        }
        if (removedSegs.length > 0) {
          showStatusMessage(
            `${removedSegs.length} recording${removedSegs.length !== 1 ? 's' : ''} removed from timeline`,
            'info',
            3000
          );
        }
      } catch (err) {
        console.debug('Timeline poll error (suppressed):', err);
      } finally {
        if (!cancelled) pollTimeoutId = setTimeout(pollForNewRecordings, POLL_INTERVAL_MS);
      }
    };

    pollTimeoutId = setTimeout(pollForNewRecordings, POLL_INTERVAL_MS);
    return () => {
      cancelled = true;
      controller.abort();
      clearTimeout(pollTimeoutId);
      clearTimeout(fadeTimeoutId as TimeoutId);
    };
  }, [idsMode, timelineUrl, isLoadingTimeline, timelineError, timelineData]);

  useEffect(() => {
    if (!idsMode || idsAvailableDates.length === 0) return undefined;

    return timelineState.subscribe(({ currentTime }) => {
      if (currentTime === null || currentTime === undefined) return;

      const playbackDate = formatTimestampAsLocalDate(currentTime);
      if (idsAvailableDates.includes(playbackDate) && playbackDate !== selectedDate) {
        setSelectedDate(prev => (prev === playbackDate ? prev : playbackDate));
      }
    });
  }, [idsMode, idsAvailableDates, selectedDate]);

  // When a recording is deleted from the timeline player, remove it from the
  // local segments list (creates the visual gap) and advance to the next segment.
  useEffect(() => {
    const handleTimelineDeleted = (e: Event) => {
      const deletedId = (e as CustomEvent<{ id: TimelineSegment['id'] }>).detail.id;

      setSegments(prev => {
        const idx = prev.findIndex(s => s.id === deletedId);
        const updated = prev.filter(s => s.id !== deletedId);

        // Advance to the next segment (or the previous if it was the last)
        if (updated.length > 0) {
          const nextIdx = Math.min(idx, updated.length - 1);
          const nextSeg = updated[nextIdx]!;

          timelineState.setState({
            timelineSegments: updated,
            currentSegmentIndex: nextIdx,
            currentTime: nextSeg.start_timestamp,
            isPlaying: false,
            forceReload: true
          });

        } else {
          timelineState.setState({
            timelineSegments: [],
            currentSegmentIndex: -1,
            currentTime: null,
            isPlaying: false
          });
        }

        return updated;
      });
    };

    window.addEventListener('timeline-recording-deleted', handleTimelineDeleted);
    return () => window.removeEventListener('timeline-recording-deleted', handleTimelineDeleted);
  }, []);

  const handleStreamChange = (e: Event) => {
    setSelectedStream((e.target as HTMLSelectElement).value);
    setSelectedCameraUuid('');
    // Blur the select so it no longer intercepts keyboard events after the
    // selection is made, preserving the current keyboard-nav mode.
    (e.target as HTMLSelectElement).blur();
  };

  const handleDateChange = (newDate: string) => {
    if (newDate && /^\d{4}-\d{2}-\d{2}$/.test(newDate)) {
      setSelectedDate(newDate);
    } else {
      setSelectedDate(currentDateInputValue());
    }
  };

  const handleIdsDayChange = useCallback((newDate: string) => {
    if (!idsAvailableDates.includes(newDate)) return;
    setSelectedDate(newDate);
  }, [idsAvailableDates]);

  const jumpIdsDay = useCallback((offset: number) => {
    if (idsSelectedDayIndex === -1) return;
    const nextDate = idsAvailableDates[idsSelectedDayIndex + offset];
    if (nextDate) {
      handleIdsDayChange(nextDate);
    }
  }, [handleIdsDayChange, idsAvailableDates, idsSelectedDayIndex]);

  // Render content based on state
  const renderContent = () => {
    const isContentLoading = idsMode ? idsLoading : isLoadingTimeline;

    if (isContentLoading) {
      return <LoadingIndicator message={idsMode ? "Loading selected recordings..." : "Loading timeline data..."} />;
    }

    if (!idsMode && timelineError) {
      return <div role="alert" className="py-12 text-center">
        <p>{t('timeline.errorLoadingData', { message: timelineError.message })}</p>
        <button type="button" className="btn-secondary mt-3" onClick={() => void refetchTimeline()}>
          {t('common.retry')}
        </button>
      </div>;
    }

    if (segments.length === 0) {
      return (
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <svg className="w-16 h-16 text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p className="text-muted-foreground text-lg">
            {t(idsMode ? 'timeline.noSelectedRecordingsVisible' : 'timeline.noRecordingsForDateAndStream')}
          </p>
        </div>
      );
    }

    return (
      <div className="timeline-workspace" data-options-open={optionsOpen}>
        {/* Video player */}
        <TimelinePlayer
          videoElementRef={videoElementRef}
          autoFullscreen={urlParams.fullscreen}
          streamConfig={streamsList.find((stream) => stream.name === selectedStream) || null}
          streamConfigs={streamsList}
        />

        {/* Playback controls (includes time display) */}
        <TimelineControls optionsOpen={optionsOpen} onToggleOptions={() => setOptionsOpen(open => !open)} />

        <div className="timeline-keyboard-hint mb-2 flex justify-end">
          <span
            data-testid="timeline-keyboard-nav-mode"
            className="rounded border border-border bg-secondary px-2 py-1 text-[11px] text-muted-foreground"
            title={t('timeline.arrowKeysHelpTitle')}
          >
            {t('timeline.arrowKeys')}: {keyboardNavigationMode === 'fine' ? t('timeline.seekOneSecond') : t('timeline.jumpRecordings')}
          </span>
        </div>

        {/* Timeline — clicking anywhere on it should not change the keyboard-nav mode */}
        <div
          id="timeline-container"
          dir="ltr"
          data-keyboard-nav-preserve
          className="relative w-full h-24 bg-secondary border border-input rounded-lg mb-2 overflow-hidden"
          ref={timelineContainerRef}
          style={{ touchAction: 'pan-y' }}
        >
          <TimelineRuler />
          <TimelineSegments segments={segments} detectionIntervals={detectionIntervals} />
          <TimelineCursor />

          {/* Inline hint — hidden on small screens where it would overlap and
              obscure the (already compressed) timeline segments (#453). The same
              guidance is available in the collapsible "How to use" help panel. */}
          <div className="timeline-inline-hint hidden sm:block absolute bottom-1 right-2 text-[10px] text-muted-foreground bg-card/75 px-1.5 py-0.5 rounded">
            {t('timeline.inlineHint')}
          </div>
        </div>

        {/* Legend — explains the segment colours so users know what the green
            (detection) bars mean without hunting through the help panel (#454). */}
        <div className="timeline-legend flex flex-wrap items-center gap-x-4 gap-y-1 mb-2 text-[11px] text-muted-foreground">
          <span className="inline-flex items-center gap-1.5">
            <span
              className="inline-block w-3 h-3 rounded-sm"
              style={{ backgroundColor: 'hsl(var(--primary))' }}
            />
            {t('timeline.legend.recording')}
          </span>
          <span className="inline-flex items-center gap-1.5">
            <span
              className="inline-block w-3 h-3 rounded-sm timeline-legend-in-progress"
            />
            {t('timeline.legend.inProgress')}
          </span>
          <span className="inline-flex items-center gap-1.5">
            <span
              className="inline-block w-3 h-3 rounded-sm"
              style={{ backgroundColor: 'hsl(var(--success))' }}
            />
            {t('timeline.legend.detection')}
          </span>
        </div>
      </div>
    );
  };

  // Get return URL for "Refine Selections" link
  const returnUrl = idsMode ? (sessionStorage.getItem(RECORDINGS_RETURN_URL_KEY) || 'recordings.html') : null;
  const navigationBounds = getLocalDayBounds(selectedDate);

  return (
    <div className="timeline-page">
      <div className="timeline-heading flex flex-wrap items-center gap-x-4 gap-y-2 mb-4">
        <h1 className="text-2xl font-bold">
          {idsMode ? t('timeline.selectedRecordingsTimeline') : t('timeline.timelinePlayback')}
        </h1>

      </div>
      <RecordingsNavigation active="timeline" recordingsUrl={returnUrl || new URLSearchParams(window.location.search).get('return_to')}
        streams={selectedStream ? [selectedStream] : []} cameras={selectedCameraUuid ? [selectedCameraUuid] : []}
        start={navigationBounds?.startTimestamp} end={navigationBounds?.endTimestamp} cursor={timelineState.currentTime} />


      <div className="timeline-context" data-context-open={contextOpen}>
        <button type="button" id="timeline-context-toggle" className="timeline-mobile-toggle timeline-context-toggle"
          aria-expanded={contextOpen} aria-controls="timeline-context-fields"
          aria-label={t('timeline.cameraAndDate')}
          onClick={() => setContextOpen(open => !open)}>
          <span>{idsMode ? t('timeline.recordingsCount', {count: segments.length}) : selectedStream || t('nav.streams')} · {formatDisplayDate(selectedDate)}</span>
          <span aria-hidden="true">{contextOpen ? '−' : '+'}</span>
        </button>
      <div id="timeline-context-fields">
      {idsMode ? (
        /* IDs mode: compact info bar */
        <div className="flex flex-wrap items-center gap-3 mb-3 px-3 py-2 bg-secondary rounded-lg text-sm">
          <span className="font-medium">
            {t('timeline.recordingsCount', { count: segments.length })}
            {idsSegmentInfo?.multi_stream && ` · ${t('timeline.streamsCount', { count: [...new Set(segments.map(s => s.stream))].length })}`}
          </span>
          {idsSegmentInfo && (
            <span className="text-xs text-muted-foreground">
              {idsSegmentInfo.start_time} — {idsSegmentInfo.end_time}
            </span>
          )}
          {idsAvailableDates.length > 0 && (
            <div className="flex flex-wrap items-center gap-2 rounded-md border border-border/60 bg-background/70 px-2 py-1">
              <span className="text-[11px] uppercase tracking-wide text-muted-foreground">{t('timeline.activeDay')}</span>
              <button
                type="button"
                data-keyboard-nav-preserve
                className="btn-secondary text-xs px-2 py-1 disabled:opacity-50"
                disabled={idsSelectedDayIndex <= 0}
                onClick={() => jumpIdsDay(-1)}
                title={t('timeline.previousSelectedDay')}
              >
                <span className="rtl-flip inline-block" aria-hidden="true">←</span>
              </button>
              <select
                data-keyboard-nav-preserve
                className="min-w-[180px] rounded border border-border bg-background px-2 py-1 text-xs"
                value={selectedDate}
                onChange={(e) => handleIdsDayChange((e.target as HTMLSelectElement).value)}
              >
                {idsAvailableDates.map((date, index) => (
                  <option key={date} value={date}>
                    {t('timeline.dayNumberWithDate', { number: index + 1, date: formatDisplayDate(date) })}
                  </option>
                ))}
              </select>
              <button
                type="button"
                data-keyboard-nav-preserve
                className="btn-secondary text-xs px-2 py-1 disabled:opacity-50"
                disabled={idsSelectedDayIndex === -1 || idsSelectedDayIndex >= idsAvailableDates.length - 1}
                onClick={() => jumpIdsDay(1)}
                title={t('timeline.nextSelectedDay')}
              >
                <span className="rtl-flip inline-block" aria-hidden="true">→</span>
              </button>
              <span className="text-xs text-muted-foreground">
                {idsSelectedDayIndex === -1 ? '' : t('timeline.dayIndexOfTotal', { index: idsSelectedDayIndex + 1, total: idsAvailableDates.length })}
                {idsVisibleSegmentCount > 0 && ` · ${t('timeline.visibleCount', { count: idsVisibleSegmentCount })}`}
              </span>
            </div>
          )}
          <div className="ms-auto flex gap-2">
            <a href={returnUrl || 'recordings.html'} data-keyboard-nav-preserve className="btn-secondary text-xs px-2 py-1">
              <span className="rtl-flip inline-block" aria-hidden="true">←</span> {t('timeline.refineSelections')}
            </a>
            <button
              data-keyboard-nav-preserve
              className="btn-primary text-xs px-2 py-1"
              onClick={() => setIsDownloadModalOpen(true)}
              disabled={selectedRecordingsForDownload.length === 0}
            >
              ↓ {t('timeline.downloadAll', { count: selectedRecordingsForDownload.length })}
            </button>
          </div>
        </div>
      ) : (
        /* Normal mode: compact single-row stream + date selectors */
        <div className="flex flex-wrap items-end gap-3 mb-3">
          <div className="flex-grow min-w-[180px]">
            <label htmlFor="stream-selector" className="block text-xs text-muted-foreground mb-1">{t('nav.streams')}</label>
            <select
              id="stream-selector"
              data-keyboard-nav-preserve
              className="w-full p-1.5 text-sm border border-border rounded bg-background text-foreground"
              value={selectedStream || ''}
              onChange={handleStreamChange}
            >
              <option value="" disabled>{t('timeline.selectStreamCount', { count: streamsList.length })}</option>
              {streamsList.map(stream => (
                <option key={stream.name} value={stream.name}>{stream.name}</option>
              ))}
            </select>
          </div>
          <div className="min-w-[160px]" data-keyboard-nav-preserve>
            <label className="block text-xs text-muted-foreground mb-1">{t('timeline.date')}</label>
            <CalendarPicker value={selectedDate} onChange={handleDateChange} />
          </div>
          {isLoadingTimeline && (
            <span className="text-xs text-muted-foreground italic">{t('common.loading')}</span>
          )}
        </div>
      )}

      </div>
      </div>

      {/* Content */}
      {renderContent()}

      <BatchDownloadModal
        isOpen={isDownloadModalOpen}
        onClose={() => setIsDownloadModalOpen(false)}
        recordings={selectedRecordingsForDownload}
        selectedIds={selectedRecordingMap}
      />

      {/* Compact help — collapsible, replaces the old large instructions block */}
      <TimelineHelp idsMode={idsMode} t={t} />
    </div>
  );
}

// Export the timeline state for use in other components
export { timelineState };
