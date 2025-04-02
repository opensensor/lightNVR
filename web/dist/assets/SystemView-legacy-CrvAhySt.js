System.register(["./preact-app-legacy-D5_kcW1v.js","./LoadingIndicator-legacy-BWKMEYQf.js","./fetch-utils-legacy-DOz1-Xee.js"],(function(e,s){"use strict";var t,o,n,l,r,a,i,d,g,c;return{setters:[e=>{t=e.h,o=e.d,n=e.A,l=e.y,r=e._,a=e.c},e=>{i=e.C},e=>{d=e.f,g=e.e,c=e.c}],execute:function(){function u({restartSystem:e,shutdownSystem:s,isRestarting:o,isShuttingDown:n}){return t`
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
          onClick=${s}
          disabled=${o||n}
        >
          Shutdown
        </button>
      </div>
    </div>
  `}function m({systemInfo:e,formatUptime:s}){return t`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">System Information</h3>
      <div class="space-y-2">
        <div class="flex justify-between">
          <span class="font-medium">Version:</span>
          <span>${e.version||"Unknown"}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Uptime:</span>
          <span>${e.uptime?s(e.uptime):"Unknown"}</span>
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
  `}function y({systemInfo:e,formatBytes:s}){const o=e.memory?.used||0,n=e.go2rtcMemory?.used||0,l=e.memory?.total||0,r=o+n,a=l?(r/l*100).toFixed(1):0,i=r?(o/r*100).toFixed(1):0,d=r?(n/r*100).toFixed(1):0;return t`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory & Storage</h3>
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Process Memory:</span>
            <div>
              <span class="inline-block px-2 py-0.5 mr-1 text-xs rounded bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
                LightNVR: ${s(o)}
              </span>
              <span class="inline-block px-2 py-0.5 text-xs rounded bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                go2rtc: ${s(n)}
              </span>
            </div>
          </div>
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
            <span>Combined: ${s(r)} / ${s(l)}</span>
            <span>${a}% of total memory</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full" style=${`width: ${a}%`}>
              <div class="bg-blue-600 h-2.5" style=${`width: ${i}%`}></div>
              <div class="bg-green-500 h-2.5" style=${`width: ${d}%`}></div>
            </div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">System Memory:</span>
            <span>${e.systemMemory?.used?s(e.systemMemory.used):"0"} / ${e.systemMemory?.total?s(e.systemMemory.total):"0"}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.systemMemory?.total?(e.systemMemory.used/e.systemMemory.total*100).toFixed(1):0}%`}></div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">LightNVR Storage:</span>
            <span>${e.disk?.used?s(e.disk.used):"0"} / ${e.disk?.total?s(e.disk.total):"0"}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.disk?.total?(e.disk.used/e.disk.total*100).toFixed(1):0}%`}></div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">System Storage:</span>
            <span>${e.systemDisk?.used?s(e.systemDisk.used):"0"} / ${e.systemDisk?.total?s(e.systemDisk.total):"0"}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${e.systemDisk?.total?(e.systemDisk.used/e.systemDisk.total*100).toFixed(1):0}%`}></div>
          </div>
        </div>
      </div>
    </div>
  `}function f({systemInfo:e}){return t`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Network Interfaces</h3>
      <div class="space-y-2">
        ${e.network?.interfaces?.length?e.network.interfaces.map((e=>t`
          <div key=${e.name} class="mb-2 pb-2 border-b border-gray-100 dark:border-gray-700 last:border-0">
            <div class="flex justify-between">
              <span class="font-medium">${e.name}:</span>
              <span>${e.address||"No IP"}</span>
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              MAC: ${e.mac||"Unknown"} | ${e.up?"Up":"Down"}
            </div>
          </div>
        `)):t`<div class="text-gray-500 dark:text-gray-400">No network interfaces found</div>`}
      </div>
    </div>
  `}function p({systemInfo:e,formatBytes:s}){return t`
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
          <span>${e.recordings?.size?s(e.recordings.size):"0"}</span>
        </div>
      </div>
    </div>
  `}function b({logs:e,logLevel:s,logCount:o,setLogLevel:n,setLogCount:l,loadLogs:r,clearLogs:a}){return t`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
      <div class="flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold">System Logs</h3>
        <div class="flex space-x-2">
          <select 
            id="log-level" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
            value=${s}
            onChange=${e=>{const t=e.target.value;console.log(`LogsView: Log level changed from ${s} to ${t}`),n(t)}}
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
            onChange=${e=>l(parseInt(e.target.value,10))}
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
            onClick=${a}
          >
            Clear Logs
          </button>
        </div>
      </div>
      <div class="logs-container bg-gray-100 dark:bg-gray-900 rounded p-4 overflow-auto max-h-96 font-mono text-sm">
        ${0===e.length?t`
          <div class="text-gray-500 dark:text-gray-400">No logs found</div>
        `:e.map(((e,s)=>t`
          <div key=${s} class="log-entry mb-1 last:mb-0">
            <span class="text-gray-500 dark:text-gray-400">${e.timestamp}</span>
            <span class="mx-2">${function(e){if(null==e)return t`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">UNKNOWN</span>`;const s=String(e).toLowerCase().trim();if("error"===s||"err"===s)return t`<span class="px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200">ERROR</span>`;if("warning"===s||"warn"===s)return t`<span class="px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200">WARN</span>`;if("info"===s)return t`<span class="px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">INFO</span>`;if("debug"===s||"dbg"===s)return t`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">DEBUG</span>`;{const s=String(e).toUpperCase();return t`<span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">${s}</span>`}}(e.level)}</span>
            <span class=${"log-message "+("error"===e.level?"text-red-600 dark:text-red-400":"")}>${e.message}</span>
          </div>
        `))}
      </div>
    </div>
  `}function w(e,s=1){if(0===e)return"0 Bytes";const t=s<0?0:s,o=Math.floor(Math.log(e)/Math.log(1024));return parseFloat((e/Math.pow(1024,o)).toFixed(t))+" "+["Bytes","KB","MB","GB","TB","PB","EB","ZB","YB"][o]}function v(e){const s=Math.floor(e/86400),t=Math.floor(e%86400/3600),o=Math.floor(e%3600/60);let n="";return s>0&&(n+=`${s}d `),(t>0||s>0)&&(n+=`${t}h `),(o>0||t>0||s>0)&&(n+=`${o}m `),n+=`${Math.floor(e%60)}s`,n}function h(e,s){if("debug"===String(s||"").toLowerCase())return!0;let t=2,o=2;const n=String(e||"").toLowerCase(),l=String(s||"").toLowerCase();return"error"===n?t=0:"warning"===n||"warn"===n?t=1:"info"===n?t=2:"debug"===n&&(t=3),"error"===l?o=0:"warning"===l||"warn"===l?o=1:"info"===l?o=2:"debug"===l&&(o=3),t<=o}function $({logLevel:e,logCount:s,onLogsReceived:t}){const[r,a]=o(!1),i=n(null),g=n(null);l((()=>{const e=localStorage.getItem("lastLogTimestamp");e&&(console.log("Loaded last log timestamp from localStorage:",e),g.current=e)}),[]);const c=()=>{if(!window.wsClient||!window.wsClient.isConnected())return void console.log("WebSocket not connected, skipping log fetch");if(!document.getElementById("system-page"))return void console.log("Not on system page, skipping log fetch");console.log("Fetching logs via WebSocket with level: debug (to get all logs, will filter on frontend)");const e={level:"debug",count:s};g.current&&(e.last_timestamp=g.current),window.wsClient.getClientId&&(e.client_id=window.wsClient.getClientId()),console.log("Sending fetch request with payload:",e);try{window.wsClient.send("fetch","system/logs",e)?console.log("Fetch request sent successfully"):console.warn("Failed to send fetch request, will retry on next poll")}catch(t){console.error("Error sending fetch request:",t)}};return l((()=>{if(window.wsClient)return console.log("Registering handler for system/logs via WebSocket (once on mount)"),window.wsClient.on("update","system/logs",(e=>{if(console.log("Received logs update via WebSocket:",e),document.getElementById("system-page")){if(e&&e.logs&&Array.isArray(e.logs)){const s=e.logs.map((e=>{const s={timestamp:e.timestamp||"Unknown",level:e.level||"info",message:e.message||""};return s.level&&(s.level=s.level.toLowerCase()),"warn"===s.level&&(s.level="warning"),s}));e.latest_timestamp&&(g.current=e.latest_timestamp,localStorage.setItem("lastLogTimestamp",e.latest_timestamp),console.log("Updated and saved last log timestamp:",e.latest_timestamp)),s.length>0?(console.log(`Received ${s.length} logs via WebSocket`),d("/api/system/logs?level=debug&count=100",{timeout:15e3,retries:1,retryDelay:1e3}).then((e=>{if(e.logs&&Array.isArray(e.logs)){const o=[...e.logs.map((e=>{const s={timestamp:e.timestamp||"Unknown",level:(e.level||"info").toLowerCase(),message:e.message||""};return"warn"===s.level&&(s.level="warning"),s}))];s.forEach((e=>{o.some((s=>s.timestamp===e.timestamp&&s.message===e.message))||o.push(e)})),o.sort(((e,s)=>new Date(s.timestamp)-new Date(e.timestamp))),t(o)}else t(s)})).catch((e=>{console.error("Error fetching existing logs:",e),t(s)}))):console.log("No logs received via WebSocket")}}else console.log("Not on system page, ignoring log update")})),()=>{console.log("Unregistering handler for system/logs via WebSocket (component unmounting)"),window.wsClient.off("update","system/logs"),i.current&&(clearInterval(i.current),i.current=null)};console.log("WebSocket client not available")}),[]),l((()=>{if(r&&!i.current){if(console.log("Starting log polling"),window.wsClient&&"function"==typeof window.wsClient.subscribe){console.log("Subscribing to system/logs via WebSocket for polling");const e={level:"debug",...g.current?{since:g.current}:{}};window.wsClient.subscribe("system/logs",e),console.log(`Subscribed to system/logs with level: debug and last_timestamp: ${g.current||"NULL"}`)}c(),console.log("Setting up polling interval for logs (every 5 seconds)"),i.current=setInterval((()=>{console.log("Polling interval triggered, fetching logs..."),c()}),5e3)}else!r&&i.current&&(console.log("Stopping log polling"),window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Unsubscribing from system/logs via WebSocket"),window.wsClient.unsubscribe("system/logs")),clearInterval(i.current),i.current=null);return()=>{i.current&&(clearInterval(i.current),i.current=null),window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Unsubscribing from system/logs via WebSocket on cleanup"),window.wsClient.unsubscribe("system/logs"))}}),[r,e]),l((()=>(console.log(`LogsPoller: Setting up polling with log level ${e}`),a(!1),setTimeout((()=>{a(!0)}),100),()=>{console.log("LogsPoller: Cleaning up on unmount"),a(!1)})),[e,s]),null}function x(){const[e,s]=o({version:"",uptime:"",cpu:{model:"",cores:0,usage:0},memory:{total:0,used:0,free:0},go2rtcMemory:{total:0,used:0,free:0},systemMemory:{total:0,used:0,free:0},disk:{total:0,used:0,free:0},systemDisk:{total:0,used:0,free:0},network:{interfaces:[]},streams:{active:0,total:0},recordings:{count:0,size:0}}),[r,x]=o([]),[k,S]=o("debug"),C=n("debug"),[L,I]=o(100),[M,E]=o(!1),[U,R]=o(!1),[D,j]=o(!0),[A,B]=o(!1),F=n(null);l((()=>(F.current=c(),W(),P(),console.log("System page loaded - no automatic polling for system info"),()=>{window.wsClient&&"function"==typeof window.wsClient.unsubscribe&&(console.log("Cleaning up any WebSocket subscriptions on unmount"),window.wsClient.unsubscribe("system/logs")),F.current&&F.current.abort()})),[]),l((()=>{if(console.log(`SystemView: Log level changed to ${k} or count changed to ${L}`),0===r.length)console.log("Initial logs load via HTTP API"),P();else{console.log("Filtering existing logs based on new log level");const e=C.current;console.log(`Filtering existing logs using logLevelRef.current: ${e}`),x((s=>s.filter((s=>h(s.level,e)))))}}),[k,L]);const W=async()=>{try{j(!0);const e=await d("/api/system/info",{signal:F.current?.signal,timeout:15e3,retries:2,retryDelay:1e3});s(e),B(!0)}catch(e){"Request was cancelled"!==e.message&&(console.error("Error loading system info:",e),B(!1))}finally{j(!1)}},P=async()=>{try{const e=C.current;console.log("Loading logs from API with level: debug (to get all logs, will filter on frontend)");const s=await d(`/api/system/logs?level=debug&count=${L}`,{signal:F.current?.signal,timeout:2e4,retries:1,retryDelay:1e3});if(s.logs&&Array.isArray(s.logs))if(s.logs.length>0&&"object"==typeof s.logs[0]&&s.logs[0].level){let t=s.logs.filter((s=>h(s.level,e)));console.log(`Filtered ${s.logs.length} logs to ${t.length} based on log level ${e}`),x(t)}else{const t=s.logs.map((e=>{let s="Unknown",t="debug",o=e;const n=e.match(/\[(.*?)\]\s*\[(.*?)\]\s*(.*)/);return n&&n.length>=4&&(s=n[1],t=n[2].toLowerCase(),o=n[3],"warn"===t&&(t="warning")),{timestamp:s,level:t,message:o}}));let o=t.filter((s=>h(s.level,e)));console.log(`Filtered ${t.length} parsed logs to ${o.length} based on log level ${e}`),x(o)}else x([])}catch(e){console.error("Error loading logs:",e),a("Error loading logs: "+e.message)}};return l((()=>{window.wsClient||"function"!=typeof WebSocketClient||(console.log("Initializing WebSocket client for system page"),window.wsClient=new WebSocketClient)}),[]),t`
    <section id="system-page" class="page">
      <${u} 
        restartSystem=${async()=>{if(confirm("Are you sure you want to restart the system?"))try{E(!0),a("Restarting system..."),await g("/api/system/restart",{method:"POST",signal:F.current?.signal,timeout:3e4,retries:0}),a("System is restarting. Please wait..."),setTimeout((()=>{window.location.reload()}),1e4)}catch(e){console.error("Error restarting system:",e),a("Error restarting system: "+e.message),E(!1)}}} 
        shutdownSystem=${async()=>{if(confirm("Are you sure you want to shut down the system?"))try{R(!0),a("Shutting down system..."),await g("/api/system/shutdown",{method:"POST",signal:F.current?.signal,timeout:3e4,retries:0}),a("System is shutting down. You will need to manually restart it.")}catch(e){console.error("Error shutting down system:",e),a("Error shutting down system: "+e.message),R(!1)}}} 
        isRestarting=${M} 
        isShuttingDown=${U} 
      />
      
      <${i}
        isLoading=${D}
        hasData=${A}
        loadingMessage="Loading system information..."
        emptyMessage="System information not available. Please try again later."
      >
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <${m} systemInfo=${e} formatUptime=${v} />
          <${y} systemInfo=${e} formatBytes=${w} />
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <${f} systemInfo=${e} />
          <${p} systemInfo=${e} formatBytes=${w} />
        </div>
        
        <${b} 
          logs=${r} 
          logLevel=${k} 
          logCount=${L} 
          setLogLevel=${e=>{console.log(`SystemView: Setting log level from ${k} to ${e}`),console.log("Current stack trace:",(new Error).stack),S(e),C.current=e,console.log(`SystemView: logLevelRef is now: ${C.current}`),setTimeout((()=>{console.log(`SystemView: After setState, logLevel is now: ${k}`),console.log(`SystemView: After setState, logLevelRef is now: ${C.current}`)}),0)}} 
          setLogCount=${I} 
          loadLogs=${P} 
          clearLogs=${async()=>{if(confirm("Are you sure you want to clear all logs?"))try{a("Clearing logs..."),await g("/api/system/logs/clear",{method:"POST",signal:F.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),a("Logs cleared successfully"),P()}catch(e){console.error("Error clearing logs:",e),a("Error clearing logs: "+e.message)}}} 
        />
        
        <${$}
          logLevel=${k}
          logCount=${L}
          onLogsReceived=${e=>{console.log("SystemView received new logs:",e.length);const s=C.current;console.log(`Filtering ${e.length} logs based on log level: ${s}`);const t=e.filter((e=>{const t=h(e.level,s);return"debug"!==e.level||"debug"!==s||t||console.warn("Debug log filtered out despite debug level selected:",e),t}));console.log(`After filtering: ${t.length} logs match the current log level`),x(t),setTimeout((()=>{const e=document.querySelector(".logs-container");e&&(e.scrollTop=e.scrollHeight)}),100)}}
        />
      <//>
    </section>
  `}e({SystemView:x,loadSystemView:function(){const e=document.getElementById("main-content");e&&r((async()=>{const{render:e}=await s.import("./preact-app-legacy-D5_kcW1v.js").then((e=>e.p));return{render:e}}),void 0,s.meta.url).then((({render:s})=>{s(t`<${x} />`,e),setTimeout((()=>{const e=new CustomEvent("refresh-system-info");window.dispatchEvent(e)}),100)}))}}),window.addEventListener("load",(()=>{window.addEventListener("refresh-system-info",(async()=>{try{(await g("/api/system/info",{timeout:15e3,retries:1,retryDelay:1e3})).ok&&console.log("System info refreshed")}catch(e){console.error("Error refreshing system info:",e)}}))}))}}}));
//# sourceMappingURL=SystemView-legacy-CrvAhySt.js.map
