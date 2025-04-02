/**
 * URL utility functions for RecordingsView
 */

/**
 * URL utilities for RecordingsView
 */
export const urlUtils = {
  /**
   * Get filters from URL
   * @returns {Object|null} Filters object or null if no filters in URL
   */
  getFiltersFromUrl: () => {
    // Get URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    
    // Check if we have any filter parameters
    if (!urlParams.has('dateRange') && !urlParams.has('page') && !urlParams.has('sort') && !urlParams.has('detection') && !urlParams.has('stream')) {
      return null;
    }
    
    // Create result object
    const result = {
      filters: {
        dateRange: 'last7days',
        startDate: '',
        startTime: '00:00',
        endDate: '',
        endTime: '23:59',
        streamId: 'all',
        recordingType: 'all'
      },
      page: 1,
      limit: 20,
      sort: 'start_time',
      order: 'desc'
    };
    
    // Date range
    if (urlParams.has('dateRange')) {
      result.filters.dateRange = urlParams.get('dateRange');
      
      if (result.filters.dateRange === 'custom') {
        if (urlParams.has('startDate')) {
          result.filters.startDate = urlParams.get('startDate');
        }
        if (urlParams.has('startTime')) {
          result.filters.startTime = urlParams.get('startTime');
        }
        if (urlParams.has('endDate')) {
          result.filters.endDate = urlParams.get('endDate');
        }
        if (urlParams.has('endTime')) {
          result.filters.endTime = urlParams.get('endTime');
        }
      }
    }
    
    // Stream
    if (urlParams.has('stream')) {
      result.filters.streamId = urlParams.get('stream');
    }
    
    // Recording type
    if (urlParams.has('detection') && urlParams.get('detection') === '1') {
      result.filters.recordingType = 'detection';
    }
    
    // Pagination
    if (urlParams.has('page')) {
      result.page = parseInt(urlParams.get('page'), 10);
    }
    if (urlParams.has('limit')) {
      result.limit = parseInt(urlParams.get('limit'), 10);
    }
    
    // Sorting
    if (urlParams.has('sort')) {
      result.sort = urlParams.get('sort');
    }
    if (urlParams.has('order')) {
      result.order = urlParams.get('order');
    }
    
    return result;
  },
  
  /**
   * Get active filters display
   * @param {Object} filters Current filters
   * @returns {Array} Array of active filter objects with key and label
   */
  getActiveFiltersDisplay: (filters) => {
    const activeFilters = [];
    
    // Check if we have any active filters
    const hasFilters = (
      filters.dateRange !== 'last7days' ||
      filters.streamId !== 'all' ||
      filters.recordingType !== 'all'
    );
    
    if (hasFilters) {
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
    }
    
    return activeFilters;
  },
  
  /**
   * Load filters from URL
   * @param {Object} filters Current filters
   * @param {Object} pagination Current pagination
   * @param {Function} setFilters Function to update filters
   * @param {Function} setPagination Function to update pagination
   * @param {Function} setSortField Function to update sort field
   * @param {Function} setSortDirection Function to update sort direction
   */
  loadFiltersFromUrl: (filters, pagination, setFilters, setPagination, setSortField, setSortDirection) => {
    // Get URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    
    // Create a new filters object based on the current filters
    const newFilters = { ...filters };
    
    // Date range
    if (urlParams.has('dateRange')) {
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
    }
    
    // Stream
    if (urlParams.has('stream')) {
      newFilters.streamId = urlParams.get('stream');
    }
    
    // Recording type - IMPORTANT: Check for this parameter even if dateRange is not present
    if (urlParams.has('detection') && urlParams.get('detection') === '1') {
      newFilters.recordingType = 'detection';
    }
    
    // Update filters state
    setFilters(newFilters);
    
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
  },
  
  /**
   * Update URL with filters
   * @param {Object} filters Current filters
   * @param {Object} pagination Current pagination
   * @param {string} sortField Current sort field
   * @param {string} sortDirection Current sort direction
   */
  updateUrlWithFilters: (filters, pagination, sortField, sortDirection) => {
    // Create URL parameters object based on current URL to preserve any existing parameters
    const params = new URLSearchParams(window.location.search);
    
    // Add a timestamp parameter to prevent caching issues
    params.set('t', Date.now().toString());
    
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
    
    // Also update the reload behavior to maintain URL parameters
    // This is the key to preserving parameters during page reload
    const reloadUrl = newUrl;
    window.onbeforeunload = function() {
      // Replace the current URL with our preserved URL just before reload
      window.history.replaceState({ path: reloadUrl }, '', reloadUrl);
    };
  }
};
