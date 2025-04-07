System.register(["./preact-app-legacy-CilCHnTM.js","./LoadingIndicator-legacy-BzphrVNR.js","./fetch-utils-legacy-DOz1-Xee.js"],(function(e,t){"use strict";var a,r,s,d,o,i,n,l,g,c;return{setters:[e=>{a=e.d,r=e.A,s=e.y,d=e.h,o=e._,i=e.c},e=>{n=e.C},e=>{l=e.e,g=e.c,c=e.f}],execute:function(){function b(){const[e,t]=a({logLevel:"2",storagePath:"/var/lib/lightnvr/recordings",storagePathHls:"",maxStorage:"0",retention:"30",autoDelete:!0,dbPath:"/var/lib/lightnvr/lightnvr.db",webPort:"8080",authEnabled:!0,username:"admin",password:"admin",bufferSize:"1024",useSwap:!0,swapSize:"128",detectionModelsPath:"",defaultDetectionThreshold:50,defaultPreBuffer:5,defaultPostBuffer:10}),[o,b]=a(!0),[u,p]=a(!1),m=r(null);s((()=>(m.current=g(),f(),()=>{m.current&&m.current.abort()})),[]);const f=async()=>{try{b(!0);const e=await c("/api/settings",{signal:m.current?.signal,timeout:15e3,retries:2,retryDelay:1e3});console.log("Settings loaded:",e);const a={logLevel:e.log_level?.toString()||"",storagePath:e.storage_path||"",storagePathHls:e.storage_path_hls||"",maxStorage:e.max_storage_size?.toString()||"",retention:e.retention_days?.toString()||"",autoDelete:e.auto_delete_oldest||!1,dbPath:e.db_path||"",webPort:e.web_port?.toString()||"",authEnabled:e.web_auth_enabled||!1,username:e.web_username||"",password:e.web_password||"",bufferSize:e.buffer_size?.toString()||"",useSwap:e.use_swap||!1,swapSize:e.swap_size?.toString()||"",detectionModelsPath:e.models_path||"",defaultDetectionThreshold:e.default_detection_threshold||50,defaultPreBuffer:e.pre_detection_buffer?.toString()||"5",defaultPostBuffer:e.post_detection_buffer?.toString()||"10"};t((e=>({...e,...a}))),p(!0)}catch(e){console.error("Error loading settings:",e),i("Error loading settings: "+e.message),p(!1)}finally{b(!1)}},h=e=>{const{name:a,value:r,type:s,checked:d}=e.target;t((e=>({...e,[a]:"checkbox"===s?d:r})))};return d`
    <section id="settings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Settings</h2>
        <div class="controls">
          <button 
            id="save-settings-btn" 
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${async()=>{try{const t={log_level:parseInt(e.logLevel,10),storage_path:e.storagePath,storage_path_hls:e.storagePathHls,max_storage_size:parseInt(e.maxStorage,10),retention_days:parseInt(e.retention,10),auto_delete_oldest:e.autoDelete,db_path:e.dbPath,web_port:parseInt(e.webPort,10),web_auth_enabled:e.authEnabled,web_username:e.username,web_password:e.password,buffer_size:parseInt(e.bufferSize,10),use_swap:e.useSwap,swap_size:parseInt(e.swapSize,10),models_path:e.detectionModelsPath,default_detection_threshold:e.defaultDetectionThreshold,pre_detection_buffer:parseInt(e.defaultPreBuffer,10),post_detection_buffer:parseInt(e.defaultPostBuffer,10)};await l("/api/settings",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(t),signal:m.current?.signal,timeout:2e4,retries:1,retryDelay:2e3}),i("Settings saved successfully")}catch(t){console.error("Error saving settings:",t),i("Error saving settings: "+t.message)}}}
          >
            Save Settings
          </button>
        </div>
      </div>
      
      <${n}
        isLoading=${o}
        hasData=${u}
        loadingMessage="Loading settings..."
        emptyMessage="No settings available. Please try again later."
      >
        <div class="settings-container space-y-6">
          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">General Settings</h3>
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-log-level" class="font-medium">Log Level</label>
              <select 
                id="setting-log-level" 
                name="logLevel"
                class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${e.logLevel}
                onChange=${h}
              >
                <option value="0">Error</option>
                <option value="1">Warning</option>
                <option value="2">Info</option>
                <option value="3">Debug</option>
              </select>
            </div>
          </div>
          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Storage Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path" class="font-medium">Storage Path</label>
            <input 
              type="text" 
              id="setting-storage-path" 
              name="storagePath"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.storagePath}
              onChange=${h}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path-hls" class="font-medium">HLS Storage Path</label>
            <div class="col-span-2">
              <input 
                type="text" 
                id="setting-storage-path-hls" 
                name="storagePathHls"
                class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${e.storagePathHls}
                onChange=${h}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Optional path for HLS segments. If not specified, Storage Path will be used.</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-max-storage" class="font-medium">Maximum Storage Size (GB)</label>
            <div class="col-span-2 flex items-center">
              <input 
                type="number" 
                id="setting-max-storage" 
                name="maxStorage"
                min="0" 
                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${e.maxStorage}
                onChange=${h}
              />
              <span class="hint ml-2 text-sm text-gray-500 dark:text-gray-400">0 = unlimited</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-retention" class="font-medium">Retention Period (days)</label>
            <input 
              type="number" 
              id="setting-retention" 
              name="retention"
              min="1" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.retention}
              onChange=${h}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auto-delete" class="font-medium">Auto Delete Oldest</label>
            <div class="col-span-2">
              <input 
                type="checkbox" 
                id="setting-auto-delete" 
                name="autoDelete"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"
                checked=${e.autoDelete}
                onChange=${h}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-db-path" class="font-medium">Database Path</label>
            <input 
              type="text" 
              id="setting-db-path" 
              name="dbPath"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.dbPath}
              onChange=${h}
            />
          </div>
          </div>
          
          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Web Interface Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-web-port" class="font-medium">Web Port</label>
            <input 
              type="number" 
              id="setting-web-port" 
              name="webPort"
              min="1" 
              max="65535" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.webPort}
              onChange=${h}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auth-enabled" class="font-medium">Enable Authentication</label>
            <div class="col-span-2">
              <input 
                type="checkbox" 
                id="setting-auth-enabled" 
                name="authEnabled"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"
                checked=${e.authEnabled}
                onChange=${h}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-username" class="font-medium">Username</label>
            <input 
              type="text" 
              id="setting-username" 
              name="username"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.username}
              onChange=${h}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-password" class="font-medium">Password</label>
            <input 
              type="password" 
              id="setting-password" 
              name="password"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.password}
              onChange=${h}
            />
          </div>
          </div>
          
          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory Optimization</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-buffer-size" class="font-medium">Buffer Size (KB)</label>
            <input 
              type="number" 
              id="setting-buffer-size" 
              name="bufferSize"
              min="128" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.bufferSize}
              onChange=${h}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-use-swap" class="font-medium">Use Swap File</label>
            <div class="col-span-2">
              <input 
                type="checkbox" 
                id="setting-use-swap" 
                name="useSwap"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"
                checked=${e.useSwap}
                onChange=${h}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-swap-size" class="font-medium">Swap Size (MB)</label>
            <input 
              type="number" 
              id="setting-swap-size" 
              name="swapSize"
              min="32" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${e.swapSize}
              onChange=${h}
            />
          </div>
          </div>
          
          <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Detection-Based Recording</h3>
          <div class="setting mb-4">
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              Configure detection-based recording for streams. When enabled, recordings will only be saved when objects are detected.
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>Motion Detection:</strong> Built-in motion detection is available without requiring any external models. 
              Select "motion" as the detection model in stream settings to use this feature.
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>Optimized Motion Detection:</strong> Memory and CPU optimized motion detection for embedded devices.
              Select "motion" as the detection model in stream settings to use this feature.
            </p>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-detection-models-path" class="font-medium">Detection Models Path</label>
            <div class="col-span-2">
              <input 
                type="text" 
                id="setting-detection-models-path" 
                name="detectionModelsPath"
                placeholder="/var/lib/lightnvr/models" 
                class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${e.detectionModelsPath}
                onChange=${h}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Directory where detection models are stored</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-detection-threshold" class="font-medium">Default Detection Threshold</label>
            <div class="col-span-2">
              <div class="flex items-center">
                <input 
                  type="range" 
                  id="setting-default-detection-threshold" 
                  name="defaultDetectionThreshold"
                  min="0" 
                  max="100" 
                  step="1" 
                  class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                  value=${e.defaultDetectionThreshold}
                  onChange=${e=>{const a=parseInt(e.target.value,10);t((e=>({...e,defaultDetectionThreshold:a})))}}
                />
                <span id="threshold-value" class="ml-2 min-w-[3rem] text-center">${e.defaultDetectionThreshold}%</span>
              </div>
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Confidence threshold for detection (0-100%)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-pre-buffer" class="font-medium">Default Pre-detection Buffer (seconds)</label>
            <div class="col-span-2">
              <input 
                type="number" 
                id="setting-default-pre-buffer" 
                name="defaultPreBuffer"
                min="0" 
                max="60" 
                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${e.defaultPreBuffer}
                onChange=${h}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Seconds of video to keep before detection</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-post-buffer" class="font-medium">Default Post-detection Buffer (seconds)</label>
            <div class="col-span-2">
              <input 
                type="number" 
                id="setting-default-post-buffer" 
                name="defaultPostBuffer"
                min="0" 
                max="300" 
                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${e.defaultPostBuffer}
                onChange=${h}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Seconds of video to keep after detection</span>
            </div>
          </div>
          </div>
        </div>
      <//>
    </section>
  `}e({SettingsView:b,loadSettingsView:function(){const e=document.getElementById("main-content");e&&o((async()=>{const{render:e}=await t.import("./preact-app-legacy-CilCHnTM.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(d`<${b} />`,e)}))}})}}}));
//# sourceMappingURL=SettingsView-legacy-Ckkat3ML.js.map
