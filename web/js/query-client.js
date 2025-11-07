/**
 * LightNVR Web Interface Query Client
 * TanStack Query (Preact Query) integration for Preact
 */

// Import from preact/compat to ensure Preact compatibility
import {h, createElement} from 'preact';
import { 
  QueryClient, 
  QueryClientProvider,
  useQuery as originalUseQuery,
  useMutation as originalUseMutation,
  useQueryClient as originalUseQueryClient
} from '@preact-signals/query';

import { fetchJSON, enhancedFetch, createRequestController } from './fetch-utils.js';

// Export QueryClient class, QueryClientProvider, and fetchJSON
export { QueryClient, QueryClientProvider, fetchJSON };

// Create a QueryClient instance
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes - data considered fresh
      cacheTime: 24 * 60 * 60 * 1000, // 24 hours - keep in cache (matches session duration)
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

// Custom hook to use the QueryClient
export function useQueryClient() {
  return originalUseQueryClient();
}

/**
 * Custom hook for data fetching with Preact Query
 * @param {string|Object} queryKeyOrOptions - Unique key for the query or options object
 * @param {string} [url] - URL to fetch (only used if first param is a string)
 * @param {Object} [options] - Fetch options (only used if first param is a string)
 * @param {Object} [queryOptions] - Preact Query options (only used if first param is a string)
 * @returns {Object} - Query result
 */
export function useQuery(queryKeyOrOptions, url, options = {}, queryOptions = {}) {
  // Check if the first parameter is an object (old style) or a string/array (new style)
  if (typeof queryKeyOrOptions === 'object' && !Array.isArray(queryKeyOrOptions) && queryKeyOrOptions !== null) {
    // Old style: useQuery({ queryKey, queryFn, ...options })
    return originalUseQuery(queryKeyOrOptions);
  } else {
    // New style: useQuery(queryKey, url, options, queryOptions)
    return originalUseQuery({
      queryKey: Array.isArray(queryKeyOrOptions) ? queryKeyOrOptions : [queryKeyOrOptions],
      queryFn: async () => {
        const controller = createRequestController();
        const fetchOptions = {
          ...options,
          signal: controller.signal
        };
        
        try {
          return await fetchJSON(url, fetchOptions);
        } catch (error) {
          console.error(`useQuery error for ${url}:`, error);
          throw error;
        }
      },
      ...queryOptions
    });
  }
}

/**
 * Custom hook for mutations with Preact Query
 * @param {Object} options - Mutation options
 * @returns {Object} - Mutation result
 */
export function useMutation(options) {
  return originalUseMutation({
    ...options
  });
}

/**
 * Custom hook for POST mutations
 * @param {string} url - URL to post to
 * @param {Object} options - Fetch options
 * @param {Object} mutationOptions - Preact Query mutation options
 * @returns {Object} - Mutation result
 */
export function usePostMutation(url, options = {}, mutationOptions = {}) {
  return useMutation({
    mutationFn: async (data) => {
      const controller = createRequestController();
      const fetchOptions = {
        ...options,
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...options.headers
        },
        body: JSON.stringify(data),
        signal: controller.signal
      };
      
      try {
        return await fetchJSON(url, fetchOptions);
      } catch (error) {
        console.error(`usePostMutation error for ${url}:`, error);
        throw error;
      }
    },
    ...mutationOptions
  });
}

/**
 * Custom hook for PUT mutations
 * @param {string} url - URL to put to
 * @param {Object} options - Fetch options
 * @param {Object} mutationOptions - Preact Query mutation options
 * @returns {Object} - Mutation result
 */
export function usePutMutation(url, options = {}, mutationOptions = {}) {
  return useMutation({
    mutationFn: async (data) => {
      const controller = createRequestController();
      const fetchOptions = {
        ...options,
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          ...options.headers
        },
        body: JSON.stringify(data),
        signal: controller.signal
      };
      
      try {
        return await fetchJSON(url, fetchOptions);
      } catch (error) {
        console.error(`usePutMutation error for ${url}:`, error);
        throw error;
      }
    },
    ...mutationOptions
  });
}

/**
 * Custom hook for DELETE mutations
 * @param {string} url - URL to delete from
 * @param {Object} options - Fetch options
 * @param {Object} mutationOptions - Preact Query mutation options
 * @returns {Object} - Mutation result
 */
export function useDeleteMutation(url, options = {}, mutationOptions = {}) {
  return useMutation({
    mutationFn: async (data) => {
      const controller = createRequestController();
      const fetchOptions = {
        ...options,
        method: 'DELETE',
        signal: controller.signal,
        ...(data && {
          headers: {
            'Content-Type': 'application/json',
            ...options.headers
          },
          body: JSON.stringify(data)
        })
      };
      
      try {
        return await fetchJSON(url, fetchOptions);
      } catch (error) {
        console.error(`useDeleteMutation error for ${url}:`, error);
        throw error;
      }
    },
    ...mutationOptions
  });
}

/**
 * Invalidate queries by key pattern
 * @param {string|Array} queryKey - Query key or pattern to invalidate
 */
export function invalidateQueries(queryKey) {
  queryClient.invalidateQueries({
    queryKey: Array.isArray(queryKey) ? queryKey : [queryKey]
  });
}

/**
 * Prefetch a query
 * @param {string|Array} queryKey - Query key
 * @param {string} url - URL to fetch
 * @param {Object} options - Fetch options
 * @param {Object} queryOptions - Preact Query options
 */
export function prefetchQuery(queryKey, url, options = {}, queryOptions = {}) {
  queryClient.prefetchQuery({
    queryKey: Array.isArray(queryKey) ? queryKey : [queryKey],
    queryFn: async () => {
      try {
        return await fetchJSON(url, options);
      } catch (error) {
        console.error(`prefetchQuery error for ${url}:`, error);
        throw error;
      }
    },
    ...queryOptions
  });
}

/**
 * Set query data directly
 * @param {string|Array} queryKey - Query key
 * @param {any} data - Data to set
 */
export function setQueryData(queryKey, data) {
  queryClient.setQueryData(
    Array.isArray(queryKey) ? queryKey : [queryKey],
    data
  );
}

/**
 * Get query data directly
 * @param {string|Array} queryKey - Query key
 * @returns {any} - Query data
 */
export function getQueryData(queryKey) {
  return queryClient.getQueryData(
    Array.isArray(queryKey) ? queryKey : [queryKey]
  );
}
