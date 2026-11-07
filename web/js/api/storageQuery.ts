import {useQuery,type AppQueryOptions,type QueryKeyInput} from '../query-client.ts';
import type {EnhancedFetchOptions} from '../fetch-utils.ts';
import {storageJSON} from './storage.ts';
/** Preserve query options while making the cache boundary validate actual native data. */
export function useStorageQuery<T>(key:QueryKeyInput,url:string,request:EnhancedFetchOptions={},query:AppQueryOptions<T,Error>={}){
 return useQuery<T>({...query,queryKey:Array.isArray(key)?key:[key],queryFn:({signal})=>storageJSON<T>(url,{...request,signal})});
}
