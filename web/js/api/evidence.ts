/** Closed F11 wire boundary. Query POSTs are read operations; writes never retry. */
import { enhancedFetch, type EnhancedFetchOptions } from '../fetch-utils.ts';
import * as guards from '../../generated/api/validators.js';
import { ContractValidationError } from './contractErrors.ts';
export type EvidenceOperation = 'getEvidenceDetections' | 'listEvidenceModels' | 'searchEvidenceLpr' | 'exportEvidenceLpr' | 'deleteEvidenceLpr' | 'checkEvidenceRecordingFile' | 'deleteEvidenceRecordingFile' | 'countEvidenceProtected' | 'syncEvidenceRecordings' | 'queryEvidenceTimeline' | 'resolveEvidenceSegment' | 'searchEvidence' | 'previewEvidenceRecordings' | 'sampleEvidenceThumbnails' | 'listEvidenceBookmarks' | 'createEvidenceBookmark' | 'getEvidenceBookmark' | 'updateEvidenceBookmark' | 'deleteEvidenceBookmark' | 'getEvidenceSnapshot' | 'getEvidenceThumbnail' | 'getEvidenceManifest' | 'redirectEvidencePlayback';
const writes = new Set<EvidenceOperation>(['deleteEvidenceLpr','deleteEvidenceRecordingFile','syncEvidenceRecordings','createEvidenceBookmark','updateEvidenceBookmark','deleteEvidenceBookmark']);
type Contract = {route:RegExp;method:string;operation:EvidenceOperation;schema:string;guard:(value:unknown)=>boolean;request?:(value:unknown)=>boolean;statuses:number[]};
const contracts: Contract[] = [
  {route:/^\/api\/detection\/results\/[^\/]+$/, method:'GET', operation:'getEvidenceDetections', schema:'EvidenceDetections', guard:guards.validateEvidenceDetections, statuses:[200]},
  {route:/^\/api\/detection\/models$/, method:'GET', operation:'listEvidenceModels', schema:'EvidenceModels', guard:guards.validateEvidenceModels, statuses:[200]},
  {route:/^\/api\/lpr\/search$/, method:'POST', operation:'searchEvidenceLpr', schema:'EvidenceLprReads', guard:guards.validateEvidenceLprReads, request:guards.validateEvidenceLprSearchRequest, statuses:[200]},
  {route:/^\/api\/lpr\/export$/, method:'POST', operation:'exportEvidenceLpr', schema:'EvidenceLprReads', guard:guards.validateEvidenceLprReads, request:guards.validateEvidenceLprExportRequest, statuses:[200]},
  {route:/^\/api\/lpr\/reads\/[^\/]+$/, method:'DELETE', operation:'deleteEvidenceLpr', schema:'EvidenceLprDeleted', guard:guards.validateEvidenceLprDeleted, statuses:[200]},
  {route:/^\/api\/recordings\/files\/check$/, method:'GET', operation:'checkEvidenceRecordingFile', schema:'EvidenceFileCheck', guard:guards.validateEvidenceFileCheck, statuses:[200]},
  {route:/^\/api\/recordings\/files$/, method:'DELETE', operation:'deleteEvidenceRecordingFile', schema:'EvidenceFileDeleted', guard:guards.validateEvidenceFileDeleted, statuses:[200,202]},
  {route:/^\/api\/recordings\/protected$/, method:'GET', operation:'countEvidenceProtected', schema:'EvidenceProtectedCount', guard:guards.validateEvidenceProtectedCount, statuses:[200]},
  {route:/^\/api\/recordings\/sync$/, method:'POST', operation:'syncEvidenceRecordings', schema:'EvidenceSyncComplete', guard:guards.validateEvidenceSyncComplete, statuses:[200]},
  {route:/^\/api\/investigations\/timeline$/, method:'POST', operation:'queryEvidenceTimeline', schema:'EvidenceTimeline', guard:guards.validateEvidenceTimeline, request:guards.validateEvidenceRangeRequest, statuses:[200]},
  {route:/^\/api\/investigations\/segment-at$/, method:'POST', operation:'resolveEvidenceSegment', schema:'EvidenceSegmentAt', guard:guards.validateEvidenceSegmentAt, request:guards.validateEvidenceSegmentAtRequest, statuses:[200]},
  {route:/^\/api\/investigations\/search$/, method:'POST', operation:'searchEvidence', schema:'EvidenceSearch', guard:guards.validateEvidenceSearch, request:guards.validateEvidenceSearchRequest, statuses:[200]},
  {route:/^\/api\/investigations\/recordings\/preview$/, method:'POST', operation:'previewEvidenceRecordings', schema:'EvidencePreview', guard:guards.validateEvidencePreview, request:guards.validateEvidenceRangeRequest, statuses:[200]},
  {route:/^\/api\/investigations\/thumbnail-samples$/, method:'POST', operation:'sampleEvidenceThumbnails', schema:'EvidenceThumbnails', guard:guards.validateEvidenceThumbnails, request:guards.validateEvidenceThumbnailRequest, statuses:[200]},
  {route:/^\/api\/investigation-bookmarks$/, method:'GET', operation:'listEvidenceBookmarks', schema:'EvidenceBookmarks', guard:guards.validateEvidenceBookmarks, statuses:[200]},
  {route:/^\/api\/investigation-bookmarks$/, method:'POST', operation:'createEvidenceBookmark', schema:'EvidenceBookmark', guard:guards.validateEvidenceBookmark, request:guards.validateEvidenceBookmarkCreate, statuses:[201]},
  {route:/^\/api\/investigation-bookmarks\/[^\/]+$/, method:'GET', operation:'getEvidenceBookmark', schema:'EvidenceBookmark', guard:guards.validateEvidenceBookmark, statuses:[200]},
  {route:/^\/api\/investigation-bookmarks\/[^\/]+$/, method:'PUT', operation:'updateEvidenceBookmark', schema:'EvidenceBookmark', guard:guards.validateEvidenceBookmark, request:guards.validateEvidenceBookmarkUpdate, statuses:[200]},
  {route:/^\/api\/investigation-bookmarks\/[^\/]+$/, method:'DELETE', operation:'deleteEvidenceBookmark', schema:'EvidenceBookmarkDeleted', guard:guards.validateEvidenceBookmarkDeleted, request:guards.validateEvidenceBookmarkDelete, statuses:[200]},
];
export class EvidenceRequestError extends Error {
  readonly name = 'EvidenceRequestError';
  constructor(readonly operation:EvidenceOperation, readonly status:number|undefined, readonly mayHaveApplied:boolean) {
    super(`${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}.`);
  }
}
/** A generic view alias cannot bypass the selected route's canonical runtime guard. */
export async function evidenceJSON<T=unknown>(url:string, options:EnhancedFetchOptions={}, fetchImpl?:typeof fetch):Promise<T> {
  const method=(options.method??'GET').toUpperCase();
  const contract=contracts.find(c=>c.method===method && c.route.test(url.split('?')[0]!));
  if(!contract) throw new Error('Unsupported evidence operation.');
  if(contract.request) {
    let body:unknown;
    try {body=typeof options.body==='string'?JSON.parse(options.body):undefined;}
    catch {throw new ContractValidationError(contract.operation,'request',contract.schema,undefined,'json');}
    if(!contract.request(body))throw new ContractValidationError(contract.operation,'request',contract.schema);
  }
  let response:Response;
  const requestOptions={timeout:15000,...options,method,credentials:'same-origin' as const,cache:'no-store' as const,redirect:'error' as const,sensitive:true,retries:writes.has(contract.operation)?0:options.retries??0};
  try {
    response=fetchImpl?await fetchImpl(url,requestOptions):await enhancedFetch(url,requestOptions);
    if(!response.ok)throw {status:response.status};
  } catch(error:unknown) {
    if(options.signal?.aborted || (error instanceof Error && error.name==='AbortError'))throw new DOMException('Request was cancelled','AbortError');
    const status=typeof error==='object' && error!==null && 'status'in error && typeof error.status==='number'?error.status:undefined;
    throw new EvidenceRequestError(contract.operation,status,writes.has(contract.operation)&&(status===undefined||status>=500));
  }
  if(!contract.statuses.includes(response.status))throw new ContractValidationError(contract.operation,'response',contract.schema,response.status,'status');
  if(!/^application\/json(?:\s*;|$)/i.test(response.headers.get('content-type')??''))throw new ContractValidationError(contract.operation,'response',contract.schema,response.status,'json');
  if(contract.operation==='exportEvidenceLpr' && response.headers.get('content-disposition')!=='attachment; filename="lightnvr-lpr-export.json"')throw new ContractValidationError(contract.operation,'response',contract.schema,response.status,'status');
  let value:unknown;
  try {value=await response.json();}catch {throw new ContractValidationError(contract.operation,'response',contract.schema,response.status,'json');}
  if(!contract.guard(value))throw new ContractValidationError(contract.operation,'response',contract.schema,response.status);
  return value as T;
}
