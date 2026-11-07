import {getPtzCapabilities,savePtzPreset,setPtzHome,ptzMove,ptzStop,ptzHome,ptzGotoPreset} from '../js/components/preact/ptzActions.ts';
import {enhancedFetch} from '../js/fetch-utils.ts';
jest.mock('../js/fetch-utils.ts',()=>({enhancedFetch:jest.fn()}));
const response=body=>new Response(JSON.stringify(body),{status:200,headers:{'Content-Type':'application/json'}});
const result={success:true,message:'Completed'};
afterEach(()=>jest.resetAllMocks());
test('saves named preset and validates refreshed list before updating controls',async()=>{
 const presets=[{token:'preset-42',name:'Front door'}];enhancedFetch.mockResolvedValueOnce(response({...result,token:'preset-42'})).mockResolvedValueOnce(response({presets,count:1}));const updated=jest.fn();
 await savePtzPreset('Front door / 1','Front door',updated);
 expect(enhancedFetch).toHaveBeenNthCalledWith(1,'/api/streams/Front%20door%20%2F%201/ptz/preset',expect.objectContaining({method:'PUT',body:'{"name":"Front door"}',retries:0,sensitive:true}));
 expect(enhancedFetch).toHaveBeenNthCalledWith(2,'/api/streams/Front%20door%20%2F%201/ptz/presets',expect.objectContaining({method:'GET'}));expect(updated).toHaveBeenCalledWith(presets);
});
test('sets current position as home',async()=>{enhancedFetch.mockResolvedValue(response(result));await setPtzHome('Lobby');expect(enhancedFetch).toHaveBeenCalledWith('/api/streams/Lobby/ptz/set-home',expect.objectContaining({method:'POST'}));});
test('rejects incomplete capability rows instead of hiding controls based on unvalidated fields',async()=>{enhancedFetch.mockResolvedValue(response({has_presets:false,has_home_position:true}));await expect(getPtzCapabilities('Lobby')).rejects.toMatchObject({phase:'response',schema:'PtzCapabilities'});});
test('exposes403 so panel can hide actions without updating presets',async()=>{enhancedFetch.mockRejectedValue(Object.assign(new Error('private payload'),{status:403}));const updated=jest.fn();await expect(savePtzPreset('Lobby','Front door',updated)).rejects.toMatchObject({status:403});expect(updated).not.toHaveBeenCalled();expect(enhancedFetch).toHaveBeenCalledTimes(1);});
test('retains presets when refresh fails after acknowledged save',async()=>{enhancedFetch.mockResolvedValueOnce(response({...result,token:'new'})).mockRejectedValueOnce(Object.assign(new Error('private'),{status:502}));const updated=jest.fn();await expect(savePtzPreset('Lobby','Front door',updated)).rejects.toMatchObject({status:502,stage:'refresh'});expect(updated).not.toHaveBeenCalled();});
test('movement actions bind exact captured camera and bodies without retries',async()=>{enhancedFetch.mockImplementation(()=>Promise.resolve(response(result)));await ptzMove('Lobby',0.5,-0.25,0);await ptzStop('Lobby');await ptzHome('Lobby');await ptzGotoPreset('Lobby','preset-1');expect(enhancedFetch.mock.calls.map(([url,options])=>[url,options.method,options.body])).toEqual([
 ['/api/streams/Lobby/ptz/move','POST','{"pan":0.5,"tilt":-0.25,"zoom":0}'],['/api/streams/Lobby/ptz/stop','POST',undefined],['/api/streams/Lobby/ptz/home','POST',undefined],['/api/streams/Lobby/ptz/goto-preset','POST','{"token":"preset-1"}']]);});
test('movement errors remain sanitized and are not retried',async()=>{enhancedFetch.mockRejectedValue(Object.assign(new Error('secret device URL'),{status:500}));await expect(ptzMove('Lobby',1,0,0)).rejects.toMatchObject({status:500,mayHaveApplied:true,message:'movePtz failed (HTTP 500).'});expect(enhancedFetch).toHaveBeenCalledTimes(1);});
