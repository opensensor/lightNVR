import {deviceRequest} from '../../js/api/device.ts';
import {withRedirectFixture} from './redirectHarness';
afterEach(()=>jest.restoreAllMocks());
test.each([307,308] as const)('ONVIF credentials never follow an HTTP %s redirect',async status=>{
 await withRedirectFixture(status,async fixture=>{
  const transport=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
  const body={url:'http://owned.invalid/onvif/device_service',username:'owned-camera-user',password:'owned-camera-secret'};
  await expect(deviceRequest('testOnvifDevice',{body,redirect:'follow',credentials:'include'})).rejects.toBeInstanceOf(Error);
  expect(fixture.sourceBodies).toEqual([JSON.stringify(body)]);expect(fixture.forwardedBodies).toEqual([]);
  expect(transport).toHaveBeenCalledWith('/api/onvif/device/test',expect.objectContaining({credentials:'same-origin',redirect:'error'}));
 });
});
