/** Runtime-validated device commands. Failed writes are never retried automatically. */
import {enhancedFetch, type EnhancedFetchOptions} from '../fetch-utils.ts';
import type {components} from '../../generated/api/schema';
import type {ContractValidator} from '../../generated/api/validators.js';
import {ContractValidationError} from './contractErrors.ts';
import {validateDaynightRequest,validateDaynightResponse,validateDaynightUpdated,validateDeviceCommandResult,validateDeviceEmptyRequest,validateDeviceSuccess,validateImagingOptionsResponse,validateImagingSettingsRequest,validateImagingSettingsResponse,validateOnvifAddRequest,validateOnvifAddResult,validateOnvifClaimRequest,validateOnvifDevicesResponse,validateOnvifDiscoveryRequest,validateOnvifDiscoveryStatus,validateOnvifProfilesResponse,validateOnvifTestRequest,validatePtzAxesRequest,validatePtzCapabilities,validatePtzGotoPresetRequest,validatePtzPresetRequest,validatePtzPresetResult,validatePtzPresets} from '../../generated/api/validators.js';
export interface DeviceResults {
  getPtzCapabilities: components['schemas']['PtzCapabilities'];
  getPtzPresets: components['schemas']['PtzPresets'];
  movePtz: components['schemas']['DeviceCommandResult'];
  absolutePtz: components['schemas']['DeviceCommandResult'];
  relativePtz: components['schemas']['DeviceCommandResult'];
  stopPtz: components['schemas']['DeviceCommandResult'];
  homePtz: components['schemas']['DeviceCommandResult'];
  setPtzHome: components['schemas']['DeviceCommandResult'];
  gotoPtzPreset: components['schemas']['DeviceCommandResult'];
  setPtzPreset: components['schemas']['PtzPresetResult'];
  getImagingSettings: components['schemas']['ImagingSettingsResponse'];
  setImagingSettings: components['schemas']['DeviceCommandResult'];
  getImagingOptions: components['schemas']['ImagingOptionsResponse'];
  getDaynight: components['schemas']['DaynightResponse'];
  setDaynight: components['schemas']['DaynightUpdated'];
  getOnvifDiscoveryStatus: components['schemas']['OnvifDiscoveryStatus'];
  getOnvifDevices: components['schemas']['OnvifDevicesResponse'];
  discoverOnvifDevices: components['schemas']['OnvifDevicesResponse'];
  getOnvifProfiles: components['schemas']['OnvifProfilesResponse'];
  addOnvifDevice: components['schemas']['OnvifAddResult'];
  claimOnvifDevice: components['schemas']['DeviceSuccess'];
  testOnvifDevice: components['schemas']['DeviceCommandResult'];
}
export type DeviceOperation = keyof DeviceResults;
interface Route {path:string;method:string;response:string;guard:ContractValidator<unknown>;request?:string;requestGuard?:ContractValidator<unknown>}
const routes: Record<DeviceOperation,Route> = {
  getPtzCapabilities: {path:"/api/streams/{name}/ptz/capabilities",method:'GET',response:'PtzCapabilities',guard:validatePtzCapabilities},
  getPtzPresets: {path:"/api/streams/{name}/ptz/presets",method:'GET',response:'PtzPresets',guard:validatePtzPresets},
  movePtz: {path:"/api/streams/{name}/ptz/move",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'PtzAxesRequest',requestGuard:validatePtzAxesRequest},
  absolutePtz: {path:"/api/streams/{name}/ptz/absolute",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'PtzAxesRequest',requestGuard:validatePtzAxesRequest},
  relativePtz: {path:"/api/streams/{name}/ptz/relative",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'PtzAxesRequest',requestGuard:validatePtzAxesRequest},
  stopPtz: {path:"/api/streams/{name}/ptz/stop",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'DeviceEmptyRequest',requestGuard:validateDeviceEmptyRequest},
  homePtz: {path:"/api/streams/{name}/ptz/home",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'DeviceEmptyRequest',requestGuard:validateDeviceEmptyRequest},
  setPtzHome: {path:"/api/streams/{name}/ptz/set-home",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'DeviceEmptyRequest',requestGuard:validateDeviceEmptyRequest},
  gotoPtzPreset: {path:"/api/streams/{name}/ptz/goto-preset",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'PtzGotoPresetRequest',requestGuard:validatePtzGotoPresetRequest},
  setPtzPreset: {path:"/api/streams/{name}/ptz/preset",method:'PUT',response:'PtzPresetResult',guard:validatePtzPresetResult,request:'PtzPresetRequest',requestGuard:validatePtzPresetRequest},
  getImagingSettings: {path:"/api/streams/{name}/imaging/settings",method:'GET',response:'ImagingSettingsResponse',guard:validateImagingSettingsResponse},
  setImagingSettings: {path:"/api/streams/{name}/imaging/settings",method:'PUT',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'ImagingSettingsRequest',requestGuard:validateImagingSettingsRequest},
  getImagingOptions: {path:"/api/streams/{name}/imaging/options",method:'GET',response:'ImagingOptionsResponse',guard:validateImagingOptionsResponse},
  getDaynight: {path:"/api/streams/{name}/daynight",method:'GET',response:'DaynightResponse',guard:validateDaynightResponse},
  setDaynight: {path:"/api/streams/{name}/daynight",method:'PUT',response:'DaynightUpdated',guard:validateDaynightUpdated,request:'DaynightRequest',requestGuard:validateDaynightRequest},
  getOnvifDiscoveryStatus: {path:"/api/onvif/discovery/status",method:'GET',response:'OnvifDiscoveryStatus',guard:validateOnvifDiscoveryStatus},
  getOnvifDevices: {path:"/api/onvif/devices",method:'GET',response:'OnvifDevicesResponse',guard:validateOnvifDevicesResponse},
  discoverOnvifDevices: {path:"/api/onvif/discovery/discover",method:'POST',response:'OnvifDevicesResponse',guard:validateOnvifDevicesResponse,request:'OnvifDiscoveryRequest',requestGuard:validateOnvifDiscoveryRequest},
  getOnvifProfiles: {path:"/api/onvif/device/profiles",method:'GET',response:'OnvifProfilesResponse',guard:validateOnvifProfilesResponse},
  addOnvifDevice: {path:"/api/onvif/device/add",method:'POST',response:'OnvifAddResult',guard:validateOnvifAddResult,request:'OnvifAddRequest',requestGuard:validateOnvifAddRequest},
  claimOnvifDevice: {path:"/api/onvif/device/claim",method:'POST',response:'DeviceSuccess',guard:validateDeviceSuccess,request:'OnvifClaimRequest',requestGuard:validateOnvifClaimRequest},
  testOnvifDevice: {path:"/api/onvif/device/test",method:'POST',response:'DeviceCommandResult',guard:validateDeviceCommandResult,request:'OnvifTestRequest',requestGuard:validateOnvifTestRequest},
};
export interface DeviceOptions extends Omit<EnhancedFetchOptions,'body'|'method'|'sensitive'|'retries'> {name?:string;body?:unknown}
export class DeviceRequestError extends Error {
  readonly name='DeviceRequestError';
  constructor(readonly operation:DeviceOperation,readonly status?:number,readonly mayHaveApplied=false){super(`${operation} failed${status===undefined?'':` (HTTP ${status})`}.`);}
}
export async function deviceRequest<K extends DeviceOperation>(operation:K, options:DeviceOptions={}):Promise<DeviceResults[K]> {
  const route=routes[operation];
  const {name,body,...fetchOptions}=options;
  if(route.path.includes('{name}') && (!name || name.length>255 || /[\x00-\x1f\x7f]/.test(name)))
    throw new ContractValidationError(operation,'request','DeviceStreamName');
  if(route.requestGuard && !(body===undefined && route.request==='DeviceEmptyRequest') && !route.requestGuard(body))
    throw new ContractValidationError(operation,'request',route.request!);
  const headers=new Headers(fetchOptions.headers);
  if(operation==='getOnvifProfiles') {
    const url=headers.get('X-Device-URL');
    if(!url||url.length>511||[headers.get('X-Username'),headers.get('X-Password')].some(value=>value!==null&&value.length>63))
      throw new ContractValidationError(operation,'request','OnvifProfileHeaders');
  }
  if(body!==undefined)headers.set('Content-Type','application/json');
  let response:Response;
  try { response=await enhancedFetch(route.path.replace('{name}',encodeURIComponent(name??'')),{timeout:20000,...fetchOptions,headers,method:route.method,body:body===undefined?undefined:JSON.stringify(body),retries:0,sensitive:true,cache:'no-store',credentials:'same-origin',redirect:'error'}); }
  catch(error:unknown) {
    if(options.signal?.aborted || error instanceof Error&&error.name==='AbortError')throw new DOMException('Request cancelled','AbortError');
    const status=typeof error==='object'&&error!==null&&'status' in error&&typeof error.status==='number'?error.status:undefined;
    throw new DeviceRequestError(operation,status,route.method!=='GET'&&(status===undefined||status>=500));
  }
  if(response.status!==200)throw new ContractValidationError(operation,'response',route.response,response.status,'status');
  if(!/^application\/json(?:;|$)/i.test(response.headers.get('content-type')??''))throw new ContractValidationError(operation,'response',route.response,response.status,'json');
  let value:unknown;
  try {value=await response.json();}catch{throw new ContractValidationError(operation,'response',route.response,response.status,'json');}
  if(options.signal?.aborted)throw new DOMException('Request cancelled','AbortError');
  if(!route.guard(value))throw new ContractValidationError(operation,'response',route.response,response.status);
  return value as DeviceResults[K];
}
