/**
 * URL utility functions for RecordingsView
 */

import { formatUtils } from './formatUtils.js';
import { getDefaultDateRange } from '../../../utils/date-utils.js';

const DEFAULT_PAGE_SIZE = 20;
const ALL_PAGE_SIZE = 'all';

const parseMultiValueParam = (value) => {
  if (!value) return [];

  return [...new Set(
    value
      .split(',')
      .map((item) => item.trim())
      .filter(Boolean)
  )];
};

const serializeMultiValueParam = (values) => {
  if (!Array.isArray(values)) return '';

  return [...new Set(
    values
      .map((item) => (typeof item === 'string' ? item.trim() : ''))
      .filter(Boolean)
  )].join(',');
};

const addMultiValue = (values, value) => {
  const normalizedValue = typeof value === 'string' ? value.trim() : '';
  if (!normalizedValue) return Array.isArray(values) ? values : [];

  const currentValues = Array.isArray(values) ? values : [];
  return currentValues.includes(normalizedValue)
    ? currentValues
    : [...currentValues, normalizedValue];
};

const removeMultiValue = (values, value) => {
  if (!Array.isArray(values)) return [];
  return values.filter((item) => item !== value);
};

const parsePaginationLimit = (value) => {
  if (value === ALL_PAGE_SIZE) {
    return { pageSize: DEFAULT_PAGE_SIZE, showAll: true };
  }

  const parsedValue = parseInt(value || String(DEFAULT_PAGE_SIZE), 10);
  return {
    pageSize: Number.isFinite(parsedValue) && parsedValue > 0 ? parsedValue : DEFAULT_PAGE_SIZE,
    showAll: false
  };
};

const serializePaginationLimit = (pagination = {}) => (
  pagination.showAll ? ALL_PAGE_SIZE : String(pagination.pageSize || DEFAULT_PAGE_SIZE)
);

const createDefaultFilters = () => {
  const { startDate, endDate } = getDefaultDateRange();

  return {
    dateRange: 'last7days',
    startDate,
    startTime: '00:00',
    endDate,
    endTime: '23:59',
    streamIds: [],
    recordingType: 'all',
    detectionLabels: [],
    tags: [],
    captureMethods: [],
    protectedStatus: 'all'
  };
};

/**
 * URL utilities for RecordingsView
 */
export const urlUtils = {
  createDefaultFilters,
  parseMultiValueParam,
  serializeMultiValueParam,
  addMultiValue,
  removeMultiValue,
  parsePaginationLimit,
  serializePaginationLimit,

  /**
   * Get filters from URL
   * @returns {Object|null} Filters object or null if no filters in URL
   */
  getFiltersFromUrl: () => {
    // Get URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    
    // Check if we have any filter parameters
    if (!urlParams.has('dateRange') &&
        !urlParams.has('page') &&
        !urlParams.has('sort') &&
        !urlParams.has('detection') &&
        !urlParams.has('stream') &&
        !urlParams.has('detection_label') &&
        !urlParams.has('tag') &&
        !urlParams.has('capture_method') &&
        !urlParams.has('protected')) {
      return null;
    }
    
    // Create result object
    const result = {
      filters: createDefaultFilters(),
      page: 1,
      limit: DEFAULT_PAGE_SIZE,
      showAll: false,
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
      result.filters.streamIds = parseMultiValueParam(urlParams.get('stream'));
    }
    
    // Recording type
    if (urlParams.has('detection')) {
      if (urlParams.get('detection') === '1') result.filters.recordingType = 'detection';
      else if (urlParams.get('detection') === '-1') result.filters.recordingType = 'no_detection';
    }

    // Detection label
    if (urlParams.has('detection_label')) {
      result.filters.detectionLabels = parseMultiValueParam(urlParams.get('detection_label'));
    }

    // Recording tags
    if (urlParams.has('tag')) {
      result.filters.tags = parseMultiValueParam(urlParams.get('tag'));
    }

    // Capture methods
    if (urlParams.has('capture_method')) {
      result.filters.captureMethods = parseMultiValueParam(urlParams.get('capture_method'));
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
      const paginationLimit = parsePaginationLimit(urlParams.get('limit'));
      result.limit = paginationLimit.pageSize;
      result.showAll = paginationLimit.showAll;
      if (paginationLimit.showAll) {
        result.page = 1;
      }
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
      filters.streamIds.length > 0 ||
      filters.recordingType !== 'all' ||
      filters.detectionLabels.length > 0 ||
      filters.tags.length > 0 ||
      filters.captureMethods.length > 0 ||
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
      filters.streamIds.forEach((streamId) => {
        activeFilters.push({ key: 'streamIds', value: streamId, label: `Stream: ${streamId}` });
      });
      
      // Recording type filter
      if (filters.recordingType === 'detection') {
        activeFilters.push({ key: 'recordingType', label: 'Detection Events Only' });
      } else if (filters.recordingType === 'no_detection') {
        activeFilters.push({ key: 'recordingType', label: 'No Detection Events Only' });
      }

      // Detection label filter
      filters.detectionLabels.forEach((label) => {
        activeFilters.push({ key: 'detectionLabels', value: label, label: `Object: ${label}` });
      });

      // Tag filter
      filters.tags.forEach((tag) => {
        activeFilters.push({ key: 'tags', value: tag, label: `Tag: ${tag}` });
      });

      // Capture method filter
      filters.captureMethods.forEach((captureMethod) => {
        activeFilters.push({
          key: 'captureMethods',
          value: captureMethod,
          label: `Capture: ${formatUtils.formatCaptureMethod(captureMethod)}`
        });
      });

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
      newFilters.streamIds = parseMultiValueParam(urlParams.get('stream'));
    }
    
    // Recording type - IMPORTANT: Check for this parameter even if dateRange is not present
    if (urlParams.has('detection')) {
      if (urlParams.get('detection') === '1') newFilters.recordingType = 'detection';
      else if (urlParams.get('detection') === '-1') newFilters.recordingType = 'no_detection';
    }

    if (urlParams.has('detection_label')) {
      newFilters.detectionLabels = parseMultiValueParam(urlParams.get('detection_label'));
    }

    if (urlParams.has('tag')) {
      newFilters.tags = parseMultiValueParam(urlParams.get('tag'));
    }

    if (urlParams.has('capture_method')) {
      newFilters.captureMethods = parseMultiValueParam(urlParams.get('capture_method'));
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
      const paginationLimit = parsePaginationLimit(urlParams.get('limit'));
      setPagination(prev => ({
        ...prev,
        currentPage: paginationLimit.showAll ? 1 : prev.currentPage,
        pageSize: paginationLimit.pageSize,
        showAll: paginationLimit.showAll
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
    const serializedStreams = serializeMultiValueParam(filters.streamIds);
    if (serializedStreams) url.searchParams.set('stream', serializedStreams);
    else url.searchParams.delete('stream');

    // Recording type filter
    if (filters.recordingType === 'detection') url.searchParams.set('detection', '1');
    else if (filters.recordingType === 'no_detection') url.searchParams.set('detection', '-1');
    else url.searchParams.delete('detection');

    // Detection label filter
    const serializedDetectionLabels = serializeMultiValueParam(filters.detectionLabels);
    if (serializedDetectionLabels) {
      url.searchParams.set('detection_label', serializedDetectionLabels);
    } else {
      url.searchParams.delete('detection_label');
    }

    const serializedTags = serializeMultiValueParam(filters.tags);
    if (serializedTags) url.searchParams.set('tag', serializedTags);
    else url.searchParams.delete('tag');

    const serializedCaptureMethods = serializeMultiValueParam(filters.captureMethods);
    if (serializedCaptureMethods) url.searchParams.set('capture_method', serializedCaptureMethods);
    else url.searchParams.delete('capture_method');

    // Protected status filter
    if (filters.protectedStatus === 'yes') url.searchParams.set('protected', '1');
    else if (filters.protectedStatus === 'no') url.searchParams.set('protected', '0');
    else url.searchParams.delete('protected');

    // Pagination
    url.searchParams.set('page', pagination.currentPage.toString());
    url.searchParams.set('limit', serializePaginationLimit(pagination));

    // Sorting
    url.searchParams.set('sort', sortField);
    url.searchParams.set('order', sortDirection);

    window.history.replaceState({}, '', url);
  }
};
