const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./preact-app-_8hsa7bb.js","../css/preact-app.css"])))=>i.map(i=>d[i]);
var m=Object.freeze,D=Object.defineProperty;var c=(e,s)=>m(D(e,"raw",{value:m(s||e.slice())}));import{d as C,u as z,i as B,y as L,h as w,_ as h,s as f,j as M}from"./preact-app-_8hsa7bb.js";import{C as I}from"./LoadingIndicator-Cb6tPvOW.js";var y;function E(){const[e,s]=C({logLevel:"2",storagePath:"/var/lib/lightnvr/recordings",storagePathHls:"",maxStorage:"0",retention:"30",autoDelete:!0,dbPath:"/var/lib/lightnvr/lightnvr.db",webPort:"8080",authEnabled:!0,username:"admin",password:"admin",webrtcDisabled:!1,bufferSize:"1024",useSwap:!0,swapSize:"128",detectionModelsPath:"",defaultDetectionThreshold:50,defaultPreBuffer:5,defaultPostBuffer:10}),{data:t,isLoading:o,error:b,refetch:k}=z(["settings"],"/api/settings",{timeout:15e3,retries:2,retryDelay:1e3}),x=B({mutationKey:["saveSettings"],mutationFn:async r=>await M("/api/settings",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(r),timeout:2e4,retries:1,retryDelay:2e3}),onSuccess:()=>{f("Settings saved successfully"),k()},onError:r=>{console.error("Error saving settings:",r),f("Error saving settings: ".concat(r.message))}});L(()=>{var r,d,i,n,l,g,u,p;if(t){console.log("Settings loaded:",t);const P={logLevel:((r=t.log_level)==null?void 0:r.toString())||"",storagePath:t.storage_path||"",storagePathHls:t.storage_path_hls||"",maxStorage:((d=t.max_storage_size)==null?void 0:d.toString())||"",retention:((i=t.retention_days)==null?void 0:i.toString())||"",autoDelete:t.auto_delete_oldest||!1,dbPath:t.db_path||"",webPort:((n=t.web_port)==null?void 0:n.toString())||"",authEnabled:t.web_auth_enabled||!1,username:t.web_username||"",password:t.web_password||"",webrtcDisabled:t.webrtc_disabled||!1,bufferSize:((l=t.buffer_size)==null?void 0:l.toString())||"",useSwap:t.use_swap||!1,swapSize:((g=t.swap_size)==null?void 0:g.toString())||"",detectionModelsPath:t.models_path||"",defaultDetectionThreshold:t.default_detection_threshold||50,defaultPreBuffer:((u=t.pre_detection_buffer)==null?void 0:u.toString())||"5",defaultPostBuffer:((p=t.post_detection_buffer)==null?void 0:p.toString())||"10"};s($=>({...$,...P}))}},[t]);const S=()=>{const r={log_level:parseInt(e.logLevel,10),storage_path:e.storagePath,storage_path_hls:e.storagePathHls,max_storage_size:parseInt(e.maxStorage,10),retention_days:parseInt(e.retention,10),auto_delete_oldest:e.autoDelete,db_path:e.dbPath,web_port:parseInt(e.webPort,10),web_auth_enabled:e.authEnabled,web_username:e.username,web_password:e.password,webrtc_disabled:e.webrtcDisabled,buffer_size:parseInt(e.bufferSize,10),use_swap:e.useSwap,swap_size:parseInt(e.swapSize,10),models_path:e.detectionModelsPath,default_detection_threshold:e.defaultDetectionThreshold,pre_detection_buffer:parseInt(e.defaultPreBuffer,10),post_detection_buffer:parseInt(e.defaultPostBuffer,10)};x.mutate(r)},a=r=>{const{name:d,value:i,type:n,checked:l}=r.target;s(g=>({...g,[d]:n==="checkbox"?l:i}))},_=r=>{const d=parseInt(r.target.value,10);s(i=>({...i,defaultDetectionThreshold:d}))};return w(y||(y=c(['\n    <section id="settings-page" class="page">\n      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">\n        <h2 class="text-xl font-bold">Settings</h2>\n        <div class="controls">\n          <button \n            id="save-settings-btn" \n            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"\n            onClick=',"\n          >\n            Save Settings\n          </button>\n        </div>\n      </div>\n      \n      <","\n        isLoading=","\n        hasData=",'\n        loadingMessage="Loading settings..."\n        emptyMessage="No settings available. Please try again later."\n      >\n        <div class="settings-container space-y-6">\n          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">\n            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">General Settings</h3>\n            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n              <label for="setting-log-level" class="font-medium">Log Level</label>\n              <select \n                id="setting-log-level" \n                name="logLevel"\n                class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n                value=',"\n                onChange=",'\n              >\n                <option value="0">Error</option>\n                <option value="1">Warning</option>\n                <option value="2">Info</option>\n                <option value="3">Debug</option>\n              </select>\n            </div>\n          </div>\n          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">\n          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Storage Settings</h3>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-storage-path" class="font-medium">Storage Path</label>\n            <input \n              type="text" \n              id="setting-storage-path" \n              name="storagePath"\n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-storage-path-hls" class="font-medium">HLS Storage Path</label>\n            <div class="col-span-2">\n              <input \n                type="text" \n                id="setting-storage-path-hls" \n                name="storagePathHls"\n                class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n                value=',"\n                onChange=",'\n              />\n              <span class="hint text-sm text-gray-500 dark:text-gray-400">Optional path for HLS segments. If not specified, Storage Path will be used.</span>\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-max-storage" class="font-medium">Maximum Storage Size (GB)</label>\n            <div class="col-span-2 flex items-center">\n              <input \n                type="number" \n                id="setting-max-storage" \n                name="maxStorage"\n                min="0" \n                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n                value=',"\n                onChange=",'\n              />\n              <span class="hint ml-2 text-sm text-gray-500 dark:text-gray-400">0 = unlimited</span>\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-retention" class="font-medium">Retention Period (days)</label>\n            <input \n              type="number" \n              id="setting-retention" \n              name="retention"\n              min="1" \n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-auto-delete" class="font-medium">Auto Delete Oldest</label>\n            <div class="col-span-2">\n              <input \n                type="checkbox" \n                id="setting-auto-delete" \n                name="autoDelete"\n                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"\n                checked=',"\n                onChange=",'\n              />\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-db-path" class="font-medium">Database Path</label>\n            <input \n              type="text" \n              id="setting-db-path" \n              name="dbPath"\n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          </div>\n          \n          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">\n          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Web Interface Settings</h3>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-web-port" class="font-medium">Web Port</label>\n            <input \n              type="number" \n              id="setting-web-port" \n              name="webPort"\n              min="1" \n              max="65535" \n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-auth-enabled" class="font-medium">Enable Authentication</label>\n            <div class="col-span-2">\n              <input \n                type="checkbox" \n                id="setting-auth-enabled" \n                name="authEnabled"\n                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"\n                checked=',"\n                onChange=",'\n              />\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-username" class="font-medium">Username</label>\n            <input \n              type="text" \n              id="setting-username" \n              name="username"\n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-password" class="font-medium">Password</label>\n            <input \n              type="password" \n              id="setting-password" \n              name="password"\n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-webrtc-disabled" class="font-medium">Disable WebRTC (Use HLS Only)</label>\n            <div class="col-span-2">\n              <input \n                type="checkbox" \n                id="setting-webrtc-disabled" \n                name="webrtcDisabled"\n                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"\n                checked=',"\n                onChange=",'\n              />\n              <span class="hint ml-2 text-sm text-gray-500 dark:text-gray-400">When enabled, all streams will use HLS instead of WebRTC</span>\n            </div>\n          </div>\n          </div>\n          \n          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">\n          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory Optimization</h3>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-buffer-size" class="font-medium">Buffer Size (KB)</label>\n            <input \n              type="number" \n              id="setting-buffer-size" \n              name="bufferSize"\n              min="128" \n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-use-swap" class="font-medium">Use Swap File</label>\n            <div class="col-span-2">\n              <input \n                type="checkbox" \n                id="setting-use-swap" \n                name="useSwap"\n                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"\n                checked=',"\n                onChange=",'\n              />\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-swap-size" class="font-medium">Swap Size (MB)</label>\n            <input \n              type="number" \n              id="setting-swap-size" \n              name="swapSize"\n              min="32" \n              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n              value=',"\n              onChange=",'\n            />\n          </div>\n          </div>\n          \n          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">\n          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Detection-Based Recording</h3>\n          <div class="setting mb-4">\n            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">\n              Configure detection-based recording for streams. When enabled, recordings will only be saved when objects are detected.\n            </p>\n            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">\n              <strong>Motion Detection:</strong> Built-in motion detection is available without requiring any external models. \n              Select "motion" as the detection model in stream settings to use this feature.\n            </p>\n            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">\n              <strong>Optimized Motion Detection:</strong> Memory and CPU optimized motion detection for embedded devices.\n              Select "motion" as the detection model in stream settings to use this feature.\n            </p>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-detection-models-path" class="font-medium">Detection Models Path</label>\n            <div class="col-span-2">\n              <input \n                type="text" \n                id="setting-detection-models-path" \n                name="detectionModelsPath"\n                placeholder="/var/lib/lightnvr/models" \n                class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n                value=',"\n                onChange=",'\n              />\n              <span class="hint text-sm text-gray-500 dark:text-gray-400">Directory where detection models are stored</span>\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-default-detection-threshold" class="font-medium">Default Detection Threshold</label>\n            <div class="col-span-2">\n              <div class="flex items-center">\n                <input \n                  type="range" \n                  id="setting-default-detection-threshold" \n                  name="defaultDetectionThreshold"\n                  min="0" \n                  max="100" \n                  step="1" \n                  class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"\n                  value=',"\n                  onChange=",'\n                />\n                <span id="threshold-value" class="ml-2 min-w-[3rem] text-center">','%</span>\n              </div>\n              <span class="hint text-sm text-gray-500 dark:text-gray-400">Confidence threshold for detection (0-100%)</span>\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-default-pre-buffer" class="font-medium">Default Pre-detection Buffer (seconds)</label>\n            <div class="col-span-2">\n              <input \n                type="number" \n                id="setting-default-pre-buffer" \n                name="defaultPreBuffer"\n                min="0" \n                max="60" \n                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n                value=',"\n                onChange=",'\n              />\n              <span class="hint text-sm text-gray-500 dark:text-gray-400">Seconds of video to keep before detection</span>\n            </div>\n          </div>\n          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">\n            <label for="setting-default-post-buffer" class="font-medium">Default Post-detection Buffer (seconds)</label>\n            <div class="col-span-2">\n              <input \n                type="number" \n                id="setting-default-post-buffer" \n                name="defaultPostBuffer"\n                min="0" \n                max="300" \n                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"\n                value=',"\n                onChange=",'\n              />\n              <span class="hint text-sm text-gray-500 dark:text-gray-400">Seconds of video to keep after detection</span>\n            </div>\n          </div>\n          </div>\n        </div>\n      <//>\n    </section>\n  '])),S,I,o,!!t,e.logLevel,a,e.storagePath,a,e.storagePathHls,a,e.maxStorage,a,e.retention,a,e.autoDelete,a,e.dbPath,a,e.webPort,a,e.authEnabled,a,e.username,a,e.password,a,e.webrtcDisabled,a,e.bufferSize,a,e.useSwap,a,e.swapSize,a,e.detectionModelsPath,a,e.defaultDetectionThreshold,_,e.defaultDetectionThreshold,e.defaultPreBuffer,a,e.defaultPostBuffer,a)}var v;function W(){const e=document.getElementById("main-content");e&&h(async()=>{const{render:s}=await import("./preact-app-_8hsa7bb.js").then(t=>t.p);return{render:s}},__vite__mapDeps([0,1]),import.meta.url).then(({render:s})=>{h(async()=>{const{QueryClientProvider:t,queryClient:o}=await import("./preact-app-_8hsa7bb.js").then(b=>b.n);return{QueryClientProvider:t,queryClient:o}},__vite__mapDeps([0,1]),import.meta.url).then(({QueryClientProvider:t,queryClient:o})=>{s(w(v||(v=c(["<"," client=","><"," /></",">"])),t,o,E,t),e)})})}export{E as SettingsView,W as loadSettingsView};
//# sourceMappingURL=SettingsView-vnwcaLDa.js.map
