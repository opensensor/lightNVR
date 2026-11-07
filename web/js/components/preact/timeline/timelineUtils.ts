/**
 * Shared timeline helpers for choosing segments and clipping them to a day view.
 */

import dayjs from 'dayjs';
import type { Dayjs } from 'dayjs';
import type {
  TimelineDayBounds,
  TimelineFlingSample,
  TimelineHourRange,
  TimelineRange,
  TimelineSegment,
} from '../../../types/timeline.ts';

// Not readonly: Array.isArray narrows a readonly array to any[].
type MaybeRanges = TimelineRange[] | null | undefined;
type SelectedDate = string | null | undefined;

interface TimelineSegmentsUrlParams {
  streamName?: string | null;
  cameraUuid?: string | null;
  startTime: string;
  endTime: string;
}

// One minute is the finest useful scale promised by UXD 03 P3.
export const MIN_TIMELINE_VIEW_HOURS = 1 / 60;
export const MAX_TIMELINE_VIEW_HOURS = 24;
export const TIMELINE_EDGE_SNAP_THRESHOLD_SECONDS = 0.5;
export const TIMELINE_FLING_DECAY_PER_MILLISECOND = 0.005;
export const TIMELINE_FLING_SAMPLE_WINDOW_MILLISECONDS = 100;
export const TIMELINE_FLING_MAX_SAMPLE_AGE_MILLISECONDS = 120;

/** Keep UUID deep links independent of historical or reused stream names. */
export function buildTimelineSegmentsUrl({ streamName, cameraUuid, startTime, endTime }: TimelineSegmentsUrlParams): string | null {
  if (!cameraUuid && !streamName) return null;
  const params = new URLSearchParams(cameraUuid
    ? { camera_uuid: cameraUuid } : { stream: streamName as string });
  params.set('start', startTime);
  params.set('end', endTime);
  return `/api/timeline/segments?${params.toString()}`;
}

export function timelineSegmentIdentity(segment: Partial<TimelineSegment> | null | undefined): string {
  if (segment?.id !== null && segment?.id !== undefined && String(segment.id) !== '') {
    return `id:${String(segment.id)}`;
  }
  const streamName = segment?.stream_name ?? segment?.stream ?? '';
  return `range:${streamName}:${segment?.start_timestamp ?? ''}:${segment?.end_timestamp ?? ''}`;
}

/** Keep playback attached to the same recording when polling reorders the list. */
export function reconcileTimelineSelection(
  previous: { timelineSegments: TimelineSegment[]; currentSegmentIndex: number },
  next: TimelineSegment[],
): { timelineSegments: TimelineSegment[]; currentSegmentIndex: number } {
  const active = Number.isInteger(previous.currentSegmentIndex)
    ? previous.timelineSegments[previous.currentSegmentIndex] : undefined;
  const identity = active ? timelineSegmentIdentity(active) : null;
  return {
    timelineSegments: next,
    currentSegmentIndex: identity === null ? -1
      : next.findIndex(segment => timelineSegmentIdentity(segment) === identity),
  };
}

export function reconcileTimelineSegments(existing: TimelineSegment[] = [], incoming: TimelineSegment[] = []) {
  const existingByKey = new Map<string, TimelineSegment>(existing.map((segment) =>
    [timelineSegmentIdentity(segment), segment]));
  const incomingKeys = new Set(incoming.map(timelineSegmentIdentity));
  return {
    added: incoming.filter((segment) => !existingByKey.has(timelineSegmentIdentity(segment))),
    removed: existing.filter((segment) => !incomingKeys.has(timelineSegmentIdentity(segment))),
    updated: incoming.filter((segment) => {
      const previous = existingByKey.get(timelineSegmentIdentity(segment));
      return previous && (previous.end_timestamp !== segment.end_timestamp ||
        previous.in_progress !== segment.in_progress ||
        previous.has_detection !== segment.has_detection);
    }),
    authoritative: [...incoming],
  };
}

export function getTimelinePreviewFrameIndex(segment: TimelineRange, timestamp: number, frameCount = 3): number {
  if (!Number.isFinite(timestamp) || !Number.isFinite(segment?.start_timestamp)
      || !Number.isFinite(segment?.end_timestamp) || frameCount <= 1) return 0;
  const duration = Math.max(segment.end_timestamp - segment.start_timestamp, 0.001);
  const progress = clamp((timestamp - segment.start_timestamp) / duration, 0, 1);
  return Math.round(progress * (frameCount - 1));
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

function getLocalDayStart(selectedDate: SelectedDate): Dayjs | null {
  if (!selectedDate || typeof selectedDate !== 'string' || !selectedDate.includes('-')) {
    return null;
  }

  const localDayStart = dayjs(selectedDate).startOf('day');
  return localDayStart.isValid() ? localDayStart : null;
}

export function getTimelineDayLengthHours(selectedDate: SelectedDate): number {
  const bounds = getLocalDayBounds(selectedDate);
  return bounds?.durationHours ?? MAX_TIMELINE_VIEW_HOURS;
}

export function normalizeTimelineRange(startHour: number, endHour: number, maxHours = MAX_TIMELINE_VIEW_HOURS): TimelineHourRange {
  const cappedMaxHours = Number.isFinite(maxHours) && maxHours > 0
    ? maxHours
    : MAX_TIMELINE_VIEW_HOURS;
  const safeStart = Number.isFinite(startHour) ? startHour : 0;
  const safeEnd = Number.isFinite(endHour) ? endHour : cappedMaxHours;
  const requestedRange = safeEnd - safeStart;
  const range = clamp(requestedRange, MIN_TIMELINE_VIEW_HOURS, cappedMaxHours);
  const maxStart = cappedMaxHours - range;
  const start = clamp(safeStart, 0, maxStart);

  return {
    startHour: start,
    endHour: start + range
  };
}

export function panTimelineRange(startHour: number, endHour: number, deltaHours: number, maxHours = MAX_TIMELINE_VIEW_HOURS): TimelineHourRange {
  const normalized = normalizeTimelineRange(startHour, endHour, maxHours);
  if (!Number.isFinite(deltaHours) || deltaHours === 0) {
    return normalized;
  }

  const range = normalized.endHour - normalized.startHour;
  const cappedMaxHours = Number.isFinite(maxHours) && maxHours > 0
    ? maxHours
    : MAX_TIMELINE_VIEW_HOURS;
  const maxStart = cappedMaxHours - range;
  const nextStart = clamp(normalized.startHour + deltaHours, 0, maxStart);

  return {
    startHour: nextStart,
    endHour: nextStart + range
  };
}

/**
 * Advance an inertial timeline pan by one animation frame.
 *
 * Velocity is expressed in timeline-hours per millisecond. Exponential decay
 * makes the travelled distance independent of the display refresh rate, while
 * stopping immediately at a day boundary avoids wasting animation frames
 * pushing against an already-clamped range.
 */
export function advanceTimelineFling(
  startHour: number,
  endHour: number,
  velocityHoursPerMillisecond: number,
  elapsedMilliseconds: number,
  maxHours = MAX_TIMELINE_VIEW_HOURS,
  decayPerMillisecond = TIMELINE_FLING_DECAY_PER_MILLISECOND
): TimelineHourRange & { velocityHoursPerMillisecond: number } {
  const normalized = normalizeTimelineRange(startHour, endHour, maxHours);
  if (
    !Number.isFinite(velocityHoursPerMillisecond) ||
    velocityHoursPerMillisecond === 0 ||
    !Number.isFinite(elapsedMilliseconds) ||
    !Number.isFinite(decayPerMillisecond) ||
    decayPerMillisecond <= 0
  ) {
    return { ...normalized, velocityHoursPerMillisecond: 0 };
  }

  if (elapsedMilliseconds <= 0) {
    return { ...normalized, velocityHoursPerMillisecond };
  }

  const decay = Math.exp(-decayPerMillisecond * elapsedMilliseconds);
  const requestedDeltaHours = velocityHoursPerMillisecond *
    ((1 - decay) / decayPerMillisecond);
  const nextRange = panTimelineRange(
    normalized.startHour,
    normalized.endHour,
    requestedDeltaHours,
    maxHours
  );
  const actualDeltaHours = nextRange.startHour - normalized.startHour;
  const hitBoundary = Math.abs(actualDeltaHours - requestedDeltaHours) > 1e-9;

  return {
    ...nextRange,
    velocityHoursPerMillisecond: hitBoundary
      ? 0
      : velocityHoursPerMillisecond * decay
  };
}

/**
 * Resolve launch velocity from timestamped drag samples. The position at the
 * start of the sampling window is interpolated, so devices that emit touchmove
 * at different rates produce the same velocity for the same physical motion.
 */
export function calculateTimelineFlingVelocity(
  samples: TimelineFlingSample[],
  releaseTime: number,
  releasePosition: number,
  sampleWindowMilliseconds = TIMELINE_FLING_SAMPLE_WINDOW_MILLISECONDS,
  maxSampleAgeMilliseconds = TIMELINE_FLING_MAX_SAMPLE_AGE_MILLISECONDS
): number {
  if (
    !Array.isArray(samples) ||
    !Number.isFinite(releaseTime) ||
    !Number.isFinite(releasePosition) ||
    !Number.isFinite(sampleWindowMilliseconds) ||
    sampleWindowMilliseconds <= 0 ||
    !Number.isFinite(maxSampleAgeMilliseconds) ||
    maxSampleAgeMilliseconds < 0
  ) {
    return 0;
  }

  const points = samples
    .filter(sample => Number.isFinite(sample?.time) && Number.isFinite(sample?.position))
    .filter(sample => sample.time <= releaseTime)
    .sort((a, b) => a.time - b.time);
  if (points.length === 0) {
    return 0;
  }

  const latestInput = points[points.length - 1]!;
  if (releaseTime - latestInput.time > maxSampleAgeMilliseconds) {
    return 0;
  }

  if (latestInput.time === releaseTime) {
    points[points.length - 1] = { time: releaseTime, position: releasePosition };
  } else {
    points.push({ time: releaseTime, position: releasePosition });
  }

  const windowStart = releaseTime - sampleWindowMilliseconds;
  let startTime = points[0]!.time;
  let startPosition = points[0]!.position;

  if (points[0]!.time < windowStart) {
    for (let index = 1; index < points.length; index++) {
      const previous = points[index - 1]!;
      const current = points[index]!;
      if (current.time < windowStart) {
        continue;
      }

      if (current.time > previous.time) {
        const ratio = (windowStart - previous.time) / (current.time - previous.time);
        startTime = windowStart;
        startPosition = previous.position + ((current.position - previous.position) * ratio);
      } else {
        startTime = current.time;
        startPosition = current.position;
      }
      break;
    }
  }

  const elapsed = releaseTime - startTime;
  return elapsed > 0 ? (releasePosition - startPosition) / elapsed : 0;
}

/**
 * Snap a cursor timestamp to the nearest recording boundary when it is within
 * the mobile gesture tolerance. Ties resolve to the earlier edge so the result
 * stays deterministic even for adjacent recordings.
 */
export function snapTimestampToRecordingEdge(
  timestamp: number,
  segments: MaybeRanges,
  options: { thresholdSeconds?: number; selectedDate?: string | null } = {}
): { timestamp: number; snapped: boolean } {
  const {
    thresholdSeconds = TIMELINE_EDGE_SNAP_THRESHOLD_SECONDS,
    selectedDate = null
  } = options;
  if (
    !Number.isFinite(timestamp) ||
    !Array.isArray(segments) ||
    segments.length === 0 ||
    !Number.isFinite(thresholdSeconds) ||
    thresholdSeconds < 0
  ) {
    return { timestamp, snapped: false };
  }

  let nearestEdge = null as number | null;
  let nearestDistance = Infinity;
  const dayBounds = selectedDate ? getLocalDayBounds(selectedDate) : null;

  segments.forEach((segment) => {
    if (!Number.isFinite(segment?.start_timestamp) || !Number.isFinite(segment?.end_timestamp)) {
      return;
    }

    let visibleStart = segment.start_timestamp;
    let visibleEnd = segment.end_timestamp;
    if (selectedDate) {
      if (!dayBounds) {
        return;
      }
      visibleStart = Math.max(visibleStart, dayBounds.startTimestamp);
      visibleEnd = Math.min(visibleEnd, dayBounds.endTimestamp);
      if (visibleEnd <= visibleStart) {
        return;
      }
      // The selected local day is a half-open interval. Keep a clipped right
      // edge inside that day rather than snapping the cursor to next midnight.
      if (visibleEnd === dayBounds.endTimestamp) {
        visibleEnd -= 0.001;
      }
    }

    [visibleStart, visibleEnd].forEach((edge) => {
      if (!Number.isFinite(edge)) {
        return;
      }
      const distance = Math.abs(timestamp - edge);
      if (
        distance < nearestDistance ||
        (Math.abs(distance - nearestDistance) <= Number.EPSILON &&
          (nearestEdge === null || edge < nearestEdge))
      ) {
        nearestEdge = edge;
        nearestDistance = distance;
      }
    });
  });

  if (nearestEdge === null || nearestDistance > thresholdSeconds) {
    return { timestamp, snapped: false };
  }

  return { timestamp: nearestEdge, snapped: true };
}

export function zoomTimelineRange(startHour: number, endHour: number, zoomFactor: number, anchorHour: number | null = null, maxHours = MAX_TIMELINE_VIEW_HOURS): TimelineHourRange {
  const normalized = normalizeTimelineRange(startHour, endHour, maxHours);
  if (!Number.isFinite(zoomFactor) || zoomFactor <= 0 || zoomFactor === 1) {
    return normalized;
  }

  const cappedMaxHours = Number.isFinite(maxHours) && maxHours > 0
    ? maxHours
    : MAX_TIMELINE_VIEW_HOURS;
  const currentRange = normalized.endHour - normalized.startHour;
  const nextRange = clamp(currentRange * zoomFactor, MIN_TIMELINE_VIEW_HOURS, cappedMaxHours);
  const resolvedAnchorHour = clamp(
    Number.isFinite(anchorHour) ? (anchorHour as number) : ((normalized.startHour + normalized.endHour) / 2),
    normalized.startHour,
    normalized.endHour
  );
  const anchorRatio = currentRange > 0
    ? (resolvedAnchorHour - normalized.startHour) / currentRange
    : 0.5;
  const nextStart = resolvedAnchorHour - (anchorRatio * nextRange);

  return normalizeTimelineRange(nextStart, nextStart + nextRange, cappedMaxHours);
}

function formatLocalDate(date: Dayjs): string {
  return dayjs(date).format('YYYY-MM-DD');
}

export function formatTimestampAsLocalDate(timestamp: number | null | undefined): string {
  if (timestamp === null || timestamp === undefined || !Number.isFinite(timestamp)) {
    return '';
  }

  return dayjs.unix(timestamp).format('YYYY-MM-DD');
}

export function getLocalDayBounds(selectedDate: SelectedDate): TimelineDayBounds | null {
  const dayStart = getLocalDayStart(selectedDate);
  if (!dayStart) {
    return null;
  }

  const nextDayStart = dayStart.add(1, 'day');

  return {
    startTimestamp: dayStart.unix(),
    endTimestamp: nextDayStart.unix(),
    durationHours: nextDayStart.diff(dayStart, 'hour', true)
  };
}

export function formatTimestampAsClock(timestamp: number | null | undefined): string {
  if (timestamp === null || timestamp === undefined) {
    return '';
  }

  return dayjs.unix(timestamp).format('HH:mm:ss');
}

export function formatPlaybackTimeLabel(timestamp: number | null | undefined, streamName = ''): string {
  const formattedTime = formatTimestampAsClock(timestamp);
  const trimmedStreamName = typeof streamName === 'string' ? streamName.trim() : '';

  if (!formattedTime) {
    return trimmedStreamName;
  }

  return trimmedStreamName ? `${trimmedStreamName} - ${formattedTime}` : formattedTime;
}

export function resolvePlaybackStreamName(segments: TimelineSegment[] | null | undefined, currentSegmentIndex: number, timestamp: number | null = null): string {
  if (!Array.isArray(segments) || segments.length === 0) {
    return '';
  }

  const currentSegment = Number.isInteger(currentSegmentIndex) && currentSegmentIndex >= 0 && currentSegmentIndex < segments.length
    ? segments[currentSegmentIndex]
    : null;
  const currentStreamName = typeof currentSegment?.stream === 'string' ? currentSegment.stream.trim() : '';
  if (currentStreamName) {
    return currentStreamName;
  }

  if (!Number.isFinite(timestamp)) {
    return '';
  }

  const containingIndex = findContainingSegmentIndex(segments, timestamp);
  if (containingIndex !== -1) {
    return typeof segments[containingIndex]?.stream === 'string'
      ? segments[containingIndex].stream.trim()
      : '';
  }

  const nearestIndex = findNearestSegmentIndex(segments, timestamp as number);
  return nearestIndex !== -1 && typeof segments[nearestIndex]?.stream === 'string'
    ? segments[nearestIndex].stream.trim()
    : '';
}

export function segmentIntersectsDay(segment: TimelineRange | null | undefined, selectedDate: SelectedDate): boolean {
  return getClippedSegmentHourRange(segment, selectedDate) !== null;
}

export function getAvailableDatesForSegments(segments: MaybeRanges): string[] {
  if (!Array.isArray(segments) || segments.length === 0) {
    return [];
  }

  const dates = new Set<string>();

  segments.forEach(segment => {
    if (!segment || !Number.isFinite(segment.start_timestamp) || !Number.isFinite(segment.end_timestamp)) {
      return;
    }

    const effectiveEnd = Math.max(segment.start_timestamp, segment.end_timestamp - 1);
    let cursor = dayjs.unix(segment.start_timestamp).startOf('day');
    const endDate = dayjs.unix(effectiveEnd).startOf('day');

    while (cursor.isBefore(endDate) || cursor.isSame(endDate, 'day')) {
      dates.add(formatLocalDate(cursor));
      cursor = cursor.add(1, 'day');
    }
  });

  return Array.from(dates).sort();
}

export function countSegmentsForDate(segments: MaybeRanges, selectedDate: SelectedDate): number {
  if (!Array.isArray(segments) || !selectedDate) {
    return 0;
  }

  return segments.reduce((count, segment) => (
    count + (segmentIntersectsDay(segment, selectedDate) ? 1 : 0)
  ), 0);
}

export function timelineOffsetToTimestamp(offsetHours: number, selectedDate: SelectedDate): number {
  const bounds = getLocalDayBounds(selectedDate);
  if (!bounds) {
    throw new Error(`timelineOffsetToTimestamp: invalid selectedDate "${selectedDate}"`);
  }

  const numericOffset = Number(offsetHours);
  if (!Number.isFinite(numericOffset)) {
    throw new Error(`timelineOffsetToTimestamp: invalid offset value "${offsetHours}"`);
  }

  const normalizedOffset = clamp(numericOffset, 0, bounds.durationHours);
  return Math.round(bounds.startTimestamp + normalizedOffset * 3600);
}

export function timestampToTimelineOffset(timestamp: number | null | undefined, selectedDate: string | null = null): number | null {
  if (timestamp === null || timestamp === undefined || !Number.isFinite(timestamp)) {
    return null;
  }

  const effectiveDate = selectedDate || formatTimestampAsLocalDate(timestamp);
  const bounds = getLocalDayBounds(effectiveDate);
  if (!bounds) {
    return null;
  }

  return (timestamp - bounds.startTimestamp) / 3600;
}

export function localClockTimeToTimestamp(timeString: string, selectedDate: SelectedDate): number | null {
  if (!selectedDate || typeof timeString !== 'string') {
    return null;
  }

  const timeMatch = timeString.match(/^(\d{2}):(\d{2}):(\d{2})$/);
  if (!timeMatch) {
    return null;
  }

  const timestamp = dayjs(`${selectedDate}T${timeString}`);
  return timestamp.isValid() ? timestamp.unix() : null;
}

export function formatTimelineOffsetLabel(offsetHours: number, selectedDate: SelectedDate): string {
  const bounds = getLocalDayBounds(selectedDate);
  if (!bounds || !Number.isFinite(offsetHours)) {
    return '';
  }

  const displayTimestamp = bounds.startTimestamp + (offsetHours * 3600);
  return dayjs.unix(displayTimestamp).format('H:mm');
}

export function findFirstVisibleSegmentIndex(segments: MaybeRanges, selectedDate: SelectedDate): number {
  if (!Array.isArray(segments) || segments.length === 0) {
    return -1;
  }

  for (let i = 0; i < segments.length; i++) {
    if (segmentIntersectsDay(segments[i], selectedDate)) {
      return i;
    }
  }

  return -1;
}

export function segmentContainsTimestamp(segment: TimelineRange | null | undefined, timestamp: number | null | undefined): boolean {
  if (!segment || timestamp === null || timestamp === undefined) return false;
  return timestamp >= segment.start_timestamp && timestamp <= segment.end_timestamp;
}

export function findContainingSegmentIndex(segments: MaybeRanges, timestamp: number | null | undefined): number {
  if (!Array.isArray(segments) || segments.length === 0) return -1;

  let bestIndex = -1;
  let bestStart = -Infinity;
  let bestEnd = Infinity;

  for (let i = 0; i < segments.length; i++) {
    const segment = segments[i]!;
    if (!segmentContainsTimestamp(segment, timestamp)) continue;

    if (
      segment.start_timestamp > bestStart ||
      (segment.start_timestamp === bestStart && segment.end_timestamp < bestEnd)
    ) {
      bestIndex = i;
      bestStart = segment.start_timestamp;
      bestEnd = segment.end_timestamp;
    }
  }

  return bestIndex;
}

export function findNearestSegmentIndex(segments: MaybeRanges, timestamp: number): number {
  if (!Array.isArray(segments) || segments.length === 0) return -1;

  let bestIndex = -1;
  let bestDistance = Infinity;
  let bestStart = Infinity;

  for (let i = 0; i < segments.length; i++) {
    const segment = segments[i]!;
    const distance = timestamp < segment.start_timestamp
      ? segment.start_timestamp - timestamp
      : timestamp > segment.end_timestamp
        ? timestamp - segment.end_timestamp
        : 0;

    if (
      distance < bestDistance ||
      (distance === bestDistance && segment.start_timestamp < bestStart)
    ) {
      bestIndex = i;
      bestDistance = distance;
      bestStart = segment.start_timestamp;
    }
  }

  return bestIndex;
}

/** Resolve a scrub to actual footage so the media clock and playhead cannot disagree in a gap. */
export function resolveTimelineSeek(segments: MaybeRanges, timestamp: number) {
  const containingIndex = findContainingSegmentIndex(segments, timestamp);
  const index = containingIndex >= 0 ? containingIndex : findNearestSegmentIndex(segments, timestamp);
  const segment = segments?.[index];
  return {
    index,
    timestamp: segment ? Math.min(segment.end_timestamp, Math.max(segment.start_timestamp, timestamp)) : timestamp,
    inGap: index >= 0 && containingIndex < 0,
  };
}

export function resolveActiveSegmentIndex(segments: MaybeRanges, currentSegmentIndex: number, timestamp: number | null | undefined = null): number {
  if (!Array.isArray(segments) || segments.length === 0) {
    return -1;
  }

  if (timestamp !== null && timestamp !== undefined) {
    const containingIndex = findContainingSegmentIndex(segments, timestamp);
    if (containingIndex !== -1) {
      return containingIndex;
    }

    return findNearestSegmentIndex(segments, timestamp);
  }

  if (Number.isInteger(currentSegmentIndex) &&
      currentSegmentIndex >= 0 &&
      currentSegmentIndex < segments.length) {
    return currentSegmentIndex;
  }

  return -1;
}

export function getSteppedVideoTime(currentTimeSeconds: number, directionSeconds: number, durationSeconds = Infinity): number {
  const safeCurrentTime = Number.isFinite(currentTimeSeconds) ? currentTimeSeconds : 0;
  const safeDirection = Number.isFinite(directionSeconds) ? directionSeconds : 0;
  const nextTime = safeCurrentTime + safeDirection;

  if (!Number.isFinite(durationSeconds)) {
    return Math.max(nextTime, 0);
  }

  const safeDuration = Math.max(durationSeconds, 0);
  return clamp(nextTime, 0, safeDuration);
}

/**
 * Return whether a timeline state update represents a seek that still needs to
 * be applied to the video element.
 *
 * At high playback rates, consecutive native `timeupdate` events can be more
 * than a second apart. Comparing only the current and previous timeline times
 * mistakes those normal playback updates for user seeks and repeatedly assigns
 * `video.currentTime`, which makes Chromium's native controls flicker (#495).
 */
export function shouldSeekPlaybackPosition(
  currentTimelineTime: number | null,
  previousTimelineTime: number | null,
  segmentStartTime: number,
  currentVideoTime: number,
  thresholdSeconds = 1
): boolean {
  if (![currentTimelineTime, previousTimelineTime, segmentStartTime, currentVideoTime, thresholdSeconds]
    .every(Number.isFinite)) {
    return false;
  }

  const timelineMoved = Math.abs((currentTimelineTime as number) - (previousTimelineTime as number)) > thresholdSeconds;
  const desiredVideoTime = (currentTimelineTime as number) - segmentStartTime;
  const videoNeedsSync = Math.abs(desiredVideoTime - currentVideoTime) > thresholdSeconds;

  return timelineMoved && videoNeedsSync;
}

export function getClippedSegmentHourRange(segment: TimelineRange | null | undefined, selectedDate: SelectedDate): TimelineHourRange | null {
  const bounds = getLocalDayBounds(selectedDate);
  if (!segment || !bounds) return null;

  const visibleStart = Math.max(segment.start_timestamp, bounds.startTimestamp);
  const visibleEnd = Math.min(segment.end_timestamp, bounds.endTimestamp);

  if (visibleEnd <= visibleStart) {
    return null;
  }

  return {
    startHour: (visibleStart - bounds.startTimestamp) / 3600,
    endHour: (visibleEnd - bounds.startTimestamp) / 3600
  };
}
