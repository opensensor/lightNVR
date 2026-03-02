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
    if (!urlParams.has('dateRange') && !urlParams.has('page') && !urlParams.has('sort') && !urlParams.has('detection') && !urlParams.has('stream') && !urlParams.has('detection_label') && !urlParams.has('protected')) {
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
        recordingType: 'all',
        detectionLabel: '',
        protectedStatus: 'all'
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

    // Detection label
    if (urlParams.has('detection_label')) {
      result.filters.detectionLabel = urlParams.get('detection_label');
    }

    // Protected status
    if (urlParams.has('protected')) {
      result.filters.protectedStatus = urlParams.get('protected') === '1' ? 'yes' : 'no';
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
      filters.recordingType !== 'all' ||
      (filters.detectionLabel && filters.detectionLabel.trim() !== '') ||
      (filters.protectedStatus && filters.protectedStatus !== 'all')
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

      // Detection label filter
      if (filters.detectionLabel && filters.detectionLabel.trim() !== '') {
        activeFilters.push({ key: 'detectionLabel', label: `Object: ${filters.detectionLabel.trim()}` });
      }

      // Protected status filter
      if (filters.protectedStatus === 'yes') {
        activeFilters.push({ key: 'protectedStatus', label: 'Protected: Yes' });
      } else if (filters.protectedStatus === 'no') {
        activeFilters.push({ key: 'protectedStatus', label: 'Protected: No' });
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

    // Protected status
    if (urlParams.has('protected')) {
      newFilters.protectedStatus = urlParams.get('protected') === '1' ? 'yes' : 'no';
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
    const url = new URL(window.location.href);

    // Date range
    url.searchParams.set('dateRange', filters.dateRange);
    if (filters.dateRange === 'custom') {
      url.searchParams.set('startDate', filters.startDate);
      url.searchParams.set('startTime', filters.startTime);
      url.searchParams.set('endDate', filters.endDate);
      url.searchParams.set('endTime', filters.endTime);
    } else {
      url.searchParams.delete('startDate');
      url.searchParams.delete('startTime');
      url.searchParams.delete('endDate');
      url.searchParams.delete('endTime');
    }

    // Stream filter
    if (filters.streamId !== 'all') url.searchParams.set('stream', filters.streamId);
    else url.searchParams.delete('stream');

    // Recording type filter
    if (filters.recordingType === 'detection') url.searchParams.set('detection', '1');
    else url.searchParams.delete('detection');

    // Detection label filter
    if (filters.detectionLabel && filters.detectionLabel.trim() !== '') {
      url.searchParams.set('detection_label', filters.detectionLabel.trim());
    } else {
      url.searchParams.delete('detection_label');
    }

    // Protected status filter
    if (filters.protectedStatus === 'yes') url.searchParams.set('protected', '1');
    else if (filters.protectedStatus === 'no') url.searchParams.set('protected', '0');
    else url.searchParams.delete('protected');

    // Pagination
    url.searchParams.set('page', pagination.currentPage.toString());
    url.searchParams.set('limit', pagination.pageSize.toString());

    // Sorting
    url.searchParams.set('sort', sortField);
    url.searchParams.set('order', sortDirection);

    window.history.replaceState({}, '', url);
  }
};
