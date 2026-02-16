/**
 * LightNVR Timeline Ruler Component
 * Pure display — reads timelineStartHour / timelineEndHour from global state
 * and renders tick marks + labels.  All range calculations live in
 * TimelinePage (auto-fit on load) and TimelineControls (zoom in/out).
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';

export function TimelineRuler() {
  const [startHour, setStartHour] = useState(timelineState.timelineStartHour ?? 0);
  const [endHour, setEndHour]     = useState(timelineState.timelineEndHour ?? 24);

  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      const s = state.timelineStartHour ?? 0;
      const e = state.timelineEndHour ?? 24;
      setStartHour(s);
      setEndHour(e);
    });
    return () => unsubscribe();
  }, []);

  // Generate hour markers and labels
  const generateHourMarkers = () => {
    const markers = [];
    const hourWidth = 100 / (endHour - startHour);

    // Add hour markers and labels
    for (let hour = Math.floor(startHour); hour <= Math.ceil(endHour); hour++) {
      if (hour >= 0 && hour <= 24) {
        const position = ((hour - startHour) / (endHour - startHour)) * 100;

        // Add hour marker
        markers.push(
          <div
            key={`tick-${hour}`}
            className="absolute top-0 w-px h-5 bg-foreground"
            style={{ left: `${position}%` }}
          ></div>
        );

        // Add hour label
        markers.push(
          <div
            key={`label-${hour}`}
            className="absolute top-0 text-xs text-muted-foreground transform -translate-x-1/2"
            style={{ left: `${position}%` }}
          >
            {hour}:00
          </div>
        );

        // Add half-hour marker when the visible range is ≤ 12 h (zoomed or auto-fit)
        if (hour < 24 && (endHour - startHour) <= 12) {
          const halfHourPosition = ((hour + 0.5 - startHour) / (endHour - startHour)) * 100;
          markers.push(
            <div
              key={`tick-${hour}-30`}
              className="absolute top-2 w-px h-3 bg-muted-foreground"
              style={{ left: `${halfHourPosition}%` }}
            ></div>
          );

          // Add 15-minute markers when visible range is ≤ 6 h
          if ((endHour - startHour) <= 6) {
            const quarterHourPosition1 = ((hour + 0.25 - startHour) / (endHour - startHour)) * 100;
            const quarterHourPosition3 = ((hour + 0.75 - startHour) / (endHour - startHour)) * 100;

            markers.push(
              <div
                key={`tick-${hour}-15`}
                className="absolute top-3 w-px h-2 bg-muted-foreground"
                style={{ left: `${quarterHourPosition1}%` }}
              ></div>
            );

            markers.push(
              <div
                key={`tick-${hour}-45`}
                className="absolute top-3 w-px h-2 bg-muted-foreground"
                style={{ left: `${quarterHourPosition3}%` }}
              ></div>
            );
          }
        }
      }
    }

    return markers;
  };

  return (
    <div className="timeline-ruler relative w-full h-8 bg-muted border-b border-border">
      {generateHourMarkers()}
      <div className="absolute bottom-0 left-0 text-xs text-muted-foreground px-1">
        {`${(endHour - startHour).toFixed(1)}h view`}
      </div>
    </div>
  );
}
