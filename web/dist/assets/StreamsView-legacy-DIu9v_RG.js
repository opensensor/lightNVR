System.register(["./preact-app-legacy-D4p72QMe.js","./LoadingIndicator-legacy-Cyq5Ey9v.js"],(function(e,t){"use strict";var r,a,s,o,d,n,i,l,c,u;return{setters:[e=>{r=e.d,a=e.h,s=e.f,o=e.u,d=e.g,n=e.i,i=e._,l=e.s,c=e.j},e=>{u=e.C}],execute:function(){function m({streamId:e,streamName:t,onClose:s,onDisable:o,onDelete:d}){const[n,i]=r(!1);return a`
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
        <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-medium">${n?"Confirm Permanent Deletion":"Stream Actions"}</h3>
          <span class="text-2xl cursor-pointer" onClick=${s}>×</span>
        </div>
        
        <div class="p-6">
          ${n?a`
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
                onClick=${()=>{d(e),s()}}
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
  `}function g(){const e=s(),[t,i]=r(!1),[g,p]=r(!1),[b,f]=r(!1),[y,h]=r(!1),[x,v]=r([]),[w,k]=r([]),[$,C]=r(null),[E,S]=r(null),[_,D]=r(""),[I,A]=r({username:"",password:""}),[T,M]=r(!1),[P,j]=r(!1),{data:B=[],isLoading:N,error:R}=o(["streams"],"/api/streams",{timeout:1e4,retries:2,retryDelay:1e3}),{data:V}=o(["detectionModels"],"/api/detection/models",{timeout:5e3,retries:1,retryDelay:1e3}),F=Array.isArray(B)?B:B.streams||[];console.log("API Response:",B),console.log("Streams data:",F),console.log("hasData:",F&&F.length>0);const[L,z]=r({name:"",url:"",enabled:!0,streamingEnabled:!0,width:1280,height:720,fps:15,codec:"h264",protocol:"0",priority:"5",segment:30,record:!0,recordAudio:!0,detectionEnabled:!1,detectionModel:"",detectionThreshold:50,detectionInterval:10,preBuffer:10,postBuffer:30}),[O,U]=r(!1),[q,H]=r(!1),[Q,K]=r(null),X=F&&F.length>0,W=V?.models||[],Y=d("/api/streams",{timeout:15e3,retries:1,retryDelay:1e3},{onSuccess:()=>{l("Stream added successfully"),ae(),e.invalidateQueries({queryKey:["streams"]})},onError:e=>{l(`Error adding stream: ${e.message}`,5e3,"error")}}),G=n({mutationFn:async e=>{const{streamName:t,...r}=e,a=`/api/streams/${encodeURIComponent(t)}`;return await c(a,{method:"PUT",headers:{"Content-Type":"application/json"},body:JSON.stringify(r),timeout:15e3,retries:1,retryDelay:1e3})},onSuccess:()=>{l("Stream updated successfully"),ae(),e.invalidateQueries({queryKey:["streams"]})},onError:e=>{l(`Error updating stream: ${e.message}`,5e3,"error")}}),J=(e,t)=>{O?G.mutate({...e,streamName:e.name},t):Y.mutate(e,t)},Z=d("/api/streams/test",{timeout:2e4,retries:1,retryDelay:2e3},{onMutate:()=>{l("Testing stream connection...")},onSuccess:e=>{e.success?l("Stream connection successful!",3e3,"success"):l(`Stream connection failed: ${e.message}`,5e3,"error")},onError:e=>{l(`Error testing stream: ${e.message}`,5e3,"error")}}),ee=n({mutationFn:async e=>{const{streamId:t}=e,r=`/api/streams/${encodeURIComponent(t)}?permanent=true`;return await c(r,{method:"DELETE",timeout:15e3,retries:1,retryDelay:1e3})},onSuccess:()=>{l("Stream successfully deleted."),re(),e.invalidateQueries({queryKey:["streams"]})},onError:e=>{l(`Error deleting stream: ${e.message}`,5e3,"error")}}),te=n({mutationFn:async e=>{const{streamId:t}=e,r=`/api/streams/${encodeURIComponent(t)}`;return await c(r,{method:"DELETE",timeout:15e3,retries:1,retryDelay:1e3})},onSuccess:()=>{l("Stream successfully disabled."),re(),e.invalidateQueries({queryKey:["streams"]})},onError:e=>{l(`Error disabling stream: ${e.message}`,5e3,"error")}}),re=()=>{H(!1),K(null)},ae=()=>{i(!1)},se=e=>{const{name:t,value:r,type:a,checked:s}=e.target;z((e=>({...e,[t]:"checkbox"===a?s:r})))},oe=e=>{const{name:t,value:r}=e.target;A((e=>({...e,[t]:r})))},de=d("/api/onvif/discovery/discover",{timeout:12e4,retries:0},{onMutate:()=>{M(!0)},onSuccess:e=>{v(e.devices||[]),M(!1)},onError:e=>{l(`Error discovering ONVIF devices: ${e.message}`,5e3,"error"),M(!1)}}),ne=n({mutationFn:({device:e,credentials:t})=>(j(!0),fetch("/api/onvif/device/profiles",{method:"GET",headers:{"X-Device-URL":`http://${e.ip_address}/onvif/device_service`,"X-Username":t.username,"X-Password":t.password}}).then((e=>{if(!e.ok)throw new Error(`HTTP error ${e.status}`);return e.json()}))),onSuccess:e=>{k(e.profiles||[]),j(!1)},onError:e=>{l(`Error loading device profiles: ${e.message}`,5e3,"error"),j(!1)}}),ie=d("/api/onvif/device/test",{timeout:15e3,retries:0},{onMutate:()=>{j(!0)},onSuccess:(e,t)=>{e.success?(l("Connection successful!",3e3,"success"),$&&le($)):(l(`Connection failed: ${e.message}`,5e3,"error"),j(!1))},onError:e=>{l(`Error testing connection: ${e.message}`,5e3,"error"),j(!1)}}),le=e=>{C(e),k([]),ne.mutate({device:e,credentials:I})};return a`
    <section id="streams-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Streams</h2>
        <div class="controls flex space-x-2">
          <button
              id="discover-onvif-btn"
              class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${()=>p(!0)}
          >
            Discover ONVIF Cameras
          </button>
          <button
              id="add-stream-btn"
              class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${()=>{z({name:"",url:"",enabled:!0,streamingEnabled:!0,width:1280,height:720,fps:15,codec:"h264",protocol:"0",priority:"5",segment:30,record:!0,recordAudio:!0,detectionEnabled:!1,detectionModel:"",detectionThreshold:50,detectionInterval:10,preBuffer:10,postBuffer:30}),U(!1),i(!0)}}
          >
            Add Stream
          </button>
        </div>
      </div>

      <${u}
          isLoading=${N}
          hasData=${X}
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
              ${F.map((t=>a`
                <tr key=${t.name} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                      <span class=${"status-indicator w-2 h-2 rounded-full mr-2 "+(t.enabled?"bg-green-500":"bg-red-500")}></span>
                      ${t.name}
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">${t.url}</td>
                  <td class="px-6 py-4 whitespace-nowrap">${t.width}x${t.height}</td>
                  <td class="px-6 py-4 whitespace-nowrap">${t.fps}</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    ${t.record?"Enabled":"Disabled"}
                    ${t.detection_based_recording?" (Detection)":""}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex space-x-2">
                      <button
                          class="p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                          onClick=${()=>(async t=>{try{const r=await e.fetchQuery({queryKey:["stream",t],queryFn:async()=>{const e=await fetch(`/api/streams/${encodeURIComponent(t)}`);if(!e.ok)throw new Error(`HTTP error ${e.status}`);return e.json()},staleTime:1e4});z({...r,width:r.width||1280,height:r.height||720,fps:r.fps||15,protocol:r.protocol?.toString()||"0",priority:r.priority?.toString()||"5",segment:r.segment_duration||30,detectionThreshold:r.detection_threshold||50,detectionInterval:r.detection_interval||10,preBuffer:r.pre_detection_buffer||10,postBuffer:r.post_detection_buffer||30,streamingEnabled:void 0===r.streaming_enabled||r.streaming_enabled,isOnvif:void 0!==r.is_onvif&&r.is_onvif,detectionEnabled:r.detection_based_recording||!1,detectionModel:r.detection_model||"",recordAudio:void 0===r.record_audio||r.record_audio}),U(!0),i(!0)}catch(R){console.error("Error loading stream details:",R),l("Error loading stream details: "+R.message)}})(t.name)}
                          title="Edit"
                      >
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                          <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                        </svg>
                      </button>
                      <button
                          class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                          onClick=${()=>(e=>{K(e),H(!0)})(t)}
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

      ${q&&Q&&a`
        <${m}
            streamId=${Q.name}
            streamName=${Q.name}
            onClose=${re}
            onDisable=${e=>{te.mutate({streamId:e})}}
            onDelete=${e=>{ee.mutate({streamId:e})}}
        />
      `}

      ${t&&a`
        <div id="stream-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">${O?"Edit Stream":"Add Stream"}</h3>
              <span class="text-2xl cursor-pointer" onClick=${ae}>×</span>
            </div>
            <div class="p-4">
              <form id="stream-form" class="space-y-4">
                <div class="form-group">
                  <label for="stream-name" class="block text-sm font-medium mb-1">Name</label>
                  <input
                      type="text"
                      id="stream-name"
                      name="name"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${O?"bg-gray-100 dark:bg-gray-800":""}"
                      value=${L.name}
                      onChange=${se}
                      disabled=${O}
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
                      value=${L.url}
                      onChange=${se}
                      required
                  />
                </div>
                <div class="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-enabled"
                      name="enabled"
                      class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                      checked=${L.enabled}
                      onChange=${se}
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
                      checked=${L.streamingEnabled}
                      onChange=${se}
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
                      checked=${L.isOnvif}
                      onChange=${se}
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
                        value=${L.width}
                        onChange=${se}
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
                        value=${L.height}
                        onChange=${se}
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
                        value=${L.fps}
                        onChange=${se}
                    />
                  </div>
                  <div class="form-group">
                    <label for="stream-codec" class="block text-sm font-medium mb-1">Codec</label>
                    <select
                        id="stream-codec"
                        name="codec"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                        value=${L.codec}
                        onChange=${se}
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
                        value=${L.protocol}
                        onChange=${se}
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
                        value=${L.priority}
                        onChange=${se}
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
                        value=${L.segment}
                        onChange=${se}
                    />
                  </div>
                </div>
                <div class="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-record"
                      name="record"
                      class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                      checked=${L.record}
                      onChange=${se}
                  />
                  <label for="stream-record" class="ml-2 block text-sm">Record</label>
                </div>
                <div class="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-record-audio"
                      name="recordAudio"
                      class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                      checked=${L.recordAudio}
                      onChange=${se}
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
                      checked=${L.detectionEnabled}
                      onChange=${se}
                  />
                  <label for="stream-detection-enabled" class="ml-2 block text-sm">Enable Detection-Based Recording</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Only record when objects are detected</span>
                </div>
                <div class="form-group" style=${L.detectionEnabled?"":"display: none"}>
                  <label for="stream-detection-model" class="block text-sm font-medium mb-1">Detection Model</label>
                  <div class="flex space-x-2">
                    <select
                        id="stream-detection-model"
                        name="detectionModel"
                        class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                        value=${L.detectionModel}
                        onChange=${se}
                    >
                      <option value="">Select a model</option>
                      ${W.map((e=>a`
                        <option key=${e.id} value=${e.id}>${e.name}</option>
                      `))}
                    </select>
                    <button
                        id="refresh-models-btn"
                        class="p-2 rounded-md bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
                        title="Refresh Models"
                        onClick=${()=>{e.invalidateQueries({queryKey:["detectionModels"]})}}
                        type="button"
                    >
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                <div class="form-group" style=${L.detectionEnabled?"":"display: none"}>
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
                        value=${L.detectionThreshold}
                        onInput=${e=>{const t=e.target.value;z((e=>({...e,detectionThreshold:t})))}}
                    />
                    <span id="stream-threshold-value" class="font-medium text-blue-600 dark:text-blue-400 min-w-[3rem] text-center">
                      ${L.detectionThreshold}%
                    </span>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4" style=${L.detectionEnabled?"":"display: none"}>
                  <div class="form-group">
                    <label for="stream-detection-interval" class="block text-sm font-medium mb-1">Detection Interval (frames)</label>
                    <input
                        type="number"
                        id="stream-detection-interval"
                        name="detectionInterval"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                        min="1"
                        max="100"
                        value=${L.detectionInterval}
                        onChange=${se}
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
                        value=${L.preBuffer}
                        onChange=${se}
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
                        value=${L.postBuffer}
                        onChange=${se}
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
                  onClick=${()=>{Z.mutate({url:L.url,protocol:parseInt(L.protocol,10)})}}
                  type="button"
              >
                Test Connection
              </button>
              <div class="space-x-2">
                <button
                    id="stream-save-btn"
                    class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                    onClick=${e=>{(e=>{e.preventDefault();const t={name:L.name,url:L.url,enabled:L.enabled,streaming_enabled:L.streamingEnabled,width:parseInt(L.width,10),height:parseInt(L.height,10),fps:parseInt(L.fps,10),codec:L.codec,protocol:parseInt(L.protocol,10),priority:parseInt(L.priority,10),segment_duration:parseInt(L.segment,10),record:L.record,detection_based_recording:L.detectionEnabled,detection_model:L.detectionModel,detection_threshold:parseInt(L.detectionThreshold,10),detection_interval:parseInt(L.detectionInterval,10),pre_detection_buffer:parseInt(L.preBuffer,10),post_detection_buffer:parseInt(L.postBuffer,10),record_audio:L.recordAudio};O&&(t.is_deleted=!1),J(t)})(e)}}
                    type="button"
                >
                  Save
                </button>
                <button
                    id="stream-cancel-btn"
                    class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                    onClick=${ae}
                    type="button"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      `}

      ${g&&a`
        <div id="onvif-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">ONVIF Camera Discovery</h3>
              <span class="text-2xl cursor-pointer" onClick=${()=>p(!1)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4 flex justify-between items-center">
                <h4 class="text-md font-medium">Discovered Devices</h4>
                <button
                    id="discover-btn"
                    class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                    onClick=${()=>{de.mutate({})}}
                    disabled=${T}
                    type="button"
                >
                  ${T?a`
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
                  ${0===x.length?a`
                    <tr>
                      <td colspan="4" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                        ${T?a`
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
                  `:x.map((e=>a`
                    <tr key=${e.ip_address} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                      <td class="px-6 py-4 whitespace-nowrap">${e.ip_address}</td>
                      <td class="px-6 py-4 whitespace-nowrap">${e.manufacturer||"Unknown"}</td>
                      <td class="px-6 py-4 whitespace-nowrap">${e.model||"Unknown"}</td>
                      <td class="px-6 py-4 whitespace-nowrap">
                        <button
                            class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none"
                            onClick=${()=>(e=>{C(e),ie.mutate({url:`http://${e.ip_address}/onvif/device_service`,username:I.username,password:I.password})})(e)}
                            disabled=${P&&$&&$.ip_address===e.ip_address}
                            type="button"
                        >
                          ${P&&$&&$.ip_address===e.ip_address?a`
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
                        value=${I.username}
                        onChange=${oe}
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
                        value=${I.password}
                        onChange=${oe}
                    />
                  </div>
                </div>
              </div>

              ${$&&w.length>0&&a`
                <div class="mt-6">
                  <h4 class="text-md font-medium mb-2">Available Profiles for ${$.ip_address}</h4>
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
                      ${w.map((e=>a`
                        <tr key=${e.token} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                          <td class="px-6 py-4 whitespace-nowrap">${e.name}</td>
                          <td class="px-6 py-4 whitespace-nowrap">${e.width}x${e.height}</td>
                          <td class="px-6 py-4 whitespace-nowrap">${e.encoding}</td>
                          <td class="px-6 py-4 whitespace-nowrap">${e.fps}</td>
                          <td class="px-6 py-4 whitespace-nowrap">
                            <button
                                class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                                onClick=${()=>(e=>{S(e),D(`${$.name||"ONVIF"}_${e.name||"Stream"}`),f(!0)})(e)}
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
                  onClick=${()=>p(!1)}
                  type="button"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      `}

      ${b&&a`
        <div id="custom-name-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">Stream Name</h3>
              <span class="text-2xl cursor-pointer" onClick=${()=>f(!1)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4">
                <label for="custom-stream-name" class="block text-sm font-medium mb-1">Enter a name for this stream:</label>
                <input
                    type="text"
                    id="custom-stream-name"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                    value=${_}
                    onChange=${e=>D(e.target.value)}
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  This name will be used to identify the stream in the system.
                </p>
              </div>
            </div>
            <div class="flex justify-end p-4 border-t border-gray-200 dark:border-gray-700 space-x-2">
              <button
                  class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                  onClick=${()=>f(!1)}
                  type="button"
              >
                Cancel
              </button>
              <button
                  class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                  onClick=${()=>{if(!$||!E||!_.trim())return void l("Missing required information",5e3,"error");h(!0);const e={name:_.trim(),url:E.stream_uri,enabled:!0,streaming_enabled:!0,width:E.width,height:E.height,fps:E.fps||15,codec:"H264"===E.encoding?"h264":"h265",protocol:"0",priority:"5",segment_duration:30,record:!0,record_audio:!0,is_onvif:!0};console.log("Adding ONVIF stream with data:",e),J(e,{onSuccess:()=>{h(!1),f(!1),p(!1)},onError:()=>{h(!1)},isEditing:!1})}}
                  type="button"
                  disabled=${!_.trim()||y}
              >
                ${y?a`
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
  `}e({StreamsView:g,loadStreamsView:function(){const e=document.getElementById("main-content");e&&i((async()=>{const{render:e}=await t.import("./preact-app-legacy-D4p72QMe.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:r})=>{i((async()=>{const{QueryClientProvider:e,queryClient:r}=await t.import("./preact-app-legacy-D4p72QMe.js").then((e=>e.n));return{QueryClientProvider:e,queryClient:r}}),void 0,t.meta.url).then((({QueryClientProvider:t,queryClient:s})=>{r(a`<${t} client=${s}><${g} /></${t}>`,e)}))}))}})}}}));
//# sourceMappingURL=StreamsView-legacy-DIu9v_RG.js.map
