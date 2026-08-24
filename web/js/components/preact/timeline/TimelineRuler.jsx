/**
 * LightNVR Timeline Ruler Component
 * Pure display — reads timelineStartHour / timelineEndHour from global state
 * and renders tick marks + labels.  All range calculations live in
 * TimelinePage (auto-fit on load) and TimelineControls (zoom in/out).
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { formatTimelineOffsetLabel, getTimelineDayLengthHours } from './timelineUtils.js';

export function TimelineRuler() {
  const [startHour, setStartHour] = useState(timelineState.timelineStartHour ?? 0);
  const [endHour, setEndHour] = useState(timelineState.timelineEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate));
  const [selectedDate, setSelectedDate] = useState(timelineState.selectedDate ?? null);

  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      const s = state.timelineStartHour ?? 0;
      const e = state.timelineEndHour ?? getTimelineDayLengthHours(state.selectedDate);
      setStartHour(s);
      setEndHour(e);
      setSelectedDate(state.selectedDate ?? null);
    });
    return () => unsubscribe();
  }, []);

  // Generate scale-aware markers. At mobile pinch depth, hour-only ticks leave
  // an apparently empty ruler, so progressively switch to 30/15/5/1-minute
  // increments as the visible window narrows.
  const generateTimeMarkers = () => {
    const markers = [];
    const dayLengthHours = getTimelineDayLengthHours(selectedDate);
    const visibleHours = endHour - startHour;
    if (visibleHours <= 0) {
      return markers;
    }

    let stepHours = 1;
    if (visibleHours <= (5 / 60)) {
      stepHours = 1 / 60;
    } else if (visibleHours <= 0.5) {
      stepHours = 5 / 60;
    } else if (visibleHours <= 2) {
      stepHours = 0.25;
    } else if (visibleHours <= 6) {
      stepHours = 0.5;
    }

    const firstMarker = Math.ceil((startHour - 1e-9) / stepHours) * stepHours;
    const markerCount = Math.ceil((endHour - firstMarker) / stepHours) + 1;

    for (let index = 0; index < markerCount; index++) {
      const hour = firstMarker + (index * stepHours);
      if (hour >= -1e-9 && hour <= dayLengthHours + 1e-9 && hour <= endHour + 1e-9) {
        const position = ((hour - startHour) / visibleHours) * 100;
        const markerKey = Math.round(hour * 3600);

        markers.push(
          <div
            key={`tick-${markerKey}`}
            className="absolute top-0 w-px h-5 bg-foreground"
            style={{ left: `${position}%` }}
          ></div>
        );

        markers.push(
          <div
            key={`label-${markerKey}`}
            className="absolute top-0 text-xs text-muted-foreground transform -translate-x-1/2"
            style={{ left: `${position}%` }}
          >
            {formatTimelineOffsetLabel(hour, selectedDate)}
          </div>
        );
      }
    }

    return markers;
  };

  const visibleHours = endHour - startHour;
  const viewLabel = visibleHours < 1
    ? `${Math.max(Math.round(visibleHours * 60), 1)}m view`
    : `${visibleHours.toFixed(visibleHours < 10 ? 1 : 0)}h view`;

  return (
    <div
      className="timeline-ruler relative w-full h-8 bg-muted border-b border-border"
      data-timeline-start-hour={startHour}
      data-timeline-end-hour={endHour}
    >
      {generateTimeMarkers()}
      <div className="absolute bottom-0 left-0 text-xs text-muted-foreground px-1">
        {viewLabel}
      </div>
    </div>
  );
}
