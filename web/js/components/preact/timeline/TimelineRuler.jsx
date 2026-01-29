/**
 * LightNVR Timeline Ruler Component
 * Displays the time ruler with hour markers
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';

/**
 * TimelineRuler component
 * @returns {JSX.Element} TimelineRuler component
 */
export function TimelineRuler() {
  // Local state
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [zoomLevel, setZoomLevel] = useState(1);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      console.log('TimelineRuler: State update received', {
        zoomLevel: state.zoomLevel,
        segmentsCount: state.timelineSegments ? state.timelineSegments.length : 0,
        currentTime: state.currentTime
      });

      // Calculate time range based on zoom level
      const hoursPerView = 24 / state.zoomLevel;

      // Calculate the center hour based on current time or segments
      let centerHour = 12; // Default to noon

      if (state.currentTime !== null) {
        // If we have a current time, use it as the center
        const currentDate = new Date(state.currentTime * 1000);
        centerHour = currentDate.getHours() + (currentDate.getMinutes() / 60) + (currentDate.getSeconds() / 3600);
      } else if (state.timelineSegments && state.timelineSegments.length > 0) {
        // If we have segments but no current time, use the middle of the segments
        let earliestHour = 24;
        let latestHour = 0;

        state.timelineSegments.forEach(segment => {
          const startTime = new Date(segment.start_timestamp * 1000);
          const endTime = new Date(segment.end_timestamp * 1000);

          const startHour = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
          const endHour = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);

          earliestHour = Math.min(earliestHour, startHour);
          latestHour = Math.max(latestHour, endHour);
        });

        centerHour = (earliestHour + latestHour) / 2;
        console.log('TimelineRuler: Calculated center from segments', { earliestHour, latestHour, centerHour });
      }

      // Calculate start and end hours based on center and zoom level
      let newStartHour = Math.max(0, centerHour - (hoursPerView / 2));
      let newEndHour = Math.min(24, newStartHour + hoursPerView);

      // Adjust start hour if end hour is at the limit
      if (newEndHour === 24 && hoursPerView < 24) {
        newStartHour = Math.max(0, 24 - hoursPerView);
        newEndHour = 24;
      } else if (newStartHour === 0 && hoursPerView < 24) {
        newEndHour = Math.min(24, hoursPerView);
      }

      // Update local state
      setStartHour(newStartHour);
      setEndHour(newEndHour);
      setZoomLevel(state.zoomLevel);

      console.log('TimelineRuler: Calculated time range', {
        newStartHour,
        newEndHour,
        hoursPerView,
        centerHour
      });

      // Only update global state if the values have actually changed
      // to prevent infinite recursion
      if (timelineState.timelineStartHour !== newStartHour ||
          timelineState.timelineEndHour !== newEndHour) {
        console.log('TimelineRuler: Updating global state with new time range');
        timelineState.setState({
          timelineStartHour: newStartHour,
          timelineEndHour: newEndHour
        });
      }
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

        // Add half-hour marker if not the last hour and we're zoomed in enough
        if (hour < 24 && zoomLevel >= 2) {
          const halfHourPosition = ((hour + 0.5 - startHour) / (endHour - startHour)) * 100;
          markers.push(
            <div
              key={`tick-${hour}-30`}
              className="absolute top-2 w-px h-3 bg-muted-foreground"
              style={{ left: `${halfHourPosition}%` }}
            ></div>
          );

          // Add 15-minute markers if zoomed in even more
          if (zoomLevel >= 4) {
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
        Zoom: {zoomLevel}x ({Math.round(24 / zoomLevel)} hours)
      </div>
    </div>
  );
}
