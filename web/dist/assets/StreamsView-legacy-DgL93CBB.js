System.register(["./preact-app-legacy-kv_02ehl.js","./LoadingIndicator-legacy-B2piEOhj.js","./fetch-utils-legacy-DOz1-Xee.js"],(function(e,t){"use strict";var r,a,s,o,n,d,i,l,c,u;return{setters:[e=>{r=e.d,a=e.h,s=e.A,o=e.y,n=e._,d=e.c},e=>{i=e.C},e=>{l=e.f,c=e.e,u=e.c}],execute:function(){function g({streamId:e,streamName:t,onClose:s,onDisable:o,onDelete:n}){const[d,i]=r(!1);return a`
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
        <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium">${d?"Confirm Permanent Deletion":"Stream Actions"}</h3>
          <span class="text-2xl cursor-pointer" onClick=${s}>×</span>
        </div>
        
        <div class="p-6">
          ${d?a`
            <div class="mb-6">
              <div class="flex items-center justify-center mb-4 text-red-600 dark:text-red-500">
                <svg class="w-12 h-12" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                </svg>
              </div>
              <h4 class="text-lg font-medium mb-2 text-center">Are you sure you want to permanently delete "${t}"?</h4>
              <p class="text-gray-600 dark:text-gray-400 mb-4 text-center">
                This action cannot be undone. The stream configuration will be permanently removed from the database.
              </p>
            </div>
            
            <div class="flex justify-center space-x-3">
              <button 
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                onClick=${()=>i(!1)}
              >
                Cancel
              </button>
              <button 
                class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                onClick=${()=>{n(e),s()}}
              >
                Yes, Delete Permanently
              </button>
            </div>
          `:a`
            <div class="mb-6">
              <h4 class="text-lg font-medium mb-2">What would you like to do with "${t}"?</h4>
              <p class="text-gray-600 dark:text-gray-400 mb-4">
                Please choose one of the following options:
              </p>
              
              <div class="space-y-4">
                <div class="p-4 border border-yellow-200 bg-yellow-50 dark:bg-yellow-900/20 dark:border-yellow-900/30 rounded-lg">
                  <h5 class="font-medium text-yellow-800 dark:text-yellow-500 mb-2">Disable Stream (Soft Delete)</h5>
                  <p class="text-gray-600 dark:text-gray-400 mb-2">
                    This option will disable the stream but keep its configuration in the database. You can re-enable it later.
                  </p>
                  <ul class="list-disc list-inside text-sm text-gray-600 dark:text-gray-400 mb-3">
                    <li>Stream will stop processing</li>
                    <li>Live streaming will be disabled</li>
                    <li>Recording will be disabled</li>
                    <li>Audio recording will be disabled</li>
                    <li>Detection-based recording will be disabled</li>
                    <li>Configuration is preserved</li>
                    <li>Existing recordings are kept</li>
                    <li>Can be re-enabled later</li>
                  </ul>
                  <button 
                    class="w-full px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                    onClick=${()=>{o(e),s()}}
                  >
                    Disable Stream
                  </button>
                </div>
                
                <div class="p-4 border border-red-200 bg-red-50 dark:bg-red-900/20 dark:border-red-900/30 rounded-lg">
                  <h5 class="font-medium text-red-800 dark:text-red-500 mb-2">Delete Stream (Permanent)</h5>
                  <p class="text-gray-600 dark:text-gray-400 mb-2">
                    This option will permanently delete the stream configuration from the database. This action cannot be undone.
                  </p>
                  <ul class="list-disc list-inside text-sm text-gray-600 dark:text-gray-400 mb-3">
                    <li>Stream will be completely removed</li>
                    <li>Configuration is deleted</li>
                    <li>Recordings remain accessible</li>
                    <li>Cannot be recovered</li>
                  </ul>
                  <button 
                    class="w-full px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                    onClick=${()=>{i(!0)}}
                  >
                    Delete Stream
                  </button>
                </div>
              </div>
            </div>
          `}
        </div>
      </div>
    </div>
  `}function p(){const[e,t]=r([]),[n,p]=r(!1),[m,b]=r(!1),[y,f]=r([]),[h,x]=r([]),[v,w]=r(null),[k,$]=r(null),[C,_]=r(!1),[E,I]=r({username:"",password:""}),[S,D]=r({name:"",url:"",enabled:!0,streamingEnabled:!0,width:1280,height:720,fps:15,codec:"h264",protocol:"0",priority:"5",segment:30,record:!0,detectionEnabled:!1,detectionModel:"",detectionThreshold:50,detectionInterval:10,preBuffer:10,postBuffer:30}),[N,O]=r([]),[T,V]=r(!1),[j,A]=r(!1),[F,P]=r(null),[B,M]=r(!1),[R,U]=r(!1),z=s(null);o((()=>(z.current=u(),L(),H(),()=>{z.current&&z.current.abort()})),[]);const L=async()=>{try{M(!0),U(!1);const e=await l("/api/streams",{signal:z.current?.signal,timeout:15e3,retries:2,retryDelay:1e3})||[];t(e),U(e.length>0)}catch(e){"Request was cancelled"!==e.message&&(console.error("Error loading streams:",e),d("Error loading streams: "+e.message),U(!1))}finally{M(!1)}},H=async()=>{try{const e=await l("/api/detection/models",{signal:z.current?.signal,timeout:1e4,retries:1,retryDelay:500});O(e.models||[])}catch(e){"Request was cancelled"!==e.message&&console.error("Error loading detection models:",e)}},J=()=>{p(!1)},q=e=>{const{name:t,value:r,type:a,checked:s}=e.target;D((e=>({...e,[t]:"checkbox"===a?s:r})))},X=e=>{const{name:t,value:r}=e.target;I((e=>({...e,[t]:r})))},[W,Y]=r(!1),[G,K]=r(""),[Q,Z]=r(!1),[ee,te]=r(!1),re=async e=>{try{const t={username:E.username,password:E.password};d("Testing ONVIF connection..."),await c("/api/onvif/device/test",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({url:e.device_service,username:t.username,password:t.password}),signal:z.current?.signal,timeout:15e3,retries:1,retryDelay:2e3}),d("ONVIF connection successful!"),(async e=>{try{w(e),x([]),Y(!0),d("Getting device profiles...");const t=await l("/api/onvif/device/profiles",{headers:{"X-Device-URL":e.device_service,"X-Username":E.username,"X-Password":E.password},signal:z.current?.signal,timeout:2e4,retries:1,retryDelay:2e3});x(t.profiles||[]),t.profiles&&t.profiles.length>0?d(`Found ${t.profiles.length} profiles`):d("No profiles found")}catch(t){console.error("Error getting device profiles:",t),d("Error getting device profiles: "+t.message,3e3,"error")}finally{Y(!1)}})(e)}catch(t){console.error("Error testing ONVIF connection:",t),d("Error testing ONVIF connection: "+t.message,3e3,"error")}};return a`
    <section id="streams-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Streams</h2>
        <div class="controls flex space-x-2">
          <button 
            id="discover-onvif-btn" 
            class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${()=>b(!0)}
          >
            Discover ONVIF Cameras
          </button>
          <button 
            id="add-stream-btn" 
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${()=>{D({name:"",url:"",enabled:!0,streamingEnabled:!0,width:1280,height:720,fps:15,codec:"h264",protocol:"0",priority:"5",segment:30,record:!0,recordAudio:!0,detectionEnabled:!1,detectionModel:"",detectionThreshold:50,detectionInterval:10,preBuffer:10,postBuffer:30}),V(!1),p(!0)}}
          >
            Add Stream
          </button>
        </div>
      </div>
      
      <${i}
        isLoading=${B}
        hasData=${R}
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
                ${e.map((e=>a`
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
                          onClick=${()=>(async e=>{try{const t=await l(`/api/streams/${encodeURIComponent(e)}`,{signal:z.current?.signal,timeout:1e4,retries:2,retryDelay:1e3});D({...t,width:t.width||1280,height:t.height||720,fps:t.fps||15,protocol:t.protocol?.toString()||"0",priority:t.priority?.toString()||"5",segment:t.segment_duration||30,detectionThreshold:t.detection_threshold||50,detectionInterval:t.detection_interval||10,preBuffer:t.pre_detection_buffer||10,postBuffer:t.post_detection_buffer||30,streamingEnabled:void 0===t.streaming_enabled||t.streaming_enabled,isOnvif:void 0!==t.is_onvif&&t.is_onvif,detectionEnabled:t.detection_based_recording||!1,detectionModel:t.detection_model||"",recordAudio:void 0===t.record_audio||t.record_audio}),V(!0),p(!0)}catch(t){console.error("Error loading stream details:",t),d("Error loading stream details: "+t.message)}})(e.name)}
                          title="Edit"
                        >
                          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                          </svg>
                        </button>
                        <button 
                          class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                          onClick=${()=>(e=>{P(e),A(!0)})(e)}
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
      
      ${j&&F&&a`
        <${g}
          streamId=${F.name}
          streamName=${F.name}
          onClose=${()=>{A(!1),P(null)}}
          onDisable=${async e=>{try{await l(`/api/streams/${encodeURIComponent(e)}`,{signal:z.current?.signal,timeout:1e4,retries:1,retryDelay:1e3}),await c(`/api/streams/${encodeURIComponent(e)}`,{method:"PUT",headers:{"Content-Type":"application/json"},body:JSON.stringify({enabled:!1,streaming_enabled:!1,record:!1,record_audio:!1,detection_based_recording:!1}),signal:z.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),d("Stream disabled successfully"),L()}catch(t){console.error("Error disabling stream:",t),d("Error disabling stream: "+t.message)}}}
          onDelete=${async e=>{try{await c(`/api/streams/${encodeURIComponent(e)}?permanent=true`,{method:"DELETE",signal:z.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),d("Stream deleted permanently"),L()}catch(t){console.error("Error deleting stream:",t),d("Error deleting stream: "+t.message)}}}
        />
      `}
      
      ${n&&a`
        <div id="stream-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">${T?"Edit Stream":"Add Stream"}</h3>
              <span class="text-2xl cursor-pointer" onClick=${J}>×</span>
            </div>
            <div class="p-4">
              <form id="stream-form" class="space-y-4">
                <div class="form-group">
                  <label for="stream-name" class="block text-sm font-medium mb-1">Name</label>
                  <input 
                    type="text" 
                    id="stream-name" 
                    name="name"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${T?"bg-gray-100 dark:bg-gray-800":""}"
                    value=${S.name}
                    onChange=${q}
                    disabled=${T}
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
                    value=${S.url}
                    onChange=${q}
                    required
                  />
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-enabled" 
                    name="enabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${S.enabled}
                    onChange=${q}
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
                    checked=${S.streamingEnabled}
                    onChange=${q}
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
                    checked=${S.isOnvif}
                    onChange=${q}
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
                      value=${S.width}
                      onChange=${q}
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
                      value=${S.height}
                      onChange=${q}
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
                      value=${S.fps}
                      onChange=${q}
                    />
                  </div>
                  <div class="form-group">
                    <label for="stream-codec" class="block text-sm font-medium mb-1">Codec</label>
                    <select 
                      id="stream-codec" 
                      name="codec"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${S.codec}
                      onChange=${q}
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
                      value=${S.protocol}
                      onChange=${q}
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
                      value=${S.priority}
                      onChange=${q}
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
                      value=${S.segment}
                      onChange=${q}
                    />
                  </div>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-record" 
                    name="record"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${S.record}
                    onChange=${q}
                  />
                  <label for="stream-record" class="ml-2 block text-sm">Record</label>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-record-audio" 
                    name="recordAudio"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${S.recordAudio}
                    onChange=${q}
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
                    checked=${S.detectionEnabled}
                    onChange=${q}
                  />
                  <label for="stream-detection-enabled" class="ml-2 block text-sm">Enable Detection-Based Recording</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Only record when objects are detected</span>
                </div>
                <div class="form-group" style=${S.detectionEnabled?"":"display: none"}>
                  <label for="stream-detection-model" class="block text-sm font-medium mb-1">Detection Model</label>
                  <div class="flex space-x-2">
                    <select 
                      id="stream-detection-model" 
                      name="detectionModel"
                      class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${S.detectionModel}
                      onChange=${q}
                    >
                      <option value="">Select a model</option>
                      ${N.map((e=>a`
                        <option key=${e.id} value=${e.id}>${e.name}</option>
                      `))}
                    </select>
                    <button 
                      id="refresh-models-btn" 
                      class="p-2 rounded-md bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
                      title="Refresh Models"
                      onClick=${H}
                      type="button"
                    >
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                <div class="form-group" style=${S.detectionEnabled?"":"display: none"}>
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
                      value=${S.detectionThreshold}
                      onInput=${e=>{const t=parseInt(e.target.value,10);D((e=>({...e,detectionThreshold:t})))}}
                    />
                    <span id="stream-threshold-value" class="font-medium text-blue-600 dark:text-blue-400 min-w-[3rem] text-center">
                      ${S.detectionThreshold}%
                    </span>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4" style=${S.detectionEnabled?"":"display: none"}>
                  <div class="form-group">
                    <label for="stream-detection-interval" class="block text-sm font-medium mb-1">Detection Interval (frames)</label>
                    <input 
                      type="number" 
                      id="stream-detection-interval" 
                      name="detectionInterval"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="1" 
                      max="100" 
                      value=${S.detectionInterval}
                      onChange=${q}
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
                      value=${S.preBuffer}
                      onChange=${q}
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
                      value=${S.postBuffer}
                      onChange=${q}
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
                onClick=${async()=>{try{d("Testing stream connection...");const e=await l("/api/streams/test",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({url:S.url,protocol:parseInt(S.protocol,10)}),signal:z.current?.signal,timeout:2e4,retries:1,retryDelay:2e3});e.success?(d("Stream connection successful!"),e.info&&D((t=>({...t,width:e.info.width||t.width,height:e.info.height||t.height,fps:e.info.fps||t.fps,codec:e.info.codec||t.codec})))):d(`Stream test failed: ${e.message||"Unknown error"}`,3e3,"error")}catch(e){console.error("Error testing stream:",e),d("Error testing stream: "+e.message,3e3,"error")}}}
                type="button"
              >
                Test Connection
              </button>
              <div class="space-x-2">
                <button 
                  id="stream-save-btn" 
                  class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                  onClick=${async()=>{try{const e={...S,width:parseInt(S.width,10),height:parseInt(S.height,10),fps:parseInt(S.fps,10),protocol:parseInt(S.protocol,10),priority:parseInt(S.priority,10),segment_duration:parseInt(S.segment,10),streaming_enabled:S.streamingEnabled,is_onvif:S.isOnvif,detection_based_recording:S.detectionEnabled,detection_model:S.detectionModel,detection_threshold:S.detectionThreshold,detection_interval:parseInt(S.detectionInterval,10),pre_detection_buffer:parseInt(S.preBuffer,10),post_detection_buffer:parseInt(S.postBuffer,10),record_audio:S.recordAudio};T&&(e.is_deleted=!1);const t=T?`/api/streams/${encodeURIComponent(S.name)}`:"/api/streams",r=T?"PUT":"POST";await c(t,{method:r,headers:{"Content-Type":"application/json"},body:JSON.stringify(e),signal:z.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),d(`Stream ${T?"updated":"added"} successfully`),J(),L()}catch(e){console.error(`Error ${T?"updating":"adding"} stream:`,e),d(`Error ${T?"updating":"adding"} stream: ${e.message}`)}}}
                  type="button"
                >
                  Save
                </button>
                <button 
                  id="stream-cancel-btn" 
                  class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                  onClick=${J}
                  type="button"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      `}
      
      ${m&&a`
        <div id="onvif-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">ONVIF Camera Discovery</h3>
              <span class="text-2xl cursor-pointer" onClick=${()=>b(!1)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4 flex justify-between items-center">
                <h4 class="text-md font-medium">Discovered Devices</h4>
                <button 
                  id="discover-btn" 
                  class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                  onClick=${async()=>{try{_(!0),d("Starting ONVIF discovery...");const e=await l("/api/onvif/discovery/discover",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({}),signal:z.current?.signal,timeout:12e4,retries:0});f(e.devices||[]),e.devices&&e.devices.length>0?d(`Discovered ${e.devices.length} ONVIF devices`):d("No ONVIF devices found")}catch(e){console.error("Error discovering ONVIF devices:",e),d("Error discovering ONVIF devices: "+e.message,3e3,"error")}finally{_(!1)}}}
                  disabled=${C}
                  type="button"
                >
                  ${C?a`
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
                    ${0===y.length?a`
                      <tr>
                        <td colspan="4" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                          ${C?a`
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
                    `:y.map((e=>a`
                      <tr key=${e.ip_address} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td class="px-6 py-4 whitespace-nowrap">${e.ip_address}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${e.manufacturer||"Unknown"}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${e.model||"Unknown"}</td>
                        <td class="px-6 py-4 whitespace-nowrap">
                          <button 
                            class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none"
                            onClick=${()=>re(e)}
                            disabled=${W&&v&&v.ip_address===e.ip_address}
                            type="button"
                          >
                            ${W&&v&&v.ip_address===e.ip_address?a`
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
                      onChange=${X}
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
                      onChange=${X}
                    />
                  </div>
                </div>
              </div>
              
              ${v&&h.length>0&&a`
                <div class="mt-6">
                  <h4 class="text-md font-medium mb-2">Available Profiles for ${v.ip_address}</h4>
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
                        ${h.map((e=>a`
                          <tr key=${e.token} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                            <td class="px-6 py-4 whitespace-nowrap">${e.name}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${e.width}x${e.height}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${e.encoding}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${e.fps}</td>
                            <td class="px-6 py-4 whitespace-nowrap">
                              <button 
                                class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                                onClick=${()=>(async e=>{try{$(e);const t=`ONVIF_${v.ip_address.replace(/\./g,"_")}_${e.name.replace(/\s+/g,"_")}`;K(t),Z(!0)}catch(t){console.error("Error preparing to add ONVIF device:",t),d("Error preparing to add ONVIF device: "+t.message,3e3,"error")}})(e)}
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
                onClick=${()=>b(!1)}
                type="button"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      `}
      
      ${Q&&a`
        <div id="custom-name-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">Stream Name</h3>
              <span class="text-2xl cursor-pointer" onClick=${()=>Z(!1)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4">
                <label for="custom-stream-name" class="block text-sm font-medium mb-1">Enter a name for this stream:</label>
                <input 
                  type="text" 
                  id="custom-stream-name" 
                  class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${G}
                  onChange=${e=>K(e.target.value)}
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  This name will be used to identify the stream in the system.
                </p>
              </div>
            </div>
            <div class="flex justify-end p-4 border-t border-gray-200 dark:border-gray-700 space-x-2">
              <button 
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                onClick=${()=>Z(!1)}
                type="button"
              >
                Cancel
              </button>
              <button 
                class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                onClick=${async()=>{try{te(!0),d("Adding ONVIF device as stream..."),await c("/api/onvif/device/add",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({device_url:v.device_service,profile_token:k.token,stream_name:G,username:E.username,password:E.password}),signal:z.current?.signal,timeout:3e4,retries:1,retryDelay:3e3}),d("ONVIF device added as stream successfully"),b(!1),Z(!1),L()}catch(e){console.error("Error adding ONVIF device as stream:",e),d("Error adding ONVIF device as stream: "+e.message,3e3,"error")}finally{te(!1)}}}
                type="button"
                disabled=${!G.trim()||ee}
              >
                ${ee?a`
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
  `}e({StreamsView:p,loadStreamsView:function(){const e=document.getElementById("main-content");e&&n((async()=>{const{render:e}=await t.import("./preact-app-legacy-kv_02ehl.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(a`<${p} />`,e)}))}})}}}));
//# sourceMappingURL=StreamsView-legacy-DgL93CBB.js.map
