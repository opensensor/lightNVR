/**
 * API functions for RecordingsView
 */

import { showStatusMessage } from '../ToastContainer.jsx';
import { formatUtils } from './formatUtils.js';
import { fetchJSON, enhancedFetch } from '../../../fetch-utils.js';
import {
  useQuery,
  useMutation,
  useQueryClient,
  usePostMutation,
} from '../../../query-client.js';

/**
 * RecordingsAPI - Handles all API calls related to recordings
 */
export const recordingsAPI = {
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
      // Build query parameters
      const params = new URLSearchParams();
      params.append('page', pagination.currentPage);
      params.append('limit', pagination.pageSize);
      params.append('sort', sortField);
      params.append('order', sortDirection);

      // Add date range filters
      if (filters.dateRange === 'custom') {
        params.append('start', `${filters.startDate}T${filters.startTime}:00`);
        params.append('end', `${filters.endDate}T${filters.endTime}:00`);
      } else {
        // Convert predefined range to actual dates
        const { start, end } = recordingsAPI.getDateRangeFromPreset(filters.dateRange);
        params.append('start', start);
        params.append('end', end);
      }

      // Add stream filter
      if (filters.streamId !== 'all') {
        params.append('stream', filters.streamId);
      }

      // Add recording type filter
      if (filters.recordingType === 'detection') {
        params.append('detection', '1');
      }

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
          timeout: 60000, // 60 second timeout for batch operations
          retries: 1,     // Retry once
          retryDelay: 2000 // 2 seconds between retries
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
        timeout: 15000, // 15 second timeout
        retries: 2,     // Retry twice
        retryDelay: 1000 // 1 second between retries
      });

      return data || [];
    } catch (error) {
      console.error('Error loading streams for filter:', error);
      showStatusMessage('Error loading streams: ' + error.message);
      return [];
    }
  },

  /**
   * Get date range from preset
   * @param {string} preset Preset name
   * @returns {Object} Start and end dates
   */
  getDateRangeFromPreset: (preset) => {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
    const todayStart = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0);

    let start, end;

    switch (preset) {
      case 'today':
        start = todayStart.toISOString();
        end = today.toISOString();
        break;
      case 'yesterday':
        const yesterday = new Date(todayStart);
        yesterday.setDate(yesterday.getDate() - 1);
        const yesterdayEnd = new Date(yesterday);
        yesterdayEnd.setHours(23, 59, 59);
        start = yesterday.toISOString();
        end = yesterdayEnd.toISOString();
        break;
      case 'last7days':
        const sevenDaysAgo = new Date(todayStart);
        sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7);
        start = sevenDaysAgo.toISOString();
        end = today.toISOString();
        break;
      case 'last30days':
        const thirtyDaysAgo = new Date(todayStart);
        thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
        start = thirtyDaysAgo.toISOString();
        end = today.toISOString();
        break;
      default:
        // Default to last 7 days
        const defaultStart = new Date(todayStart);
        defaultStart.setDate(defaultStart.getDate() - 7);
        start = defaultStart.toISOString();
        end = today.toISOString();
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
      // Build query parameters
      const params = new URLSearchParams();
      params.append('page', pagination.currentPage);
      params.append('limit', pagination.pageSize);
      params.append('sort', sortField);
      params.append('order', sortDirection);

      // Add date range filters
      if (filters.dateRange === 'custom') {
        params.append('start', `${filters.startDate}T${filters.startTime}:00`);
        params.append('end', `${filters.endDate}T${filters.endTime}:00`);
      } else {
        // Convert predefined range to actual dates
        const { start, end } = recordingsAPI.getDateRangeFromPreset(filters.dateRange);
        params.append('start', start);
        params.append('end', end);
      }

      // Add stream filter
      if (filters.streamId !== 'all') {
        params.append('stream', filters.streamId);
      }

      // Add recording type filter
      if (filters.recordingType === 'detection') {
        params.append('detection', '1');
      }

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
        // Process recordings in batches to avoid too many parallel requests
        const batchSize = 5;
        for (let i = 0; i < data.recordings.length; i += batchSize) {
          const batch = data.recordings.slice(i, i + batchSize);
          await Promise.all(batch.map(async (recording) => {
            try {
              recording.has_detections = await recordingsAPI.checkRecordingHasDetections(recording);
            } catch (error) {
              console.error(`Error checking detections for recording ${recording.id}:`, error);
              recording.has_detections = false;
            }
          }));
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
      .filter(([_, isSelected]) => isSelected)
      .map(([id, _]) => parseInt(id, 10));

    if (selectedIds.length === 0) {
      showStatusMessage('No recordings selected');
      return { succeeded: 0, failed: 0 };
    }

    try {
      // Show batch delete modal
      if (typeof window.showBatchDeleteModal === 'function') {
        window.showBatchDeleteModal();
      }

      // Use HTTP for batch delete (WebSockets were removed)
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

      const result = await response.json();

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
    const maxAttempts = 120; // 2 minutes max (120 * 1 second)
    let attempts = 0;

    while (attempts < maxAttempts) {
      try {
        const response = await fetch(`/api/recordings/batch-delete/progress/${jobId}`);
        if (!response.ok) {
          throw new Error(`Failed to get progress: ${response.statusText}`);
        }

        const progress = await response.json();

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
      // Create filter object
      const filter = {};

      // Add date range filters
      if (filters.dateRange === 'custom') {
        filter.start = `${filters.startDate}T${filters.startTime}:00`;
        filter.end = `${filters.endDate}T${filters.endTime}:00`;
      } else {
        // Convert predefined range to actual dates
        const { start, end } = recordingsAPI.getDateRangeFromPreset(filters.dateRange);
        filter.start = start;
        filter.end = end;
      }

      // Add stream filter
      if (filters.streamId !== 'all') {
        filter.stream_name = filters.streamId; // Changed from 'stream' to 'stream_name' to match API expectations
      }

      // Add recording type filter
      if (filters.recordingType === 'detection') {
        filter.detection = 1;
      }

      console.log('Deleting with filter:', filter);

      // Show batch delete modal with indeterminate progress initially
      if (typeof window.showBatchDeleteModal === 'function') {
        window.showBatchDeleteModal();

        // Update the progress UI with an indeterminate state
        if (typeof window.updateBatchDeleteProgress === 'function') {
          window.updateBatchDeleteProgress({
            current: 0,
            total: 0, // We don't know the total yet
            succeeded: 0,
            failed: 0,
            status: `Preparing to delete recordings matching filter...`,
            complete: false
          });
        }
      }

      // Get the total count from the current page's filter
      // This will help us set a more accurate progress indicator
      let totalCount = 0;
      try {
        // Build query parameters for the API request
        const params = new URLSearchParams();

        // Add date range parameters
        if (filter.start) {
          params.append('start', filter.start);
        }

        if (filter.end) {
          params.append('end', filter.end);
        }

        // Add stream filter
        if (filter.stream_name) {
          params.append('stream', filter.stream_name);
        }

        // Add detection filter
        if (filter.detection) {
          params.append('detection', '1');
        }

        // Set page size to 1 to minimize data transfer, we just need the total count
        params.append('page', '1');
        params.append('limit', '1');

        console.log('Getting total count with params:', params.toString());

        // Fetch recordings to get pagination info
        const response = await fetch(`/api/recordings?${params.toString()}`);
        if (response.ok) {
          const data = await response.json();
          if (data && data.pagination && data.pagination.total) {
            totalCount = data.pagination.total;
            console.log(`Found ${totalCount} recordings matching filter`);

            // Update the progress UI with the total count
            if (typeof window.updateBatchDeleteProgress === 'function') {
              window.updateBatchDeleteProgress({
                current: 0,
                total: totalCount,
                succeeded: 0,
                failed: 0,
                status: `Found ${totalCount} recordings matching filter. Starting deletion...`,
                complete: false
              });
            }
          }
        }
      } catch (countError) {
        console.warn('Error getting recording count:', countError);
        // Continue anyway, we'll just show an indeterminate progress
      }

      // Use HTTP for batch delete with filter (WebSockets were removed)
      return recordingsAPI.deleteAllFilteredRecordingsHttp(filter);
    } catch (error) {
      console.error('Error in delete all operation:', error);
      showStatusMessage('Error in delete all operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
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
      // Convert timestamps to seconds
      const startTime = Math.floor(new Date(recording.start_time).getTime() / 1000);
      const endTime = Math.floor(new Date(recording.end_time).getTime() / 1000);

      // Query the detections API to check if there are any detections in this time range
      const params = new URLSearchParams({
        start: startTime,
        end: endTime
      });

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
      // Convert timestamps to seconds
      const startTime = Math.floor(new Date(recording.start_time).getTime() / 1000);
      const endTime = Math.floor(new Date(recording.end_time).getTime() / 1000);

      // Query the detections API to get detections in this time range
      const params = new URLSearchParams({
        start: startTime,
        end: endTime
      });

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
    const title = `${recording.stream} - ${formatUtils.formatDateTime(recording.start_time)}`;
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
   * Download recording
   * @param {Object} recording Recording to download
   */
  downloadRecording: (recording) => {
    // Create download link
    const downloadUrl = `/api/recordings/download/${recording.id}`;
    const link = document.createElement('a');
    link.href = downloadUrl;
    link.download = `${recording.stream}_${new Date(recording.start_time).toISOString().replace(/[:.]/g, '-')}.mp4`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);

    showStatusMessage('Download started');
  }
};
