System.register(["./query-client-legacy-BQ_U5NnD.js","./Footer-legacy-B5sb7m6V.js","./LoadingIndicator-legacy-Xson3LBG.js","./websocket-client-legacy-BO-Yc0A6.js"],(function(e,t){"use strict";var r,a,o,n,s,d,i,l,c,g,u,p,m,h,b,y,f,w,v,x,k,D;return{setters:[e=>{r=e.a,a=e.b,o=e.c,n=e.e,s=e.f,d=e.g,i=e.d,l=e.A,c=e.y,g=e.u,u=e.E,p=e.q,m=e.Q},e=>{h=e.h,b=e.c,y=e.D,f=e.d,w=e.H,v=e.F},e=>{x=e.C},e=>{k=e.W,D=e.B}],execute:function(){var e=document.createElement("style");function t(e,t=3e3){let r=document.getElementById("status-message-container");r||(r=document.createElement("div"),r.id="status-message-container",r.className="fixed bottom-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col items-center",document.body.appendChild(r));const a=document.createElement("div");a.className="bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg mb-2 transition-all duration-300 opacity-0 transform translate-y-2",a.textContent=e,r.appendChild(a),setTimeout((()=>{a.classList.remove("opacity-0","translate-y-2")}),10),setTimeout((()=>{a.classList.add("opacity-0","translate-y-2"),setTimeout((()=>{a.parentNode===r&&r.removeChild(a),0===r.children.length&&document.body.removeChild(r)}),300)}),t)}function $(){let e=document.getElementById("batch-delete-modal-container");e||(e=document.createElement("div"),e.id="batch-delete-modal-container",document.body.appendChild(e)),e.innerHTML='\n        <div id="batch-delete-modal" class="modal hidden fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center z-50">\n            <div class="modal-content relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md mx-auto p-6 w-full">\n                <div class="modal-header flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">\n                    <h3 id="batch-delete-modal-title" class="text-xl font-bold text-gray-900 dark:text-white">Batch Delete Progress</h3>\n                    <button id="batch-delete-close-btn" class="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 text-2xl font-bold">&times;</button>\n                </div>\n                <div class="modal-body">\n                    <div id="batch-delete-status" class="mb-4 text-gray-700 dark:text-gray-300">\n                        Preparing to delete recordings...\n                    </div>\n                    <div class="progress-container bg-gray-200 dark:bg-gray-700 rounded-full h-4 mb-4">\n                        <div id="batch-delete-progress-bar" class="bg-green-500 h-4 rounded-full text-center text-xs text-white" style="width: 0%"></div>\n                    </div>\n                    <div class="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-6">\n                        <div id="batch-delete-count">0 / 0</div>\n                        <div id="batch-delete-percentage">0%</div>\n                    </div>\n                    <div id="batch-delete-details" class="mb-4">\n                        <div class="flex justify-between mb-2">\n                            <span class="text-gray-700 dark:text-gray-300">Succeeded:</span>\n                            <span id="batch-delete-succeeded" class="font-bold text-green-600 dark:text-green-400">0</span>\n                        </div>\n                        <div class="flex justify-between">\n                            <span class="text-gray-700 dark:text-gray-300">Failed:</span>\n                            <span id="batch-delete-failed" class="font-bold text-red-600 dark:text-red-400">0</span>\n                        </div>\n                    </div>\n                    <div id="batch-delete-message" class="text-sm italic text-gray-600 dark:text-gray-400 mb-4"></div>\n                </div>\n                <div class="modal-footer flex justify-end pt-2 border-t border-gray-200 dark:border-gray-700">\n                    <button id="batch-delete-done-btn" class="btn btn-primary hidden">Done</button>\n                    <button id="batch-delete-cancel-btn" class="btn btn-secondary">Cancel</button>\n                </div>\n            </div>\n        </div>\n    ';const t=document.getElementById("batch-delete-modal"),r=document.getElementById("batch-delete-close-btn"),a=document.getElementById("batch-delete-done-btn"),o=document.getElementById("batch-delete-cancel-btn");r&&r.addEventListener("click",C),a&&a.addEventListener("click",C),o&&o.addEventListener("click",T),window.addEventListener("click",(e=>{e.target===t&&C()}))}function S(){const e=document.getElementById("batch-delete-modal");e||$(),function(){const e=document.getElementById("batch-delete-progress-bar");e&&(e.style.width="0%");const t=document.getElementById("batch-delete-status");t&&(t.textContent="Preparing to delete recordings...");const r=document.getElementById("batch-delete-count");r&&(r.textContent="0 / 0");const a=document.getElementById("batch-delete-percentage");a&&(a.textContent="0%");const o=document.getElementById("batch-delete-succeeded");o&&(o.textContent="0");const n=document.getElementById("batch-delete-failed");n&&(n.textContent="0");const s=document.getElementById("batch-delete-message");s&&(s.textContent="");const d=document.getElementById("batch-delete-done-btn");d&&d.classList.add("hidden");const i=document.getElementById("batch-delete-cancel-btn");i&&i.classList.remove("hidden")}(),e.classList.remove("hidden")}function C(){const e=document.getElementById("batch-delete-modal");e&&e.classList.add("hidden")}function T(){C(),t("Batch delete operation cancelled",5e3)}function R({filters:e,setFilters:t,pagination:r,setPagination:a,streams:o,filtersVisible:n,applyFilters:s,resetFilters:d,handleDateRangeChange:i,setDefaultDateRange:l}){return h`
    <aside id="filters-sidebar" 
           class=${"filters-sidebar w-full md:w-64 bg-white dark:bg-gray-800 rounded-lg shadow p-4 md:sticky md:top-4 md:self-start transition-all duration-300 "+(n?"":"hidden md:block")}>
      <div class="filter-group mb-4">
        <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Date Range</h3>
        <div class="filter-option mb-2">
          <label for="date-range-select" class="block mb-1 text-sm font-medium">Quick Select:</label>
          <select id="date-range-select" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${e.dateRange}
                  onChange=${i}>
            <option value="today">Today</option>
            <option value="yesterday">Yesterday</option>
            <option value="last7days">Last 7 Days</option>
            <option value="last30days">Last 30 Days</option>
            <option value="custom">Custom Range</option>
          </select>
        </div>
        
        <div id="custom-date-range" 
             class="filter-option space-y-3"
             style=${"custom"===e.dateRange?"display: block":"display: none"}>
          <div class="space-y-1">
            <label for="start-date" class="block text-sm font-medium">Start Date:</label>
            <input type="date" id="start-date" 
                   class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                   value=${e.startDate}
                   onChange=${e=>t((t=>({...t,startDate:e.target.value})))} />
            <div class="mt-1">
              <label for="start-time" class="block text-sm font-medium">Time:</label>
              <input type="time" id="start-time" 
                     class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                     value=${e.startTime}
                     onChange=${e=>t((t=>({...t,startTime:e.target.value})))} />
            </div>
          </div>
          
          <div class="space-y-1">
            <label for="end-date" class="block text-sm font-medium">End Date:</label>
            <input type="date" id="end-date" 
                   class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                   value=${e.endDate}
                   onChange=${e=>t((t=>({...t,endDate:e.target.value})))} />
            <div class="mt-1">
              <label for="end-time" class="block text-sm font-medium">Time:</label>
              <input type="time" id="end-time" 
                     class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                     value=${e.endTime}
                     onChange=${e=>t((t=>({...t,endTime:e.target.value})))} />
            </div>
          </div>
        </div>
      </div>
      
      <div class="filter-group mb-4">
        <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Stream</h3>
        <div class="filter-option">
          <select id="stream-filter" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${e.streamId}
                  onChange=${e=>t((t=>({...t,streamId:e.target.value})))}>
            <option value="all">All Streams</option>
            ${o.map((e=>h`
              <option key=${e.name} value=${e.name}>${e.name}</option>
            `))}
          </select>
        </div>
      </div>
      
      <div class="filter-group mb-4">
        <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Recording Type</h3>
        <div class="filter-option">
          <select id="detection-filter" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${e.recordingType}
                  onChange=${e=>t((t=>({...t,recordingType:e.target.value})))}>
            <option value="all">All Recordings</option>
            <option value="detection">Detection Events Only</option>
          </select>
        </div>
      </div>
      
      <div class="filter-group mb-4">
        <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Display Options</h3>
        <div class="filter-option">
          <label for="page-size" class="block mb-1 text-sm font-medium">Items per page:</label>
          <select id="page-size" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${r.pageSize}
                  onChange=${e=>a((t=>({...t,pageSize:parseInt(e.target.value,10)})))}>
            <option value="10">10</option>
            <option value="20">20</option>
            <option value="50">50</option>
            <option value="100">100</option>
          </select>
        </div>
      </div>
      
      <div class="filter-actions flex space-x-2">
        <button id="apply-filters-btn" 
                class="flex-1 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                onClick=${s}>
          Apply Filters
        </button>
        <button id="reset-filters-btn" 
                class="flex-1 px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                onClick=${d}>
          Reset
        </button>
      </div>
    </aside>
  `}function P({activeFiltersDisplay:e,removeFilter:t,hasActiveFilters:r}){return r?h`
    <div id="active-filters" 
         class="active-filters mb-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg flex flex-wrap gap-2">
      ${e.map(((e,r)=>h`
        <div key=${r} class="filter-tag inline-flex items-center px-3 py-1 rounded-full text-sm bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-200">
          <span>${e.label}</span>
          <button class="ml-2 text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 focus:outline-none"
                  onClick=${()=>t(e.key)}>
            ×
          </button>
        </div>
      `))}
    </div>
  `:null}e.textContent=".sortable{position:relative;cursor:pointer;-webkit-user-select:none;-moz-user-select:none;user-select:none}.sortable:hover{--tw-bg-opacity: 1;background-color:rgb(243 244 246 / var(--tw-bg-opacity, 1))}@media (prefers-color-scheme: dark){.sortable:hover{--tw-bg-opacity: 1;background-color:rgb(55 65 81 / var(--tw-bg-opacity, 1))}}.sort-icon{margin-left:.375rem;display:inline-block;width:.625rem;opacity:.5}.sort-asc .sort-icon,.sort-desc .sort-icon{opacity:1}.filters-sidebar{width:100%;flex-shrink:0;border-radius:.5rem;padding:1rem;--tw-shadow: 0 1px 3px 0 rgb(0 0 0 / .1), 0 1px 2px -1px rgb(0 0 0 / .1);--tw-shadow-colored: 0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color);box-shadow:var(--tw-ring-offset-shadow, 0 0 #0000),var(--tw-ring-shadow, 0 0 #0000),var(--tw-shadow)}@media (min-width: 768px){.filters-sidebar{width:18rem}}.filters-sidebar{transition:transform .3s ease,opacity .3s ease,width .3s ease}.filters-sidebar.collapsed{margin-top:0;margin-bottom:0;max-height:0px;overflow:hidden;padding-top:0;padding-bottom:0;opacity:0}.filter-tag{display:inline-flex;align-items:center;border-radius:9999px;--tw-bg-opacity: 1;background-color:rgb(219 234 254 / var(--tw-bg-opacity, 1));padding:.25rem .75rem;font-size:.875rem;line-height:1.25rem;--tw-text-opacity: 1;color:rgb(29 78 216 / var(--tw-text-opacity, 1))}@media (prefers-color-scheme: dark){.filter-tag{background-color:rgba(30,58,138,.3);--tw-text-opacity: 1;color:rgb(147 197 253 / var(--tw-text-opacity, 1))}}.filter-tag .remove-filter{margin-left:.5rem;cursor:pointer;font-size:1rem;line-height:1.5rem;font-weight:700;line-height:1}@media (max-width: 576px){#recordings-table{min-width:500px;max-width:100%}.recordings-container,.overflow-x-auto{max-width:100%}}\n/*$vite$:1*/",document.head.appendChild(e),window.updateBatchDeleteProgress=function(e){console.log("Updating batch delete progress UI:",e),S();const t=document.getElementById("batch-delete-progress-bar");if(t){if(console.log(`Updating progress bar: current=${e.current}, total=${e.total}`),e.total>0){const r=Math.round(e.current/e.total*100);console.log(`Setting progress bar width to ${r}%`),t.style.width=`${r}%`,t.classList.remove("animate-pulse"),t.offsetWidth}else if(e.current>0){const r=Math.min(90,e.current/10);console.log(`Setting progress bar width to ${r}% (estimated)`),t.style.width=`${r}%`,t.classList.add("animate-pulse"),t.offsetWidth}else e.complete?(console.log("Setting progress bar to 100% (complete)"),t.style.width="100%",t.classList.remove("animate-pulse"),t.offsetWidth):(console.log("Setting progress bar to 50% (indeterminate)"),t.style.width="50%",t.classList.add("animate-pulse"),t.offsetWidth);e.error?(t.classList.add("bg-red-500"),t.classList.remove("bg-blue-500")):(t.classList.add("bg-blue-500"),t.classList.remove("bg-red-500"))}const r=document.getElementById("batch-delete-status");r&&e.status&&(r.textContent=e.status);const a=document.getElementById("batch-delete-count");a&&(e.total>0?a.textContent=`${e.current} / ${e.total}`:a.textContent=`${e.current} / ?`);const o=document.getElementById("batch-delete-percentage");if(o)if(e.total>0){const t=Math.round(e.current/e.total*100);o.textContent=`${t}%`}else e.complete?o.textContent="100%":o.textContent="In progress";const n=document.getElementById("batch-delete-succeeded");n&&(n.textContent=e.succeeded||"0");const s=document.getElementById("batch-delete-failed");if(s&&(s.textContent=e.failed||"0"),e.complete){const a=document.getElementById("batch-delete-done-btn");a&&a.classList.remove("hidden");const o=document.getElementById("batch-delete-cancel-btn");o&&o.classList.add("hidden"),!r||e.status&&"Preparing to delete recordings..."!==e.status||(r.textContent="Batch delete operation complete"),t&&(t.style.width="100%",t.classList.remove("animate-pulse"))}},window.batchDeleteRecordingsByHttpRequest=function(e){return console.log("Using HTTP fallback for batch delete with params:",e),new Promise(((r,a)=>{S();let o=0;e.ids?o=e.ids.length:e.filter&&e.totalCount&&(o=e.totalCount),updateBatchDeleteProgress({current:0,total:o,status:"Using HTTP fallback for batch delete operation",succeeded:0,failed:0}),fetch("/api/recordings/batch-delete",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(e)}).then((e=>{if(!e.ok)throw new Error(`HTTP error ${e.status}: ${e.statusText}`);return e.json()})).then((e=>{console.log("HTTP batch delete result:",e);const a=e.total||o||0,n=e.succeeded||0,s=e.failed||0;updateBatchDeleteProgress({current:a,total:a,succeeded:n,failed:s,status:"Batch delete operation complete",complete:!0}),t(e.success?`Successfully deleted ${n} recordings`:`Deleted ${n} recordings with ${s} failures`,5e3),setTimeout((()=>{"function"==typeof loadRecordings&&loadRecordings()}),1e3),r(e)})).catch((e=>{console.error("HTTP batch delete error:",e),updateBatchDeleteProgress({current:0,total:0,succeeded:0,failed:0,status:`Error: ${e.message||"Unknown error"}`,complete:!0,error:!0}),t(`Error: ${e.message||"Unknown error"}`,5e3),a(e)}))}))},document.addEventListener("DOMContentLoaded",(()=>{if(console.log("Initializing batch delete modal"),$(),!document.getElementById("batch-delete-modal-container")){console.error("Batch delete modal container not found, creating it");const e=document.createElement("div");e.id="batch-delete-modal-container",document.body.appendChild(e),$()}"function"!=typeof window.updateBatchDeleteProgress&&(console.error("updateBatchDeleteProgress function not available, setting it up"),window.updateBatchDeleteProgress=updateBatchDeleteProgress),"function"!=typeof window.showBatchDeleteModal&&(console.error("showBatchDeleteModal function not available, setting it up"),window.showBatchDeleteModal=S)})),window.showBatchDeleteModal=S,window.updateBatchDeleteProgress=updateBatchDeleteProgress,window.initBatchDeleteModal=$;const I=e=>e?new Date(e).toLocaleString():"",E=e=>{if(!e)return"00:00:00";const t=Math.floor(e/3600),r=Math.floor(e%3600/60),a=Math.floor(e%60);return[t.toString().padStart(2,"0"),r.toString().padStart(2,"0"),a.toString().padStart(2,"0")].join(":")};function B({recordings:e,sortField:t,sortDirection:r,sortBy:a,selectedRecordings:o,toggleRecordingSelection:n,selectAll:s,toggleSelectAll:d,getSelectedCount:i,openDeleteModal:l,playRecording:c,downloadRecording:g,deleteRecording:u,recordingsTableBodyRef:p,pagination:m}){return h`
    <div class="recordings-container bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden w-full">
      <div class="batch-actions p-3 border-b border-gray-200 dark:border-gray-700 flex flex-wrap gap-2 items-center">
        <div class="selected-count text-sm text-gray-600 dark:text-gray-400 mr-2">
          ${i()>0?`${i()} recording${1!==i()?"s":""} selected`:"No recordings selected"}
        </div>
        <button 
          class="px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          disabled=${0===i()}
          onClick=${()=>l("selected")}>
          Delete Selected
        </button>
        <button 
          class="px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
          onClick=${()=>l("all")}>
          Delete All Filtered
        </button>
      </div>
      <div class="overflow-x-auto">
        <table id="recordings-table" class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th class="w-10 px-4 py-3">
                <input 
                  type="checkbox" 
                  checked=${s}
                  onChange=${d}
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                />
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${()=>a("stream_name")}>
                <div class="flex items-center">
                  Stream
                  ${"stream_name"===t&&h`
                    <span class="sort-icon ml-1">${"asc"===r?"▲":"▼"}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${()=>a("start_time")}>
                <div class="flex items-center">
                  Start Time
                  ${"start_time"===t&&h`
                    <span class="sort-icon ml-1">${"asc"===r?"▲":"▼"}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Duration
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${()=>a("size_bytes")}>
                <div class="flex items-center">
                  Size
                  ${"size_bytes"===t&&h`
                    <span class="sort-icon ml-1">${"asc"===r?"▲":"▼"}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Detections
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody ref=${p} class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
            ${0===e.length?h`
              <tr>
                <td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                  ${0===m.totalItems?"No recordings found":"Loading recordings..."}
                </td>
              </tr>
            `:e.map((e=>h`
              <tr key=${e.id} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                <td class="px-4 py-4 whitespace-nowrap">
                  <input 
                    type="checkbox" 
                    checked=${!!o[e.id]}
                    onChange=${()=>n(e.id)}
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                  />
                </td>
                <td class="px-6 py-4 whitespace-nowrap">${e.stream||""}</td>
                <td class="px-6 py-4 whitespace-nowrap">${I(e.start_time)}</td>
                <td class="px-6 py-4 whitespace-nowrap">${E(e.duration)}</td>
                <td class="px-6 py-4 whitespace-nowrap">${e.size||""}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  ${e.has_detections?h`
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100">
                      <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                        <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path>
                      </svg>
                      Yes
                    </span>
                  `:""}
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex space-x-2">
                    <button class="p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                            onClick=${()=>c(e)}
                            title="Play">
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                    <button class="p-1 rounded-full text-green-600 hover:bg-green-100 dark:text-green-400 dark:hover:bg-green-900 focus:outline-none"
                            onClick=${()=>g(e)}
                            title="Download">
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                    <button class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                            onClick=${()=>u(e)}
                            title="Delete">
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
  `}function L({pagination:e,goToPage:t}){return h`
    <div class="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-gray-200 dark:border-gray-700">
      <div class="pagination-info text-sm text-gray-600 dark:text-gray-400 mb-2 sm:mb-0">
        Showing <span id="pagination-showing">${e.startItem}-${e.endItem}</span> of <span id="pagination-total">${e.totalItems}</span> recordings
      </div>
      <div class="pagination-buttons flex items-center space-x-1">
        <button id="pagination-first" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="First Page"
                onClick=${()=>t(1)}
                disabled=${1===e.currentPage}>
          «
        </button>
        <button id="pagination-prev" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="Previous Page"
                onClick=${()=>t(e.currentPage-1)}
                disabled=${1===e.currentPage}>
          ‹
        </button>
        <span id="pagination-current" class="px-2 text-sm text-gray-700 dark:text-gray-300">
          Page ${e.currentPage} of ${e.totalPages}
        </span>
        <button id="pagination-next" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="Next Page"
                onClick=${()=>t(e.currentPage+1)}
                disabled=${e.currentPage===e.totalPages}>
          ›
        </button>
        <button id="pagination-last" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="Last Page"
                onClick=${()=>t(e.totalPages)}
                disabled=${e.currentPage===e.totalPages}>
          »
        </button>
      </div>
    </div>
  `}const M={hooks:{useStreams:()=>n("streams","/api/streams",{timeout:15e3,retries:2,retryDelay:1e3}),useRecordings:(e,t,r,a)=>{const o=new URLSearchParams;if(o.append("page",t.currentPage),o.append("limit",t.pageSize),o.append("sort",r),o.append("order",a),"custom"===e.dateRange)o.append("start",`${e.startDate}T${e.startTime}:00`),o.append("end",`${e.endDate}T${e.endTime}:00`);else{const{start:t,end:r}=M.getDateRangeFromPreset(e.dateRange);o.append("start",t),o.append("end",r)}return"all"!==e.streamId&&o.append("stream",e.streamId),"detection"===e.recordingType&&o.append("detection","1"),n(["recordings",e,t,r,a],`/api/recordings?${o.toString()}`,{timeout:3e4,retries:2,retryDelay:1e3},{onSuccess:e=>{e.recordings&&e.recordings.length>0&&e.recordings.forEach((e=>{e.has_detections=!1}))}})},useRecordingDetections:e=>{if(!(e&&e.id&&e.stream&&e.start_time&&e.end_time))return{data:{detections:[]}};const t=Math.floor(new Date(e.start_time).getTime()/1e3),r=Math.floor(new Date(e.end_time).getTime()/1e3),a=new URLSearchParams({start:t,end:r});return n(["recording-detections",e.id],`/api/detection/results/${e.stream}?${a.toString()}`,{timeout:15e3,retries:1,retryDelay:1e3},{enabled:!!e.id})},useDeleteRecording:()=>{const e=r();return o({mutationFn:async e=>{const t=`/api/recordings/${e}`;return await s(t,{method:"DELETE",timeout:15e3,retries:1,retryDelay:1e3})},onSuccess:()=>{e.invalidateQueries({queryKey:["recordings"]}),b("Recording deleted successfully")},onError:e=>{console.error("Error deleting recording:",e),b("Error deleting recording: "+e.message)}})},useBatchDeleteRecordings:()=>{const e=r();return a("/api/recordings/batch-delete",{timeout:6e4,retries:1,retryDelay:2e3},{onSuccess:t=>{e.invalidateQueries({queryKey:["recordings"]});const r=t.succeeded,a=t.failed;b(r>0&&0===a?`Successfully deleted ${r} recording${1!==r?"s":""}`:r>0&&a>0?`Deleted ${r} recording${1!==r?"s":""}, but failed to delete ${a}`:`Failed to delete ${a} recording${1!==a?"s":""}`)},onError:e=>{console.error("Error in batch delete operation:",e),b("Error in batch delete operation: "+e.message)}})}},loadStreams:async()=>{try{return await s("/api/streams",{timeout:15e3,retries:2,retryDelay:1e3})||[]}catch(e){return console.error("Error loading streams for filter:",e),b("Error loading streams: "+e.message),[]}},getDateRangeFromPreset:e=>{const t=new Date,r=new Date(t.getFullYear(),t.getMonth(),t.getDate(),23,59,59),a=new Date(t.getFullYear(),t.getMonth(),t.getDate(),0,0,0);let o,n;switch(e){case"today":o=a.toISOString(),n=r.toISOString();break;case"yesterday":const e=new Date(a);e.setDate(e.getDate()-1);const t=new Date(e);t.setHours(23,59,59),o=e.toISOString(),n=t.toISOString();break;case"last7days":const s=new Date(a);s.setDate(s.getDate()-7),o=s.toISOString(),n=r.toISOString();break;case"last30days":const d=new Date(a);d.setDate(d.getDate()-30),o=d.toISOString(),n=r.toISOString();break;default:const i=new Date(a);i.setDate(i.getDate()-7),o=i.toISOString(),n=r.toISOString()}return{start:o,end:n}},loadRecordings:async(e,t,r,a)=>{try{const o=new URLSearchParams;if(o.append("page",t.currentPage),o.append("limit",t.pageSize),o.append("sort",r),o.append("order",a),"custom"===e.dateRange)o.append("start",`${e.startDate}T${e.startTime}:00`),o.append("end",`${e.endDate}T${e.endTime}:00`);else{const{start:t,end:r}=M.getDateRangeFromPreset(e.dateRange);o.append("start",t),o.append("end",r)}"all"!==e.streamId&&o.append("stream",e.streamId),"detection"===e.recordingType&&o.append("detection","1"),console.log("API Request:",`/api/recordings?${o.toString()}`);const n=await s(`/api/recordings?${o.toString()}`,{timeout:3e4,retries:2,retryDelay:1e3});if(console.log("Recordings data received:",n),n.recordings&&n.recordings.length>0){const e=5;for(let t=0;t<n.recordings.length;t+=e){const r=n.recordings.slice(t,t+e);await Promise.all(r.map((async e=>{try{e.has_detections=await M.checkRecordingHasDetections(e)}catch(t){console.error(`Error checking detections for recording ${e.id}:`,t),e.has_detections=!1}})))}}return n}catch(o){throw console.error("Error loading recordings:",o),b("Error loading recordings: "+o.message),o}},deleteRecording:async e=>{try{return await d(`/api/recordings/${e.id}`,{method:"DELETE",timeout:15e3,retries:1,retryDelay:1e3}),b("Recording deleted successfully"),!0}catch(t){return console.error("Error deleting recording:",t),b("Error deleting recording: "+t.message),!1}},deleteSelectedRecordings:async e=>{const t=Object.entries(e).filter((([e,t])=>t)).map((([e,t])=>parseInt(e,10)));if(0===t.length)return b("No recordings selected"),{succeeded:0,failed:0};try{if(window.wsClient){if(console.log("Using WebSocket for batch delete operation"),window.wsClient.isConnected()||(console.log("WebSocket not connected, connecting now..."),window.wsClient.connect()),!window.batchDeleteClient){if("undefined"==typeof BatchDeleteRecordingsClient)return console.warn("BatchDeleteRecordingsClient not available, falling back to HTTP"),M.deleteSelectedRecordingsHttp(t);console.log("Creating new BatchDeleteRecordingsClient"),window.batchDeleteClient=new BatchDeleteRecordingsClient(window.wsClient)}return"function"==typeof showBatchDeleteModal&&showBatchDeleteModal(),await window.batchDeleteClient.deleteWithProgress({ids:t})}return console.log("WebSocket client not available, using HTTP for batch delete"),M.deleteSelectedRecordingsHttp(t)}catch(r){return console.error("Error in batch delete operation:",r),b("Error in batch delete operation: "+r.message),{succeeded:0,failed:0}}},deleteSelectedRecordingsHttp:async e=>{try{const t=await d("/api/recordings/batch-delete",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({ids:e}),timeout:6e4,retries:1,retryDelay:2e3}),r=await t.json(),a=r.succeeded,o=r.failed;return b(a>0&&0===o?`Successfully deleted ${a} recording${1!==a?"s":""}`:a>0&&o>0?`Deleted ${a} recording${1!==a?"s":""}, but failed to delete ${o}`:`Failed to delete ${o} recording${1!==o?"s":""}`),r}catch(t){return console.error("Error in HTTP batch delete operation:",t),b("Error in batch delete operation: "+t.message),{succeeded:0,failed:0}}},deleteAllFilteredRecordings:async e=>{try{const a={};if("custom"===e.dateRange)a.start=`${e.startDate}T${e.startTime}:00`,a.end=`${e.endDate}T${e.endTime}:00`;else{const{start:t,end:r}=M.getDateRangeFromPreset(e.dateRange);a.start=t,a.end=r}"all"!==e.streamId&&(a.stream_name=e.streamId),"detection"===e.recordingType&&(a.detection=1),console.log("Deleting with filter:",a),"function"==typeof showBatchDeleteModal&&(showBatchDeleteModal(),"function"==typeof window.updateBatchDeleteProgress&&window.updateBatchDeleteProgress({current:0,total:0,succeeded:0,failed:0,status:"Preparing to delete recordings matching filter...",complete:!1}));let o=0;try{const e=new URLSearchParams;a.start&&e.append("start",a.start),a.end&&e.append("end",a.end),a.stream_name&&e.append("stream",a.stream_name),a.detection&&e.append("detection","1"),e.append("page","1"),e.append("limit","1"),console.log("Getting total count with params:",e.toString());const t=await fetch(`/api/recordings?${e.toString()}`);if(t.ok){const e=await t.json();e&&e.pagination&&e.pagination.total&&(o=e.pagination.total,console.log(`Found ${o} recordings matching filter`),"function"==typeof window.updateBatchDeleteProgress&&window.updateBatchDeleteProgress({current:0,total:o,succeeded:0,failed:0,status:`Found ${o} recordings matching filter. Starting deletion...`,complete:!1}))}}catch(t){console.warn("Error getting recording count:",t)}const n=e=>(console.error("Error in delete all operation:",e),b("Error in delete all operation: "+e.message),"function"==typeof window.updateBatchDeleteProgress&&window.updateBatchDeleteProgress({current:0,total:0,succeeded:0,failed:0,status:`Error: ${e.message}`,complete:!0}),{succeeded:0,failed:0});if(!window.wsClient)return console.log("WebSocket client not available, using HTTP for batch delete with filter"),M.deleteAllFilteredRecordingsHttp(a);{if(console.log("Using WebSocket for batch delete with filter"),window.wsClient.isConnected()||(console.log("WebSocket not connected, connecting now..."),window.wsClient.connect()),!window.batchDeleteClient){if("undefined"==typeof BatchDeleteRecordingsClient)return console.warn("BatchDeleteRecordingsClient not available, falling back to HTTP"),M.deleteAllFilteredRecordingsHttp(a);console.log("Creating new BatchDeleteRecordingsClient for filtered delete"),window.batchDeleteClient=new BatchDeleteRecordingsClient(window.wsClient)}console.log("Using WebSocket client ID for filtered batch delete:",window.wsClient.getClientId());const e=new Promise(((e,t)=>{setTimeout((()=>{t(new Error("Operation timed out or server crashed. Some recordings may have been deleted."))}),6e4)}));try{return await Promise.race([window.batchDeleteClient.deleteWithProgress({filter:a,totalCount:o}),e])}catch(r){return console.error("WebSocket error or timeout:",r),setTimeout((()=>{"function"==typeof loadRecordings&&loadRecordings()}),1e3),n(r)}}}catch(a){return console.error("Error in delete all operation:",a),b("Error in delete all operation: "+a.message),{succeeded:0,failed:0}}},deleteAllFilteredRecordingsHttp:async e=>{try{const t=await d("/api/recordings/batch-delete",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({filter:e}),timeout:12e4,retries:1,retryDelay:3e3}),r=await t.json(),a=r.succeeded,o=r.failed;return b(a>0&&0===o?`Successfully deleted ${a} recording${1!==a?"s":""}`:a>0&&o>0?`Deleted ${a} recording${1!==a?"s":""}, but failed to delete ${o}`:`Failed to delete ${o} recording${1!==o?"s":""}`),r}catch(t){return console.error("Error in HTTP delete all operation:",t),b("Error in delete all operation: "+t.message),{succeeded:0,failed:0}}},checkRecordingHasDetections:async e=>{if(!(e&&e.id&&e.stream&&e.start_time&&e.end_time))return!1;try{const t=Math.floor(new Date(e.start_time).getTime()/1e3),r=Math.floor(new Date(e.end_time).getTime()/1e3),a=new URLSearchParams({start:t,end:r}),o=await s(`/api/detection/results/${e.stream}?${a.toString()}`,{timeout:1e4,retries:1,retryDelay:500});return o.detections&&o.detections.length>0}catch(t){return console.error("Error checking detections:",t),!1}},getRecordingDetections:async e=>{if(!(e&&e.id&&e.stream&&e.start_time&&e.end_time))return[];try{const t=Math.floor(new Date(e.start_time).getTime()/1e3),r=Math.floor(new Date(e.end_time).getTime()/1e3),a=new URLSearchParams({start:t,end:r});return(await s(`/api/detection/results/${e.stream}?${a.toString()}`,{timeout:15e3,retries:1,retryDelay:1e3})).detections||[]}catch(t){return console.error("Error getting detections:",t),[]}},playRecording:(e,t)=>{if(console.log("Play recording clicked:",e),!e.id)return console.error("Recording has no id property:",e),void b("Error: Recording has no id property");const r=`/api/recordings/play/${e.id}`,a=`${e.stream} - ${I(e.start_time)}`,o=`/api/recordings/download/${e.id}`;console.log("Video URL:",r),console.log("Title:",a),console.log("Download URL:",o),t(r,a,o),console.log("Video modal should be shown now")},downloadRecording:e=>{const t=`/api/recordings/download/${e.id}`,r=document.createElement("a");r.href=t,r.download=`${e.stream}_${new Date(e.start_time).toISOString().replace(/[:.]/g,"-")}.mp4`,document.body.appendChild(r),r.click(),document.body.removeChild(r),b("Download started")}},z=()=>{const e=new URLSearchParams(window.location.search);if(!(e.has("dateRange")||e.has("page")||e.has("sort")||e.has("detection")||e.has("stream")))return null;const t={filters:{dateRange:"last7days",startDate:"",startTime:"00:00",endDate:"",endTime:"23:59",streamId:"all",recordingType:"all"},page:1,limit:20,sort:"start_time",order:"desc"};return e.has("dateRange")&&(t.filters.dateRange=e.get("dateRange"),"custom"===t.filters.dateRange&&(e.has("startDate")&&(t.filters.startDate=e.get("startDate")),e.has("startTime")&&(t.filters.startTime=e.get("startTime")),e.has("endDate")&&(t.filters.endDate=e.get("endDate")),e.has("endTime")&&(t.filters.endTime=e.get("endTime")))),e.has("stream")&&(t.filters.streamId=e.get("stream")),e.has("detection")&&"1"===e.get("detection")&&(t.filters.recordingType="detection"),e.has("page")&&(t.page=parseInt(e.get("page"),10)),e.has("limit")&&(t.limit=parseInt(e.get("limit"),10)),e.has("sort")&&(t.sort=e.get("sort")),e.has("order")&&(t.order=e.get("order")),t},F=e=>{const t=[];if("last7days"!==e.dateRange||"all"!==e.streamId||"all"!==e.recordingType){if("last7days"!==e.dateRange){let r="";switch(e.dateRange){case"today":r="Today";break;case"yesterday":r="Yesterday";break;case"last30days":r="Last 30 Days";break;case"custom":r=`${e.startDate} to ${e.endDate}`}t.push({key:"dateRange",label:`Date: ${r}`})}"all"!==e.streamId&&t.push({key:"streamId",label:`Stream: ${e.streamId}`}),"all"!==e.recordingType&&t.push({key:"recordingType",label:"Detection Events Only"})}return t},_=(e,t,r,a)=>{const o=new URLSearchParams(window.location.search);o.set("t",Date.now().toString()),o.set("dateRange",e.dateRange),"custom"===e.dateRange?(o.set("startDate",e.startDate),o.set("startTime",e.startTime),o.set("endDate",e.endDate),o.set("endTime",e.endTime)):(o.delete("startDate"),o.delete("startTime"),o.delete("endDate"),o.delete("endTime")),"all"!==e.streamId?o.set("stream",e.streamId):o.delete("stream"),"detection"===e.recordingType?o.set("detection","1"):o.delete("detection"),o.set("page",t.currentPage.toString()),o.set("limit",t.pageSize.toString()),o.set("sort",r),o.set("order",a);const n=`${window.location.pathname}?${o.toString()}`;window.history.pushState({path:n},"",n);const s=n;window.onbeforeunload=function(){window.history.replaceState({path:s},"",s)}};function O(){const[e,t]=i([]),[a,o]=i([]),[n,s]=i(!0),[d,u]=i("start_time"),[p,m]=i("desc"),[h,w]=i({dateRange:"last7days",startDate:"",startTime:"00:00",endDate:"",endTime:"23:59",streamId:"all",recordingType:"all"}),[v,$]=i({currentPage:1,pageSize:20,totalItems:0,totalPages:1,startItem:0,endItem:0}),[S,C]=i(!1),[T,I]=i([]),[E,O]=i({}),[H,j]=i(!1),[A,U]=i(!1),[W,N]=i("selected"),V=l(null);r();const{data:q,isLoading:Y,error:J}=M.hooks.useStreams();c((()=>{q&&Array.isArray(q)&&o(q)}),[q]),c((()=>{J&&(console.error("Error loading streams for filter:",J),b("Error loading streams: "+J.message))}),[J]),c((()=>{if(void 0!==k){if(window.wsClient=new k,console.log("WebSocket client initialized at application level"),window.wsClient){console.log("Initial WebSocket connection state:",{connected:window.wsClient.isConnected(),clientId:window.wsClient.getClientId()});const e=window.wsClient.connect;window.wsClient.connect=function(){const t=e.apply(this,arguments);if(this.socket){const e=this.socket.onopen;this.socket.onopen=t=>{console.log("WebSocket connection opened at application level"),e&&e.call(this,t)};const t=this.socket.onerror;this.socket.onerror=e=>{console.error("WebSocket error at application level:",e),t&&t.call(this,e)};const r=this.socket.onclose;this.socket.onclose=e=>{console.log(`WebSocket connection closed at application level: ${e.code} ${e.reason}`),r&&r.call(this,e)};const a=this.socket.onmessage;this.socket.onmessage=e=>{e.data.includes('"type":"welcome"')||console.log("WebSocket message received at application level"),a&&a.call(this,e)}}return t};const t=window.wsClient.handleMessage;window.wsClient.handleMessage=function(e){const r=this.clientId;t.call(this,e);const a=this.clientId;r!==a&&a&&console.log(`WebSocket client ID changed at application level: ${a}`)}}void 0!==D&&(window.batchDeleteClient=new D(window.wsClient),console.log("Batch delete client initialized"))}Q();const e=z();if(e){console.log("Found URL filters:",e);const t=new URLSearchParams(window.location.search);t.has("detection")&&"1"===t.get("detection")&&(e.filters.recordingType="detection"),w(e.filters),$((t=>({...t,currentPage:e.page||1,pageSize:e.limit||20}))),u(e.sort||"start_time"),m(e.order||"desc")}return ee(),window.addEventListener("resize",ee),()=>{window.removeEventListener("resize",ee)}}),[]),c((()=>{oe()}),[h]);const Q=()=>{const e=new Date,t=new Date(e);t.setDate(e.getDate()-7),w((r=>({...r,endDate:e.toISOString().split("T")[0],startDate:t.toISOString().split("T")[0]})))},{data:K,isLoading:G,error:X,refetch:Z}=M.hooks.useRecordings(h,v,d,p);c((()=>{if(K){const e=K.recordings||[];t(e),re(e.length>0),K.pagination&&ae(K,v.currentPage)}}),[K]),c((()=>{X&&(console.error("Error loading recordings:",X),b("Error loading recordings: "+X.message),re(!1))}),[X]);const ee=()=>{window.innerWidth<768?s(!1):s(!0)},[te,re]=i(!1),ae=(e,t)=>{if(t=t||v.currentPage,e.pagination){const r=e.pagination.limit||20,a=e.pagination.total||0,o=e.pagination.pages||1;let n=0,s=0;e.recordings.length>0&&(n=(t-1)*r+1,s=Math.min(n+e.recordings.length-1,a)),console.log("Pagination update:",{currentPage:t,pageSize:r,totalItems:a,totalPages:o,startItem:n,endItem:s,recordingsLength:e.recordings.length}),$((e=>({...e,totalItems:a,totalPages:o,pageSize:r,startItem:n,endItem:s})))}else{const r=v.pageSize,a=e.total||0,o=Math.ceil(a/r)||1;let n=0,s=0;e.recordings.length>0&&(n=(t-1)*r+1,s=Math.min(n+e.recordings.length-1,a)),console.log("Pagination update (fallback):",{currentPage:t,pageSize:r,totalItems:a,totalPages:o,startItem:n,endItem:s,recordingsLength:e.recordings.length}),$((e=>({...e,totalItems:a,totalPages:o,startItem:n,endItem:s})))}},oe=()=>{const e=F(h);C(e.length>0),I(e)},ne=(e=!0)=>{e&&$((e=>({...e,currentPage:1}))),_(h,e?{...v,currentPage:1}:v,d,p)},se=()=>Object.values(E).filter(Boolean).length,de=()=>{U(!1)},ie=(e,r,a)=>{u(e),m(r),$((e=>({...e,currentPage:a}))),setTimeout((()=>{const o={...v,currentPage:a};_(h,o,e,r),M.loadRecordings(h,o,e,r).then((e=>{console.log("Recordings data received:",e),t(e.recordings||[]),ae(e,a)})).catch((e=>{console.error("Error loading recordings:",e),b("Error loading recordings: "+e.message)}))}),0)},{mutate:le}=M.hooks.useDeleteRecording();return g("section",{id:"recordings-page",class:"page",children:[g("div",{class:"page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow",children:[g("div",{class:"flex items-center",children:[g("h2",{class:"text-xl font-bold",children:"Recordings"}),g("div",{class:"ml-4 flex",children:[g("a",{href:"recordings.html",class:"px-3 py-1 bg-blue-500 text-white rounded-l-md",children:"Table View"}),g("a",{href:"timeline.html",class:"px-3 py-1 bg-gray-300 text-gray-700 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-400 dark:hover:bg-gray-600 rounded-r-md",children:"Timeline View"})]})]}),g("button",{id:"toggle-filters-btn",class:"p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none",title:"Toggle Filters",onClick:()=>{s(!n)},children:g("svg",{class:"w-5 h-5",fill:"currentColor",viewBox:"0 0 20 20",xmlns:"http://www.w3.org/2000/svg",children:g("path",{"fill-rule":"evenodd",d:"M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z","clip-rule":"evenodd"})})})]}),g("div",{class:"recordings-layout flex flex-col md:flex-row gap-4 w-full",children:[g(R,{filters:h,setFilters:w,pagination:v,setPagination:$,streams:a,filtersVisible:n,applyFilters:ne,resetFilters:()=>{const e={dateRange:"last7days",startDate:"",startTime:"00:00",endDate:"",endTime:"23:59",streamId:"all",recordingType:"all"},t=new Date,r=new Date(t);r.setDate(t.getDate()-7),e.endDate=t.toISOString().split("T")[0],e.startDate=r.toISOString().split("T")[0],w(e),$((e=>({...e,currentPage:1}))),u("start_time"),m("desc");const a=window.location.pathname;window.history.pushState({path:a},"",a)},handleDateRangeChange:e=>{const t=e.target.value;if(w((e=>({...e,dateRange:t}))),!("custom"!==t||h.startDate&&h.endDate)){const e=new Date,t=new Date(e);t.setDate(e.getDate()-7),w((r=>({...r,endDate:e.toISOString().split("T")[0],startDate:t.toISOString().split("T")[0]})))}},setDefaultDateRange:Q}),g("div",{class:"recordings-content flex-1",children:[g(P,{activeFiltersDisplay:T,removeFilter:e=>{switch(e){case"dateRange":w((e=>({...e,dateRange:"last7days"})));break;case"streamId":w((e=>({...e,streamId:"all"})));break;case"recordingType":w((e=>({...e,recordingType:"all"})))}ne()},hasActiveFilters:S}),g(x,{isLoading:G,hasData:te,loadingMessage:"Loading recordings...",emptyMessage:"No recordings found matching your criteria",children:[g(B,{recordings:e,sortField:d,sortDirection:p,sortBy:e=>{d===e?m("asc"===p?"desc":"asc"):(m("start_time"===e?"desc":"asc"),u(e)),$((e=>({...e,currentPage:1}))),_(h,{...v,currentPage:1},e,e===d?"asc"===p?"desc":"asc":"start_time"===e?"desc":"asc")},selectedRecordings:E,toggleRecordingSelection:e=>{O((t=>({...t,[e]:!t[e]})))},selectAll:H,toggleSelectAll:()=>{const t=!H;j(t);const r={};t&&e.forEach((e=>{r[e.id]=!0})),O(r)},getSelectedCount:se,openDeleteModal:e=>{N(e),U(!0)},playRecording:e=>{M.playRecording(e,f)},downloadRecording:e=>{M.downloadRecording(e)},deleteRecording:e=>{confirm(`Are you sure you want to delete this recording from ${e.stream}?`)&&le(e.id)},recordingsTableBodyRef:V,pagination:v}),g(L,{pagination:v,goToPage:e=>{e<1||e>v.totalPages||($((t=>({...t,currentPage:e}))),_(h,{...v,currentPage:e},d,p))}})]})]})]}),g(y,{isOpen:A,onClose:de,onConfirm:async()=>{de();const e=new URLSearchParams(window.location.search),t=e.get("sort")||d,r=e.get("order")||p,a=parseInt(e.get("page"),10)||v.currentPage;if("selected"===W){const e=await M.deleteSelectedRecordings(E);O({}),j(!1),e.succeeded>0&&ie(t,r,a)}else{const e=await M.deleteAllFilteredRecordings(h);O({}),j(!1),e.succeeded>0&&((e=v.currentPage,t=!0)=>{console.log("Loading recordings with filters:",JSON.stringify(h));const r={...v,currentPage:e};$(r),t&&_(h,r,d,p)})()}},mode:W,count:se()})]})}document.addEventListener("DOMContentLoaded",(()=>{const e=document.getElementById("main-content");e&&u(g(m,{client:p,children:[g(w,{}),g(O,{}),g(v,{})]}),e)}))}}}));
//# sourceMappingURL=recordings-legacy-Ba1nE7-M.js.map
