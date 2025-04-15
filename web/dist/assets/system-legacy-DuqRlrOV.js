System.register(["./query-client-legacy-BQ_U5NnD.js","./layout-legacy-CDxh3f01.js","./Footer-legacy-B5sb7m6V.js","./LoadingIndicator-legacy-Xson3LBG.js","./websocket-client-legacy-BO-Yc0A6.js"],(function(e,t){"use strict";var s,o,n,r,l,i,a,d,c,g,u,m,b,y,f,p;return{setters:[e=>{s=e.d,o=e.A,n=e.y,r=e.e,l=e.c,i=e.f,a=e.u,d=e.E,c=e.q,g=e.Q},null,e=>{u=e.h,m=e.c,b=e.H,y=e.F},e=>{f=e.C},e=>{p=e.W}],execute:function(){function e({restartSystem:e,shutdownSystem:t,isRestarting:s,isShuttingDown:o}){return u`
    <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
      <h2 class="text-xl font-bold">System</h2>
      <div class="controls space-x-2">
        <button 
          id="restart-btn" 
          class="px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick=${e}
          disabled=${s||o}
        >
          Restart
        </button>
        <button 
          id="shutdown-btn" 
          class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick=${t}
          disabled=${s||o}
        >
          Shutdown
        </button>
      </div>
    </div>
  `}function t({systemInfo:e,formatUptime:t}){return u`
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
  `}function w({systemInfo:e,formatBytes:t}){const s=e.memory?.used||0,o=e.go2rtcMemory?.used||0,n=e.memory?.total||0,r=s+o,l=n?(r/n*100).toFixed(1):0,i=r?(s/r*100).toFixed(1):0,a=r?(o/r*100).toFixed(1):0;return u`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory & Storage</h3>
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Process Memory:</span>
            <div>
              <span class="inline-block px-2 py-0.5 mr-1 text-xs rounded bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
                LightNVR: ${t(s)}
              </span>
              <span class="inline-block px-2 py-0.5 text-xs rounded bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                go2rtc: ${t(o)}
              </span>
            </div>
          </div>
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
            <span>Combined: ${t(r)} / ${t(n)}</span>
            <span>${l}% of total memory</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full" style=${`width: ${l}%`}>
              <div class="bg-blue-600 h-2.5" style=${`width: ${i}%`}></div>
              <div class="bg-green-500 h-2.5" style=${`width: ${a}%`}></div>
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
  `}function v({systemInfo:e,formatBytes:t}){if(!e.streamStorage||!Array.isArray(e.streamStorage)||0===e.streamStorage.length)return u`
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Stream Storage</h3>
        <div class="text-gray-500 dark:text-gray-400 text-center py-4">
          No stream storage information available
        </div>
      </div>
    `;const s=e.streamStorage.reduce(((e,t)=>e+t.size),0),o=e.disk?.total||0,n=o?(s/o*100).toFixed(1):0,r=e.streamStorage.map((e=>({name:e.name,size:e.size,count:e.count,slicePercent:s?(e.size/s*100).toFixed(1):0})));r.sort(((e,t)=>t.size-e.size));const l=["bg-blue-600","bg-green-500","bg-yellow-500","bg-red-500","bg-purple-500","bg-pink-500","bg-indigo-500","bg-teal-500"];return u`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Stream Storage</h3>
      
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Storage per Stream:</span>
            <div class="flex flex-wrap justify-end gap-1">
              ${r.map(((e,s)=>u`
                <span class="inline-block px-2 py-0.5 text-xs rounded ${l[s%l.length].replace("bg-","bg-opacity-20 bg-")} ${l[s%l.length].replace("bg-","text-")}">
                  ${e.name}: ${t(e.size)}
                </span>
              `))}
            </div>
          </div>
          
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
            <span>Combined: ${t(s)} / ${t(o)}</span>
            <span>${n}% of total storage</span>
          </div>
          
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full" style=${`width: ${n}%`}>
              ${r.map(((e,t)=>u`
                <div class="${l[t%l.length]} h-2.5" style=${`width: ${e.slicePercent}%`}></div>
              `))}
            </div>
          </div>
          
          <div class="mt-4">
            <h4 class="font-medium mb-2">Stream Details:</h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
              ${r.map(((e,s)=>u`
                <a href="recordings.html?stream=${encodeURIComponent(e.name)}" 
                   class="flex items-center p-2 rounded bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors cursor-pointer">
                  <div class="w-3 h-3 rounded-full mr-2 ${l[s%l.length]}"></div>
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
  `}function h({systemInfo:e}){return u`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Network Interfaces</h3>
      <div class="space-y-2">
        ${e.network?.interfaces?.length?e.network.interfaces.map((e=>u`
          <div key=${e.name} class="mb-2 pb-2 border-b border-gray-100 dark:border-gray-700 last:border-0">
            <div class="flex justify-between">
              <span class="font-medium">${e.name}:</span>
              <span>${e.address||"No IP"}</span>
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              MAC: ${e.mac||"Unknown"} | ${e.up?"Up":"Down"}
            </div>
          </div>
        `)):u`<div class="text-gray-500 dark:text-gray-400">No network interfaces found</div>`}
      </div>
    </div>
  `}function k({systemInfo:e,formatBytes:t}){return u`
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
  `}function x({logs:e,logLevel:t,logCount:s,setLogLevel:o,setLogCount:n,loadLogs:r,clearLogs:l}){return u`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
      <div class="flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold">System Logs</h3>
        <div class="flex space-x-2">
          <select 
            id="log-level" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${t}
            onChange=${e=>{const s=e.target.value;console.log(`LogsView: Log level changed from ${t} to ${s}`),o(s)}}
          >
            <option value="error">Error</option>
            <option value="warning">Warning</option>
            <option value="info">Info</option>
            <option value="debug">Debug</option>
          </select>
          <select 
            id="log-count" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${s}
            onChange=${e=>n(parseInt(e.target.value,10))}
          >
            <option value="50">50 lines</option>
            <option value="100">100 lines</option>
            <option value="200">200 lines</option>
            <option value="500">500 lines</option>
          </select>
          <button 
            id="refresh-logs-btn" 
            class="px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${r}
          >
            Refresh
          </button>
          <button 
            id="clear-logs-btn" 
            class="px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${l}
          >
            Clear Logs
          </button>
        </div>
      </div>
      <div class="logs-container bg-gray-100 dark:bg-gray-900 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        ${0===e.length?u`
          <div class="text-gray-500 dark:text-gray-400">No logs found</div>
        `:e.map(((e,t)=>u`
          <div key=${t} class="log-entry mb-1 last:mb-0">
            <span class="text-gray-500 dark:text-gray-400">${e.timestamp}</span>
            <span class="mx-2">${function(e){if(null==e)return u`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">UNKNOWN</span>`;const t=String(e).toLowerCase().trim();if("error"===t||"err"===t)return u`<span class="px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200">ERROR</span>`;if("warning"===t||"warn"===t)return u`<span class="px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200">WARN</span>`;if("info"===t)return u`<span class="px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">INFO</span>`;if("debug"===t||"dbg"===t)return u`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">DEBUG</span>`;{const t=String(e).toUpperCase();return u`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">${t}</span>`}}(e.level)}</span>
            <span class=${"log-message "+("error"===e.level?"text-red-600 dark:text-red-400":"")}>${e.message}</span>
          </div>
        `))}
      </div>
    </div>
  `}function $(e,t=1){if(0===e)return"0 Bytes";const s=t<0?0:t,o=Math.floor(Math.log(e)/Math.log(1024));return parseFloat((e/Math.pow(1024,o)).toFixed(s))+" "+["Bytes","KB","MB","GB","TB","PB","EB","ZB","YB"][o]}function S(e){const t=Math.floor(e/86400),s=Math.floor(e%86400/3600),o=Math.floor(e%3600/60);let n="";return t>0&&(n+=`${t}d `),(s>0||t>0)&&(n+=`${s}h `),(o>0||s>0||t>0)&&(n+=`${o}m `),n+=`${Math.floor(e%60)}s`,n}function C({logLevel:e,logCount:t,onLogsReceived:r}){const[l,i]=s(!1),a=o(null),d=o(null);n((()=>{const e=localStorage.getItem("lastLogTimestamp");e&&(console.log("Loaded last log timestamp from localStorage:",e),d.current=e)}),[]);const c=()=>{if(!window.wsClient)return void console.log("WebSocket client not available, will retry on next poll");if(!window.wsClient.isConnected())return console.log("WebSocket not connected, attempting to connect"),void window.wsClient.connect();if(!document.getElementById("system-page"))return void console.log("Not on system page, skipping log fetch");console.log("Fetching logs via WebSocket with level: debug (to get all logs, will filter on frontend)");const e={level:"debug",count:t};d.current&&(e.last_timestamp=d.current),window.wsClient.getClientId&&(e.client_id=window.wsClient.getClientId()),console.log("Sending fetch request with payload:",e);try{window.wsClient.send("fetch","system/logs",e)?console.log("Fetch request sent successfully"):console.warn("Failed to send fetch request, will retry on next poll")}catch(s){console.error("Error sending fetch request:",s)}};return n((()=>{if(window.wsClient)return e();{console.log("WebSocket client not available, will check again later");const t=setInterval((()=>{window.wsClient&&(console.log("WebSocket client now available, setting up handlers"),clearInterval(t),e())}),1e3);return()=>{clearInterval(t)}}function e(){return console.log("Setting up WebSocket handlers for logs"),console.log("Registering handler for system/logs via WebSocket (once on mount)"),window.wsClient.on("update","system/logs",(e=>{if(console.log("Received logs update via WebSocket:",e),document.getElementById("system-page")){if(e&&e.logs&&Array.isArray(e.logs)){const t=e.logs.map((e=>{const t={timestamp:e.timestamp||"Unknown",level:e.level||"info",message:e.message||""};return t.level&&(t.level=t.level.toLowerCase()),"warn"===t.level&&(t.level="warning"),t}));e.latest_timestamp&&(d.current=e.latest_timestamp,localStorage.setItem("lastLogTimestamp",e.latest_timestamp),console.log("Updated and saved last log timestamp:",e.latest_timestamp)),t.length>0?(console.log(`Received ${t.length} logs via WebSocket`),t.sort(((e,t)=>new Date(t.timestamp)-new Date(e.timestamp))),r(t)):console.log("No logs received via WebSocket")}}else console.log("Not on system page, ignoring log update")})),()=>{console.log("Unregistering handler for system/logs via WebSocket (component unmounting)"),window.wsClient.off("update","system/logs"),a.current&&(clearInterval(a.current),a.current=null)}}}),[]),n((()=>{if(l&&!a.current){if(console.log("Starting log polling"),window.wsClient&&"function"==typeof window.wsClient.subscribe){console.log("Subscribing to system/logs via WebSocket for polling");const e={level:"debug",...d.current?{since:d.current}:{}};window.wsClient.subscribe("system/logs",e),console.log(`Subscribed to system/logs with level: debug and last_timestamp: ${d.current||"NULL"}`)}c(),console.log("Setting up polling interval for logs (every 5 seconds)"),a.current=setInterval((()=>{console.log("Polling interval triggered, fetching logs..."),c()}),5e3)}else!l&&a.current&&(console.log("Stopping log polling"),window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Unsubscribing from system/logs via WebSocket"),window.wsClient.unsubscribe("system/logs")),clearInterval(a.current),a.current=null);return()=>{a.current&&(clearInterval(a.current),a.current=null),window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Unsubscribing from system/logs via WebSocket on cleanup"),window.wsClient.unsubscribe("system/logs"))}}),[l,e]),n((()=>{const e=()=>{console.log("Received refresh-logs-websocket event, triggering fetch"),c()};return window.addEventListener("refresh-logs-websocket",e),()=>{window.removeEventListener("refresh-logs-websocket",e)}}),[]),n((()=>(console.log(`LogsPoller: Setting up polling with log level ${e}`),i(!1),setTimeout((()=>{i(!0)}),100),()=>{console.log("LogsPoller: Cleaning up on unmount"),i(!1)})),[e,t]),null}function L(){const[d,c]=s({version:"",uptime:"",cpu:{model:"",cores:0,usage:0},memory:{total:0,used:0,free:0},go2rtcMemory:{total:0,used:0,free:0},systemMemory:{total:0,used:0,free:0},disk:{total:0,used:0,free:0},systemDisk:{total:0,used:0,free:0},network:{interfaces:[]},streams:{active:0,total:0},recordings:{count:0,size:0}}),[g,u]=s([]),[b,y]=s("debug"),L=o("debug"),[I,M]=s(100),[W,j]=s(!1),[U,E]=s(!1),[R,B]=s(!1),{data:D,isLoading:F,error:P,refetch:N}=r(["systemInfo"],"/api/system/info",{timeout:15e3,retries:2,retryDelay:1e3}),z=l({mutationKey:["clearLogs"],mutationFn:async()=>await i("/api/system/logs/clear",{method:"POST",timeout:1e4,retries:1}),onSuccess:()=>{m("Logs cleared successfully"),u([])},onError:e=>{console.error("Error clearing logs:",e),m(`Error clearing logs: ${e.message}`)}});n((()=>{D&&B(!0)}),[D]);const A=l({mutationFn:async()=>await i("/api/system/restart",{method:"POST",timeout:3e4,retries:0}),onMutate:()=>{j(!0),m("Restarting system...")},onSuccess:()=>{m("System is restarting. Please wait..."),setTimeout((()=>{window.location.reload()}),1e4)},onError:e=>{console.error("Error restarting system:",e),m(`Error restarting system: ${e.message}`),j(!1)}}),T=l({mutationFn:async()=>await i("/api/system/shutdown",{method:"POST",timeout:3e4,retries:0}),onMutate:()=>{E(!0),m("Shutting down system...")},onSuccess:()=>{m("System is shutting down. You will need to manually restart it.")},onError:e=>{console.error("Error shutting down system:",e),m(`Error shutting down system: ${e.message}`),E(!1)}});return n((()=>{D&&c(D)}),[D]),n((()=>()=>{window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Cleaning up any WebSocket subscriptions on unmount"),window.wsClient.unsubscribe("system/logs"))}),[]),n((()=>{if(void 0!==p){if(window.wsClient=new p,console.log("WebSocket client initialized at application level"),window.wsClient){console.log("Initial WebSocket connection state:",{connected:window.wsClient.isConnected(),clientId:window.wsClient.getClientId()});const e=window.wsClient.connect;window.wsClient.connect=function(){const t=e.apply(this,arguments);if(this.socket){const e=this.socket.onopen;this.socket.onopen=t=>{console.log("WebSocket connection opened at application level"),e&&e.call(this,t)};const t=this.socket.onerror;this.socket.onerror=e=>{console.error("WebSocket error at application level:",e),t&&t.call(this,e)};const s=this.socket.onclose;this.socket.onclose=e=>{console.log(`WebSocket connection closed at application level: ${e.code} ${e.reason}`),s&&s.call(this,e)};const o=this.socket.onmessage;this.socket.onmessage=e=>{e.data.includes('"type":"welcome"')||console.log("WebSocket message received at application level"),o&&o.call(this,e)}}return t};const t=window.wsClient.handleMessage;window.wsClient.handleMessage=function(e){const s=this.clientId;t.call(this,e);const o=this.clientId;s!==o&&o&&console.log(`WebSocket client ID changed at application level: ${o}`)}}}else console.log("WebSocketClient is not defined, cannot initialize WebSocket client")}),[]),a("section",{id:"system-page",class:"page",children:[a(e,{restartSystem:()=>{confirm("Are you sure you want to restart the system?")&&A.mutate()},shutdownSystem:()=>{confirm("Are you sure you want to shut down the system?")&&T.mutate()},isRestarting:W,isShuttingDown:U}),a(f,{isLoading:F,hasData:R,loadingMessage:"Loading system information...",emptyMessage:"System information not available. Please try again later.",children:[a("div",{class:"grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",children:[a(t,{systemInfo:d,formatUptime:S}),a(w,{systemInfo:d,formatBytes:$})]}),a("div",{class:"grid grid-cols-1 gap-4 mb-4",children:a(v,{systemInfo:d,formatBytes:$})}),a("div",{class:"grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",children:[a(h,{systemInfo:d}),a(k,{systemInfo:d,formatBytes:$})]}),a(x,{logs:g,logLevel:b,logCount:I,setLogLevel:e=>{console.log(`SystemView: Setting log level from ${b} to ${e}`),y(e),L.current=e},setLogCount:M,loadLogs:()=>{if(window.wsClient&&window.wsClient.isConnected()){console.log("Manually triggering WebSocket fetch for logs");const e=new CustomEvent("refresh-logs-websocket");window.dispatchEvent(e)}},clearLogs:()=>{confirm("Are you sure you want to clear all logs?")&&z.mutate()}}),a(C,{logLevel:b,logCount:I,onLogsReceived:e=>{console.log("SystemView received new logs:",e.length);const t=L.current,s=e.filter((e=>function(e,t){if("debug"===String(t||"").toLowerCase())return!0;let s=2,o=2;const n=String(e||"").toLowerCase(),r=String(t||"").toLowerCase();return"error"===n?s=0:"warning"===n||"warn"===n?s=1:"info"===n?s=2:"debug"===n&&(s=3),"error"===r?o=0:"warning"===r||"warn"===r?o=1:"info"===r?o=2:"debug"===r&&(o=3),s<=o}(e.level,t)));u(s)}})]})]})}document.addEventListener("DOMContentLoaded",(()=>{const e=document.getElementById("main-content");e&&d(a(g,{client:c,children:[a(b,{}),a(L,{}),a(y,{})]}),e)}))}}}));
//# sourceMappingURL=system-legacy-DuqRlrOV.js.map
