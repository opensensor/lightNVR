/** Stream lifecycle JSON boundary. Streaming bytes/signaling retain their own protocol clients. */
import {enhancedFetch} from '../fetch-utils.ts';
import type {EnhancedFetchOptions} from '../fetch-utils.ts';
import * as validators from '../../generated/api/validators.js';
import type {ContractValidator} from '../../generated/api/validators.js';
export type MediaOptions=Omit<EnhancedFetchOptions,'method'|'body'|'sensitive'>;
export class MediaContractError extends Error {
  readonly name='MediaContractError';
  constructor(readonly operation:string,readonly phase:'request'|'response'|'transport',readonly status?:number,readonly mayHaveApplied=false){
    super(`${operation} ${phase} failed${status===undefined?'':` (HTTP ${status})`}.`);
  }
}
function serialize(body:unknown,guard:ContractValidator<unknown>,operation:string):string {
  if(!guard(body))throw new MediaContractError(operation,'request');
  try{return JSON.stringify(body);}catch{throw new MediaContractError(operation,'request');}
}
function camera(name:string):string {
  if(!name||name.includes('/'))throw new MediaContractError('streamPath','request');
  return `/api/streams/${encodeURIComponent(name)}`;
}
async function call<T>(url:string,operation:string,guard:ContractValidator<T>,options:MediaOptions={},method='GET',body?:string,status=200):Promise<T>{
  const mutation=method!=='GET'&&operation!=='probeStream';
  let response:Response;
  try{
    const headers=new Headers(options.headers);if(body!==undefined)headers.set('Content-Type','application/json');
    response=await enhancedFetch(url,{timeout:20000,...options,retries:0,method,body,headers,cache:'no-store',credentials:'same-origin',redirect:'error',sensitive:true});
  }catch(error:unknown){
    if(options.signal?.aborted)throw new DOMException('Request cancelled','AbortError');
    const status=typeof error==='object'&&error!==null&&'status' in error&&typeof error.status==='number'?error.status:undefined;
    throw new MediaContractError(operation,'transport',status,mutation);
  }
  if(response.status!==status)throw new MediaContractError(operation,'response',response.status,mutation);
  let value:unknown;
  try{value=await response.json();}catch{throw new MediaContractError(operation,'response',response.status,mutation);}
  if(options.signal?.aborted)throw new DOMException('Request cancelled','AbortError');
  if(!guard(value))throw new MediaContractError(operation,'response',response.status,mutation);
  return value;
}
export const createStream=async(body:unknown,options:MediaOptions={})=>call('/api/streams','createStream',validators.validateStreamCreated,options,'POST',serialize(body,validators.validateStreamCreateRequest,'createStream'),201);
export const probeStream=async(body:unknown,options:MediaOptions={})=>call('/api/streams/test','probeStream',validators.validateStreamProbeResponse,options,'POST',serialize(body,validators.validateStreamProbeRequest,'probeStream'));
export const getStream=async(name:string,options:MediaOptions={})=>call(camera(name),'getStream',validators.validateStreamSingleResponse,options);
export const deleteStream=async(name:string,permanent=false,options:MediaOptions={})=>call(camera(name)+(permanent?'?permanent=true':''),'deleteStream',validators.validateStreamDeleteAccepted,options,'DELETE',undefined,202);
export const refreshStream=async(name:string,options:MediaOptions={})=>call(camera(name)+'/refresh','refreshStream',validators.validateStreamRefreshAccepted,options,'POST',undefined,202);
export const getStreamRecording=async(name:string,options:MediaOptions={})=>call(camera(name)+'/recording','getStreamRecording',validators.validateStreamRecordingState,options);
export const controlStreamRecording=async(name:string,action:'start'|'stop',options:MediaOptions={})=>call(camera(name)+'/recording','controlStreamRecording',validators.validateStreamRecordingChanged,options,'POST',serialize({action},validators.validateStreamRecordingRequest,'controlStreamRecording'),action==='start'?202:200);
export const getStreamZones=async(name:string,options:MediaOptions={})=>call(camera(name)+'/zones','getStreamZones',validators.validateStreamZones,options);
export const saveStreamZones=async(name:string,body:unknown,options:MediaOptions={})=>call(camera(name)+'/zones','saveStreamZones',validators.validateStreamZonesSaved,options,'POST',serialize(body,validators.validateStreamZonesRequest,'saveStreamZones'));
export const deleteStreamZones=async(name:string,options:MediaOptions={})=>call(camera(name)+'/zones','deleteStreamZones',validators.validateStreamZonesDeleted,options,'DELETE');
export const getStreamRetention=async(name:string,options:MediaOptions={})=>call(camera(name)+'/retention','getStreamRetention',validators.validateStreamRetention,options);
export const setStreamRetention=async(name:string,body:unknown,options:MediaOptions={})=>call(camera(name)+'/retention','setStreamRetention',validators.validateStreamRetention,options,'PUT',serialize(body,validators.validateStreamRetentionRequest,'setStreamRetention'));
export const getStreamDetectionEngines=async(name:string,options:MediaOptions={})=>call(camera(name)+'/detection-engines','getStreamDetectionEngines',validators.validateStreamDetectionEngines,options);
export const setStreamDetectionEngines=async(name:string,body:unknown,options:MediaOptions={})=>call(camera(name)+'/detection-engines','setStreamDetectionEngines',validators.validateStreamDetectionEngines,options,'PUT',serialize(body,validators.validateStreamDetectionEnginesRequest,'setStreamDetectionEngines'));
export const getIceServers=async(options:MediaOptions={})=>call('/api/ice-servers','getIceServers',validators.validateIceServersResponse,options);
