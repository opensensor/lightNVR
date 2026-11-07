/** Owned loopback ONVIF device; records actual SOAP bytes, never contacts hardware. */
import {createServer} from 'node:http';
export async function startDevice() {
  const requests: {action:string;body:string;path:string}[]=[];
  let base='', fault=false, empty=false, streamUri='';
  const server=createServer(async(req,res)=>{
    let body=''; for await(const chunk of req) body+=chunk;
    const action=body.match(/<(?:\w+:)?(GetServices|GetCapabilities|GetProfiles|GetSnapshotUri|GetStreamUri|GetImagingSettings|GetOptions|SetImagingSettings|GetConfigurationOptions|GetConfigurations|GetConfiguration|GetNodes|GetNode|GetPresets|SetPreset|GotoPreset|ContinuousMove|AbsoluteMove|RelativeMove|Stop|GotoHomePosition|SetHomePosition)\b/)?.[1]??'Unknown';
    requests.push({action,body,path:req.url??''});
    let content=`<tptz:${action}Response/>`;
    const services=['Media','PTZ','Imaging'].map(name=>`<tt:${name}><tt:XAddr>${base}/onvif/${name.toLowerCase()}_service</tt:XAddr></tt:${name}>`).join('');
    if(action==='GetServices')content=`<tds:GetServicesResponse>${['media','ptz','imaging'].map(name=>`<tds:Service><tds:Namespace>http://www.onvif.org/ver20/${name}/wsdl</tds:Namespace><tds:XAddr>${base}/onvif/${name}_service</tds:XAddr></tds:Service>`).join('')}</tds:GetServicesResponse>`;
    if(action==='GetCapabilities')content=`<tds:GetCapabilitiesResponse><tds:Capabilities>${services}</tds:Capabilities></tds:GetCapabilitiesResponse>`;
    if(action==='GetProfiles')content=`<trt:GetProfilesResponse>${empty?'':`<trt:Profiles token="Profile_1"><tt:Name>Owned camera</tt:Name><tt:VideoSourceConfiguration token="source-config"><tt:SourceToken>VideoSource_1</tt:SourceToken></tt:VideoSourceConfiguration><tt:VideoEncoderConfiguration><tt:Encoding>H264</tt:Encoding><tt:Resolution><tt:Width>320</tt:Width><tt:Height>240</tt:Height></tt:Resolution><tt:RateControl><tt:FrameRateLimit>10</tt:FrameRateLimit><tt:BitrateLimit>128</tt:BitrateLimit></tt:RateControl></tt:VideoEncoderConfiguration><tt:PTZConfiguration token="ptz-config"><tt:NodeToken>node1</tt:NodeToken></tt:PTZConfiguration></trt:Profiles>`}</trt:GetProfilesResponse>`;
    if(action==='GetSnapshotUri'||action==='GetStreamUri')content=`<trt:${action}Response><trt:MediaUri><tt:Uri>${action==='GetStreamUri'?streamUri:''}</tt:Uri></trt:MediaUri></trt:${action}Response>`;
    if(action==='GetImagingSettings')content='<timg:GetImagingSettingsResponse><timg:ImagingSettings><tt:Brightness>43</tt:Brightness><tt:Contrast>32</tt:Contrast><tt:IrCutFilter>Auto</tt:IrCutFilter><tt:WideDynamicRange><tt:Mode>ON</tt:Mode><tt:Level>12</tt:Level></tt:WideDynamicRange></timg:ImagingSettings></timg:GetImagingSettingsResponse>';
    if(action==='GetOptions')content='<timg:GetOptionsResponse><timg:ImagingOptions><tt:Brightness><tt:Min>0</tt:Min><tt:Max>100</tt:Max></tt:Brightness><tt:IrCutFilterModes>On</tt:IrCutFilterModes><tt:IrCutFilterModes>Off</tt:IrCutFilterModes><tt:IrCutFilterModes>Auto</tt:IrCutFilterModes></timg:ImagingOptions></timg:GetOptionsResponse>';
    if(action==='GetConfigurations')content='<tptz:GetConfigurationsResponse><tt:PTZConfiguration token="ptz-config"><tt:DefaultContinuousPanTiltVelocitySpace>owned-space</tt:DefaultContinuousPanTiltVelocitySpace><tt:PanTiltLimits><tt:Range><tt:XRange><tt:Min>-2</tt:Min><tt:Max>2</tt:Max></tt:XRange><tt:YRange><tt:Min>-3</tt:Min><tt:Max>3</tt:Max></tt:YRange></tt:Range></tt:PanTiltLimits></tt:PTZConfiguration></tptz:GetConfigurationsResponse>';
    if(action==='GetPresets')content='<tptz:GetPresetsResponse><tptz:Preset token="preset-one"><tt:Name>Owned preset</tt:Name></tptz:Preset></tptz:GetPresetsResponse>';
    if(action==='SetPreset')content='<tptz:SetPresetResponse><tptz:PresetToken>preset-new</tptz:PresetToken></tptz:SetPresetResponse>';
    if(fault)content='<s:Fault><s:Code><s:Value>s:Sender</s:Value></s:Code><s:Reason><s:Text>Owned fault</s:Text></s:Reason></s:Fault>';
    res.writeHead(200,{'Content-Type':'application/soap+xml'});
    res.end(`<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:tds="http://www.onvif.org/ver10/device/wsdl" xmlns:trt="http://www.onvif.org/ver10/media/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl" xmlns:timg="http://www.onvif.org/ver20/imaging/wsdl"><s:Body>${content}</s:Body></s:Envelope>`);
  });
  await new Promise<void>(resolve=>server.listen(0,'127.0.0.1',resolve));
  const address=server.address();if(!address||typeof address==='string')throw Error('No owned device address');
  base=`http://127.0.0.1:${address.port}`;
  return {base,port:address.port,requests,setStreamUri(value:string){streamUri=value;},setFault(value:boolean){fault=value;},setEmpty(value:boolean){empty=value;},close:()=>new Promise<void>((resolve,reject)=>server.close(error=>error?reject(error):resolve()))};
}
