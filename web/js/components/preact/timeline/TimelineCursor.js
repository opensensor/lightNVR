/**
 * LightNVR Timeline Cursor Component
 * Displays the playback cursor on the timeline
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect } from '../../../preact.hooks.module.js';
import { timelineState } from './TimelinePage.js';

/**
 * TimelineCursor component
 * @returns {JSX.Element} TimelineCursor component
 */
export function TimelineCursor() {
  // Local state
  const [position, setPosition] = useState(0);
  const [visible, setVisible] = useState(false);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [currentTime, setCurrentTime] = useState(null);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      setStartHour(state.timelineStartHour);
      setEndHour(state.timelineEndHour);
      setCurrentTime(state.currentTime);
      
      // Update time display
      updateTimeDisplay(state.currentTime);
      
      // Update cursor position
      updateCursorPosition(state.currentTime, state.timelineStartHour, state.timelineEndHour);
    });
    
    return () => unsubscribe();
  }, []);

  // Update cursor position
  const updateCursorPosition = (time, startHr, endHr) => {
    if (time === null) {
      setVisible(false);
      return;
    }
    
    // Calculate cursor position
    const date = new Date(time * 1000);
    const hour = date.getHours() + (date.getMinutes() / 60) + (date.getSeconds() / 3600);
    
    // Check if the current time is within the visible range
    if (hour < startHr || hour > endHr) {
      setVisible(false);
      return;
    }
    
    // Calculate position as percentage
    const position = ((hour - startHr) / (endHr - startHr)) * 100;
    
    // Update cursor position
    setPosition(position);
    setVisible(true);
  };

  // Update time display
  const updateTimeDisplay = (time) => {
    if (time === null) return;
    
    const timeDisplay = document.getElementById('time-display');
    if (!timeDisplay) return;
    
    const date = new Date(time * 1000);
    
    // Format date and time
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');
    
    // Display date and time
    timeDisplay.textContent = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  };

  return html`
    <div 
      class="timeline-cursor absolute top-0 h-full w-0.5 bg-red-500 z-10 pointer-events-none transition-all duration-100"
      style="left: ${position}%; display: ${visible ? 'block' : 'none'};"
    ></div>
  `;
}
