/**
 * Formatting utility functions for RecordingsView
 */

export const formatUtils = {
  /**
   * Format date time
   * @param {string} isoString ISO date string
   * @returns {string} Formatted date time
   */
  formatDateTime: (isoString) => {
    if (!isoString) return '';
    
    const date = new Date(isoString);
    return date.toLocaleString();
  },
  
  /**
   * Format duration
   * @param {number} seconds Duration in seconds
   * @returns {string} Formatted duration
   */
  formatDuration: (seconds) => {
    if (!seconds) return '00:00:00';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    
    return [
      hours.toString().padStart(2, '0'),
      minutes.toString().padStart(2, '0'),
      secs.toString().padStart(2, '0')
    ].join(':');
  },
  
  /**
   * Build a timeline URL for a recording, converting the UTC start_time to local date/time.
   * The API returns start_time as "YYYY-MM-DD HH:MM:SS UTC". TimelinePage interprets the
   * date/time URL params as local time, so we must convert before building the URL.
   * @param {string} stream - Stream name
   * @param {string} startTime - UTC timestamp string from API ("YYYY-MM-DD HH:MM:SS UTC")
   * @returns {string} Timeline URL with local date and time params
   */
  getTimelineUrl: (stream, startTime) => {
    const base = `timeline.html?stream=${encodeURIComponent(stream || '')}`;
    if (!startTime) return `${base}&date=&time=`;
    const utcMatch = startTime.match(/^(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2}):(\d{2})/);
    if (!utcMatch) return `${base}&date=&time=`;
    const [, year, month, day, hours, minutes, seconds] = utcMatch;
    // Create a Date from explicit UTC components so JS converts to local automatically
    const utcDate = new Date(Date.UTC(
      parseInt(year), parseInt(month) - 1, parseInt(day),
      parseInt(hours), parseInt(minutes), parseInt(seconds)
    ));
    const date = `${utcDate.getFullYear()}-${String(utcDate.getMonth() + 1).padStart(2, '0')}-${String(utcDate.getDate()).padStart(2, '0')}`;
    const time = `${String(utcDate.getHours()).padStart(2, '0')}:${String(utcDate.getMinutes()).padStart(2, '0')}:${String(utcDate.getSeconds()).padStart(2, '0')}`;
    return `${base}&date=${date}&time=${time}`;
  },

  /**
   * Format file size
   * @param {number} bytes Size in bytes
   * @returns {string} Formatted file size
   */
  formatFileSize: (bytes) => {
    if (!bytes) return '0 B';
    
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let i = 0;
    let size = bytes;
    
    while (size >= 1024 && i < units.length - 1) {
      size /= 1024;
      i++;
    }
    
    return `${size.toFixed(1)} ${units[i]}`;
  }
};
