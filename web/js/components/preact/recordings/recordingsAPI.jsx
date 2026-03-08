/**
 * API functions for RecordingsView
 */

import { showStatusMessage } from '../ToastContainer.jsx';
import { formatUtils } from './formatUtils.js';
import { urlUtils } from './urlUtils.js';
import { fetchJSON, enhancedFetch } from '../../../fetch-utils.js';
import {
  useQuery,
  useMutation,
  useQueryClient,
  usePostMutation,
} from '../../../query-client.js';
import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
import customParseFormat from 'dayjs/plugin/customParseFormat';

// Initialize dayjs plugins
dayjs.extend(utc);
dayjs.extend(customParseFormat);

const getRecordingStartTime = (recording) =>
  recording?.start_time_unix ?? recording?.start_time;

// Default timeout/retry configuration for recordings API calls
const DEFAULT_TIMEOUT = 15000;       // 15 second timeout
const DEFAULT_RETRIES = 2;           // Retry twice
const DEFAULT_RETRY_DELAY = 1000;    // 1 second between retries

// Batch delete specific configuration
const BATCH_DELETE_TIMEOUT = 60000;          // 60 second timeout for batch operations
const BATCH_DELETE_RETRIES = 1;              // Retry once
const BATCH_DELETE_RETRY_DELAY = 2000;       // 2 seconds between retries
const BATCH_DELETE_POLL_MAX_ATTEMPTS = 120;  // 2 minutes max (120 * 1 second)

/**
 * Parse a recording timestamp to Unix seconds.
 * Accepts a Unix epoch number (already in seconds), an ISO 8601 string
 * (e.g. "2026-02-18T05:00:00Z"), or the legacy "YYYY-MM-DD HH:mm:ss UTC" string.
 * @param {number|string} value - Unix epoch (s), ISO string, or legacy UTC string
 * @returns {number} Unix timestamp in seconds, or 0 if parsing fails
 */
const parseRecordingTimestamp = (value) => {
  if (value === null || value === undefined || value === '') return 0;
  if (typeof value === 'number') return value > 0 ? value : 0;
  // String: ISO 8601 or legacy format — dayjs.utc handles both
  const parsed = dayjs.utc(value);
  return parsed.isValid() ? parsed.unix() : 0;
};

const getDetectionTimeRangeParams = (recording) => {
  const startTime = parseRecordingTimestamp(recording.start_time);
  const endTime = parseRecordingTimestamp(recording.end_time);

  if (startTime === 0 || endTime === 0) {
    return null;
  }

  return new URLSearchParams({
    // Backend detection results API parses Unix timestamps in seconds.
    start: startTime.toString(),
    end: endTime.toString()
  });
};

const appendMultiValueParam = (params, key, values) => {
  const serializedValue = urlUtils.serializeMultiValueParam(values);
  if (serializedValue) params.append(key, serializedValue);
};

const applyDateRangeParams = (params, filters) => {
  if (filters.dateRange === 'custom') {
    params.append('start', dayjs(`${filters.startDate}T${filters.startTime}:00`).toISOString());
    params.append('end', dayjs(`${filters.endDate}T${filters.endTime}:00`).toISOString());
  } else {
    const { start, end } = recordingsAPI.getDateRangeFromPreset(filters.dateRange);
    params.append('start', start);
    params.append('end', end);
  }
};

const applyFilterParams = (params, filters) => {
  appendMultiValueParam(params, 'stream', filters.streamIds);

  if (filters.recordingType === 'detection') {
    params.append('has_detection', '1');
  } else if (filters.recordingType === 'no_detection') {
    params.append('has_detection', '-1');
  }

  appendMultiValueParam(params, 'detection_label', filters.detectionLabels);
  appendMultiValueParam(params, 'tag', filters.tags);
  appendMultiValueParam(params, 'capture_method', filters.captureMethods);

  if (filters.protectedStatus === 'yes') {
    params.append('protected', '1');
  } else if (filters.protectedStatus === 'no') {
    params.append('protected', '0');
  }
};

const buildRecordingsQueryParams = (filters, pagination, sortField, sortDirection) => {
  const params = new URLSearchParams();
  params.append('page', pagination.currentPage);
  params.append('limit', pagination.pageSize);
  params.append('sort', sortField);
  params.append('order', sortDirection);
  applyDateRangeParams(params, filters);
  applyFilterParams(params, filters);
  return params;
};

const buildFilterObject = (filters) => {
  const filter = {};

  if (filters.dateRange === 'custom') {
    filter.start = dayjs(`${filters.startDate}T${filters.startTime}:00`).toISOString();
    filter.end = dayjs(`${filters.endDate}T${filters.endTime}:00`).toISOString();
  } else {
    const { start, end } = recordingsAPI.getDateRangeFromPreset(filters.dateRange);
    filter.start = start;
    filter.end = end;
  }

  const streamFilter = urlUtils.serializeMultiValueParam(filters.streamIds);
  if (streamFilter) filter.stream = streamFilter;

  if (filters.recordingType === 'detection') {
    filter.detection = 1;
  } else if (filters.recordingType === 'no_detection') {
    filter.detection = -1;
  }

  const detectionLabelFilter = urlUtils.serializeMultiValueParam(filters.detectionLabels);
  if (detectionLabelFilter) filter.detection_label = detectionLabelFilter;

  const tagFilter = urlUtils.serializeMultiValueParam(filters.tags);
  if (tagFilter) filter.tag = tagFilter;

  const captureMethodFilter = urlUtils.serializeMultiValueParam(filters.captureMethods);
  if (captureMethodFilter) filter.capture_method = captureMethodFilter;

  if (filters.protectedStatus === 'yes') {
    filter.protected = 1;
  } else if (filters.protectedStatus === 'no') {
    filter.protected = 0;
  }

  return filter;
};

/**
 * RecordingsAPI - Handles all API calls related to recordings
 */
export const recordingsAPI = {
  parseRecordingTimestamp,

  /**
   * Custom hooks for preact-query
   */
  hooks: {
    /**
     * Hook to fetch streams list
     * @returns {Object} Query result
     */
    useStreams: () => {
      return useQuery('streams', '/api/streams', {
        timeout: 15000, // 15 second timeout
        retries: 2,     // Retry twice
        retryDelay: 1000 // 1 second between retries
      });
    },

    /**
     * Hook to fetch recordings with filters
     * @param {Object} filters Filter settings
     * @param {Object} pagination Pagination settings
     * @param {string} sortField Sort field
     * @param {string} sortDirection Sort direction
     * @returns {Object} Query result
     */
    useRecordings: (filters, pagination, sortField, sortDirection) => {
      const params = buildRecordingsQueryParams(filters, pagination, sortField, sortDirection);

      // Create query key that includes all filter parameters
      const queryKey = ['recordings', filters, pagination, sortField, sortDirection];

      return useQuery(
        queryKey,
        `/api/recordings?${params.toString()}`,
        {
          timeout: 30000, // 30 second timeout for potentially large data
          retries: 2,     // Retry twice
          retryDelay: 1000 // 1 second between retries
        },
        // No special handling needed - we rely on the backend API for detection information
      );
    },

    // useRecordingDetections hook removed - we rely on the backend API for detection information

    /**
     * Hook to delete a recording
     * @returns {Object} Mutation result
     */
    useDeleteRecording: () => {
      const queryClient = useQueryClient();

      return useMutation({
        mutationFn: async (recordingId) => {
          const url = `/api/recordings/${recordingId}`;
          return await fetchJSON(url, {
            method: 'DELETE',
            timeout: 15000, // 15 second timeout
            retries: 1,     // Retry once
            retryDelay: 1000 // 1 second between retries
          });
        },
        onSuccess: () => {
          // Invalidate recordings queries to refresh the list
          queryClient.invalidateQueries({ queryKey: ['recordings'] });
          showStatusMessage('Recording deleted successfully');
        },
        onError: (error) => {
          console.error('Error deleting recording:', error);
          showStatusMessage('Error deleting recording: ' + error.message);
        }
      });
    },

    /**
     * Hook to delete multiple recordings
     * @returns {Object} Mutation result
     */
    useBatchDeleteRecordings: () => {
      const queryClient = useQueryClient();

      return usePostMutation(
        '/api/recordings/batch-delete',
        {
          timeout: BATCH_DELETE_TIMEOUT,
          retries: BATCH_DELETE_RETRIES,
          retryDelay: BATCH_DELETE_RETRY_DELAY
        },
        {
          onSuccess: (result) => {
            // Invalidate recordings queries to refresh the list
            queryClient.invalidateQueries({ queryKey: ['recordings'] });

            const successCount = result.succeeded;
            const errorCount = result.failed;

            // Show status message
            if (successCount > 0 && errorCount === 0) {
              showStatusMessage(`Successfully deleted ${successCount} recording${successCount !== 1 ? 's' : ''}`);
            } else if (successCount > 0 && errorCount > 0) {
              showStatusMessage(`Deleted ${successCount} recording${successCount !== 1 ? 's' : ''}, but failed to delete ${errorCount}`);
            } else {
              showStatusMessage(`Failed to delete ${errorCount} recording${errorCount !== 1 ? 's' : ''}`);
            }
          },
          onError: (error) => {
            console.error('Error in batch delete operation:', error);
            showStatusMessage('Error in batch delete operation: ' + error.message);
          }
        }
      );
    }
  },
  /**
   * Load streams from API
   * @returns {Promise<Array>} Array of streams
   */
  loadStreams: async () => {
    try {
      const data = await fetchJSON('/api/streams', {
        timeout: DEFAULT_TIMEOUT,
        retries: DEFAULT_RETRIES,
        retryDelay: DEFAULT_RETRY_DELAY
      });

      return data || [];
    } catch (error) {
      console.error('Error loading streams for filter:', error);
      showStatusMessage('Error loading streams: ' + error.message);
      return [];
    }
  },

  /**
   * Get date range from preset.
   * Returns ISO 8601 UTC strings that the backend's ISO 8601 parser understands.
   * @param {string} preset Preset name
   * @returns {Object} Start and end ISO 8601 strings in UTC
   */
  getDateRangeFromPreset: (preset) => {
    // Use dayjs to build date boundaries in the local timezone, then convert to UTC ISO strings.
    const startOfToday = dayjs().startOf('day');
    const endOfToday   = dayjs().endOf('day');

    let start, end;

    switch (preset) {
      case 'today':
        start = startOfToday.toISOString();
        end   = endOfToday.toISOString();
        break;
      case 'yesterday':
        start = startOfToday.subtract(1, 'day').toISOString();
        end   = endOfToday.subtract(1, 'day').toISOString();
        break;
      case 'last7days':
        start = startOfToday.subtract(7, 'day').toISOString();
        end   = endOfToday.toISOString();
        break;
      case 'last30days':
        start = startOfToday.subtract(30, 'day').toISOString();
        end   = endOfToday.toISOString();
        break;
      default:
        start = startOfToday.subtract(7, 'day').toISOString();
        end   = endOfToday.toISOString();
    }

    return { start, end };
  },

  /**
   * Load recordings
   * @param {Object} filters Filter settings
   * @param {Object} pagination Pagination settings
   * @param {string} sortField Sort field
   * @param {string} sortDirection Sort direction
   * @returns {Promise<Object>} Recordings data and pagination info
   */
  loadRecordings: async (filters, pagination, sortField, sortDirection) => {
    try {
      const params = buildRecordingsQueryParams(filters, pagination, sortField, sortDirection);

      // Log the API request
      console.log('API Request:', `/api/recordings?${params.toString()}`);

      // Fetch recordings with enhanced fetch
      const data = await fetchJSON(`/api/recordings?${params.toString()}`, {
        timeout: 30000, // 30 second timeout for potentially large data
        retries: 2,     // Retry twice
        retryDelay: 1000 // 1 second between retries
      });

      console.log('Recordings data received:', data);

      // Set has_detections to false by default instead of making API calls
      // This prevents unnecessary detection API calls on the recordings page
      if (data.recordings && data.recordings.length > 0) {
        for (const recording of data.recordings) {
          // Default to false on the recordings list only when the backend has not provided a value;
          // detailed checks can be done on-demand elsewhere
          if (recording.has_detections === undefined) {
            recording.has_detections = false;
          }
        }
      }

      return data;
    } catch (error) {
      console.error('Error loading recordings:', error);
      showStatusMessage('Error loading recordings: ' + error.message);
      throw error;
    }
  },

  /**
   * Delete a single recording
   * @param {Object} recording Recording to delete
   * @returns {Promise<boolean>} Success status
   */
  deleteRecording: async (recording) => {
    try {
      await enhancedFetch(`/api/recordings/${recording.id}`, {
        method: 'DELETE',
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });

      showStatusMessage('Recording deleted successfully');
      return true;
    } catch (error) {
      console.error('Error deleting recording:', error);
      showStatusMessage('Error deleting recording: ' + error.message);
      return false;
    }
  },

  /**
   * Delete selected recordings
   * @param {Object} selectedRecordings Object with recording IDs as keys
   * @returns {Promise<Object>} Result with success and error counts
   */
  deleteSelectedRecordings: async (selectedRecordings) => {
    const selectedIds = Object.entries(selectedRecordings)
      .filter(([recordingId, isSelected]) => isSelected)
      .map(([id, _]) => parseInt(id, 10));

    if (selectedIds.length === 0) {
      showStatusMessage('No recordings selected');
      return { succeeded: 0, failed: 0 };
    }

    try {
      // Route through BatchDeleteModal for confirmation + progress
      if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
        return await window.batchDeleteRecordingsByHttpRequest({ ids: selectedIds });
      }
      // Fallback to direct HTTP if modal not mounted
      return recordingsAPI.deleteSelectedRecordingsHttp(selectedIds);
    } catch (error) {
      console.error('Error in batch delete operation:', error);
      showStatusMessage('Error in batch delete operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
    }
  },

  /**
   * Delete selected recordings using HTTP (fallback)
   * @param {Array<number>} selectedIds Array of recording IDs
   * @returns {Promise<Object>} Result with success and error counts
   */
  deleteSelectedRecordingsHttp: async (selectedIds) => {
    try {
      // Use the batch delete endpoint with enhanced fetch
      const response = await enhancedFetch('/api/recordings/batch-delete', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          ids: selectedIds
        }),
        timeout: 60000, // 60 second timeout for batch operations
        retries: 1,     // Retry once
        retryDelay: 2000 // 2 seconds between retries
      });

      return await recordingsAPI.handleBatchDeleteResponse(response);
    } catch (error) {
      console.error('Error in HTTP batch delete operation:', error);
      showStatusMessage('Error in batch delete operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
    }
  },

  /**
   * Poll for batch delete progress
   * @param {string} jobId Job ID to poll
   * @returns {Promise<Object>} Final result with success and error counts
   */
  pollBatchDeleteProgress: async (jobId) => {
    const maxAttempts = BATCH_DELETE_POLL_MAX_ATTEMPTS;
    let attempts = 0;

    while (attempts < maxAttempts) {
      try {
        const progress = await fetchJSON(`/api/recordings/batch-delete/progress/${jobId}`, {
          method: 'GET',
          // Ensure each polling request has bounded duration and internal retries
          timeout: 10000,       // 10 seconds per request
          retries: 2,           // retry a couple of times on failure
          retryDelay: 1000      // 1 second between internal retries
        });

        // Update progress UI if available
        if (typeof window.updateBatchDeleteProgress === 'function') {
          window.updateBatchDeleteProgress({
            current: progress.current || 0,
            total: progress.total || 0,
            succeeded: progress.succeeded || 0,
            failed: progress.failed || 0,
            status: progress.status_message || 'Processing...',
            complete: progress.complete || false
          });
        }

        // Check if complete
        if (progress.complete) {
          return {
            succeeded: progress.succeeded || 0,
            failed: progress.failed || 0
          };
        }

        // Wait 1 second before next poll
        await new Promise(resolve => setTimeout(resolve, 1000));
        attempts++;
      } catch (error) {
        console.error('Error polling batch delete progress:', error);
        // Continue polling on error
        await new Promise(resolve => setTimeout(resolve, 1000));
        attempts++;
      }
    }

    // Timeout
    throw new Error('Batch delete operation timed out');
  },

  /**
   * Delete all recordings matching current filter
   * @param {Object} filters Current filters
   * @returns {Promise<Object>} Result with success and error counts
   */
  deleteAllFilteredRecordings: async (filters) => {
    try {
      const filter = buildFilterObject(filters);

      console.log('Deleting with filter:', filter);

      // Get total count first so the confirmation modal can show it
      let totalCount = 0;
      try {
        const params = new URLSearchParams();
        if (filter.start) params.append('start', filter.start);
        if (filter.end) params.append('end', filter.end);
        if (filter.stream) params.append('stream', filter.stream);
        if (filter.detection === 1) params.append('has_detection', '1');
        else if (filter.detection === -1) params.append('has_detection', '-1');
        if (filter.detection_label) params.append('detection_label', filter.detection_label);
        if (filter.tag) params.append('tag', filter.tag);
        if (filter.capture_method) params.append('capture_method', filter.capture_method);
        if (filter.protected === 1) params.append('protected', '1');
        else if (filter.protected === 0) params.append('protected', '0');
        params.append('page', '1');
        params.append('limit', '1');

        const data = await fetchJSON('/api/recordings?' + params.toString());
        if (data?.pagination?.total) {
          totalCount = data.pagination.total;
          console.log(`Found ${totalCount} recordings matching filter`);
        }
      } catch (countError) {
        console.warn('Error getting recording count:', countError);
      }

      // Route through BatchDeleteModal for confirmation + progress
      if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
        return await window.batchDeleteRecordingsByHttpRequest({ filter, totalCount });
      }

      // Fallback to direct HTTP if modal not mounted
      return recordingsAPI.deleteAllFilteredRecordingsHttp(filter);
    } catch (error) {
      console.error('Error in delete all operation:', error);
      showStatusMessage('Error in delete all operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
    }
  },

  /**
   * Internal helper to process batch delete HTTP responses.
   * Handles optional async polling, status message display, and returns the final result.
   * @param {Response} deleteResponse Response from the batch delete request
   * @returns {Promise<Object>} Result with success and error counts
   */
  handleBatchDeleteResponse: async (deleteResponse) => {
    const result = await deleteResponse.json();

    // Check if we got a job_id (async operation) or direct result (sync operation)
    if (result.job_id) {
      console.log('Batch delete started with job_id:', result.job_id);

      // Poll for progress
      const finalResult = await recordingsAPI.pollBatchDeleteProgress(result.job_id);

      // Show status message
      const successCount = finalResult.succeeded || 0;
      const errorCount = finalResult.failed || 0;

      if (successCount > 0 && errorCount === 0) {
        showStatusMessage(`Successfully deleted ${successCount} recording${successCount !== 1 ? 's' : ''}`);
      } else if (successCount > 0 && errorCount > 0) {
        showStatusMessage(`Deleted ${successCount} recording${successCount !== 1 ? 's' : ''}, but failed to delete ${errorCount}`);
      } else if (errorCount > 0) {
        showStatusMessage(`Failed to delete ${errorCount} recording${errorCount !== 1 ? 's' : ''}`);
      }

      return finalResult;
    } else {
      // Direct result (old sync behavior)
      const successCount = result.succeeded || 0;
      const errorCount = result.failed || 0;

      // Show status message
      if (successCount > 0 && errorCount === 0) {
        showStatusMessage(`Successfully deleted ${successCount} recording${successCount !== 1 ? 's' : ''}`);
      } else if (successCount > 0 && errorCount > 0) {
        showStatusMessage(`Deleted ${successCount} recording${successCount !== 1 ? 's' : ''}, but failed to delete ${errorCount}`);
      } else if (errorCount > 0) {
        showStatusMessage(`Failed to delete ${errorCount} recording${errorCount !== 1 ? 's' : ''}`);
      }

      return result;
    }
  },

  /**
   * Delete all recordings matching filter using HTTP (fallback)
   * @param {Object} filter Filter object
   * @returns {Promise<Object>} Result with success and error counts
   */
  deleteAllFilteredRecordingsHttp: async (filter) => {
    try {
      // Use the batch delete endpoint with filter and enhanced fetch
      const deleteResponse = await enhancedFetch('/api/recordings/batch-delete', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          filter: filter
        }),
        timeout: 120000, // 120 second timeout for potentially large batch operations
        retries: 1,      // Retry once
        retryDelay: 3000 // 3 seconds between retries
      });

      return await recordingsAPI.handleBatchDeleteResponse(deleteResponse);
    } catch (error) {
      console.error('Error in HTTP delete all operation:', error);
      showStatusMessage('Error in delete all operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
    }
  },

  /**
   * Check if a recording has associated detections
   * @param {Object} recording Recording to check
   * @returns {Promise<boolean>} True if the recording has detections, false otherwise
   */
  checkRecordingHasDetections: async (recording) => {
    if (!recording || !recording.id || !recording.stream || !recording.start_time || !recording.end_time) {
      return false;
    }

    try {
      const params = getDetectionTimeRangeParams(recording);

      if (!params) {
        console.error('Failed to parse recording timestamps');
        return false;
      }

      const data = await fetchJSON(`/api/detection/results/${recording.stream}?${params.toString()}`, {
        timeout: 10000, // 10 second timeout
        retries: 1,     // Retry once
        retryDelay: 500 // 0.5 second between retries
      });

      return data.detections && data.detections.length > 0;
    } catch (error) {
      console.error('Error checking detections:', error);
      return false;
    }
  },

  /**
   * Get detections for a recording
   * @param {Object} recording Recording to get detections for
   * @returns {Promise<Array>} Array of detections
   */
  getRecordingDetections: async (recording) => {
    if (!recording || !recording.id || !recording.stream || !recording.start_time || !recording.end_time) {
      return [];
    }

    try {
      const params = getDetectionTimeRangeParams(recording);

      if (!params) {
        console.error('Failed to parse recording timestamps');
        return [];
      }

      const data = await fetchJSON(`/api/detection/results/${recording.stream}?${params.toString()}`, {
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });

      return data.detections || [];
    } catch (error) {
      console.error('Error getting detections:', error);
      return [];
    }
  },

  /**
   * Play recording
   * @param {Object} recording Recording to play
   * @param {Function} showVideoModal Function to show video modal
   */
  playRecording: (recording, showVideoModal) => {
    console.log('Play recording clicked:', recording);

    // Check if recording has an id property
    if (!recording.id) {
      console.error('Recording has no id property:', recording);
      showStatusMessage('Error: Recording has no id property');
      return;
    }

    // Build video URL
    const videoUrl = `/api/recordings/play/${recording.id}`;
    const title = `${recording.stream} - ${formatUtils.formatDateTime(getRecordingStartTime(recording))}`;
    const downloadUrl = `/api/recordings/download/${recording.id}`;

    console.log('Video URL:', videoUrl);
    console.log('Title:', title);
    console.log('Download URL:', downloadUrl);

    // Check if we're using the context-based showVideoModal or the direct function
    if (window.__modalContext && window.__modalContext.showVideoModal) {
      // Use the context-based function if available
      window.__modalContext.showVideoModal(videoUrl, title, downloadUrl);
    } else {
      // Fall back to the provided function
      showVideoModal(videoUrl, title, downloadUrl);
    }

    console.log('Video modal should be shown now');
  },

  /**
   * Get all unique recording tags
   * @returns {Promise<string[]>} Array of unique tags
   */
  getAllRecordingTags: async () => {
    try {
      const data = await fetchJSON('/api/recordings/tags', {
        timeout: 10000,
        retries: 1,
        retryDelay: 500
      });
      return data.tags || [];
    } catch (error) {
      console.error('Error fetching recording tags:', error);
      return [];
    }
  },

  /**
   * Get all unique detection labels
   * @returns {Promise<string[]>} Array of unique detection labels
   */
  getAllDetectionLabels: async () => {
    try {
      const data = await fetchJSON('/api/recordings/detection-labels', {
        timeout: 10000,
        retries: 1,
        retryDelay: 500
      });
      return data.labels || [];
    } catch (error) {
      console.error('Error fetching detection labels:', error);
      return [];
    }
  },

  /**
   * Get tags for a specific recording
   * @param {number} recordingId Recording ID
   * @returns {Promise<string[]>} Array of tags
   */
  getRecordingTags: async (recordingId) => {
    try {
      const data = await fetchJSON(`/api/recordings/${recordingId}/tags`, {
        timeout: 10000,
        retries: 1,
        retryDelay: 500
      });
      return data.tags || [];
    } catch (error) {
      console.error('Error fetching recording tags:', error);
      return [];
    }
  },

  /**
   * Set tags for a specific recording (replace all)
   * @param {number} recordingId Recording ID
   * @param {string[]} tags Array of tags
   * @returns {Promise<Object>} Result with id and tags
   */
  setRecordingTags: async (recordingId, tags) => {
    try {
      const data = await fetchJSON(`/api/recordings/${recordingId}/tags`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tags }),
        timeout: 10000,
        retries: 1,
        retryDelay: 500
      });
      return data;
    } catch (error) {
      console.error('Error setting recording tags:', error);
      showStatusMessage('Error setting tags: ' + error.message);
      throw error;
    }
  },

  /**
   * Batch add/remove tags for multiple recordings
   * @param {number[]} ids Array of recording IDs
   * @param {string[]} addTags Tags to add
   * @param {string[]} removeTags Tags to remove
   * @returns {Promise<Object>} Result with counts
   */
  batchUpdateRecordingTags: async (ids, addTags = [], removeTags = []) => {
    try {
      const data = await fetchJSON('/api/recordings/batch-tags', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ids, add: addTags, remove: removeTags }),
        timeout: 30000,
        retries: 1,
        retryDelay: 1000
      });
      return data;
    } catch (error) {
      console.error('Error in batch tag update:', error);
      showStatusMessage('Error updating tags: ' + error.message);
      throw error;
    }
  },

  /**
   * Download recording
   * @param {Object} recording Recording to download
   */
  downloadRecording: (recording) => {
    // Create download link
    const downloadUrl = `/api/recordings/download/${recording.id}`;
    const link = document.createElement('a');
    link.href = downloadUrl;
    // Use dayjs to build a filename timestamp from Unix epoch or ISO string
    const tsValue = recording.start_time_unix ?? recording.start_time;
    const timestamp = typeof tsValue === 'number'
      ? dayjs.unix(tsValue)
      : dayjs.utc(tsValue);
    const formattedTime = timestamp.isValid()
      ? timestamp.local().format('YYYY-MM-DDTHH-mm-ss')
      : 'unknown';
    link.download = `${recording.stream}_${formattedTime}.mp4`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);

    showStatusMessage('Download started');
  }
};
