/** Contract boundary for the reviewed fleet mutations and live workspace reads.
 * Runtime route selection is closed; unsupported URLs never fall back to unchecked JSON.
 * Mutations have zero transport retries so an ambiguous response cannot duplicate writes.
 */
import {enhancedFetch, type EnhancedFetchOptions} from '../fetch-utils.ts';
import {ContractValidationError} from './contractErrors.ts';
import type {ContractValidator} from '../../generated/api/validators.js';
import {validateCameraCollection,validateCameraCollectionCreate,validateCameraCollectionMembersRequest,validateCameraCollectionMembersResponse,validateCameraCollectionUpdate,validateCameraFavorite,validateCameraFavoriteRemoval,validateCameraFavoritesResponse,validateCameraLocationAssignment,validateCameraLocationAssignmentRequest,validateCameraTagAssignmentsRequest,validateCameraTagAssignmentsResponse,validateFleetLocation,validateFleetLocationCreate,validateFleetLocationUpdate,validateFleetMutationAck,validateFleetRevisionRequest,validateFleetSavedView,validateFleetTag,validateFleetTagCreate,validateFleetTagMergeRequest,validateFleetTagMergeResult,validateFleetTagUpdate,validateFleetViewCreate,validateFleetViewUpdate,validateFloorPlan,validateFloorPlanCreate,validateFloorPlanDeletion,validateFloorPlanUpdate,validateFloorPlansResponse,validateLiveLayout,validateLiveLayoutCreate,validateLiveLayoutUpdate,validateLiveLayoutsResponse} from '../../generated/api/validators.js';
export type FleetWorkspaceOperation = 'createFleetLocation' | 'updateFleetLocation' | 'deleteFleetLocation' | 'createFleetTag' | 'updateFleetTag' | 'deleteFleetTag' | 'createCameraCollection' | 'updateCameraCollection' | 'deleteCameraCollection' | 'setCameraLocation' | 'setCameraTags' | 'mergeFleetTag' | 'setCameraCollectionMembers' | 'createFleetView' | 'updateFleetView' | 'deleteFleetView' | 'listLiveLayouts' | 'createLiveLayout' | 'updateLiveLayout' | 'deleteLiveLayout' | 'listCameraFavorites' | 'addCameraFavorite' | 'deleteCameraFavorite' | 'listFloorPlans' | 'getFloorPlan' | 'createFloorPlan' | 'updateFloorPlan' | 'deleteFloorPlan' | 'getFloorPlanBackground' | 'setFloorPlanBackground' | 'deleteFloorPlanBackground' | 'fleetWorkspace';
interface Operation {method:string; path:RegExp; id:FleetWorkspaceOperation; response:string; guard:ContractValidator<unknown>; request?:string; requestGuard?:ContractValidator<unknown>; status:number; binary?:boolean}
const operations:Operation[] = [
{method:"POST",path:/^\/api\/locations$/,id:"createFleetLocation",response:"FleetLocation",guard:validateFleetLocation,status:201,request:"FleetLocationCreate",requestGuard:validateFleetLocationCreate},
{method:"PUT",path:/^\/api\/locations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"updateFleetLocation",response:"FleetLocation",guard:validateFleetLocation,status:200,request:"FleetLocationUpdate",requestGuard:validateFleetLocationUpdate},
{method:"DELETE",path:/^\/api\/locations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteFleetLocation",response:"FleetMutationAck",guard:validateFleetMutationAck,status:200},
{method:"POST",path:/^\/api\/camera-tags$/,id:"createFleetTag",response:"FleetTag",guard:validateFleetTag,status:201,request:"FleetTagCreate",requestGuard:validateFleetTagCreate},
{method:"PUT",path:/^\/api\/camera-tags\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"updateFleetTag",response:"FleetTag",guard:validateFleetTag,status:200,request:"FleetTagUpdate",requestGuard:validateFleetTagUpdate},
{method:"DELETE",path:/^\/api\/camera-tags\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteFleetTag",response:"FleetMutationAck",guard:validateFleetMutationAck,status:200},
{method:"POST",path:/^\/api\/camera-collections$/,id:"createCameraCollection",response:"CameraCollection",guard:validateCameraCollection,status:201,request:"CameraCollectionCreate",requestGuard:validateCameraCollectionCreate},
{method:"PUT",path:/^\/api\/camera-collections\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"updateCameraCollection",response:"CameraCollection",guard:validateCameraCollection,status:200,request:"CameraCollectionUpdate",requestGuard:validateCameraCollectionUpdate},
{method:"DELETE",path:/^\/api\/camera-collections\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteCameraCollection",response:"FleetMutationAck",guard:validateFleetMutationAck,status:200},
{method:"PUT",path:/^\/api\/cameras\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/location$/,id:"setCameraLocation",response:"CameraLocationAssignment",guard:validateCameraLocationAssignment,status:200,request:"CameraLocationAssignmentRequest",requestGuard:validateCameraLocationAssignmentRequest},
{method:"PUT",path:/^\/api\/cameras\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/tags$/,id:"setCameraTags",response:"CameraTagAssignmentsResponse",guard:validateCameraTagAssignmentsResponse,status:200,request:"CameraTagAssignmentsRequest",requestGuard:validateCameraTagAssignmentsRequest},
{method:"POST",path:/^\/api\/camera-tags\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/merge$/,id:"mergeFleetTag",response:"FleetTagMergeResult",guard:validateFleetTagMergeResult,status:200,request:"FleetTagMergeRequest",requestGuard:validateFleetTagMergeRequest},
{method:"PUT",path:/^\/api\/camera-collections\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/members$/,id:"setCameraCollectionMembers",response:"CameraCollectionMembersResponse",guard:validateCameraCollectionMembersResponse,status:200,request:"CameraCollectionMembersRequest",requestGuard:validateCameraCollectionMembersRequest},
{method:"POST",path:/^\/api\/fleet\/views$/,id:"createFleetView",response:"FleetSavedView",guard:validateFleetSavedView,status:201,request:"FleetViewCreate",requestGuard:validateFleetViewCreate},
{method:"PUT",path:/^\/api\/fleet\/views\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"updateFleetView",response:"FleetSavedView",guard:validateFleetSavedView,status:200,request:"FleetViewUpdate",requestGuard:validateFleetViewUpdate},
{method:"DELETE",path:/^\/api\/fleet\/views\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteFleetView",response:"FleetMutationAck",guard:validateFleetMutationAck,status:200,request:"FleetRevisionRequest",requestGuard:validateFleetRevisionRequest},
{method:"GET",path:/^\/api\/live\/layouts$/,id:"listLiveLayouts",response:"LiveLayoutsResponse",guard:validateLiveLayoutsResponse,status:200},
{method:"POST",path:/^\/api\/live\/layouts$/,id:"createLiveLayout",response:"LiveLayout",guard:validateLiveLayout,status:201,request:"LiveLayoutCreate",requestGuard:validateLiveLayoutCreate},
{method:"PUT",path:/^\/api\/live\/layouts\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"updateLiveLayout",response:"LiveLayout",guard:validateLiveLayout,status:200,request:"LiveLayoutUpdate",requestGuard:validateLiveLayoutUpdate},
{method:"DELETE",path:/^\/api\/live\/layouts\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteLiveLayout",response:"FleetMutationAck",guard:validateFleetMutationAck,status:200,request:"FleetRevisionRequest",requestGuard:validateFleetRevisionRequest},
{method:"GET",path:/^\/api\/camera-favorites$/,id:"listCameraFavorites",response:"CameraFavoritesResponse",guard:validateCameraFavoritesResponse,status:200},
{method:"PUT",path:/^\/api\/camera-favorites\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"addCameraFavorite",response:"CameraFavorite",guard:validateCameraFavorite,status:200},
{method:"DELETE",path:/^\/api\/camera-favorites\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteCameraFavorite",response:"CameraFavoriteRemoval",guard:validateCameraFavoriteRemoval,status:200},
{method:"GET",path:/^\/api\/live\/plans$/,id:"listFloorPlans",response:"FloorPlansResponse",guard:validateFloorPlansResponse,status:200},
{method:"GET",path:/^\/api\/live\/plans\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"getFloorPlan",response:"FloorPlan",guard:validateFloorPlan,status:200},
{method:"POST",path:/^\/api\/live\/plans$/,id:"createFloorPlan",response:"FloorPlan",guard:validateFloorPlan,status:201,request:"FloorPlanCreate",requestGuard:validateFloorPlanCreate},
{method:"PUT",path:/^\/api\/live\/plans\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"updateFloorPlan",response:"FloorPlan",guard:validateFloorPlan,status:200,request:"FloorPlanUpdate",requestGuard:validateFloorPlanUpdate},
{method:"DELETE",path:/^\/api\/live\/plans\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:"deleteFloorPlan",response:"FloorPlanDeletion",guard:validateFloorPlanDeletion,status:200,request:"FleetRevisionRequest",requestGuard:validateFleetRevisionRequest},
{method:"PUT",path:/^\/api\/live\/plans\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/background$/,id:"setFloorPlanBackground",response:"FloorPlan",guard:validateFloorPlan,status:200,binary:true},
{method:"DELETE",path:/^\/api\/live\/plans\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/background$/,id:"deleteFloorPlanBackground",response:"FloorPlan",guard:validateFloorPlan,status:200},
 ];
export class FleetWriteError extends Error {
  readonly name='FleetWriteError';
  constructor(readonly operation:string,readonly status?:number){super(`${operation} failed${status===undefined?'':` (HTTP ${status})`}.`);}
}
/** T is the legacy view model; wire data always passes the operation's generated guard first. */
export async function fleetWriteJSON<T=unknown>(url:string,options:EnhancedFetchOptions={}):Promise<T>{
  const method=(options.method||'GET').toUpperCase();
  const op=operations.find(candidate=>candidate.method===method&&candidate.path.test(url));
  if(!op)throw new ContractValidationError('fleetWorkspace','request','route');
  if(op.requestGuard){
    let value:unknown;
    try{value=typeof options.body==='string'?JSON.parse(options.body):undefined;}catch{throw new ContractValidationError(op.id,'request',op.request!);}
    if(!op.requestGuard(value))throw new ContractValidationError(op.id,'request',op.request!);
  }
  const headers=new Headers(options.headers);
  if(op.requestGuard)headers.set('Content-Type','application/json');
  if(op.binary){
    const body=options.body;
    if(!(body instanceof Blob)||body.size<4||body.size>786432||!['image/png','image/jpeg'].includes(headers.get('Content-Type')||''))
      throw new ContractValidationError(op.id,'request','FloorPlanBackground');
  }
  let response:Response;
  try{response=await enhancedFetch(url,{timeout:20000,...options,headers,method,retries:0,cache:'no-store',sensitive:true,credentials:'same-origin',redirect:'error'});}
  catch(error:unknown){
    if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError'))throw new DOMException('Request cancelled','AbortError');
    const status = typeof error === 'object' && error !== null && 'status' in error && typeof error.status === 'number'
      ? error.status : undefined;
    throw new FleetWriteError(op.id,status);
  }
  if(response.status!==op.status)throw new ContractValidationError(op.id,'response',op.response,response.status,'status');
  if(!/^application\/json(?:;|$)/i.test(response.headers.get('content-type')||''))throw new ContractValidationError(op.id,'response',op.response,response.status,'json');
  let value:unknown;
  try{value=await response.json();}catch{throw new ContractValidationError(op.id,'response',op.response,response.status,'json');}
  if(options.signal?.aborted)throw new DOMException('Request cancelled','AbortError');
  if(!op.guard(value))throw new ContractValidationError(op.id,'response',op.response,response.status);
  return value as T;
}
