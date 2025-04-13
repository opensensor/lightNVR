System.register(["./preact-app-legacy-C9dbqWYg.js","./LoadingIndicator-legacy-hA5eHV8R.js"],(function(e,t){"use strict";var s,o,n,r,l,i,a,d,g,c;return{setters:[e=>{s=e.h,o=e.d,n=e.A,r=e.y,l=e.u,i=e.i,a=e._,d=e.s,g=e.j},e=>{c=e.C}],execute:function(){function u({restartSystem:e,shutdownSystem:t,isRestarting:o,isShuttingDown:n}){return s`
    <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
      <h2 class="text-xl font-bold">System</h2>
      <div class="controls space-x-2">
        <button 
          id="restart-btn" 
          class="px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick=${e}
          disabled=${o||n}
        >
          Restart
        </button>
        <button 
          id="shutdown-btn" 
          class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick=${t}
          disabled=${o||n}
        >
          Shutdown
        </button>
      </div>
    </div>
  `}function m({systemInfo:e,formatUptime:t}){return s`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">System Information</h3>
      <div class="space-y-2">
        <div class="flex justify-between">
          <span class="font-medium">Version:</span>
          <span>${e.version||"Unknown"}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Uptime:</span>
          <span>${e.uptime?t(e.uptime):"Unknown"}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">CPU Model:</span>
          <span>${e.cpu?.model||"Unknown"}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">CPU Cores:</span>
          <span>${e.cpu?.cores||"Unknown"}</span>
        </div>
        <div class="flex justify-between items-center">
          <span class="font-medium">CPU Usage:</span>
          <div class="w-32 bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.cpu?.usage||0}%`}></div>
          </div>
          <span>${e.cpu?.usage?`${e.cpu.usage.toFixed(1)}%`:"Unknown"}</span>
        </div>
      </div>
    </div>
  `}function y({systemInfo:e,formatBytes:t}){const o=e.memory?.used||0,n=e.go2rtcMemory?.used||0,r=e.memory?.total||0,l=o+n,i=r?(l/r*100).toFixed(1):0,a=l?(o/l*100).toFixed(1):0,d=l?(n/l*100).toFixed(1):0;return s`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory & Storage</h3>
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Process Memory:</span>
            <div>
              <span class="inline-block px-2 py-0.5 mr-1 text-xs rounded bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
                LightNVR: ${t(o)}
              </span>
              <span class="inline-block px-2 py-0.5 text-xs rounded bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                go2rtc: ${t(n)}
              </span>
            </div>
          </div>
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
            <span>Combined: ${t(l)} / ${t(r)}</span>
            <span>${i}% of total memory</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full" style=${`width: ${i}%`}>
              <div class="bg-blue-600 h-2.5" style=${`width: ${a}%`}></div>
              <div class="bg-green-500 h-2.5" style=${`width: ${d}%`}></div>
            </div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">System Memory:</span>
            <span>${e.systemMemory?.used?t(e.systemMemory.used):"0"} / ${e.systemMemory?.total?t(e.systemMemory.total):"0"}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.systemMemory?.total?(e.systemMemory.used/e.systemMemory.total*100).toFixed(1):0}%`}></div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">LightNVR Storage:</span>
            <span>${e.disk?.used?t(e.disk.used):"0"} / ${e.disk?.total?t(e.disk.total):"0"}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.disk?.total?(e.disk.used/e.disk.total*100).toFixed(1):0}%`}></div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">System Storage:</span>
            <span>${e.systemDisk?.used?t(e.systemDisk.used):"0"} / ${e.systemDisk?.total?t(e.systemDisk.total):"0"}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.systemDisk?.total?(e.systemDisk.used/e.systemDisk.total*100).toFixed(1):0}%`}></div>
          </div>
        </div>
      </div>
    </div>
  `}function b({systemInfo:e,formatBytes:t}){if(!e.streamStorage||!Array.isArray(e.streamStorage)||0===e.streamStorage.length)return s`
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Stream Storage</h3>
        <div class="text-gray-500 dark:text-gray-400 text-center py-4">
          No stream storage information available
        </div>
      </div>
    `;const o=e.streamStorage.reduce(((e,t)=>e+t.size),0),n=e.disk?.total||0,r=n?(o/n*100).toFixed(1):0,l=e.streamStorage.map((e=>({name:e.name,size:e.size,count:e.count,slicePercent:o?(e.size/o*100).toFixed(1):0})));l.sort(((e,t)=>t.size-e.size));const i=["bg-blue-600","bg-green-500","bg-yellow-500","bg-red-500","bg-purple-500","bg-pink-500","bg-indigo-500","bg-teal-500"];return s`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Stream Storage</h3>
      
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Storage per Stream:</span>
            <div class="flex flex-wrap justify-end gap-1">
              ${l.map(((e,o)=>s`
                <span class="inline-block px-2 py-0.5 text-xs rounded ${i[o%i.length].replace("bg-","bg-opacity-20 bg-")} ${i[o%i.length].replace("bg-","text-")}">
                  ${e.name}: ${t(e.size)}
                </span>
              `))}
            </div>
          </div>
          
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
            <span>Combined: ${t(o)} / ${t(n)}</span>
            <span>${r}% of total storage</span>
          </div>
          
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full" style=${`width: ${r}%`}>
              ${l.map(((e,t)=>s`
                <div class="${i[t%i.length]} h-2.5" style=${`width: ${e.slicePercent}%`}></div>
              `))}
            </div>
          </div>
          
          <div class="mt-4">
            <h4 class="font-medium mb-2">Stream Details:</h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
              ${l.map(((e,o)=>s`
                <a href="recordings.html?stream=${encodeURIComponent(e.name)}" 
                   class="flex items-center p-2 rounded bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors cursor-pointer">
                  <div class="w-3 h-3 rounded-full mr-2 ${i[o%i.length]}"></div>
                  <div>
                    <div class="font-medium">${e.name}</div>
                    <div class="text-xs text-gray-500 dark:text-gray-400">
                      ${t(e.size)} (${e.slicePercent}%) • ${e.count} recordings
                    </div>
                  </div>
                </a>
              `))}
            </div>
          </div>
        </div>
      </div>
    </div>
  `}function f({systemInfo:e}){return s`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Network Interfaces</h3>
      <div class="space-y-2">
        ${e.network?.interfaces?.length?e.network.interfaces.map((e=>s`
          <div key=${e.name} class="mb-2 pb-2 border-b border-gray-100 dark:border-gray-700 last:border-0">
            <div class="flex justify-between">
              <span class="font-medium">${e.name}:</span>
              <span>${e.address||"No IP"}</span>
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              MAC: ${e.mac||"Unknown"} | ${e.up?"Up":"Down"}
            </div>
          </div>
        `)):s`<div class="text-gray-500 dark:text-gray-400">No network interfaces found</div>`}
      </div>
    </div>
  `}function p({systemInfo:e,formatBytes:t}){return s`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Streams & Recordings</h3>
      <div class="space-y-2">
        <div class="flex justify-between">
          <span class="font-medium">Active Streams:</span>
          <span>${e.streams?.active||0} / ${e.streams?.total||0}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Recordings:</span>
          <span>${e.recordings?.count||0}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Recordings Size:</span>
          <span>${e.recordings?.size?t(e.recordings.size):"0"}</span>
        </div>
      </div>
    </div>
  `}function w({logs:e,logLevel:t,logCount:o,setLogLevel:n,setLogCount:r,loadLogs:l,clearLogs:i}){return s`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
      <div class="flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold">System Logs</h3>
        <div class="flex space-x-2">
          <select 
            id="log-level" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${t}
            onChange=${e=>{const s=e.target.value;console.log(`LogsView: Log level changed from ${t} to ${s}`),n(s)}}
          >
            <option value="error">Error</option>
            <option value="warning">Warning</option>
            <option value="info">Info</option>
            <option value="debug">Debug</option>
          </select>
          <select 
            id="log-count" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${o}
            onChange=${e=>r(parseInt(e.target.value,10))}
          >
            <option value="50">50 lines</option>
            <option value="100">100 lines</option>
            <option value="200">200 lines</option>
            <option value="500">500 lines</option>
          </select>
          <button 
            id="refresh-logs-btn" 
            class="px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${l}
          >
            Refresh
          </button>
          <button 
            id="clear-logs-btn" 
            class="px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${i}
          >
            Clear Logs
          </button>
        </div>
      </div>
      <div class="logs-container bg-gray-100 dark:bg-gray-900 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        ${0===e.length?s`
          <div class="text-gray-500 dark:text-gray-400">No logs found</div>
        `:e.map(((e,t)=>s`
          <div key=${t} class="log-entry mb-1 last:mb-0">
            <span class="text-gray-500 dark:text-gray-400">${e.timestamp}</span>
            <span class="mx-2">${function(e){if(null==e)return s`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">UNKNOWN</span>`;const t=String(e).toLowerCase().trim();if("error"===t||"err"===t)return s`<span class="px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200">ERROR</span>`;if("warning"===t||"warn"===t)return s`<span class="px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200">WARN</span>`;if("info"===t)return s`<span class="px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">INFO</span>`;if("debug"===t||"dbg"===t)return s`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">DEBUG</span>`;{const t=String(e).toUpperCase();return s`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">${t}</span>`}}(e.level)}</span>
            <span class=${"log-message "+("error"===e.level?"text-red-600 dark:text-red-400":"")}>${e.message}</span>
          </div>
        `))}
      </div>
    </div>
  `}function v(e,t=1){if(0===e)return"0 Bytes";const s=t<0?0:t,o=Math.floor(Math.log(e)/Math.log(1024));return parseFloat((e/Math.pow(1024,o)).toFixed(s))+" "+["Bytes","KB","MB","GB","TB","PB","EB","ZB","YB"][o]}function h(e){const t=Math.floor(e/86400),s=Math.floor(e%86400/3600),o=Math.floor(e%3600/60);let n="";return t>0&&(n+=`${t}d `),(s>0||t>0)&&(n+=`${s}h `),(o>0||s>0||t>0)&&(n+=`${o}m `),n+=`${Math.floor(e%60)}s`,n}function $(e,t){if("debug"===String(t||"").toLowerCase())return!0;let s=2,o=2;const n=String(e||"").toLowerCase(),r=String(t||"").toLowerCase();return"error"===n?s=0:"warning"===n||"warn"===n?s=1:"info"===n?s=2:"debug"===n&&(s=3),"error"===r?o=0:"warning"===r||"warn"===r?o=1:"info"===r?o=2:"debug"===r&&(o=3),s<=o}function k({logLevel:e,logCount:t,onLogsReceived:s}){const[l,i]=o(!1),a=n(null),d=n(null);r((()=>{const e=localStorage.getItem("lastLogTimestamp");e&&(console.log("Loaded last log timestamp from localStorage:",e),d.current=e)}),[]);const g=()=>{if(!window.wsClient)return void console.log("WebSocket client not available, will retry on next poll");if(!window.wsClient.isConnected())return console.log("WebSocket not connected, attempting to connect"),void window.wsClient.connect();if(!document.getElementById("system-page"))return void console.log("Not on system page, skipping log fetch");console.log("Fetching logs via WebSocket with level: debug (to get all logs, will filter on frontend)");const e={level:"debug",count:t};d.current&&(e.last_timestamp=d.current),window.wsClient.getClientId&&(e.client_id=window.wsClient.getClientId()),console.log("Sending fetch request with payload:",e);try{window.wsClient.send("fetch","system/logs",e)?console.log("Fetch request sent successfully"):console.warn("Failed to send fetch request, will retry on next poll")}catch(s){console.error("Error sending fetch request:",s)}};return r((()=>{if(window.wsClient)return e();{console.log("WebSocket client not available, will check again later");const t=setInterval((()=>{window.wsClient&&(console.log("WebSocket client now available, setting up handlers"),clearInterval(t),e())}),1e3);return()=>{clearInterval(t)}}function e(){return console.log("Setting up WebSocket handlers for logs"),console.log("Registering handler for system/logs via WebSocket (once on mount)"),window.wsClient.on("update","system/logs",(e=>{if(console.log("Received logs update via WebSocket:",e),document.getElementById("system-page")){if(e&&e.logs&&Array.isArray(e.logs)){const t=e.logs.map((e=>{const t={timestamp:e.timestamp||"Unknown",level:e.level||"info",message:e.message||""};return t.level&&(t.level=t.level.toLowerCase()),"warn"===t.level&&(t.level="warning"),t}));e.latest_timestamp&&(d.current=e.latest_timestamp,localStorage.setItem("lastLogTimestamp",e.latest_timestamp),console.log("Updated and saved last log timestamp:",e.latest_timestamp)),t.length>0?(console.log(`Received ${t.length} logs via WebSocket`),t.sort(((e,t)=>new Date(t.timestamp)-new Date(e.timestamp))),s(t)):console.log("No logs received via WebSocket")}}else console.log("Not on system page, ignoring log update")})),()=>{console.log("Unregistering handler for system/logs via WebSocket (component unmounting)"),window.wsClient.off("update","system/logs"),a.current&&(clearInterval(a.current),a.current=null)}}}),[]),r((()=>{if(l&&!a.current){if(console.log("Starting log polling"),window.wsClient&&"function"==typeof window.wsClient.subscribe){console.log("Subscribing to system/logs via WebSocket for polling");const e={level:"debug",...d.current?{since:d.current}:{}};window.wsClient.subscribe("system/logs",e),console.log(`Subscribed to system/logs with level: debug and last_timestamp: ${d.current||"NULL"}`)}g(),console.log("Setting up polling interval for logs (every 5 seconds)"),a.current=setInterval((()=>{console.log("Polling interval triggered, fetching logs..."),g()}),5e3)}else!l&&a.current&&(console.log("Stopping log polling"),window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Unsubscribing from system/logs via WebSocket"),window.wsClient.unsubscribe("system/logs")),clearInterval(a.current),a.current=null);return()=>{a.current&&(clearInterval(a.current),a.current=null),window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Unsubscribing from system/logs via WebSocket on cleanup"),window.wsClient.unsubscribe("system/logs"))}}),[l,e]),r((()=>{const e=()=>{console.log("Received refresh-logs-websocket event, triggering fetch"),g()};return window.addEventListener("refresh-logs-websocket",e),()=>{window.removeEventListener("refresh-logs-websocket",e)}}),[]),r((()=>(console.log(`LogsPoller: Setting up polling with log level ${e}`),i(!1),setTimeout((()=>{i(!0)}),100),()=>{console.log("LogsPoller: Cleaning up on unmount"),i(!1)})),[e,t]),null}function x(){const[e,t]=o({version:"",uptime:"",cpu:{model:"",cores:0,usage:0},memory:{total:0,used:0,free:0},go2rtcMemory:{total:0,used:0,free:0},systemMemory:{total:0,used:0,free:0},disk:{total:0,used:0,free:0},systemDisk:{total:0,used:0,free:0},network:{interfaces:[]},streams:{active:0,total:0},recordings:{count:0,size:0}}),[a,x]=o([]),[S,C]=o("debug"),L=n("debug"),[I,M]=o(100),[E,W]=o(!1),[j,U]=o(!1),[R,F]=o(!1),{data:P,isLoading:B,error:D,refetch:A}=l(["systemInfo"],"/api/system/info",{timeout:15e3,retries:2,retryDelay:1e3}),N=n(!1),{data:T,refetch:z}=l(["logs",I],`/api/system/logs?level=debug&count=${I}`,{timeout:2e4,retries:1,retryDelay:1e3,enabled:!N.current}),V=i({mutationKey:["clearLogs"],mutationFn:async()=>await g("/api/system/logs/clear",{method:"POST",timeout:1e4,retries:1}),onSuccess:()=>{d("Logs cleared successfully"),x([])},onError:e=>{console.error("Error clearing logs:",e),d(`Error clearing logs: ${e.message}`)}});r((()=>{P&&F(!0)}),[P]);const _=i({mutationFn:async()=>await g("/api/system/restart",{method:"POST",timeout:3e4,retries:0}),onMutate:()=>{W(!0),d("Restarting system...")},onSuccess:()=>{d("System is restarting. Please wait..."),setTimeout((()=>{window.location.reload()}),1e4)},onError:e=>{console.error("Error restarting system:",e),d(`Error restarting system: ${e.message}`),W(!1)}}),q=i({mutationFn:async()=>await g("/api/system/shutdown",{method:"POST",timeout:3e4,retries:0}),onMutate:()=>{U(!0),d("Shutting down system...")},onSuccess:()=>{d("System is shutting down. You will need to manually restart it.")},onError:e=>{console.error("Error shutting down system:",e),d(`Error shutting down system: ${e.message}`),U(!1)}});return r((()=>{P&&t(P)}),[P]),r((()=>{if(T&&T.logs&&Array.isArray(T.logs)){const e=L.current;if(T.logs.length>0&&"object"==typeof T.logs[0]&&T.logs[0].level){let t=T.logs.filter((t=>$(t.level,e)));console.log(`Filtered ${T.logs.length} logs to ${t.length} based on log level ${e}`),x(t)}else{const t=T.logs.map((e=>{let t="Unknown",s="debug",o=e;const n=e.match(/\[(.*?)\]\s*\[(.*?)\]\s*(.*)/);return n&&n.length>=4&&(t=n[1],s=n[2].toLowerCase(),o=n[3],"warn"===s&&(s="warning")),{timestamp:t,level:s,message:o}}));let s=t.filter((t=>$(t.level,e)));console.log(`Filtered ${t.length} parsed logs to ${s.length} based on log level ${e}`),x(s)}}else x([])}),[T]),r((()=>{if(console.log(`SystemView: Log level changed to ${S} or count changed to ${I}`),T&&T.logs&&Array.isArray(T.logs)){console.log("Filtering existing logs based on new log level");const e=L.current;console.log(`Filtering existing logs using logLevelRef.current: ${e}`),x((t=>t.filter((t=>$(t.level,e)))))}}),[S,I]),r((()=>()=>{window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Cleaning up any WebSocket subscriptions on unmount"),window.wsClient.unsubscribe("system/logs"))}),[]),r((()=>{if(window.wsClient){console.log("WebSocket client is available in SystemView");const e=e=>{!e&&N.current&&(console.log("WebSocket disconnected, switching to HTTP for logs"),N.current=!1,z())};if("function"==typeof window.wsClient.addConnectionChangeListener)return console.log("Adding WebSocket connection change listener"),window.wsClient.addConnectionChangeListener(e),()=>{console.log("Removing WebSocket connection change listener"),window.wsClient.removeConnectionChangeListener(e)};{console.log("WebSocket client does not support connection change listeners");const e=e=>{console.log("Received websocket-fallback event"),N.current&&(console.log("WebSocket fallback detected, switching to HTTP for logs"),N.current=!1,z())};return window.addEventListener("websocket-fallback",e),()=>{window.removeEventListener("websocket-fallback",e)}}}console.log("WebSocket client not available in SystemView, it should be initialized in preact-app.js"),N.current=!1}),[]),s`
    <section id="system-page" class="page">
      <${u}
        restartSystem=${()=>{confirm("Are you sure you want to restart the system?")&&_.mutate()}}
        shutdownSystem=${()=>{confirm("Are you sure you want to shut down the system?")&&q.mutate()}}
        isRestarting=${E}
        isShuttingDown=${j}
      />

      <${c}
        isLoading=${B}
        hasData=${R}
        loadingMessage="Loading system information..."
        emptyMessage="System information not available. Please try again later."
      >
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <${m} systemInfo=${e} formatUptime=${h} />
          <${y} systemInfo=${e} formatBytes=${v} />
        </div>

        <div class="grid grid-cols-1 gap-4 mb-4">
          <${b} systemInfo=${e} formatBytes=${v} />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <${f} systemInfo=${e} />
          <${p} systemInfo=${e} formatBytes=${v} />
        </div>

        <${w}
          logs=${a}
          logLevel=${S}
          logCount=${I}
          setLogLevel=${e=>{console.log(`SystemView: Setting log level from ${S} to ${e}`),C(e),L.current=e}}
          setLogCount=${M}
          loadLogs=${()=>{if(N.current&&window.wsClient&&window.wsClient.isConnected()){console.log("Manually triggering WebSocket fetch for logs");const e=new CustomEvent("refresh-logs-websocket");window.dispatchEvent(e)}else console.log("Using HTTP fetch for logs"),z()}}
          clearLogs=${()=>{confirm("Are you sure you want to clear all logs?")&&V.mutate()}}
        />

        <${k}
          logLevel=${S}
          logCount=${I}
          onLogsReceived=${e=>{console.log("SystemView received new logs:",e.length),N.current||(console.log("Setting usingWebSocketForLogs to true"),N.current=!0);const t=L.current,s=e.filter((e=>$(e.level,t)));x(s)}}
        />
      <//>
    </section>
  `}e({SystemView:x,loadSystemView:function(){const e=document.getElementById("main-content");e&&a((async()=>{const{render:e}=await t.import("./preact-app-legacy-C9dbqWYg.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:o})=>{a((async()=>{const{QueryClientProvider:e,queryClient:s}=await t.import("./preact-app-legacy-C9dbqWYg.js").then((e=>e.n));return{QueryClientProvider:e,queryClient:s}}),void 0,t.meta.url).then((({QueryClientProvider:t,queryClient:n})=>{o(s`<${t} client=${n}><${x} /></${t}>`,e),setTimeout((()=>{const e=new CustomEvent("refresh-system-info");window.dispatchEvent(e)}),100)}))}))}}),window.addEventListener("load",(()=>{window.addEventListener("refresh-system-info",(async()=>{try{await g("/api/system/info",{timeout:15e3,retries:1,retryDelay:1e3}),console.log("System info refreshed")}catch(e){console.error("Error refreshing system info:",e)}}))}))}}}));
//# sourceMappingURL=SystemView-legacy-DbCMU0ib.js.map
