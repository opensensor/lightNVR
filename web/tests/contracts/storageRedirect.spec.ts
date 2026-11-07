import {storageJSON} from '../../js/api/storage.ts';
import {withRedirectFixture} from './redirectHarness';
afterEach(()=>jest.restoreAllMocks());
test.each([307,308] as const)('storage credentials never follow an HTTP %s redirect',async status=>{
 await withRedirectFixture(status,async fixture=>{
  const transport=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
  const body=JSON.stringify({name:'owned-private-target',root_path:'',target_type:'s3',endpoint:'https://owned.invalid',credential_ref:'owned-private-credential-reference'});
  await expect(storageJSON('/api/storage-targets',{method:'POST',body,redirect:'follow',credentials:'include',retries:7})).rejects.toBeInstanceOf(Error);
  expect(fixture.sourceBodies).toEqual([body]);expect(fixture.forwardedBodies).toEqual([]);
  expect(transport).toHaveBeenCalledWith('/api/storage-targets',expect.objectContaining({credentials:'same-origin',redirect:'error'}));
 });
});
