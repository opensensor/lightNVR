/** Closed, validated event administration boundary. No administrative write retries. */
import {enhancedFetch,type EnhancedFetchOptions} from '../fetch-utils.ts';
import {ContractValidationError} from './contractErrors.ts';
import type {ContractValidator} from '../../generated/api/validators.js';
import {validateEventCatalog,validateEventDestination,validateEventDestinationCreate,validateEventDestinationUpdate,validateEventDestinations,validateEventMutationAck,validateEventRoute,validateEventRouteCreate,validateEventRoutePreview,validateEventRoutePreviewRequest,validateEventRouteUpdate,validateEventRoutes,validateMotionTrigger,validateMotionTriggered} from '../../generated/api/validators.js';
export type EventOperation = 'getEventCatalog' | 'listEventDestinations' | 'createEventDestination' | 'getEventDestination' | 'updateEventDestination' | 'deleteEventDestination' | 'listEventRoutes' | 'createEventRoute' | 'getEventRoute' | 'updateEventRoute' | 'deleteEventRoute' | 'previewEventRoute' | 'triggerMotion' | 'eventRequest';
interface Operation {method:string;path:RegExp;id:EventOperation;schema:string;guard:ContractValidator<unknown>;request?:string;requestGuard?:ContractValidator<unknown>;status:number}
const operations:Operation[] = [
{method:'GET',path:/^\/api\/events\/catalog$/,id:'getEventCatalog',schema:'EventCatalog',guard:validateEventCatalog,status:200},
{method:'GET',path:/^\/api\/event-destinations$/,id:'listEventDestinations',schema:'EventDestinations',guard:validateEventDestinations,status:200},
{method:'POST',path:/^\/api\/event-destinations$/,id:'createEventDestination',schema:'EventDestination',guard:validateEventDestination,status:201,request:'EventDestinationCreate',requestGuard:validateEventDestinationCreate},
{method:'GET',path:/^\/api\/event-destinations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'getEventDestination',schema:'EventDestination',guard:validateEventDestination,status:200},
{method:'PUT',path:/^\/api\/event-destinations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'updateEventDestination',schema:'EventDestination',guard:validateEventDestination,status:200,request:'EventDestinationUpdate',requestGuard:validateEventDestinationUpdate},
{method:'DELETE',path:/^\/api\/event-destinations\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'deleteEventDestination',schema:'EventMutationAck',guard:validateEventMutationAck,status:200},
{method:'GET',path:/^\/api\/event-routes$/,id:'listEventRoutes',schema:'EventRoutes',guard:validateEventRoutes,status:200},
{method:'POST',path:/^\/api\/event-routes$/,id:'createEventRoute',schema:'EventRoute',guard:validateEventRoute,status:201,request:'EventRouteCreate',requestGuard:validateEventRouteCreate},
{method:'GET',path:/^\/api\/event-routes\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'getEventRoute',schema:'EventRoute',guard:validateEventRoute,status:200},
{method:'PUT',path:/^\/api\/event-routes\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'updateEventRoute',schema:'EventRoute',guard:validateEventRoute,status:200,request:'EventRouteUpdate',requestGuard:validateEventRouteUpdate},
{method:'DELETE',path:/^\/api\/event-routes\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/,id:'deleteEventRoute',schema:'EventMutationAck',guard:validateEventMutationAck,status:200},
{method:'POST',path:/^\/api\/event-routes\/preview$/,id:'previewEventRoute',schema:'EventRoutePreview',guard:validateEventRoutePreview,status:200,request:'EventRoutePreviewRequest',requestGuard:validateEventRoutePreviewRequest},
{method:'POST',path:/^\/api\/motion\/trigger$/,id:'triggerMotion',schema:'MotionTriggered',guard:validateMotionTriggered,status:202,request:'MotionTrigger',requestGuard:validateMotionTrigger},
 ];
export class EventRequestError extends Error {
 readonly name='EventRequestError';
 constructor(readonly operation:EventOperation,readonly status?:number,readonly mayHaveApplied=false){
  super(`${operation} failed${status===undefined?'':` (HTTP ${status})`}.${mayHaveApplied?' Refresh state before retrying.':''}`);
 }
}
export async function eventJSON<T=unknown>(url:string,options:EnhancedFetchOptions={}):Promise<T>{
 const method=(options.method||'GET').toUpperCase(),[pathname='',query]=url.split('?');
 const op=operations.find(operation=>operation.method===method&&operation.path.test(pathname));
 if(!op)throw new ContractValidationError('eventRequest','request','EventRoute');
 if(method==='DELETE'){
  const params=new URLSearchParams(query),value=params.get('revision');
  if(!value||!/^[1-9][0-9]*$/.test(value)||!Number.isSafeInteger(Number(value))||[...params.keys()].filter(key=>key==='revision').length!==1)
   throw new ContractValidationError(op.id,'request','EventRevision');
 }
 if(op.requestGuard){
  let value:unknown;
  try{value=JSON.parse(typeof options.body==='string'?options.body:'');}catch{throw new ContractValidationError(op.id,'request',op.request!);}
  if(!op.requestGuard(value))throw new ContractValidationError(op.id,'request',op.request!);
 }
 const headers=new Headers(options.headers);if(op.requestGuard)headers.set('Content-Type','application/json');
 let response:Response;
 try{response=await enhancedFetch(url,{timeout:20000,...options,headers,method,retries:method==='GET'?(options.retries??1):0,cache:'no-store',credentials:'same-origin',redirect:'error',sensitive:true});}
 catch(error:unknown){
  if (options.signal?.aborted || (error instanceof Error && error.name === 'AbortError'))throw new DOMException('Request cancelled','AbortError');
  const status=typeof error==='object'&&error!==null&&'status'in error&&typeof error.status==='number'?error.status:undefined;
  throw new EventRequestError(op.id,status,method!=='GET'&&op.id!=='previewEventRoute'&&(status===undefined||status>=500));
 }
 if(response.status!==op.status)throw new ContractValidationError(op.id,'response',op.schema,response.status,'status');
 if(!/^application\/json(?:;|$)/i.test(response.headers.get('content-type')||''))throw new ContractValidationError(op.id,'response',op.schema,response.status,'json');
 let value:unknown;
 try{value=await response.json();}catch{throw new ContractValidationError(op.id,'response',op.schema,response.status,'json');}
 if(options.signal?.aborted)throw new DOMException('Request cancelled','AbortError');
 if(!op.guard(value))throw new ContractValidationError(op.id,'response',op.schema,response.status);
 return value as T;
}
