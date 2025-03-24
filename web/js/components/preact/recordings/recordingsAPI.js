/**
 * API functions for RecordingsView
 */

import { showStatusMessage } from '../UI.js';
import { formatUtils } from './formatUtils.js';

/**
 * RecordingsAPI - Handles all API calls related to recordings
 */
export const recordingsAPI = {
  /**
   * Load streams from API
   * @returns {Promise<Array>} Array of streams
   */
  loadStreams: async () => {
    try {
      const response = await fetch('/api/streams');
      if (!response.ok) {
        throw new Error('Failed to load streams');
      }
      
      const data = await response.json();
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
      
      // Fetch recordings
      const response = await fetch(`/api/recordings?${params.toString()}`);
      if (!response.ok) {
        throw new Error('Failed to load recordings');
      }
      
      const data = await response.json();
      console.log('Recordings data received:', data);
      
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
      const response = await fetch(`/api/recordings/${recording.id}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) {
        throw new Error('Failed to delete recording');
      }
      
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
      // Use the batch delete endpoint
      const response = await fetch('/api/recordings/batch-delete', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          ids: selectedIds
        })
      });
      
      if (!response.ok) {
        throw new Error('Failed to delete recordings');
      }
      
      const result = await response.json();
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
      
      return result;
    } catch (error) {
      console.error('Error in batch delete operation:', error);
      showStatusMessage('Error in batch delete operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
    }
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
      
      // Use the batch delete endpoint with filter
      const deleteResponse = await fetch('/api/recordings/batch-delete', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          filter: filter
        })
      });
      
      if (!deleteResponse.ok) {
        throw new Error('Failed to delete recordings');
      }
      
      const result = await deleteResponse.json();
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
      
      return result;
    } catch (error) {
      console.error('Error in delete all operation:', error);
      showStatusMessage('Error in delete all operation: ' + error.message);
      return { succeeded: 0, failed: 0 };
    }
  },
  
  /**
   * Play recording
   * @param {Object} recording Recording to play
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
    
    // Show video modal
    showVideoModal(videoUrl, title, downloadUrl);
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
