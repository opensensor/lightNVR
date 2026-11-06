import {queryFleetCameras,previewFleetSelector,listFleetViews,getFleetView,listCameraCollections,getCameraCollection,getCameraCollectionMembers,previewCameraCollection,listFleetLocations,getFleetLocation,listFleetTags,getFleetTag,getCameraTagAssignments,FleetRequestError} from '../../js/api/fleet.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const uuid='00000000-0000-4000-8000-000000000001',privateValue='do-not-expose-private';
const selector={version:1,expression:{op:'all'}};
const query={selector_version:1,queue_version:1,preview:false,page:1,page_size:50,total:0,total_pages:0,sort_by:'name',sort_order:'asc',cameras:[],queues:[]};
const view={uuid,name:'Fixture',is_shared:false,owned:true,selector,search:'',collection_uuid:null,columns:['name'],sort_by:'name',sort_order:'asc',revision:1,created_at:0,updated_at:0};
const location={uuid,name:'Fixture',parent_uuid:null,type:'site',sort_order:0,is_system:false,metadata:{fixture:true},child_count:0,camera_count:0,created_at:0,updated_at:0};
const tag={uuid,label:'Fixture',color:'',description:'',camera_count:0,created_at:0,updated_at:0};
const collection={uuid,name:'Fixture',description:'',type:'static',shared:false,owner_user_id:1,member_count:0,effective_count:0,created_at:0,updated_at:0,selector:null,selector_redacted:false};
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(body),{status}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('all thirteen fleet read operations accept complete wire shapes',async()=>{
  const cases:[()=>Promise<unknown>,unknown][]=[
    [()=>queryFleetCameras({}),query],[()=>previewFleetSelector({}),{...query,preview:true}],
    [()=>listFleetViews(),{views:[view],count:1,can_share:false}],[()=>getFleetView(uuid),view],
    [()=>listCameraCollections(),{collections:[collection],count:1}],[()=>getCameraCollection(uuid),collection],
    [()=>getCameraCollectionMembers(uuid),{collection_uuid:uuid,camera_uuids:[],count:0}],
    [()=>previewCameraCollection(uuid),{collection_uuid:uuid,matched_count:0,sample:[]}],
    [()=>listFleetLocations(),{locations:[location],count:1}],[()=>getFleetLocation(uuid),location],
    [()=>listFleetTags(),{tags:[tag],count:1}],[()=>getFleetTag(uuid),tag],
    [()=>getCameraTagAssignments(uuid),{camera_uuid:uuid,tags:[tag],count:1}],
  ];
  for(const [call,body] of cases){respond(body);await expect(call()).resolves.toEqual(body);}
});
test.each([{}, {...query,cameras:null},{...query,cameras:[{camera_uuid:uuid,name:privateValue}]},{...query,total:'0'},{...query,secret:privateValue}])('malformed camera query data cannot render as fleet rows',async body=>{
  respond(body);await expect(queryFleetCameras({selector})).rejects.toBeInstanceOf(ContractValidationError);
});
test('collection and catalog reads reject missing fields and secret-bearing envelopes',async()=>{
  const fetch=respond({collections:[{...collection,secret:privateValue}],count:1});await expect(listCameraCollections()).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).toHaveBeenCalledTimes(1);
  respond({locations:[{uuid,name:privateValue}],count:1});await expect(listFleetLocations()).rejects.toBeInstanceOf(ContractValidationError);
  respond({tags:null,count:0});await expect(listFleetTags()).rejects.toBeInstanceOf(ContractValidationError);
  respond({views:[{...view,selector:{version:99}}],count:1,can_share:false});await expect(listFleetViews()).rejects.toBeInstanceOf(ContractValidationError);
});
test.each([{page:0},{page_size:201},{selector:{version:1,expression:{op:'invalid'}}},{sort_order:'sideways'},{facets:'true'}])('invalid query input never reaches the server',body=>{
  const fetch=respond(query);expect(()=>queryFleetCameras(body)).toThrow(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
});
test('preview enforces its smaller bound and malformed success statuses are rejected',async()=>{
  const fetch=respond(query);expect(()=>previewFleetSelector({page_size:51})).toThrow(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
  respond({...query,preview:true,page_size:51});await expect(previewFleetSelector({})).rejects.toBeInstanceOf(ContractValidationError);
  respond(query,202);await expect(queryFleetCameras({})).rejects.toMatchObject({issue:'status'});
});
test.each(['../other','fixture',uuid+'/members',uuid+'?key='+privateValue])('bad UUID %s cannot alter the request path',id=>{
  const fetch=respond({});expect(()=>getCameraCollectionMembers(id)).toThrow(ContractValidationError);expect(()=>getCameraTagAssignments(id)).toThrow(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
});
test('private transport preserves caller Headers credentials without logging raw HTTP errors',async()=>{
  const fetch=respond(query);await queryFleetCameras({}, {headers:new Headers({'X-API-Key':privateValue})});
  expect(new Headers(fetch.mock.calls[0]?.[1]?.headers).get('X-API-Key')).toBe(privateValue);expect(fetch.mock.calls[0]?.[1]?.cache).toBe('no-store');
  respond({error:privateValue},500);await expect(listFleetTags({retries:0})).rejects.toEqual(new FleetRequestError('listFleetTags',500));
});
test('bad JSON and cancellation remain sanitized failures',async()=>{
  jest.spyOn(global,'fetch').mockResolvedValue(new Response(privateValue));await expect(listFleetViews()).rejects.toMatchObject({issue:'json'});
  const controller=new AbortController();controller.abort(privateValue);const fetch=respond({});fetch.mockClear();await expect(listFleetLocations({signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();
});
