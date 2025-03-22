/**
 * LightNVR Web Interface RecordingsView Component
 * Preact component for the recordings page
 */

import { h } from '../../preact.min.js';
import { html } from '../../preact-app.js';
import { useState, useEffect, useRef, useCallback } from '../../preact.hooks.module.js';
import { showStatusMessage, showVideoModal } from './UI.js';

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
    // Create URL parameters object
    const params = new URLSearchParams();
    
    // Add date range
    params.append('dateRange', filters.dateRange);
    if (filters.dateRange === 'custom') {
      params.append('startDate', filters.startDate);
      params.append('startTime', filters.startTime);
      params.append('endDate', filters.endDate);
      params.append('endTime', filters.endTime);
    }
    
    // Add stream filter if not "all"
    if (filters.streamId !== 'all') {
      params.append('stream', filters.streamId);
    }
    
    // Add recording type if not "all"
    if (filters.recordingType !== 'all') {
      params.append('detection', '1');
    }
    
    // Add pagination
    params.append('page', pagination.currentPage);
    params.append('limit', pagination.pageSize);
    
    // Add sorting
    params.append('sort', sortField);
    params.append('order', sortDirection);
    
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
  const loadRecordings = async () => {
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
      
      // Update URL with filters
      updateUrlWithFilters();
      
      // Fetch recordings
      const response = await fetch(`/api/recordings?${params.toString()}`);
      if (!response.ok) {
        throw new Error('Failed to load recordings');
      }
      
      const data = await response.json();
      console.log('Recordings data received:', data);
      
      // Store recordings in the component state
      setRecordings(data.recordings || []);
      
      // Update pagination
      if (data.pagination) {
        setPagination(prev => ({
          ...prev,
          totalItems: data.pagination.total || 0,
          totalPages: data.pagination.pages || 1,
          currentPage: data.pagination.page || 1,
          pageSize: data.pagination.limit || 20,
          startItem: data.recordings.length > 0 ? (data.pagination.page - 1) * data.pagination.limit + 1 : 0,
          endItem: Math.min((data.pagination.page - 1) * data.pagination.limit + data.recordings.length, data.pagination.total)
        }));
      } else {
        setPagination(prev => ({
          ...prev,
          totalItems: data.total || 0,
          totalPages: Math.ceil(data.total / prev.pageSize) || 1,
          startItem: data.recordings.length > 0 ? (prev.currentPage - 1) * prev.pageSize + 1 : 0,
          endItem: Math.min((prev.currentPage - 1) * prev.pageSize + data.recordings.length, data.total)
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
    
    setPagination(prev => ({
      ...prev,
      currentPage: 1
    }));
    
    loadRecordings();
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
    
    loadRecordings();
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
    // Build video URL
    const videoUrl = `/api/recordings/play/${recording.id}`;
    const title = `${recording.stream} - ${formatDateTime(recording.start_time)}`;
    const downloadUrl = `/api/recordings/download/${recording.id}`;
    
    // Show video modal
    showVideoModal(videoUrl, title, downloadUrl);
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
  
  // Delete recording
  const deleteRecording = async (recording) => {
    if (!confirm(`Are you sure you want to delete this recording from ${recording.stream}?`)) {
      return;
    }
    
    try {
      const response = await fetch(`/api/recordings/${recording.id}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) {
        throw new Error('Failed to delete recording');
      }
      
      showStatusMessage('Recording deleted successfully');
      loadRecordings();
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
        <aside id="filters-sidebar" 
               class=${`filters-sidebar w-full md:w-64 bg-white dark:bg-gray-800 rounded-lg shadow p-4 md:sticky md:top-4 md:self-start transition-all duration-300 ${!filtersVisible ? 'hidden md:block' : ''}`}>
          <div class="filter-group mb-4">
            <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Date Range</h3>
            <div class="filter-option mb-2">
              <label for="date-range-select" class="block mb-1 text-sm font-medium">Quick Select:</label>
              <select id="date-range-select" 
                      class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${filters.dateRange}
                      onChange=${handleDateRangeChange}>
                <option value="today">Today</option>
                <option value="yesterday">Yesterday</option>
                <option value="last7days">Last 7 Days</option>
                <option value="last30days">Last 30 Days</option>
                <option value="custom">Custom Range</option>
              </select>
            </div>
            
            <div id="custom-date-range" 
                 class="filter-option space-y-3"
                 style=${filters.dateRange === 'custom' ? 'display: block' : 'display: none'}>
              <div class="space-y-1">
                <label for="start-date" class="block text-sm font-medium">Start Date:</label>
                <input type="date" id="start-date" 
                       class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                       value=${filters.startDate}
                       onChange=${e => setFilters(prev => ({ ...prev, startDate: e.target.value }))} />
                <div class="mt-1">
                  <label for="start-time" class="block text-sm font-medium">Time:</label>
                  <input type="time" id="start-time" 
                         class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                         value=${filters.startTime}
                         onChange=${e => setFilters(prev => ({ ...prev, startTime: e.target.value }))} />
                </div>
              </div>
              
              <div class="space-y-1">
                <label for="end-date" class="block text-sm font-medium">End Date:</label>
                <input type="date" id="end-date" 
                       class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                       value=${filters.endDate}
                       onChange=${e => setFilters(prev => ({ ...prev, endDate: e.target.value }))} />
                <div class="mt-1">
                  <label for="end-time" class="block text-sm font-medium">Time:</label>
                  <input type="time" id="end-time" 
                         class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                         value=${filters.endTime}
                         onChange=${e => setFilters(prev => ({ ...prev, endTime: e.target.value }))} />
                </div>
              </div>
            </div>
          </div>
          
          <div class="filter-group mb-4">
            <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Stream</h3>
            <div class="filter-option">
              <select id="stream-filter" 
                      class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${filters.streamId}
                      onChange=${e => setFilters(prev => ({ ...prev, streamId: e.target.value }))}>
                <option value="all">All Streams</option>
                ${streams.map(stream => html`
                  <option key=${stream.name} value=${stream.name}>${stream.name}</option>
                `)}
              </select>
            </div>
          </div>
          
          <div class="filter-group mb-4">
            <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Recording Type</h3>
            <div class="filter-option">
              <select id="detection-filter" 
                      class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${filters.recordingType}
                      onChange=${e => setFilters(prev => ({ ...prev, recordingType: e.target.value }))}>
                <option value="all">All Recordings</option>
                <option value="detection">Detection Events Only</option>
              </select>
            </div>
          </div>
          
          <div class="filter-group mb-4">
            <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Display Options</h3>
            <div class="filter-option">
              <label for="page-size" class="block mb-1 text-sm font-medium">Items per page:</label>
              <select id="page-size" 
                      class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${pagination.pageSize}
                      onChange=${e => setPagination(prev => ({ ...prev, pageSize: parseInt(e.target.value, 10) }))}>
                <option value="10">10</option>
                <option value="20">20</option>
                <option value="50">50</option>
                <option value="100">100</option>
              </select>
            </div>
          </div>
          
          <div class="filter-actions flex space-x-2">
            <button id="apply-filters-btn" 
                    class="flex-1 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                    onClick=${applyFilters}>
              Apply Filters
            </button>
            <button id="reset-filters-btn" 
                    class="flex-1 px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                    onClick=${resetFilters}>
              Reset
            </button>
          </div>
        </aside>
        
        <!-- Main content area -->
        <div class="recordings-content flex-1">
          ${hasActiveFilters && html`
            <div id="active-filters" 
                 class="active-filters mb-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg flex flex-wrap gap-2">
              ${activeFiltersDisplay.map((filter, index) => html`
                <div key=${index} class="filter-tag inline-flex items-center px-3 py-1 rounded-full text-sm bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-200">
                  <span>${filter.label}</span>
                  <button class="ml-2 text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 focus:outline-none"
                          onClick=${() => removeFilter(filter.key)}>
                    ×
                  </button>
                </div>
              `)}
            </div>
          `}
          
          <div class="recordings-container bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
            <div class="overflow-x-auto">
              <table id="recordings-table" class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead class="bg-gray-50 dark:bg-gray-700">
                  <tr>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                        onClick=${() => sortBy('stream_name')}>
                      <div class="flex items-center">
                        Stream
                        ${sortField === 'stream_name' && html`
                          <span class="sort-icon ml-1">${sortDirection === 'asc' ? '▲' : '▼'}</span>
                        `}
                      </div>
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                        onClick=${() => sortBy('start_time')}>
                      <div class="flex items-center">
                        Start Time
                        ${sortField === 'start_time' && html`
                          <span class="sort-icon ml-1">${sortDirection === 'asc' ? '▲' : '▼'}</span>
                        `}
                      </div>
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      Duration
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                        onClick=${() => sortBy('size_bytes')}>
                      <div class="flex items-center">
                        Size
                        ${sortField === 'size_bytes' && html`
                          <span class="sort-icon ml-1">${sortDirection === 'asc' ? '▲' : '▼'}</span>
                        `}
                      </div>
                    </th>
                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody ref=${recordingsTableBodyRef} class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                  ${recordings.length === 0 ? html`
                    <tr>
                      <td colspan="5" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                        ${pagination.totalItems === 0 ? 'No recordings found' : 'Loading recordings...'}
                      </td>
                    </tr>
                  ` : recordings.map(recording => html`
                    <tr key=${recording.id} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                      <td class="px-6 py-4 whitespace-nowrap">${recording.stream || ''}</td>
                      <td class="px-6 py-4 whitespace-nowrap">${formatDateTime(recording.start_time)}</td>
                      <td class="px-6 py-4 whitespace-nowrap">${formatDuration(recording.duration)}</td>
                      <td class="px-6 py-4 whitespace-nowrap">${recording.size || ''}</td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <div class="flex space-x-2">
                          <button class="p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                                  onClick=${() => playRecording(recording)}
                                  title="Play">
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                            </svg>
                          </button>
                          <button class="p-1 rounded-full text-green-600 hover:bg-green-100 dark:text-green-400 dark:hover:bg-green-900 focus:outline-none"
                                  onClick=${() => downloadRecording(recording)}
                                  title="Download">
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                              <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                            </svg>
                          </button>
                          <button class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                                  onClick=${() => deleteRecording(recording)}
                                  title="Delete">
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                              <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                            </svg>
                          </button>
                        </div>
                      </td>
                    </tr>
                  `)}
                </tbody>
              </table>
            </div>
            <div class="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-gray-200 dark:border-gray-700">
              <div class="pagination-info text-sm text-gray-600 dark:text-gray-400 mb-2 sm:mb-0">
                Showing <span id="pagination-showing">${pagination.startItem}-${pagination.endItem}</span> of <span id="pagination-total">${pagination.totalItems}</span> recordings
              </div>
              <div class="pagination-buttons flex items-center space-x-1">
                <button id="pagination-first" 
                        class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                        title="First Page"
                        onClick=${() => goToPage(1)}
                        disabled=${pagination.currentPage === 1}>
                  «
                </button>
                <button id="pagination-prev" 
                        class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                        title="Previous Page"
                        onClick=${() => goToPage(pagination.currentPage - 1)}
                        disabled=${pagination.currentPage === 1}>
                  ‹
                </button>
                <span id="pagination-current" class="px-2 text-sm text-gray-700 dark:text-gray-300">
                  Page ${pagination.currentPage} of ${pagination.totalPages}
                </span>
                <button id="pagination-next" 
                        class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                        title="Next Page"
                        onClick=${() => goToPage(pagination.currentPage + 1)}
                        disabled=${pagination.currentPage === pagination.totalPages}>
                  ›
                </button>
                <button id="pagination-last" 
                        class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                        title="Last Page"
                        onClick=${() => goToPage(pagination.totalPages)}
                        disabled=${pagination.currentPage === pagination.totalPages}>
                  »
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
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
