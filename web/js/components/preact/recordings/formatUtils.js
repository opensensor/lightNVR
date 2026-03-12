/**
 * Formatting utility functions for RecordingsView
 */

import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';

dayjs.extend(utc);

const CAPTURE_METHOD_TRANSLATIONS = {
  scheduled: {
    key: 'recordings.captureMethod.continuous',
    fallback: 'Continuous'
  },
  detection: {
    key: 'recordings.captureMethod.detection',
    fallback: 'Detection-triggered'
  },
  motion: {
    key: 'recordings.captureMethod.motion',
    fallback: 'Motion-triggered'
  },
  manual: {
    key: 'recordings.captureMethod.manual',
    fallback: 'Manual'
  }
};

const translateWithFallback = (t, key, fallback) => {
  if (typeof t !== 'function') {
    return fallback;
  }

  const translated = t(key);
  return translated && translated !== key ? translated : fallback;
};

export const formatUtils = {
  /**
   * Format a capture method for display
   * @param {string} value Capture method value from the API
   * @returns {string} User-friendly capture method label
   */
  formatCaptureMethod: (value, t = null) => {
    const normalizedValue = typeof value === 'string' ? value.trim() : '';
    const translation = CAPTURE_METHOD_TRANSLATIONS[normalizedValue];

    if (translation) {
      return translateWithFallback(t, translation.key, translation.fallback);
    }

    return normalizedValue
      ? normalizedValue.replace(/_/g, ' ')
      : translateWithFallback(t, 'common.unknown', 'Unknown');
  },

  /**
   * Format a timestamp for display in the user's local timezone.
   * Accepts a Unix epoch number (seconds), an ISO 8601 string, or the legacy
   * "YYYY-MM-DD HH:mm:ss UTC" string returned by older server versions.
   * @param {number|string} value Unix epoch (s), ISO string, or legacy UTC string
   * @returns {string} Localised date/time string
   */
  formatDateTime: (value) => {
    if (value === null || value === undefined || value === '') return '';

    let d;
    if (typeof value === 'number') {
      // Unix epoch in seconds
      d = dayjs.unix(value);
    } else {
      // ISO 8601 (e.g. "2026-02-18T05:00:00Z") or legacy "YYYY-MM-DD HH:mm:ss UTC"
      d = dayjs.utc(value);
    }

    return d.isValid() ? d.local().format('YYYY-MM-DD HH:mm:ss') : '';
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
   * Accepts a Unix epoch number (seconds), an ISO 8601 string, or the legacy
   * "YYYY-MM-DD HH:mm:ss UTC" string. TimelinePage interprets the date/time URL params
   * as local time, so we convert to local before building the URL.
   * @param {string} stream - Stream name
   * @param {number|string} startTime - Unix epoch (s), ISO string, or legacy UTC string
   * @returns {string} Timeline URL with local date and time params
   */
  getTimelineUrl: (stream, startTime) => {
    const base = `timeline.html?stream=${encodeURIComponent(stream || '')}`;
    if (startTime === null || startTime === undefined || startTime === '') return `${base}&date=&time=`;

    let local;
    if (typeof startTime === 'number') {
      local = dayjs.unix(startTime).local();
    } else {
      local = dayjs.utc(startTime).local();
    }

    if (!local.isValid()) return `${base}&date=&time=`;

    const date = local.format('YYYY-MM-DD');
    const time = local.format('HH:mm:ss');
    return `${base}&date=${date}&time=${time}`;
  },

  /**
   * Format file size
   * @param {number} bytes Size in bytes
   * @returns {string} Formatted file size
   */
  formatFileSize: (bytes) => {
    // Treat null/undefined/non-number/NaN as 0 B
    if (bytes == null || typeof bytes !== 'number' || !Number.isFinite(bytes)) {
      return '0 B';
    }

    // Guard against negative values by clamping to zero
    if (bytes < 0) {
      bytes = 0;
    }

    if (bytes === 0) return '0 B';

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
