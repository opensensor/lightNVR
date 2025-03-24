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
