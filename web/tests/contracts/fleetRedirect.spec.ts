import {fleetWriteJSON} from '../../js/api/fleetWrites.ts';
import {queryFleetCameras} from '../../js/api/fleet.ts';
import {withRedirectFixture} from './redirectHarness';
afterEach(()=>jest.restoreAllMocks());
for(const status of [307,308] as const){
 test(`fleet mutation never forwards body across HTTP ${status}`,async()=>{
  await withRedirectFixture(status,async fixture=>{
   const transport=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
   const body=JSON.stringify({label:'private-owned-camera-label'});
   await expect(fleetWriteJSON('/api/camera-tags',{method:'POST',body,redirect:'follow',credentials:'include',retries:7})).rejects.toBeInstanceOf(Error);
   expect(fixture.sourceBodies).toEqual([body]);expect(fixture.forwardedBodies).toEqual([]);
   expect(transport).toHaveBeenCalledWith('/api/camera-tags',expect.objectContaining({credentials:'same-origin',redirect:'error'}));
  });
 });
 test(`fleet selector never forwards camera identifiers across HTTP ${status}`,async()=>{
  await withRedirectFixture(status,async fixture=>{
   const transport=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
   const body={selector:{version:1,expression:{op:'camera_uuid',values:['00000000-0000-4000-8000-000000000001']}}};
   await expect(queryFleetCameras(body,{redirect:'follow',credentials:'include',retries:0})).rejects.toBeInstanceOf(Error);
   expect(fixture.sourceBodies).toEqual([JSON.stringify(body)]);expect(fixture.forwardedBodies).toEqual([]);
   expect(transport).toHaveBeenCalledWith('/api/fleet/cameras/query',expect.objectContaining({credentials:'same-origin',redirect:'error'}));
  });
 });
}
