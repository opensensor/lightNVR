System.register(["./preact-app-legacy-XZUKaCaj.js","./LoadingIndicator-legacy-hqIGWaUk.js","./fetch-utils-legacy-DOz1-Xee.js"],(function(e,t){"use strict";var r,a,s,o,d,n,i,l,c,u;return{setters:[e=>{r=e.d,a=e.A,s=e.y,o=e.h,d=e._,n=e.c},e=>{i=e.C},e=>{l=e.f,c=e.e,u=e.c}],execute:function(){function g(){const[e,t]=r([]),[d,g]=r(!1),[p,m]=r(!1),[b,y]=r([]),[f,h]=r([]),[x,v]=r(null),[w,k]=r(null),[$,C]=r(!1),[E,_]=r({username:"",password:""}),[I,S]=r({name:"",url:"",enabled:!0,streamingEnabled:!0,width:1280,height:720,fps:15,codec:"h264",protocol:"0",priority:"5",segment:30,record:!0,detectionEnabled:!1,detectionModel:"",detectionThreshold:50,detectionInterval:10,preBuffer:10,postBuffer:30}),[O,N]=r([]),[D,V]=r(!1),[T,F]=r(!1),[A,j]=r(!1),P=a(null);s((()=>(P.current=u(),B(),M(),()=>{P.current&&P.current.abort()})),[]);const B=async()=>{try{F(!0),j(!1);const e=await l("/api/streams",{signal:P.current?.signal,timeout:15e3,retries:2,retryDelay:1e3})||[];t(e),j(e.length>0)}catch(e){"Request was cancelled"!==e.message&&(console.error("Error loading streams:",e),n("Error loading streams: "+e.message),j(!1))}finally{F(!1)}},M=async()=>{try{const e=await l("/api/detection/models",{signal:P.current?.signal,timeout:1e4,retries:1,retryDelay:500});N(e.models||[])}catch(e){"Request was cancelled"!==e.message&&console.error("Error loading detection models:",e)}},R=()=>{g(!1)},U=e=>{const{name:t,value:r,type:a,checked:s}=e.target;S((e=>({...e,[t]:"checkbox"===a?s:r})))},L=e=>{const{name:t,value:r}=e.target;_((e=>({...e,[t]:r})))},[z,H]=r(!1),[J,q]=r(""),[X,G]=r(!1),[W,K]=r(!1),Q=async e=>{try{const t={username:E.username,password:E.password};n("Testing ONVIF connection..."),await c("/api/onvif/device/test",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({url:e.device_service,username:t.username,password:t.password}),signal:P.current?.signal,timeout:15e3,retries:1,retryDelay:2e3}),n("ONVIF connection successful!"),(async e=>{try{v(e),h([]),H(!0),n("Getting device profiles...");const t=await l("/api/onvif/device/profiles",{headers:{"X-Device-URL":e.device_service,"X-Username":E.username,"X-Password":E.password},signal:P.current?.signal,timeout:2e4,retries:1,retryDelay:2e3});h(t.profiles||[]),t.profiles&&t.profiles.length>0?n(`Found ${t.profiles.length} profiles`):n("No profiles found")}catch(t){console.error("Error getting device profiles:",t),n("Error getting device profiles: "+t.message,3e3,"error")}finally{H(!1)}})(e)}catch(t){console.error("Error testing ONVIF connection:",t),n("Error testing ONVIF connection: "+t.message,3e3,"error")}};return o`
    <section id="streams-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Streams</h2>
        <div class="controls flex space-x-2">
          <button 
            id="discover-onvif-btn" 
            class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${()=>m(!0)}
          >
            Discover ONVIF Cameras
          </button>
          <button 
            id="add-stream-btn" 
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${()=>{S({name:"",url:"",enabled:!0,streamingEnabled:!0,width:1280,height:720,fps:15,codec:"h264",protocol:"0",priority:"5",segment:30,record:!0,recordAudio:!0,detectionEnabled:!1,detectionModel:"",detectionThreshold:50,detectionInterval:10,preBuffer:10,postBuffer:30}),V(!1),g(!0)}}
          >
            Add Stream
          </button>
        </div>
      </div>
      
      <${i}
        isLoading=${T}
        hasData=${A}
        loadingMessage="Loading streams..."
        emptyMessage="No streams configured yet. Click 'Add Stream' to create one."
      >
        <div class="streams-container bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
          <div class="overflow-x-auto">
            <table id="streams-table" class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
              <thead class="bg-gray-50 dark:bg-gray-700">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">URL</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Resolution</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">FPS</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Recording</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Actions</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                ${e.map((e=>o`
                  <tr key=${e.name} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center">
                        <span class=${"status-indicator w-2 h-2 rounded-full mr-2 "+(e.enabled?"bg-green-500":"bg-red-500")}></span>
                        ${e.name}
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">${e.url}</td>
                    <td class="px-6 py-4 whitespace-nowrap">${e.width}x${e.height}</td>
                    <td class="px-6 py-4 whitespace-nowrap">${e.fps}</td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      ${e.record?"Enabled":"Disabled"}
                      ${e.detection_based_recording?" (Detection)":""}
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex space-x-2">
                        <button 
                          class="p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                          onClick=${()=>(async e=>{try{const t=await l(`/api/streams/${encodeURIComponent(e)}`,{signal:P.current?.signal,timeout:1e4,retries:2,retryDelay:1e3});S({...t,width:t.width||1280,height:t.height||720,fps:t.fps||15,protocol:t.protocol?.toString()||"0",priority:t.priority?.toString()||"5",segment:t.segment_duration||30,detectionThreshold:t.detection_threshold||50,detectionInterval:t.detection_interval||10,preBuffer:t.pre_detection_buffer||10,postBuffer:t.post_detection_buffer||30,streamingEnabled:void 0===t.streaming_enabled||t.streaming_enabled,isOnvif:void 0!==t.is_onvif&&t.is_onvif,detectionEnabled:t.detection_based_recording||!1,detectionModel:t.detection_model||"",recordAudio:void 0===t.record_audio||t.record_audio}),V(!0),g(!0)}catch(t){console.error("Error loading stream details:",t),n("Error loading stream details: "+t.message)}})(e.name)}
                          title="Edit"
                        >
                          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                          </svg>
                        </button>
                        <button 
                          class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                          onClick=${()=>(async e=>{if(confirm(`Are you sure you want to delete stream "${e}"?`))try{await c(`/api/streams/${encodeURIComponent(e)}`,{method:"DELETE",signal:P.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),n("Stream deleted successfully"),B()}catch(t){console.error("Error deleting stream:",t),n("Error deleting stream: "+t.message)}})(e.name)}
                          title="Delete"
                        >
                          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                          </svg>
                        </button>
                      </div>
                    </td>
                  </tr>
                `))}
              </tbody>
            </table>
          </div>
        </div>
      <//>
      
      ${d&&o`
        <div id="stream-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">${D?"Edit Stream":"Add Stream"}</h3>
              <span class="text-2xl cursor-pointer" onClick=${R}>×</span>
            </div>
            <div class="p-4">
              <form id="stream-form" class="space-y-4">
                <div class="form-group">
                  <label for="stream-name" class="block text-sm font-medium mb-1">Name</label>
                  <input 
                    type="text" 
                    id="stream-name" 
                    name="name"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${D?"bg-gray-100 dark:bg-gray-800":""}"
                    value=${I.name}
                    onChange=${U}
                    disabled=${D}
                    required
                  />
                </div>
                <div class="form-group">
                  <label for="stream-url" class="block text-sm font-medium mb-1">URL</label>
                  <input 
                    type="text" 
                    id="stream-url" 
                    name="url"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                    placeholder="rtsp://example.com/stream" 
                    value=${I.url}
                    onChange=${U}
                    required
                  />
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-enabled" 
                    name="enabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${I.enabled}
                    onChange=${U}
                  />
                  <label for="stream-enabled" class="ml-2 block text-sm">Stream Active</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Enable/disable all stream processing</span>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-streaming-enabled" 
                    name="streamingEnabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${I.streamingEnabled}
                    onChange=${U}
                  />
                  <label for="stream-streaming-enabled" class="ml-2 block text-sm">Live View Enabled</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Enable/disable live viewing in browser</span>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-is-onvif" 
                    name="isOnvif"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${I.isOnvif}
                    onChange=${U}
                  />
                  <label for="stream-is-onvif" class="ml-2 block text-sm">ONVIF Camera</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Mark this stream as an ONVIF camera for special handling</span>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="form-group">
                    <label for="stream-width" class="block text-sm font-medium mb-1">Width</label>
                    <input 
                      type="number" 
                      id="stream-width" 
                      name="width"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="320" 
                      max="1920" 
                      value=${I.width}
                      onChange=${U}
                    />
                  </div>
                  <div class="form-group">
                    <label for="stream-height" class="block text-sm font-medium mb-1">Height</label>
                    <input 
                      type="number" 
                      id="stream-height" 
                      name="height"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="240" 
                      max="1080" 
                      value=${I.height}
                      onChange=${U}
                    />
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div class="form-group">
                    <label for="stream-fps" class="block text-sm font-medium mb-1">FPS</label>
                    <input 
                      type="number" 
                      id="stream-fps" 
                      name="fps"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="1" 
                      max="30" 
                      value=${I.fps}
                      onChange=${U}
                    />
                  </div>
                  <div class="form-group">
                    <label for="stream-codec" class="block text-sm font-medium mb-1">Codec</label>
                    <select 
                      id="stream-codec" 
                      name="codec"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${I.codec}
                      onChange=${U}
                    >
                      <option value="h264">H.264</option>
                      <option value="h265">H.265</option>
                    </select>
                  </div>
                  <div class="form-group">
                    <label for="stream-protocol" class="block text-sm font-medium mb-1">Protocol</label>
                    <select 
                      id="stream-protocol" 
                      name="protocol"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${I.protocol}
                      onChange=${U}
                    >
                      <option value="0">TCP</option>
                      <option value="1">UDP</option>
                    </select>
                    <span class="text-xs text-gray-500 dark:text-gray-400">Connection protocol (ONVIF cameras use either TCP or UDP)</span>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="form-group">
                    <label for="stream-priority" class="block text-sm font-medium mb-1">Priority</label>
                    <select 
                      id="stream-priority" 
                      name="priority"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${I.priority}
                      onChange=${U}
                    >
                      <option value="1">Low (1)</option>
                      <option value="5">Medium (5)</option>
                      <option value="10">High (10)</option>
                    </select>
                  </div>
                  <div class="form-group">
                    <label for="stream-segment" class="block text-sm font-medium mb-1">Segment Duration (seconds)</label>
                    <input 
                      type="number" 
                      id="stream-segment" 
                      name="segment"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="60" 
                      max="3600" 
                      value=${I.segment}
                      onChange=${U}
                    />
                  </div>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-record" 
                    name="record"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${I.record}
                    onChange=${U}
                  />
                  <label for="stream-record" class="ml-2 block text-sm">Record</label>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-record-audio" 
                    name="recordAudio"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${I.recordAudio}
                    onChange=${U}
                  />
                  <label for="stream-record-audio" class="ml-2 block text-sm">Record Audio</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Include audio in recordings if available in the stream</span>
                </div>
                
                <!-- Detection-based recording options -->
                <div class="mt-6 mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">
                  <h4 class="text-md font-medium">Detection-Based Recording</h4>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-detection-enabled" 
                    name="detectionEnabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${I.detectionEnabled}
                    onChange=${U}
                  />
                  <label for="stream-detection-enabled" class="ml-2 block text-sm">Enable Detection-Based Recording</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Only record when objects are detected</span>
                </div>
                <div class="form-group" style=${I.detectionEnabled?"":"display: none"}>
                  <label for="stream-detection-model" class="block text-sm font-medium mb-1">Detection Model</label>
                  <div class="flex space-x-2">
                    <select 
                      id="stream-detection-model" 
                      name="detectionModel"
                      class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${I.detectionModel}
                      onChange=${U}
                    >
                      <option value="">Select a model</option>
                      ${O.map((e=>o`
                        <option key=${e.id} value=${e.id}>${e.name}</option>
                      `))}
                    </select>
                    <button 
                      id="refresh-models-btn" 
                      class="p-2 rounded-md bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
                      title="Refresh Models"
                      onClick=${M}
                      type="button"
                    >
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                <div class="form-group" style=${I.detectionEnabled?"":"display: none"}>
                  <label for="stream-detection-threshold" class="block text-sm font-medium mb-1">Detection Threshold</label>
                  <div class="flex items-center space-x-2">
                    <input 
                      type="range" 
                      id="stream-detection-threshold" 
                      name="detectionThreshold"
                      class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                      min="0" 
                      max="100" 
                      step="1" 
                      value=${I.detectionThreshold}
                      onInput=${e=>{const t=parseInt(e.target.value,10);S((e=>({...e,detectionThreshold:t})))}}
                    />
                    <span id="stream-threshold-value" class="font-medium text-blue-600 dark:text-blue-400 min-w-[3rem] text-center">
                      ${I.detectionThreshold}%
                    </span>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4" style=${I.detectionEnabled?"":"display: none"}>
                  <div class="form-group">
                    <label for="stream-detection-interval" class="block text-sm font-medium mb-1">Detection Interval (frames)</label>
                    <input 
                      type="number" 
                      id="stream-detection-interval" 
                      name="detectionInterval"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="1" 
                      max="100" 
                      value=${I.detectionInterval}
                      onChange=${U}
                    />
                    <span class="text-xs text-gray-500 dark:text-gray-400">Detect on every Nth frame</span>
                  </div>
                  <div class="form-group">
                    <label for="stream-pre-buffer" class="block text-sm font-medium mb-1">Pre-detection Buffer (seconds)</label>
                    <input 
                      type="number" 
                      id="stream-pre-buffer" 
                      name="preBuffer"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="0" 
                      max="60" 
                      value=${I.preBuffer}
                      onChange=${U}
                    />
                    <span class="text-xs text-gray-500 dark:text-gray-400">Seconds to keep before detection</span>
                  </div>
                  <div class="form-group">
                    <label for="stream-post-buffer" class="block text-sm font-medium mb-1">Post-detection Buffer (seconds)</label>
                    <input 
                      type="number" 
                      id="stream-post-buffer" 
                      name="postBuffer"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="0" 
                      max="300" 
                      value=${I.postBuffer}
                      onChange=${U}
                    />
                    <span class="text-xs text-gray-500 dark:text-gray-400">Seconds to keep after detection</span>
                  </div>
                </div>
              </form>
            </div>
            <div class="flex justify-between p-4 border-t border-gray-200 dark:border-gray-700">
              <button 
                id="stream-test-btn" 
                class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                onClick=${async()=>{try{n("Testing stream connection...");const e=await l("/api/streams/test",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({url:I.url,protocol:parseInt(I.protocol,10)}),signal:P.current?.signal,timeout:2e4,retries:1,retryDelay:2e3});e.success?(n("Stream connection successful!"),e.info&&S((t=>({...t,width:e.info.width||t.width,height:e.info.height||t.height,fps:e.info.fps||t.fps,codec:e.info.codec||t.codec})))):n(`Stream test failed: ${e.message||"Unknown error"}`,3e3,"error")}catch(e){console.error("Error testing stream:",e),n("Error testing stream: "+e.message,3e3,"error")}}}
                type="button"
              >
                Test Connection
              </button>
              <div class="space-x-2">
                <button 
                  id="stream-save-btn" 
                  class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                  onClick=${async()=>{try{const e={...I,width:parseInt(I.width,10),height:parseInt(I.height,10),fps:parseInt(I.fps,10),protocol:parseInt(I.protocol,10),priority:parseInt(I.priority,10),segment_duration:parseInt(I.segment,10),streaming_enabled:I.streamingEnabled,is_onvif:I.isOnvif,detection_based_recording:I.detectionEnabled,detection_model:I.detectionModel,detection_threshold:I.detectionThreshold,detection_interval:parseInt(I.detectionInterval,10),pre_detection_buffer:parseInt(I.preBuffer,10),post_detection_buffer:parseInt(I.postBuffer,10),record_audio:I.recordAudio},t=D?`/api/streams/${encodeURIComponent(I.name)}`:"/api/streams",r=D?"PUT":"POST";await c(t,{method:r,headers:{"Content-Type":"application/json"},body:JSON.stringify(e),signal:P.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),n(`Stream ${D?"updated":"added"} successfully`),R(),B()}catch(e){console.error(`Error ${D?"updating":"adding"} stream:`,e),n(`Error ${D?"updating":"adding"} stream: ${e.message}`)}}}
                  type="button"
                >
                  Save
                </button>
                <button 
                  id="stream-cancel-btn" 
                  class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                  onClick=${R}
                  type="button"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      `}
      
      ${p&&o`
        <div id="onvif-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">ONVIF Camera Discovery</h3>
              <span class="text-2xl cursor-pointer" onClick=${()=>m(!1)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4 flex justify-between items-center">
                <h4 class="text-md font-medium">Discovered Devices</h4>
                <button 
                  id="discover-btn" 
                  class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                  onClick=${async()=>{try{C(!0),n("Starting ONVIF discovery...");const e=await l("/api/onvif/discovery/discover",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({}),signal:P.current?.signal,timeout:12e4,retries:0});y(e.devices||[]),e.devices&&e.devices.length>0?n(`Discovered ${e.devices.length} ONVIF devices`):n("No ONVIF devices found")}catch(e){console.error("Error discovering ONVIF devices:",e),n("Error discovering ONVIF devices: "+e.message,3e3,"error")}finally{C(!1)}}}
                  disabled=${$}
                  type="button"
                >
                  ${$?o`
                    <span class="flex items-center">
                      Discovering
                      <span class="ml-1 flex space-x-1">
                        <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                      </span>
                    </span>
                  `:"Start Discovery"}
                </button>
              </div>
              
              <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                  <thead class="bg-gray-50 dark:bg-gray-700">
                    <tr>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">IP Address</th>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Manufacturer</th>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Model</th>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Actions</th>
                    </tr>
                  </thead>
                  <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                    ${0===b.length?o`
                      <tr>
                        <td colspan="4" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                          ${$?o`
                            <div class="flex items-center justify-center">
                              <span>Discovering devices</span>
                              <span class="ml-1 flex space-x-1">
                                <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-gray-500 dark:bg-gray-400 rounded-full"></span>
                                <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-gray-500 dark:bg-gray-400 rounded-full"></span>
                                <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-gray-500 dark:bg-gray-400 rounded-full"></span>
                              </span>
                            </div>
                          `:'No devices discovered yet. Click "Start Discovery" to scan your network.'}
                        </td>
                      </tr>
                    `:b.map((e=>o`
                      <tr key=${e.ip_address} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td class="px-6 py-4 whitespace-nowrap">${e.ip_address}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${e.manufacturer||"Unknown"}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${e.model||"Unknown"}</td>
                        <td class="px-6 py-4 whitespace-nowrap">
                          <button 
                            class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none"
                            onClick=${()=>Q(e)}
                            disabled=${z&&x&&x.ip_address===e.ip_address}
                            type="button"
                          >
                            ${z&&x&&x.ip_address===e.ip_address?o`
                              <span class="flex items-center">
                                Loading
                                <span class="ml-1 flex space-x-1">
                                  <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                                  <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                                  <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                                </span>
                              </span>
                            `:"Connect"}
                          </button>
                        </td>
                      </tr>
                    `))}
                  </tbody>
                </table>
              </div>
              
              <div class="mt-6 mb-4">
                <h4 class="text-md font-medium mb-2">Authentication</h4>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
                  Enter credentials to connect to the selected ONVIF device. Credentials are not needed for discovery, only for connecting to devices.
                </p>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="form-group">
                    <label for="onvif-username" class="block text-sm font-medium mb-1">Username</label>
                    <input 
                      type="text" 
                      id="onvif-username" 
                      name="username"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      placeholder="admin" 
                      value=${E.username}
                      onChange=${L}
                    />
                  </div>
                  <div class="form-group">
                    <label for="onvif-password" class="block text-sm font-medium mb-1">Password</label>
                    <input 
                      type="password" 
                      id="onvif-password" 
                      name="password"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      placeholder="password" 
                      value=${E.password}
                      onChange=${L}
                    />
                  </div>
                </div>
              </div>
              
              ${x&&f.length>0&&o`
                <div class="mt-6">
                  <h4 class="text-md font-medium mb-2">Available Profiles for ${x.ip_address}</h4>
                  <div class="overflow-x-auto">
                    <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                      <thead class="bg-gray-50 dark:bg-gray-700">
                        <tr>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Resolution</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Encoding</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">FPS</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Actions</th>
                        </tr>
                      </thead>
                      <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                        ${f.map((e=>o`
                          <tr key=${e.token} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                            <td class="px-6 py-4 whitespace-nowrap">${e.name}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${e.width}x${e.height}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${e.encoding}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${e.fps}</td>
                            <td class="px-6 py-4 whitespace-nowrap">
                              <button 
                                class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                                onClick=${()=>(async e=>{try{k(e);const t=`ONVIF_${x.ip_address.replace(/\./g,"_")}_${e.name.replace(/\s+/g,"_")}`;q(t),G(!0)}catch(t){console.error("Error preparing to add ONVIF device:",t),n("Error preparing to add ONVIF device: "+t.message,3e3,"error")}})(e)}
                                type="button"
                              >
                                Add as Stream
                              </button>
                            </td>
                          </tr>
                        `))}
                      </tbody>
                    </table>
                  </div>
                </div>
              `}
            </div>
            <div class="flex justify-end p-4 border-t border-gray-200 dark:border-gray-700">
              <button 
                id="onvif-close-btn" 
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                onClick=${()=>m(!1)}
                type="button"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      `}
      
      ${X&&o`
        <div id="custom-name-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">Stream Name</h3>
              <span class="text-2xl cursor-pointer" onClick=${()=>G(!1)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4">
                <label for="custom-stream-name" class="block text-sm font-medium mb-1">Enter a name for this stream:</label>
                <input 
                  type="text" 
                  id="custom-stream-name" 
                  class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${J}
                  onChange=${e=>q(e.target.value)}
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  This name will be used to identify the stream in the system.
                </p>
              </div>
            </div>
            <div class="flex justify-end p-4 border-t border-gray-200 dark:border-gray-700 space-x-2">
              <button 
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                onClick=${()=>G(!1)}
                type="button"
              >
                Cancel
              </button>
              <button 
                class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                onClick=${async()=>{try{K(!0),n("Adding ONVIF device as stream..."),await c("/api/onvif/device/add",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({device_url:x.device_service,profile_token:w.token,stream_name:J,username:E.username,password:E.password}),signal:P.current?.signal,timeout:3e4,retries:1,retryDelay:3e3}),n("ONVIF device added as stream successfully"),m(!1),G(!1),B()}catch(e){console.error("Error adding ONVIF device as stream:",e),n("Error adding ONVIF device as stream: "+e.message,3e3,"error")}finally{K(!1)}}}
                type="button"
                disabled=${!J.trim()||W}
              >
                ${W?o`
                  <span class="flex items-center">
                    Adding
                    <span class="ml-1 flex space-x-1">
                      <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                    </span>
                  </span>
                `:"Add Stream"}
              </button>
            </div>
          </div>
        </div>
      `}
    </section>
  `}e({StreamsView:g,loadStreamsView:function(){const e=document.getElementById("main-content");e&&d((async()=>{const{render:e}=await t.import("./preact-app-legacy-XZUKaCaj.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(o`<${g} />`,e)}))}})}}}));
//# sourceMappingURL=StreamsView-legacy-BoAAsaA3.js.map
