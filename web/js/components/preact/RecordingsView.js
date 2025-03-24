/**
 * LightNVR Web Interface RecordingsView Component
 * Preact component for the recordings page
 */

import { h } from '../../preact.min.js';
import { html } from '../../html-helper.js';
import { useState, useEffect, useRef, useCallback } from '../../preact.hooks.module.js';
import { showStatusMessage, showVideoModal, DeleteConfirmationModal } from './UI.js';

// Import components
import { FiltersSidebar } from './recordings/FiltersSidebar.js';
import { ActiveFilters } from './recordings/ActiveFilters.js';
import { RecordingsTable } from './recordings/RecordingsTable.js';
import { PaginationControls } from './recordings/PaginationControls.js';

// Import utilities
import { formatUtils } from './recordings/formatUtils.js';
import { recordingsAPI } from './recordings/recordingsAPI.js';
import { urlUtils } from './recordings/urlUtils.js';

/**
 * RecordingsView component
 * @returns {JSX.Element} RecordingsView component
 */
export function RecordingsView() {
  const [recordings, setRecordings] = useState([]);
  const [streams, setStreams] = useState([]);
  const [filtersVisible, setFiltersVisible] = useState(true);
  const [sortField, setSortField] = useState('start_time');
  const [sortDirection, setSortDirection] = useState('desc');
  const [filters, setFilters] = useState({
    dateRange: 'last7days',
    startDate: '',
    startTime: '00:00',
    endDate: '',
    endTime: '23:59',
    streamId: 'all',
    recordingType: 'all'
  });
  const [pagination, setPagination] = useState({
    currentPage: 1,
    pageSize: 20,
    totalItems: 0,
    totalPages: 1,
    startItem: 0,
    endItem: 0
  });
  const [hasActiveFilters, setHasActiveFilters] = useState(false);
  const [activeFiltersDisplay, setActiveFiltersDisplay] = useState([]);
  const [selectedRecordings, setSelectedRecordings] = useState({});
  const [selectAll, setSelectAll] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [deleteMode, setDeleteMode] = useState('selected'); // 'selected' or 'all'
  const recordingsTableBodyRef = useRef(null);
  
  // Initialize component
  useEffect(() => {
    // Set default date range
    setDefaultDateRange();
    
    // First load streams to populate the filter dropdown
    loadStreams().then(() => {
      // Load filters from URL if present
      loadFiltersFromUrl();
      
      // Then load recordings data
      loadRecordings();
    });
    
    // Handle responsive behavior
    handleResponsiveFilters();
    window.addEventListener('resize', handleResponsiveFilters);
    
    // Cleanup
    return () => {
      window.removeEventListener('resize', handleResponsiveFilters);
    };
  }, []);
  
  // Update active filters when filters change
  useEffect(() => {
    updateActiveFilters();
  }, [filters]);
  
  // Set default date range
  const setDefaultDateRange = () => {
    const now = new Date();
    const sevenDaysAgo = new Date(now);
    sevenDaysAgo.setDate(now.getDate() - 7);
    
    setFilters(prev => ({
      ...prev,
      endDate: now.toISOString().split('T')[0],
      startDate: sevenDaysAgo.toISOString().split('T')[0]
    }));
  };
  
  // Load streams from API
  const loadStreams = async () => {
    try {
      const response = await fetch('/api/streams');
      if (!response.ok) {
        throw new Error('Failed to load streams');
      }
      
      const data = await response.json();
      setStreams(data || []);
    } catch (error) {
      console.error('Error loading streams for filter:', error);
      showStatusMessage('Error loading streams: ' + error.message);
    }
  };
  
  // Load filters from URL
  const loadFiltersFromUrl = () => {
    // Get URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    
    // Date range
    if (urlParams.has('dateRange')) {
      const newFilters = { ...filters };
      newFilters.dateRange = urlParams.get('dateRange');
      
      if (newFilters.dateRange === 'custom') {
        if (urlParams.has('startDate')) {
          newFilters.startDate = urlParams.get('startDate');
        }
        if (urlParams.has('startTime')) {
          newFilters.startTime = urlParams.get('startTime');
        }
        if (urlParams.has('endDate')) {
          newFilters.endDate = urlParams.get('endDate');
        }
        if (urlParams.has('endTime')) {
          newFilters.endTime = urlParams.get('endTime');
        }
      }
      
      // Stream
      if (urlParams.has('stream')) {
        newFilters.streamId = urlParams.get('stream');
      }
      
      // Recording type
      if (urlParams.has('detection') && urlParams.get('detection') === '1') {
        newFilters.recordingType = 'detection';
      }
      
      setFilters(newFilters);
    }
    
    // Pagination
    if (urlParams.has('page')) {
      setPagination(prev => ({
        ...prev,
        currentPage: parseInt(urlParams.get('page'), 10)
      }));
    }
    if (urlParams.has('limit')) {
      setPagination(prev => ({
        ...prev,
        pageSize: parseInt(urlParams.get('limit'), 10)
      }));
    }
    
    // Sorting
    if (urlParams.has('sort')) {
      setSortField(urlParams.get('sort'));
    }
    if (urlParams.has('order')) {
      setSortDirection(urlParams.get('order'));
    }
  };
  
  // Update URL with filters
  const updateUrlWithFilters = () => {
    // Create URL parameters object based on current URL to preserve any existing parameters
    const params = new URLSearchParams(window.location.search);
    
    // Update or add date range parameters
    params.set('dateRange', filters.dateRange);
    
    // Handle custom date range
    if (filters.dateRange === 'custom') {
      params.set('startDate', filters.startDate);
      params.set('startTime', filters.startTime);
      params.set('endDate', filters.endDate);
      params.set('endTime', filters.endTime);
    } else {
      // Remove custom date parameters if not using custom date range
      params.delete('startDate');
      params.delete('startTime');
      params.delete('endDate');
      params.delete('endTime');
    }
    
    // Update stream filter
    if (filters.streamId !== 'all') {
      params.set('stream', filters.streamId);
    } else {
      params.delete('stream');
    }
    
    // Update recording type filter
    if (filters.recordingType === 'detection') {
      params.set('detection', '1');
    } else {
      params.delete('detection');
    }
    
    // Update pagination
    params.set('page', pagination.currentPage.toString());
    params.set('limit', pagination.pageSize.toString());
    
    // Update sorting
    params.set('sort', sortField);
    params.set('order', sortDirection);
    
    // Update URL without reloading the page
    const newUrl = `${window.location.pathname}?${params.toString()}`;
    window.history.pushState({ path: newUrl }, '', newUrl);
  };
  
  // Handle responsive filters
  const handleResponsiveFilters = () => {
    // On mobile, hide filters by default
    if (window.innerWidth < 768) {
      setFiltersVisible(false);
    } else {
      setFiltersVisible(true);
    }
  };
  
  // Toggle filters visibility
  const toggleFilters = () => {
    setFiltersVisible(!filtersVisible);
  };
  
  // Get date range from preset
  const getDateRangeFromPreset = (preset) => {
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
  };
  
  // Load recordings
  const loadRecordings = async (page = pagination.currentPage, updateUrl = true) => {
    try {
      // Show loading state
      setRecordings([]);
      
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
        const { start, end } = getDateRangeFromPreset(filters.dateRange);
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
      
      // Update URL with filters if requested
      if (updateUrl) {
        updateUrlWithFilters();
      }
      
      // Fetch recordings
      const response = await fetch(`/api/recordings?${params.toString()}`);
      if (!response.ok) {
        throw new Error('Failed to load recordings');
      }
      
      const data = await response.json();
      console.log('Recordings data received:', data);
      
      // Store recordings in the component state
      setRecordings(data.recordings || []);
      
      // Update pagination without changing the current page
      if (data.pagination) {
        setPagination(prev => ({
          ...prev,
          totalItems: data.pagination.total || 0,
          totalPages: data.pagination.pages || 1,
          // Keep the current page from state instead of using the one from the response
          // currentPage: data.pagination.page || 1,
          pageSize: data.pagination.limit || 20,
          startItem: data.recordings.length > 0 ? (pagination.currentPage - 1) * data.pagination.limit + 1 : 0,
          endItem: Math.min((pagination.currentPage - 1) * data.pagination.limit + data.recordings.length, data.pagination.total)
        }));
      } else {
        setPagination(prev => ({
          ...prev,
          totalItems: data.total || 0,
          totalPages: Math.ceil(data.total / prev.pageSize) || 1,
          // Keep the current page
          startItem: data.recordings.length > 0 ? (pagination.currentPage - 1) * prev.pageSize + 1 : 0,
          endItem: Math.min((pagination.currentPage - 1) * prev.pageSize + data.recordings.length, data.total)
        }));
      }
    } catch (error) {
      console.error('Error loading recordings:', error);
      showStatusMessage('Error loading recordings: ' + error.message);
    }
  };
  
  // Handle date range change
  const handleDateRangeChange = (e) => {
    const newDateRange = e.target.value;
    
    setFilters(prev => ({
      ...prev,
      dateRange: newDateRange
    }));
    
    if (newDateRange === 'custom') {
      // If custom is selected, make sure we have default dates
      if (!filters.startDate || !filters.endDate) {
        const now = new Date();
        const sevenDaysAgo = new Date(now);
        sevenDaysAgo.setDate(now.getDate() - 7);
        
        setFilters(prev => ({
          ...prev,
          endDate: now.toISOString().split('T')[0],
          startDate: sevenDaysAgo.toISOString().split('T')[0]
        }));
      }
    }
  };
  
  // Update active filters
  const updateActiveFilters = () => {
    const hasFilters = (
      filters.dateRange !== 'last7days' ||
      filters.streamId !== 'all' ||
      filters.recordingType !== 'all'
    );
    
    setHasActiveFilters(hasFilters);
    
    if (hasFilters) {
      const activeFilters = [];
      
      // Date range filter
      if (filters.dateRange !== 'last7days') {
        let label = '';
        switch (filters.dateRange) {
          case 'today':
            label = 'Today';
            break;
          case 'yesterday':
            label = 'Yesterday';
            break;
          case 'last30days':
            label = 'Last 30 Days';
            break;
          case 'custom':
            label = `${filters.startDate} to ${filters.endDate}`;
            break;
        }
        activeFilters.push({ key: 'dateRange', label: `Date: ${label}` });
      }
      
      // Stream filter
      if (filters.streamId !== 'all') {
        activeFilters.push({ key: 'streamId', label: `Stream: ${filters.streamId}` });
      }
      
      // Recording type filter
      if (filters.recordingType !== 'all') {
        activeFilters.push({ key: 'recordingType', label: 'Detection Events Only' });
      }
      
      setActiveFiltersDisplay(activeFilters);
    } else {
      setActiveFiltersDisplay([]);
    }
  };
  
  // Apply filters
  const applyFilters = () => {
    // Reset to first page when applying filters
    setPagination(prev => ({
      ...prev,
      currentPage: 1
    }));
    
    loadRecordings();
  };
  
  // Reset filters
  const resetFilters = () => {
    // Reset filter state
    setFilters({
      dateRange: 'last7days',
      startDate: '',
      startTime: '00:00',
      endDate: '',
      endTime: '23:59',
      streamId: 'all',
      recordingType: 'all'
    });
    
    setDefaultDateRange();
    
    // Reset pagination to first page
    setPagination(prev => ({
      ...prev,
      currentPage: 1
    }));
    
    // Clear all URL parameters by replacing the current URL with the base URL
    const baseUrl = window.location.pathname;
    window.history.pushState({ path: baseUrl }, '', baseUrl);
    
    // Load recordings with default settings
    loadRecordings(1, false);
  };
  
  // Remove filter
  const removeFilter = (key) => {
    switch (key) {
      case 'dateRange':
        setFilters(prev => ({
          ...prev,
          dateRange: 'last7days'
        }));
        break;
      case 'streamId':
        setFilters(prev => ({
          ...prev,
          streamId: 'all'
        }));
        break;
      case 'recordingType':
        setFilters(prev => ({
          ...prev,
          recordingType: 'all'
        }));
        break;
    }
    
    applyFilters();
  };
  
  // Sort by field
  const sortBy = (field) => {
    if (sortField === field) {
      // Toggle direction if already sorting by this field
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      // Default to descending for start_time, ascending for others
      setSortDirection(field === 'start_time' ? 'desc' : 'asc');
      setSortField(field);
    }
    
    loadRecordings();
  };
  
  // Go to page
  const goToPage = (page) => {
    if (page < 1 || page > pagination.totalPages) return;
    
    setPagination(prev => ({
      ...prev,
      currentPage: page
    }));
    
    // Create URL parameters object with current filters
    const params = new URLSearchParams(window.location.search);
    
    // Update only the page parameter
    params.set('page', page.toString());
    
    // Update URL without reloading the page
    const newUrl = `${window.location.pathname}?${params.toString()}`;
    window.history.pushState({ path: newUrl }, '', newUrl);
    
    // Use setTimeout to ensure the pagination state is updated before loading recordings
    setTimeout(() => {
      loadRecordings();
    }, 0);
  };
  
  // Format date time
  const formatDateTime = (isoString) => {
    if (!isoString) return '';
    
    const date = new Date(isoString);
    return date.toLocaleString();
  };
  
  // Format duration
  const formatDuration = (seconds) => {
    if (!seconds) return '00:00:00';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    
    return [
      hours.toString().padStart(2, '0'),
      minutes.toString().padStart(2, '0'),
      secs.toString().padStart(2, '0')
    ].join(':');
  };
  
  // Format file size
  const formatFileSize = (bytes) => {
    if (!bytes) return '0 B';
    
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let i = 0;
    let size = bytes;
    
    while (size >= 1024 && i < units.length - 1) {
      size /= 1024;
      i++;
    }
    
    return `${size.toFixed(1)} ${units[i]}`;
  };
  
  // Play recording
  const playRecording = (recording) => {
    console.log('Play recording clicked:', recording);
    
    // Check if recording has an id property
    if (!recording.id) {
      console.error('Recording has no id property:', recording);
      showStatusMessage('Error: Recording has no id property');
      return;
    }
    
    // Build video URL
    const videoUrl = `/api/recordings/play/${recording.id}`;
    const title = `${recording.stream} - ${formatDateTime(recording.start_time)}`;
    const downloadUrl = `/api/recordings/download/${recording.id}`;
    
    console.log('Video URL:', videoUrl);
    console.log('Title:', title);
    console.log('Download URL:', downloadUrl);
    
    // Show video modal
    showVideoModal(videoUrl, title, downloadUrl);
    console.log('Video modal should be shown now');
  };
  
  // Download recording
  const downloadRecording = (recording) => {
    // Create download link
    const downloadUrl = `/api/recordings/download/${recording.id}`;
    const link = document.createElement('a');
    link.href = downloadUrl;
    link.download = `${recording.stream}_${new Date(recording.start_time).toISOString().replace(/[:.]/g, '-')}.mp4`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    
    showStatusMessage('Download started');
  };
  
  // Toggle selection of a recording
  const toggleRecordingSelection = (recordingId) => {
    setSelectedRecordings(prev => ({
      ...prev,
      [recordingId]: !prev[recordingId]
    }));
  };

  // Toggle select all recordings
  const toggleSelectAll = () => {
    const newSelectAll = !selectAll;
    setSelectAll(newSelectAll);
    
    const newSelectedRecordings = {};
    if (newSelectAll) {
      // Select all recordings
      recordings.forEach(recording => {
        newSelectedRecordings[recording.id] = true;
      });
    }
    // Always update selectedRecordings, even when deselecting all
    setSelectedRecordings(newSelectedRecordings);
  };

  // Get count of selected recordings
  const getSelectedCount = () => {
    return Object.values(selectedRecordings).filter(Boolean).length;
  };

  // Open delete confirmation modal
  const openDeleteModal = (mode) => {
    setDeleteMode(mode);
    setIsDeleteModalOpen(true);
  };

  // Close delete confirmation modal
  const closeDeleteModal = () => {
    setIsDeleteModalOpen(false);
  };

  // Handle delete confirmation
  const handleDeleteConfirm = () => {
    closeDeleteModal();
    if (deleteMode === 'selected') {
      deleteSelectedRecordings();
    } else {
      deleteAllFilteredRecordings();
    }
  };

  // Delete selected recordings
  const deleteSelectedRecordings = async () => {
    
    const selectedIds = Object.entries(selectedRecordings)
      .filter(([_, isSelected]) => isSelected)
      .map(([id, _]) => parseInt(id, 10));
    
    if (selectedIds.length === 0) {
      showStatusMessage('No recordings selected');
      return;
    }
    
    try {
      // Save current URL parameters before deletion
      const currentUrlParams = new URLSearchParams(window.location.search);
      const currentSortField = currentUrlParams.get('sort') || sortField;
      const currentSortDirection = currentUrlParams.get('order') || sortDirection;
      const currentPage = parseInt(currentUrlParams.get('page'), 10) || pagination.currentPage;
      
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
      
      // Reset selection
      setSelectedRecordings({});
      setSelectAll(false);
      
      // Reload recordings with preserved parameters
      const reloadWithPreservedParams = async () => {
        // Set the sort parameters directly
        const tempSortField = currentSortField;
        const tempSortDirection = currentSortDirection;
        
        // Set state with the saved values
        setSortField(tempSortField);
        setSortDirection(tempSortDirection);
        setPagination(prev => ({
          ...prev,
          currentPage: currentPage
        }));
        
        // Wait for state to update
        setTimeout(() => {
          // Build query parameters using the same approach as updateUrlWithFilters
          const params = new URLSearchParams(window.location.search);
          
          // Update pagination, sort, and order parameters
          params.set('page', currentPage.toString());
          params.set('limit', pagination.pageSize.toString());
          params.set('sort', tempSortField);
          params.set('order', tempSortDirection);
          
          // Update URL with preserved parameters
          const newUrl = `${window.location.pathname}?${params.toString()}`;
          window.history.pushState({ path: newUrl }, '', newUrl);
          
          // Fetch recordings with preserved parameters
          fetch(`/api/recordings?${params.toString()}`)
            .then(response => {
              if (!response.ok) {
                throw new Error('Failed to load recordings');
              }
              return response.json();
            })
            .then(data => {
              console.log('Recordings data received:', data);
              
              // Store recordings in the component state
              setRecordings(data.recordings || []);
              
              // Update pagination without changing the current page
              if (data.pagination) {
                setPagination(prev => ({
                  ...prev,
                  totalItems: data.pagination.total || 0,
                  totalPages: data.pagination.pages || 1,
                  // Keep the current page
                  pageSize: data.pagination.limit || 20,
                  startItem: data.recordings.length > 0 ? (currentPage - 1) * data.pagination.limit + 1 : 0,
                  endItem: Math.min((currentPage - 1) * data.pagination.limit + data.recordings.length, data.pagination.total)
                }));
              } else {
                setPagination(prev => ({
                  ...prev,
                  totalItems: data.total || 0,
                  totalPages: Math.ceil(data.total / prev.pageSize) || 1,
                  // Keep the current page
                  startItem: data.recordings.length > 0 ? (currentPage - 1) * prev.pageSize + 1 : 0,
                  endItem: Math.min((currentPage - 1) * prev.pageSize + data.recordings.length, data.total)
                }));
              }
            })
            .catch(error => {
              console.error('Error loading recordings:', error);
              showStatusMessage('Error loading recordings: ' + error.message);
            });
        }, 0);
      };
      
      // Execute the reload function
      reloadWithPreservedParams();
    } catch (error) {
      console.error('Error in batch delete operation:', error);
      showStatusMessage('Error in batch delete operation: ' + error.message);
    }
  };

  // Delete all recordings matching current filter
  const deleteAllFilteredRecordings = async () => {
    
    try {
      // Create filter object
      const filter = {};
      
      // Add date range filters
      if (filters.dateRange === 'custom') {
        filter.start = `${filters.startDate}T${filters.startTime}:00`;
        filter.end = `${filters.endDate}T${filters.endTime}:00`;
      } else {
        // Convert predefined range to actual dates
        const { start, end } = getDateRangeFromPreset(filters.dateRange);
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
      
      // Reset selection
      setSelectedRecordings({});
      setSelectAll(false);
      
      // Reload recordings
      loadRecordings();
    } catch (error) {
      console.error('Error in delete all operation:', error);
      showStatusMessage('Error in delete all operation: ' + error.message);
    }
  };

  // Delete a single recording
  const deleteRecording = async (recording) => {
    if (!confirm(`Are you sure you want to delete this recording from ${recording.stream}?`)) {
      return;
    }
    
    try {
      // Save current URL parameters before deletion
      const currentUrlParams = new URLSearchParams(window.location.search);
      const currentSortField = currentUrlParams.get('sort') || sortField;
      const currentSortDirection = currentUrlParams.get('order') || sortDirection;
      const currentPage = parseInt(currentUrlParams.get('page'), 10) || pagination.currentPage;
      
      const response = await fetch(`/api/recordings/${recording.id}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) {
        throw new Error('Failed to delete recording');
      }
      
      showStatusMessage('Recording deleted successfully');
      
      // Create a function to reload with preserved parameters
      const reloadWithPreservedParams = async () => {
        // Set the sort parameters directly
        const tempSortField = currentSortField;
        const tempSortDirection = currentSortDirection;
        
        // Set state with the saved values
        setSortField(tempSortField);
        setSortDirection(tempSortDirection);
        setPagination(prev => ({
          ...prev,
          currentPage: currentPage
        }));
        
        // Wait for state to update
        setTimeout(() => {
          // Build query parameters using the same approach as updateUrlWithFilters
          const params = new URLSearchParams(window.location.search);
          
          // Update pagination, sort, and order parameters
          params.set('page', currentPage.toString());
          params.set('limit', pagination.pageSize.toString());
          params.set('sort', tempSortField);
          params.set('order', tempSortDirection);
          
          // Update URL with preserved parameters
          const newUrl = `${window.location.pathname}?${params.toString()}`;
          window.history.pushState({ path: newUrl }, '', newUrl);
          
          // Fetch recordings with preserved parameters
          fetch(`/api/recordings?${params.toString()}`)
            .then(response => {
              if (!response.ok) {
                throw new Error('Failed to load recordings');
              }
              return response.json();
            })
            .then(data => {
              console.log('Recordings data received:', data);
              
              // Store recordings in the component state
              setRecordings(data.recordings || []);
              
              // Update pagination without changing the current page
              if (data.pagination) {
                setPagination(prev => ({
                  ...prev,
                  totalItems: data.pagination.total || 0,
                  totalPages: data.pagination.pages || 1,
                  // Keep the current page
                  pageSize: data.pagination.limit || 20,
                  startItem: data.recordings.length > 0 ? (currentPage - 1) * data.pagination.limit + 1 : 0,
                  endItem: Math.min((currentPage - 1) * data.pagination.limit + data.recordings.length, data.pagination.total)
                }));
              } else {
                setPagination(prev => ({
                  ...prev,
                  totalItems: data.total || 0,
                  totalPages: Math.ceil(data.total / prev.pageSize) || 1,
                  // Keep the current page
                  startItem: data.recordings.length > 0 ? (currentPage - 1) * prev.pageSize + 1 : 0,
                  endItem: Math.min((currentPage - 1) * prev.pageSize + data.recordings.length, data.total)
                }));
              }
            })
            .catch(error => {
              console.error('Error loading recordings:', error);
              showStatusMessage('Error loading recordings: ' + error.message);
            });
        }, 0);
      };
      
      // Execute the reload function
      reloadWithPreservedParams();
    } catch (error) {
      console.error('Error deleting recording:', error);
      showStatusMessage('Error deleting recording: ' + error.message);
    }
  };
  
  return html`
    <section id="recordings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Recordings</h2>
        <button id="toggle-filters-btn" 
                class="p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
                title="Toggle Filters"
                onClick=${toggleFilters}>
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
          </svg>
        </button>
      </div>
      
      <div class="recordings-layout flex flex-col md:flex-row gap-4">
        <!-- Sidebar for filters -->
        <${FiltersSidebar}
          filters=${filters}
          setFilters=${setFilters}
          pagination=${pagination}
          setPagination=${setPagination}
          streams=${streams}
          filtersVisible=${filtersVisible}
          applyFilters=${applyFilters}
          resetFilters=${resetFilters}
          handleDateRangeChange=${handleDateRangeChange}
          setDefaultDateRange=${setDefaultDateRange}
        />
        
        <!-- Main content area -->
        <div class="recordings-content flex-1">
          <${ActiveFilters}
            activeFiltersDisplay=${activeFiltersDisplay}
            removeFilter=${removeFilter}
            hasActiveFilters=${hasActiveFilters}
          />
          
          <${RecordingsTable}
            recordings=${recordings}
            sortField=${sortField}
            sortDirection=${sortDirection}
            sortBy=${sortBy}
            selectedRecordings=${selectedRecordings}
            toggleRecordingSelection=${toggleRecordingSelection}
            selectAll=${selectAll}
            toggleSelectAll=${toggleSelectAll}
            getSelectedCount=${getSelectedCount}
            openDeleteModal=${openDeleteModal}
            playRecording=${playRecording}
            downloadRecording=${downloadRecording}
            deleteRecording=${deleteRecording}
            recordingsTableBodyRef=${recordingsTableBodyRef}
            pagination=${pagination}
          />
          
          <${PaginationControls}
            pagination=${pagination}
            goToPage=${goToPage}
          />
        </div>
      </div>
      
      <${DeleteConfirmationModal}
        isOpen=${isDeleteModalOpen}
        onClose=${closeDeleteModal}
        onConfirm=${handleDeleteConfirm}
        mode=${deleteMode}
        count=${getSelectedCount()}
      />
    </section>
  `;
}

/**
 * Load RecordingsView component
 */
export function loadRecordingsView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the RecordingsView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${RecordingsView} />`, mainContent);
  });
}
