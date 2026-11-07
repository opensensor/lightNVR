import {deviceRequest} from '../../js/api/device.ts';
import {enhancedFetch} from '../../js/fetch-utils.ts';
jest.mock('../../js/fetch-utils.ts',()=>({enhancedFetch:jest.fn()}));
const fetchMock=jest.mocked(enhancedFetch);
afterEach(()=>jest.resetAllMocks());
test.each([
 ['movePtz',{name:'A',body:{pan:NaN}}],['movePtz',{name:'A',body:{pan:Infinity}}],['movePtz',{name:'A',body:{pan:'fast'}}],['movePtz',{name:'A',body:{pan:0,unknown:1}}],
 ['setDaynight',{name:'A',body:{mode:'day',force_persistence:'false'}}],['setImagingSettings',{name:'A',body:{brightness:1,contrast:'bad'}}],['setPtzPreset',{name:'A',body:{name:'x'.repeat(64)}}],['testOnvifDevice',{body:{url:'http://127.0.0.1',password:4}}],['getOnvifProfiles',{headers:{}}]
] as const)('%s rejects invalid request before network',async(operation,options)=>{await expect(deviceRequest(operation,options)).rejects.toMatchObject({phase:'request'});expect(fetchMock).not.toHaveBeenCalled();});
test.each([
 [200,'application/json','{"success":true,"message":"private","extra":"secret"}'],[202,'application/json','{"success":true,"message":"private"}'],[200,'text/html','{"success":true,"message":"private"}'],[200,'application/json','broken-json']
])('rejects invalid acknowledgment status=%s type=%s',async(status,type,body)=>{fetchMock.mockResolvedValue(new Response(body,{status:Number(status),headers:{'Content-Type':String(type)}}));await expect(deviceRequest('movePtz',{name:'A',body:{pan:0}})).rejects.toMatchObject({phase:'response',mayHaveApplied:true});expect(fetchMock).toHaveBeenCalledTimes(1);});
test('preserves abort signal without leaking transport details',async()=>{const controller=new AbortController();controller.abort();fetchMock.mockRejectedValue(new Error('private network detail'));await expect(deviceRequest('getDaynight',{name:'A',signal:controller.signal})).rejects.toMatchObject({name:'AbortError',message:'Request cancelled'});});
