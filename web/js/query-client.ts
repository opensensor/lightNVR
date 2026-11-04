/**
 * LightNVR Web Interface Query Client
 * TanStack Query (Preact Query) integration for Preact
 */

import {
  QueryClient, 
  QueryClientProvider,
  useQuery as originalUseQuery,
  useMutation as originalUseMutation,
  useQueryClient as originalUseQueryClient
} from '@preact-signals/query';

import type {
  FetchQueryOptions,
  QueryKey,
  Updater,
  UseMutationOptions,
  UseMutationResult,
  UseQueryOptions,
  UseQueryResult,
} from '@preact-signals/query';

import { fetchJSON, createRequestController } from './fetch-utils.ts';
import type { EnhancedFetchOptions } from './fetch-utils.ts';

// Export QueryClient class, QueryClientProvider, and fetchJSON
export { QueryClient, QueryClientProvider, fetchJSON };

export type {
  QueryKey,
  UseMutationOptions,
  UseMutationResult,
  UseQueryResult,
} from '@preact-signals/query';

/** A bare string is shorthand for a one-element key. */
export type QueryKeyInput = string | QueryKey;

/**
 * TanStack's query options, with placeholderData widened to accept the
 * `(previous) => previous` form that StreamsView, FleetView, and
 * StaticCollectionMembers use. That is the v5 signature: query-core 4.x calls the
 * function with no arguments, so `previous` is always undefined.
 */
export type AppQueryOptions<
  TQueryFnData = unknown,
  TError = Error,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
> = Omit<UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>, 'placeholderData'> & {
  placeholderData?: TQueryFnData | ((previous?: undefined) => TQueryFnData | undefined);
};

/**
 * TanStack's mutation result, plus the `isPending` that StreamsView, UsersView,
 * cameraFavorites, and WorkspacesTab read. That is the v5 name: query-core 4.x
 * reports `isLoading` and never sets `isPending`, so it is always undefined.
 */
export type AppMutationResult<
  TData = unknown,
  TError = Error,
  TVariables = void,
  TContext = unknown,
> = UseMutationResult<TData, TError, TVariables, TContext> & { isPending?: undefined };

/**
 * Fetch options for the JSON mutation helpers. They merge `headers` with object
 * spread, so only the plain-object form of HeadersInit survives. They set method
 * and body themselves.
 */
export type JsonMutationFetchOptions = Omit<EnhancedFetchOptions, 'headers' | 'method' | 'body'> & {
  headers?: Record<string, string>;
};

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
export function useQueryClient(): QueryClient {
  return originalUseQueryClient();
}

/**
 * Custom hook for data fetching with Preact Query
 * @param {string|Object} queryKeyOrOptions - Unique key for the query or options object
 * @param {string} [url] - URL to fetch (only used if first param is a string)
 * @param {Object} [options] - Fetch options (only used if first param is a string)
 * @param {Object} [queryOptions] - Preact Query options (only used if first param is a string)
 * @returns {Object} - Query result
 *
 * Type parameters come first as TQueryFnData, so `useQuery<Foo>(key, url)` types
 * data as Foo. They default to unknown data and to Error, which is what fetchJSON
 * rejects with.
 */
export function useQuery<
  TQueryFnData = unknown,
  TError = Error,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(options: AppQueryOptions<TQueryFnData, TError, TData, TQueryKey>): UseQueryResult<TData, TError>;
export function useQuery<TQueryFnData = unknown, TError = Error, TData = TQueryFnData>(
  queryKey: QueryKeyInput,
  url: string,
  options?: EnhancedFetchOptions,
  queryOptions?: AppQueryOptions<TQueryFnData, TError, TData>,
): UseQueryResult<TData, TError>;
export function useQuery<
  TQueryFnData,
  TError,
  TData,
  TQueryKey extends QueryKey,
>(
  queryKeyOrOptions: QueryKeyInput | AppQueryOptions<TQueryFnData, TError, TData, TQueryKey>,
  url?: string,
  options: EnhancedFetchOptions = {},
  queryOptions: AppQueryOptions<TQueryFnData, TError, TData> = {},
): UseQueryResult<TData, TError> {
  // Check if the first parameter is an object (old style) or a string/array (new style)
  if (typeof queryKeyOrOptions === 'object' && !Array.isArray(queryKeyOrOptions) && queryKeyOrOptions !== null) {
    // Old style: useQuery({ queryKey, queryFn, ...options })
    return originalUseQuery(queryKeyOrOptions as AppQueryOptions<TQueryFnData, TError, TData, TQueryKey>);
  } else {
    // New style: useQuery(queryKey, url, options, queryOptions)
    return originalUseQuery<TQueryFnData, TError, TData>({
      queryKey: Array.isArray(queryKeyOrOptions) ? queryKeyOrOptions : [queryKeyOrOptions],
      queryFn: async () => {
        const controller = createRequestController();
        const fetchOptions = {
          ...options,
          signal: controller.signal
        };
        
        try {
          // The key-and-url overload requires url, and this branch is that overload.
          return await fetchJSON<TQueryFnData>(url!, fetchOptions);
        } catch (error) {
          console.error('useQuery error for', url, ':', error);
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
export function useMutation<TData = unknown, TError = Error, TVariables = void, TContext = unknown>(
  options: UseMutationOptions<TData, TError, TVariables, TContext>,
): AppMutationResult<TData, TError, TVariables, TContext> {
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
export function usePostMutation<TData = unknown, TVariables = unknown, TError = Error, TContext = unknown>(
  url: string,
  options: JsonMutationFetchOptions = {},
  mutationOptions: UseMutationOptions<TData, TError, TVariables, TContext> = {},
): AppMutationResult<TData, TError, TVariables, TContext> {
  return useMutation<TData, TError, TVariables, TContext>({
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
        return await fetchJSON<TData>(url, fetchOptions);
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
export function usePutMutation<TData = unknown, TVariables = unknown, TError = Error, TContext = unknown>(
  url: string,
  options: JsonMutationFetchOptions = {},
  mutationOptions: UseMutationOptions<TData, TError, TVariables, TContext> = {},
): AppMutationResult<TData, TError, TVariables, TContext> {
  return useMutation<TData, TError, TVariables, TContext>({
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
        return await fetchJSON<TData>(url, fetchOptions);
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
export function useDeleteMutation<TData = unknown, TVariables = unknown, TError = Error, TContext = unknown>(
  url: string,
  options: JsonMutationFetchOptions = {},
  mutationOptions: UseMutationOptions<TData, TError, TVariables, TContext> = {},
): AppMutationResult<TData, TError, TVariables, TContext> {
  return useMutation<TData, TError, TVariables, TContext>({
    mutationFn: async (data) => {
      const controller = createRequestController();
      const fetchOptions = {
        ...options,
        method: 'DELETE',
        signal: controller.signal,
        ...((data && {
          headers: {
            'Content-Type': 'application/json',
            ...options.headers
          },
          body: JSON.stringify(data)
        }) as { headers: Record<string, string>; body: string } | undefined)
      };
      
      try {
        return await fetchJSON<TData>(url, fetchOptions);
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
export function invalidateQueries(queryKey: QueryKeyInput): void {
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
export function prefetchQuery<TQueryFnData = unknown, TError = Error, TData = TQueryFnData>(
  queryKey: QueryKeyInput,
  url: string,
  options: EnhancedFetchOptions = {},
  queryOptions: FetchQueryOptions<TQueryFnData, TError, TData> = {},
): void {
  queryClient.prefetchQuery<TQueryFnData, TError, TData>({
    queryKey: Array.isArray(queryKey) ? queryKey : [queryKey],
    queryFn: async () => {
      try {
        return await fetchJSON<TQueryFnData>(url, options);
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
 * @param {T} data - Data to set, or an updater function
 */
export function setQueryData<T>(queryKey: QueryKeyInput, data: Updater<T | undefined, T | undefined>): void {
  queryClient.setQueryData<T>(
    Array.isArray(queryKey) ? queryKey : [queryKey],
    data
  );
}

/**
 * Get query data directly
 * @param {string|Array} queryKey - Query key
 * @returns {T | undefined} - Query data
 */
export function getQueryData<T = unknown>(queryKey: QueryKeyInput): T | undefined {
  return queryClient.getQueryData<T>(
    Array.isArray(queryKey) ? queryKey : [queryKey]
  );
}
