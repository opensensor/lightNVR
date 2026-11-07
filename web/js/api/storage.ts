/** Closed, generated contract boundary for storage configuration and lifecycle. */
import {enhancedFetch,type EnhancedFetchOptions} from '../fetch-utils.ts';
import {ContractValidationError} from './contractErrors.ts';
import type {ContractValidator} from '../../generated/api/validators.js';
import {validateFleetMutationAck,validateStorageArchiveResponse,validateStorageArchiveRetryRequest,validateStorageArchiveRetryResponse,validateStorageCleanupRequest,validateStorageCleanupResponse,validateStorageComplianceResponse,validateStorageHealthResponse,validateStorageMigration,validateStorageMigrationCreate,validateStorageMigrationsResponse,validateStoragePoliciesResponse,validateStoragePolicy,validateStoragePolicyApplyRequest,validateStoragePolicyApplyResponse,validateStoragePolicyCreate,validateStoragePolicyPreview,validateStoragePolicyPreviewRequest,validateStoragePolicyUpdate,validateStoragePool,validateStoragePoolCreate,validateStoragePoolUpdate,validateStoragePoolsResponse,validateStorageTarget,validateStorageTargetCreate,validateStorageTargetUpdate,validateStorageTargetsResponse} from '../../generated/api/validators.js';
export type StorageOperation = 'listStorageTargets' | 'createStorageTarget' | 'getStorageTarget' | 'updateStorageTarget' | 'deleteStorageTarget' | 'listStoragePools' | 'createStoragePool' | 'getStoragePool' | 'updateStoragePool' | 'deleteStoragePool' | 'listStoragePolicies' | 'createStoragePolicy' | 'getStoragePolicy' | 'updateStoragePolicy' | 'deleteStoragePolicy' | 'probeStorageTarget' | 'previewStoragePolicy' | 'applyStoragePolicy' | 'listStorageMigrations' | 'createStorageMigration' | 'getStorageMigration' | 'cancelStorageMigration' | 'retryStorageMigration' | 'getStorageArchive' | 'retryStorageArchive' | 'getStorageCompliance' | 'getStorageHealth' | 'triggerStorageCleanup' | 'storageRequest';
interface Operation {method:string;path:RegExp;id:StorageOperation;response:string;guard:ContractValidator<unknown>;request?:string;requestGuard?:ContractValidator<unknown>;status:number}
const operations:Operation[]=[
{method:'GET',path:/^\/api\/storage-targets$/,id:'listStorageTargets',response:'StorageTargetsResponse',guard:validateStorageTargetsResponse,status:200},
{method:'POST',path:/^\/api\/storage-targets$/,id:'createStorageTarget',response:'StorageTarget',guard:validateStorageTarget,status:201,request:'StorageTargetCreate',requestGuard:validateStorageTargetCreate},
{method:'GET',path:/^\/api\/storage-targets\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'getStorageTarget',response:'StorageTarget',guard:validateStorageTarget,status:200},
{method:'PUT',path:/^\/api\/storage-targets\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'updateStorageTarget',response:'StorageTarget',guard:validateStorageTarget,status:200,request:'StorageTargetUpdate',requestGuard:validateStorageTargetUpdate},
{method:'DELETE',path:/^\/api\/storage-targets\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'deleteStorageTarget',response:'FleetMutationAck',guard:validateFleetMutationAck,status:200},
{method:'GET',path:/^\/api\/storage-pools$/,id:'listStoragePools',response:'StoragePoolsResponse',guard:validateStoragePoolsResponse,status:200},
{method:'POST',path:/^\/api\/storage-pools$/,id:'createStoragePool',response:'StoragePool',guard:validateStoragePool,status:201,request:'StoragePoolCreate',requestGuard:validateStoragePoolCreate},
{method:'GET',path:/^\/api\/storage-pools\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'getStoragePool',response:'StoragePool',guard:validateStoragePool,status:200},
{method:'PUT',path:/^\/api\/storage-pools\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'updateStoragePool',response:'StoragePool',guard:validateStoragePool,status:200,request:'StoragePoolUpdate',requestGuard:validateStoragePoolUpdate},
{method:'DELETE',path:/^\/api\/storage-pools\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'deleteStoragePool',response:'FleetMutationAck',guard:validateFleetMutationAck,status:200},
{method:'GET',path:/^\/api\/storage-policies$/,id:'listStoragePolicies',response:'StoragePoliciesResponse',guard:validateStoragePoliciesResponse,status:200},
{method:'POST',path:/^\/api\/storage-policies$/,id:'createStoragePolicy',response:'StoragePolicy',guard:validateStoragePolicy,status:201,request:'StoragePolicyCreate',requestGuard:validateStoragePolicyCreate},
{method:'GET',path:/^\/api\/storage-policies\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'getStoragePolicy',response:'StoragePolicy',guard:validateStoragePolicy,status:200},
{method:'PUT',path:/^\/api\/storage-policies\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'updateStoragePolicy',response:'StoragePolicy',guard:validateStoragePolicy,status:200,request:'StoragePolicyUpdate',requestGuard:validateStoragePolicyUpdate},
{method:'DELETE',path:/^\/api\/storage-policies\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'deleteStoragePolicy',response:'FleetMutationAck',guard:validateFleetMutationAck,status:200},
{method:'POST',path:/^\/api\/storage-targets\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/probe$/,id:'probeStorageTarget',response:'StorageTarget',guard:validateStorageTarget,status:200},
{method:'POST',path:/^\/api\/storage-policies\/preview$/,id:'previewStoragePolicy',response:'StoragePolicyPreview',guard:validateStoragePolicyPreview,status:200,request:'StoragePolicyPreviewRequest',requestGuard:validateStoragePolicyPreviewRequest},
{method:'POST',path:/^\/api\/storage-policies\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/apply$/,id:'applyStoragePolicy',response:'StoragePolicyApplyResponse',guard:validateStoragePolicyApplyResponse,status:200,request:'StoragePolicyApplyRequest',requestGuard:validateStoragePolicyApplyRequest},
{method:'GET',path:/^\/api\/storage-migrations$/,id:'listStorageMigrations',response:'StorageMigrationsResponse',guard:validateStorageMigrationsResponse,status:200},
{method:'POST',path:/^\/api\/storage-migrations$/,id:'createStorageMigration',response:'StorageMigration',guard:validateStorageMigration,status:202,request:'StorageMigrationCreate',requestGuard:validateStorageMigrationCreate},
{method:'GET',path:/^\/api\/storage-migrations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'getStorageMigration',response:'StorageMigration',guard:validateStorageMigration,status:200},
{method:'POST',path:/^\/api\/storage-migrations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/cancel$/,id:'cancelStorageMigration',response:'StorageMigration',guard:validateStorageMigration,status:200},
{method:'POST',path:/^\/api\/storage-migrations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/retry$/,id:'retryStorageMigration',response:'StorageMigration',guard:validateStorageMigration,status:200},
{method:'GET',path:/^\/api\/storage-archive$/,id:'getStorageArchive',response:'StorageArchiveResponse',guard:validateStorageArchiveResponse,status:200},
{method:'POST',path:/^\/api\/storage-archive\/retry$/,id:'retryStorageArchive',response:'StorageArchiveRetryResponse',guard:validateStorageArchiveRetryResponse,status:202,request:'StorageArchiveRetryRequest',requestGuard:validateStorageArchiveRetryRequest},
{method:'GET',path:/^\/api\/storage-compliance$/,id:'getStorageCompliance',response:'StorageComplianceResponse',guard:validateStorageComplianceResponse,status:200},
{method:'GET',path:/^\/api\/storage\/health$/,id:'getStorageHealth',response:'StorageHealthResponse',guard:validateStorageHealthResponse,status:200},
{method:'POST',path:/^\/api\/storage\/cleanup$/,id:'triggerStorageCleanup',response:'StorageCleanupResponse',guard:validateStorageCleanupResponse,status:200,request:'StorageCleanupRequest',requestGuard:validateStorageCleanupRequest},
 ];
export class StorageRequestError extends Error{
 readonly name='StorageRequestError';
 constructor(readonly operation:StorageOperation,readonly status?:number,readonly mayHaveApplied=false){super(`${operation} failed${status===undefined?'':` (HTTP ${status})`}.`);}
}
/** All T view models are populated only after the operation-specific wire guard. */
export async function storageJSON<T=unknown>(url:string,options:EnhancedFetchOptions={}):Promise<T>{
 const method=(options.method||'GET').toUpperCase();
 const [path,query]=url.split('?');
 const op=operations.find(candidate=>candidate.method===method&&candidate.path.test(path!));
 if(!op)throw new ContractValidationError('storageRequest','request','route');
 if(method==='DELETE'){
  const params=new URLSearchParams(query),revision=params.get('revision');
  if([...params.keys()].some(key=>key!=='revision')||params.getAll('revision').length!==1||!revision||!/^\d+$/.test(revision)||!Number.isSafeInteger(Number(revision))||Number(revision)<1)throw new ContractValidationError(op.id,'request','revision');
 }else if(query)throw new ContractValidationError(op.id,'request','query');
 if(op.requestGuard){
  let body:unknown;
  try{body=options.body===undefined&&op.id==='triggerStorageCleanup'?{}:typeof options.body==='string'?JSON.parse(options.body):undefined;}catch{throw new ContractValidationError(op.id,'request',op.request!);}
  if(!op.requestGuard(body))throw new ContractValidationError(op.id,'request',op.request!);
 }
 const headers=new Headers(options.headers);if(op.requestGuard)headers.set('Content-Type','application/json');
 let response:Response;
 try{response=await enhancedFetch(url,{timeout:20000,...options,headers,method,retries:method==='GET'?(options.retries??1):0,cache:'no-store',sensitive:true,credentials:'same-origin',redirect:'error'});}
 catch(error:unknown){
  if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError'))throw new DOMException('Request cancelled','AbortError');
  const status=typeof error==='object'&&error!==null&&'status'in error&&typeof error.status==='number'?error.status:undefined;
  throw new StorageRequestError(op.id,status,method!=='GET');
 }
 if(response.status!==op.status)throw new ContractValidationError(op.id,'response',op.response,response.status,'status');
 if(!/^application\/json(?:;|$)/i.test(response.headers.get('content-type')||''))throw new ContractValidationError(op.id,'response',op.response,response.status,'json');
 let value:unknown;try{value=await response.json();}catch{throw new ContractValidationError(op.id,'response',op.response,response.status,'json');}
 if(options.signal?.aborted)throw new DOMException('Request cancelled','AbortError');
 if(!op.guard(value))throw new ContractValidationError(op.id,'response',op.response,response.status);
 return value as T;
}
