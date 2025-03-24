/**
 * URL utility functions for RecordingsView
 */

/**
 * URL utilities for RecordingsView
 */
export const urlUtils = {
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
  }
};
