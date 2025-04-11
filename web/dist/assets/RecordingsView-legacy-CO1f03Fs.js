System.register(["./preact-app-legacy-CTlOn5Tw.js","./LoadingIndicator-legacy-DNL-Bq68.js"],(function(e,t){"use strict";var r,a,o,n,s,i,d,l,c,g,p,u,m,y,h;return{setters:[e=>{r=e.h,a=e.f,o=e.g,n=e.i,s=e.u,i=e.c,d=e.j,l=e.k,c=e.d,g=e.A,p=e.y,u=e.D,m=e._,y=e.l},e=>{h=e.C}],execute:function(){function b({filters:e,setFilters:t,pagination:a,setPagination:o,streams:n,filtersVisible:s,applyFilters:i,resetFilters:d,handleDateRangeChange:l,setDefaultDateRange:c}){return r`
    <aside id="filters-sidebar" 
           class=${"filters-sidebar w-full md:w-64 bg-white dark:bg-gray-800 rounded-lg shadow p-4 md:sticky md:top-4 md:self-start transition-all duration-300 "+(s?"":"hidden md:block")}>
      <div class="filter-group mb-4">
        <h3 class="text-lg font-medium mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">Date Range</h3>
        <div class="filter-option mb-2">
          <label for="date-range-select" class="block mb-1 text-sm font-medium">Quick Select:</label>
          <select id="date-range-select" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${e.dateRange}
                  onChange=${l}>
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
            ${n.map((e=>r`
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
                  value=${a.pageSize}
                  onChange=${e=>o((t=>({...t,pageSize:parseInt(e.target.value,10)})))}>
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
                onClick=${i}>
          Apply Filters
        </button>
        <button id="reset-filters-btn" 
                class="flex-1 px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                onClick=${d}>
          Reset
        </button>
      </div>
    </aside>
  `}function f({activeFiltersDisplay:e,removeFilter:t,hasActiveFilters:a}){return a?r`
    <div id="active-filters" 
         class="active-filters mb-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg flex flex-wrap gap-2">
      ${e.map(((e,a)=>r`
        <div key=${a} class="filter-tag inline-flex items-center px-3 py-1 rounded-full text-sm bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-200">
          <span>${e.label}</span>
          <button class="ml-2 text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 focus:outline-none"
                  onClick=${()=>t(e.key)}>
            ×
          </button>
        </div>
      `))}
    </div>
  `:null}e({RecordingsView:k,loadRecordingsView:function(){const e=document.getElementById("main-content");e&&m((async()=>{const{render:e}=await t.import("./preact-app-legacy-CTlOn5Tw.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:a})=>{m((async()=>{const{QueryClientProvider:e,queryClient:r}=await t.import("./preact-app-legacy-CTlOn5Tw.js").then((e=>e.o));return{QueryClientProvider:e,queryClient:r}}),void 0,t.meta.url).then((({QueryClientProvider:t,queryClient:o})=>{a(r`<${t} client=${o}><${k} /></${t}>`,e)}))}))}});const w={formatDateTime:e=>e?new Date(e).toLocaleString():"",formatDuration:e=>{if(!e)return"00:00:00";const t=Math.floor(e/3600),r=Math.floor(e%3600/60),a=Math.floor(e%60);return[t.toString().padStart(2,"0"),r.toString().padStart(2,"0"),a.toString().padStart(2,"0")].join(":")},formatFileSize:e=>{if(!e)return"0 B";const t=["B","KB","MB","GB","TB"];let r=0,a=e;for(;a>=1024&&r<t.length-1;)a/=1024,r++;return`${a.toFixed(1)} ${t[r]}`}};function v({recordings:e,sortField:t,sortDirection:a,sortBy:o,selectedRecordings:n,toggleRecordingSelection:s,selectAll:i,toggleSelectAll:d,getSelectedCount:l,openDeleteModal:c,playRecording:g,downloadRecording:p,deleteRecording:u,recordingsTableBodyRef:m,pagination:y}){return r`
    <div class="recordings-container bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden w-full">
      <div class="batch-actions p-3 border-b border-gray-200 dark:border-gray-700 flex flex-wrap gap-2 items-center">
        <div class="selected-count text-sm text-gray-600 dark:text-gray-400 mr-2">
          ${l()>0?`${l()} recording${1!==l()?"s":""} selected`:"No recordings selected"}
        </div>
        <button 
          class="px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          disabled=${0===l()}
          onClick=${()=>c("selected")}>
          Delete Selected
        </button>
        <button 
          class="px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
          onClick=${()=>c("all")}>
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
                  checked=${i}
                  onChange=${d}
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                />
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${()=>o("stream_name")}>
                <div class="flex items-center">
                  Stream
                  ${"stream_name"===t&&r`
                    <span class="sort-icon ml-1">${"asc"===a?"▲":"▼"}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${()=>o("start_time")}>
                <div class="flex items-center">
                  Start Time
                  ${"start_time"===t&&r`
                    <span class="sort-icon ml-1">${"asc"===a?"▲":"▼"}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Duration
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${()=>o("size_bytes")}>
                <div class="flex items-center">
                  Size
                  ${"size_bytes"===t&&r`
                    <span class="sort-icon ml-1">${"asc"===a?"▲":"▼"}</span>
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
          <tbody ref=${m} class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
            ${0===e.length?r`
              <tr>
                <td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                  ${0===y.totalItems?"No recordings found":"Loading recordings..."}
                </td>
              </tr>
            `:e.map((e=>r`
              <tr key=${e.id} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                <td class="px-4 py-4 whitespace-nowrap">
                  <input 
                    type="checkbox" 
                    checked=${!!n[e.id]}
                    onChange=${()=>s(e.id)}
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                  />
                </td>
                <td class="px-6 py-4 whitespace-nowrap">${e.stream||""}</td>
                <td class="px-6 py-4 whitespace-nowrap">${w.formatDateTime(e.start_time)}</td>
                <td class="px-6 py-4 whitespace-nowrap">${w.formatDuration(e.duration)}</td>
                <td class="px-6 py-4 whitespace-nowrap">${e.size||""}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  ${e.has_detections?r`
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
                            onClick=${()=>g(e)}
                            title="Play">
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                    <button class="p-1 rounded-full text-green-600 hover:bg-green-100 dark:text-green-400 dark:hover:bg-green-900 focus:outline-none"
                            onClick=${()=>p(e)}
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
  `}function $({pagination:e,goToPage:t}){return r`
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
  `}const D={hooks:{useStreams:()=>s("streams","/api/streams",{timeout:15e3,retries:2,retryDelay:1e3}),useRecordings:(e,t,r,a)=>{const o=new URLSearchParams;if(o.append("page",t.currentPage),o.append("limit",t.pageSize),o.append("sort",r),o.append("order",a),"custom"===e.dateRange)o.append("start",`${e.startDate}T${e.startTime}:00`),o.append("end",`${e.endDate}T${e.endTime}:00`);else{const{start:t,end:r}=D.getDateRangeFromPreset(e.dateRange);o.append("start",t),o.append("end",r)}return"all"!==e.streamId&&o.append("stream",e.streamId),"detection"===e.recordingType&&o.append("detection","1"),s(["recordings",e,t,r,a],`/api/recordings?${o.toString()}`,{timeout:3e4,retries:2,retryDelay:1e3},{onSuccess:async e=>{if(e.recordings&&e.recordings.length>0){const t=5;for(let r=0;r<e.recordings.length;r+=t){const a=e.recordings.slice(r,r+t);await Promise.all(a.map((async e=>{try{e.has_detections=await D.checkRecordingHasDetections(e)}catch(t){console.error(`Error checking detections for recording ${e.id}:`,t),e.has_detections=!1}})))}}}})},useRecordingDetections:e=>{if(!(e&&e.id&&e.stream&&e.start_time&&e.end_time))return{data:{detections:[]}};const t=Math.floor(new Date(e.start_time).getTime()/1e3),r=Math.floor(new Date(e.end_time).getTime()/1e3),a=new URLSearchParams({start:t,end:r});return s(["recording-detections",e.id],`/api/detection/results/${e.stream}?${a.toString()}`,{timeout:15e3,retries:1,retryDelay:1e3},{enabled:!!e.id})},useDeleteRecording:()=>{const e=a();return n("/api/recordings/:id",{timeout:15e3,retries:1,retryDelay:1e3},{onSuccess:()=>{e.invalidateQueries({queryKey:["recordings"]}),i("Recording deleted successfully")},onError:e=>{console.error("Error deleting recording:",e),i("Error deleting recording: "+e.message)}})},useBatchDeleteRecordings:()=>{const e=a();return o("/api/recordings/batch-delete",{timeout:6e4,retries:1,retryDelay:2e3},{onSuccess:t=>{e.invalidateQueries({queryKey:["recordings"]});const r=t.succeeded,a=t.failed;i(r>0&&0===a?`Successfully deleted ${r} recording${1!==r?"s":""}`:r>0&&a>0?`Deleted ${r} recording${1!==r?"s":""}, but failed to delete ${a}`:`Failed to delete ${a} recording${1!==a?"s":""}`)},onError:e=>{console.error("Error in batch delete operation:",e),i("Error in batch delete operation: "+e.message)}})}},loadStreams:async()=>{try{return await d("/api/streams",{timeout:15e3,retries:2,retryDelay:1e3})||[]}catch(e){return console.error("Error loading streams for filter:",e),i("Error loading streams: "+e.message),[]}},getDateRangeFromPreset:e=>{const t=new Date,r=new Date(t.getFullYear(),t.getMonth(),t.getDate(),23,59,59),a=new Date(t.getFullYear(),t.getMonth(),t.getDate(),0,0,0);let o,n;switch(e){case"today":o=a.toISOString(),n=r.toISOString();break;case"yesterday":const e=new Date(a);e.setDate(e.getDate()-1);const t=new Date(e);t.setHours(23,59,59),o=e.toISOString(),n=t.toISOString();break;case"last7days":const s=new Date(a);s.setDate(s.getDate()-7),o=s.toISOString(),n=r.toISOString();break;case"last30days":const i=new Date(a);i.setDate(i.getDate()-30),o=i.toISOString(),n=r.toISOString();break;default:const d=new Date(a);d.setDate(d.getDate()-7),o=d.toISOString(),n=r.toISOString()}return{start:o,end:n}},loadRecordings:async(e,t,r,a)=>{try{const o=new URLSearchParams;if(o.append("page",t.currentPage),o.append("limit",t.pageSize),o.append("sort",r),o.append("order",a),"custom"===e.dateRange)o.append("start",`${e.startDate}T${e.startTime}:00`),o.append("end",`${e.endDate}T${e.endTime}:00`);else{const{start:t,end:r}=D.getDateRangeFromPreset(e.dateRange);o.append("start",t),o.append("end",r)}"all"!==e.streamId&&o.append("stream",e.streamId),"detection"===e.recordingType&&o.append("detection","1"),console.log("API Request:",`/api/recordings?${o.toString()}`);const n=await d(`/api/recordings?${o.toString()}`,{timeout:3e4,retries:2,retryDelay:1e3});if(console.log("Recordings data received:",n),n.recordings&&n.recordings.length>0){const e=5;for(let t=0;t<n.recordings.length;t+=e){const r=n.recordings.slice(t,t+e);await Promise.all(r.map((async e=>{try{e.has_detections=await D.checkRecordingHasDetections(e)}catch(t){console.error(`Error checking detections for recording ${e.id}:`,t),e.has_detections=!1}})))}}return n}catch(o){throw console.error("Error loading recordings:",o),i("Error loading recordings: "+o.message),o}},deleteRecording:async e=>{try{return await l(`/api/recordings/${e.id}`,{method:"DELETE",timeout:15e3,retries:1,retryDelay:1e3}),i("Recording deleted successfully"),!0}catch(t){return console.error("Error deleting recording:",t),i("Error deleting recording: "+t.message),!1}},deleteSelectedRecordings:async e=>{const t=Object.entries(e).filter((([e,t])=>t)).map((([e,t])=>parseInt(e,10)));if(0===t.length)return i("No recordings selected"),{succeeded:0,failed:0};try{if(window.wsClient&&window.wsClient.isConnected()){if(console.log("Using WebSocket for batch delete operation"),!window.batchDeleteClient){if("undefined"==typeof BatchDeleteRecordingsClient)return console.warn("BatchDeleteRecordingsClient not available, falling back to HTTP"),D.deleteSelectedRecordingsHttp(t);window.batchDeleteClient=new BatchDeleteRecordingsClient(window.wsClient)}return"function"==typeof showBatchDeleteModal&&showBatchDeleteModal(),await window.batchDeleteClient.deleteWithProgress({ids:t})}return console.log("WebSocket not connected, using HTTP for batch delete"),D.deleteSelectedRecordingsHttp(t)}catch(r){return console.error("Error in batch delete operation:",r),i("Error in batch delete operation: "+r.message),{succeeded:0,failed:0}}},deleteSelectedRecordingsHttp:async e=>{try{const t=await l("/api/recordings/batch-delete",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({ids:e}),timeout:6e4,retries:1,retryDelay:2e3}),r=await t.json(),a=r.succeeded,o=r.failed;return i(a>0&&0===o?`Successfully deleted ${a} recording${1!==a?"s":""}`:a>0&&o>0?`Deleted ${a} recording${1!==a?"s":""}, but failed to delete ${o}`:`Failed to delete ${o} recording${1!==o?"s":""}`),r}catch(t){return console.error("Error in HTTP batch delete operation:",t),i("Error in batch delete operation: "+t.message),{succeeded:0,failed:0}}},deleteAllFilteredRecordings:async e=>{try{const a={};if("custom"===e.dateRange)a.start=`${e.startDate}T${e.startTime}:00`,a.end=`${e.endDate}T${e.endTime}:00`;else{const{start:t,end:r}=D.getDateRangeFromPreset(e.dateRange);a.start=t,a.end=r}"all"!==e.streamId&&(a.stream_name=e.streamId),"detection"===e.recordingType&&(a.detection=1),console.log("Deleting with filter:",a),"function"==typeof showBatchDeleteModal&&(showBatchDeleteModal(),"function"==typeof window.updateBatchDeleteProgress&&window.updateBatchDeleteProgress({current:0,total:0,succeeded:0,failed:0,status:"Preparing to delete recordings matching filter...",complete:!1}));let o=0;try{const e=new URLSearchParams;a.start&&e.append("start",a.start),a.end&&e.append("end",a.end),a.stream_name&&e.append("stream",a.stream_name),a.detection&&e.append("detection","1"),e.append("page","1"),e.append("limit","1"),console.log("Getting total count with params:",e.toString());const t=await fetch(`/api/recordings?${e.toString()}`);if(t.ok){const e=await t.json();e&&e.pagination&&e.pagination.total&&(o=e.pagination.total,console.log(`Found ${o} recordings matching filter`),"function"==typeof window.updateBatchDeleteProgress&&window.updateBatchDeleteProgress({current:0,total:o,succeeded:0,failed:0,status:`Found ${o} recordings matching filter. Starting deletion...`,complete:!1}))}}catch(t){console.warn("Error getting recording count:",t)}const n=e=>(console.error("Error in delete all operation:",e),i("Error in delete all operation: "+e.message),"function"==typeof window.updateBatchDeleteProgress&&window.updateBatchDeleteProgress({current:0,total:0,succeeded:0,failed:0,status:`Error: ${e.message}`,complete:!0}),{succeeded:0,failed:0});if(!window.wsClient||!window.wsClient.isConnected())return console.log("WebSocket not connected, using HTTP for batch delete with filter"),D.deleteAllFilteredRecordingsHttp(a);{if(console.log("Using WebSocket for batch delete with filter"),!window.batchDeleteClient){if("undefined"==typeof BatchDeleteRecordingsClient)return console.warn("BatchDeleteRecordingsClient not available, falling back to HTTP"),D.deleteAllFilteredRecordingsHttp(a);window.batchDeleteClient=new BatchDeleteRecordingsClient(window.wsClient)}const e=new Promise(((e,t)=>{setTimeout((()=>{t(new Error("Operation timed out or server crashed. Some recordings may have been deleted."))}),6e4)}));try{return await Promise.race([window.batchDeleteClient.deleteWithProgress({filter:a,totalCount:o}),e])}catch(r){return console.error("WebSocket error or timeout:",r),setTimeout((()=>{"function"==typeof loadRecordings&&loadRecordings()}),1e3),n(r)}}}catch(a){return console.error("Error in delete all operation:",a),i("Error in delete all operation: "+a.message),{succeeded:0,failed:0}}},deleteAllFilteredRecordingsHttp:async e=>{try{const t=await l("/api/recordings/batch-delete",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({filter:e}),timeout:12e4,retries:1,retryDelay:3e3}),r=await t.json(),a=r.succeeded,o=r.failed;return i(a>0&&0===o?`Successfully deleted ${a} recording${1!==a?"s":""}`:a>0&&o>0?`Deleted ${a} recording${1!==a?"s":""}, but failed to delete ${o}`:`Failed to delete ${o} recording${1!==o?"s":""}`),r}catch(t){return console.error("Error in HTTP delete all operation:",t),i("Error in delete all operation: "+t.message),{succeeded:0,failed:0}}},checkRecordingHasDetections:async e=>{if(!(e&&e.id&&e.stream&&e.start_time&&e.end_time))return!1;try{const t=Math.floor(new Date(e.start_time).getTime()/1e3),r=Math.floor(new Date(e.end_time).getTime()/1e3),a=new URLSearchParams({start:t,end:r}),o=await d(`/api/detection/results/${e.stream}?${a.toString()}`,{timeout:1e4,retries:1,retryDelay:500});return o.detections&&o.detections.length>0}catch(t){return console.error("Error checking detections:",t),!1}},getRecordingDetections:async e=>{if(!(e&&e.id&&e.stream&&e.start_time&&e.end_time))return[];try{const t=Math.floor(new Date(e.start_time).getTime()/1e3),r=Math.floor(new Date(e.end_time).getTime()/1e3),a=new URLSearchParams({start:t,end:r});return(await d(`/api/detection/results/${e.stream}?${a.toString()}`,{timeout:15e3,retries:1,retryDelay:1e3})).detections||[]}catch(t){return console.error("Error getting detections:",t),[]}},playRecording:(e,t)=>{if(console.log("Play recording clicked:",e),!e.id)return console.error("Recording has no id property:",e),void i("Error: Recording has no id property");const r=`/api/recordings/play/${e.id}`,a=`${e.stream} - ${w.formatDateTime(e.start_time)}`,o=`/api/recordings/download/${e.id}`;console.log("Video URL:",r),console.log("Title:",a),console.log("Download URL:",o),t(r,a,o),console.log("Video modal should be shown now")},downloadRecording:e=>{const t=`/api/recordings/download/${e.id}`,r=document.createElement("a");r.href=t,r.download=`${e.stream}_${new Date(e.start_time).toISOString().replace(/[:.]/g,"-")}.mp4`,document.body.appendChild(r),r.click(),document.body.removeChild(r),i("Download started")}},x={getFiltersFromUrl:()=>{const e=new URLSearchParams(window.location.search);if(!(e.has("dateRange")||e.has("page")||e.has("sort")||e.has("detection")||e.has("stream")))return null;const t={filters:{dateRange:"last7days",startDate:"",startTime:"00:00",endDate:"",endTime:"23:59",streamId:"all",recordingType:"all"},page:1,limit:20,sort:"start_time",order:"desc"};return e.has("dateRange")&&(t.filters.dateRange=e.get("dateRange"),"custom"===t.filters.dateRange&&(e.has("startDate")&&(t.filters.startDate=e.get("startDate")),e.has("startTime")&&(t.filters.startTime=e.get("startTime")),e.has("endDate")&&(t.filters.endDate=e.get("endDate")),e.has("endTime")&&(t.filters.endTime=e.get("endTime")))),e.has("stream")&&(t.filters.streamId=e.get("stream")),e.has("detection")&&"1"===e.get("detection")&&(t.filters.recordingType="detection"),e.has("page")&&(t.page=parseInt(e.get("page"),10)),e.has("limit")&&(t.limit=parseInt(e.get("limit"),10)),e.has("sort")&&(t.sort=e.get("sort")),e.has("order")&&(t.order=e.get("order")),t},getActiveFiltersDisplay:e=>{const t=[];if("last7days"!==e.dateRange||"all"!==e.streamId||"all"!==e.recordingType){if("last7days"!==e.dateRange){let r="";switch(e.dateRange){case"today":r="Today";break;case"yesterday":r="Yesterday";break;case"last30days":r="Last 30 Days";break;case"custom":r=`${e.startDate} to ${e.endDate}`}t.push({key:"dateRange",label:`Date: ${r}`})}"all"!==e.streamId&&t.push({key:"streamId",label:`Stream: ${e.streamId}`}),"all"!==e.recordingType&&t.push({key:"recordingType",label:"Detection Events Only"})}return t},loadFiltersFromUrl:(e,t,r,a,o,n)=>{const s=new URLSearchParams(window.location.search),i={...e};s.has("dateRange")&&(i.dateRange=s.get("dateRange"),"custom"===i.dateRange&&(s.has("startDate")&&(i.startDate=s.get("startDate")),s.has("startTime")&&(i.startTime=s.get("startTime")),s.has("endDate")&&(i.endDate=s.get("endDate")),s.has("endTime")&&(i.endTime=s.get("endTime")))),s.has("stream")&&(i.streamId=s.get("stream")),s.has("detection")&&"1"===s.get("detection")&&(i.recordingType="detection"),r(i),s.has("page")&&a((e=>({...e,currentPage:parseInt(s.get("page"),10)}))),s.has("limit")&&a((e=>({...e,pageSize:parseInt(s.get("limit"),10)}))),s.has("sort")&&o(s.get("sort")),s.has("order")&&n(s.get("order"))},updateUrlWithFilters:(e,t,r,a)=>{const o=new URLSearchParams(window.location.search);o.set("t",Date.now().toString()),o.set("dateRange",e.dateRange),"custom"===e.dateRange?(o.set("startDate",e.startDate),o.set("startTime",e.startTime),o.set("endDate",e.endDate),o.set("endTime",e.endTime)):(o.delete("startDate"),o.delete("startTime"),o.delete("endDate"),o.delete("endTime")),"all"!==e.streamId?o.set("stream",e.streamId):o.delete("stream"),"detection"===e.recordingType?o.set("detection","1"):o.delete("detection"),o.set("page",t.currentPage.toString()),o.set("limit",t.pageSize.toString()),o.set("sort",r),o.set("order",a);const n=`${window.location.pathname}?${o.toString()}`;window.history.pushState({path:n},"",n);const s=n;window.onbeforeunload=function(){window.history.replaceState({path:s},"",s)}}};function k(){const[e,t]=c([]),[o,n]=c([]),[s,d]=c(!0),[l,m]=c("start_time"),[w,k]=c("desc"),[S,R]=c({dateRange:"last7days",startDate:"",startTime:"00:00",endDate:"",endTime:"23:59",streamId:"all",recordingType:"all"}),[T,P]=c({currentPage:1,pageSize:20,totalItems:0,totalPages:1,startItem:0,endItem:0}),[C,I]=c(!1),[F,E]=c([]),[_,B]=c({}),[M,z]=c(!1),[L,O]=c(!1),[A,H]=c("selected"),U=g(null);a();const{data:j,isLoading:V,error:W}=D.hooks.useStreams();p((()=>{j&&Array.isArray(j)&&n(j)}),[j]),p((()=>{W&&(console.error("Error loading streams for filter:",W),i("Error loading streams: "+W.message))}),[W]),p((()=>{N();const e=x.getFiltersFromUrl();if(e){console.log("Found URL filters:",e);const t=new URLSearchParams(window.location.search);t.has("detection")&&"1"===t.get("detection")&&(e.filters.recordingType="detection"),R(e.filters),P((t=>({...t,currentPage:e.page||1,pageSize:e.limit||20}))),m(e.sort||"start_time"),k(e.order||"desc")}return G(),window.addEventListener("resize",G),()=>{window.removeEventListener("resize",G)}}),[]),p((()=>{ee()}),[S]);const N=()=>{const e=new Date,t=new Date(e);t.setDate(e.getDate()-7),R((r=>({...r,endDate:e.toISOString().split("T")[0],startDate:t.toISOString().split("T")[0]})))},{data:q,isLoading:Q,error:Y,refetch:K}=D.hooks.useRecordings(S,T,l,w);p((()=>{if(q){const e=q.recordings||[];t(e),X(e.length>0),q.pagination&&Z(q,T.currentPage)}}),[q]),p((()=>{Y&&(console.error("Error loading recordings:",Y),i("Error loading recordings: "+Y.message),X(!1))}),[Y]);const G=()=>{window.innerWidth<768?d(!1):d(!0)},[J,X]=c(!1),Z=(e,t)=>{if(t=t||T.currentPage,e.pagination){const r=e.pagination.limit||20,a=e.pagination.total||0,o=e.pagination.pages||1;let n=0,s=0;e.recordings.length>0&&(n=(t-1)*r+1,s=Math.min(n+e.recordings.length-1,a)),console.log("Pagination update:",{currentPage:t,pageSize:r,totalItems:a,totalPages:o,startItem:n,endItem:s,recordingsLength:e.recordings.length}),P((e=>({...e,totalItems:a,totalPages:o,pageSize:r,startItem:n,endItem:s})))}else{const r=T.pageSize,a=e.total||0,o=Math.ceil(a/r)||1;let n=0,s=0;e.recordings.length>0&&(n=(t-1)*r+1,s=Math.min(n+e.recordings.length-1,a)),console.log("Pagination update (fallback):",{currentPage:t,pageSize:r,totalItems:a,totalPages:o,startItem:n,endItem:s,recordingsLength:e.recordings.length}),P((e=>({...e,totalItems:a,totalPages:o,startItem:n,endItem:s})))}},ee=()=>{const e=x.getActiveFiltersDisplay(S);I(e.length>0),E(e)},te=(e=!0)=>{e&&P((e=>({...e,currentPage:1}))),x.updateUrlWithFilters(S,e?{...T,currentPage:1}:T,l,w)},re=()=>Object.values(_).filter(Boolean).length,ae=()=>{O(!1)},{mutate:oe}=D.hooks.useBatchDeleteRecordings(),{mutate:ne}=D.hooks.useDeleteRecording();return r`
    <section id="recordings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div class="flex items-center">
          <h2 class="text-xl font-bold">Recordings</h2>
          <div class="ml-4 flex">
            <a href="recordings.html" class="px-3 py-1 bg-blue-500 text-white rounded-l-md">Table View</a>
            <a href="timeline.html" class="px-3 py-1 bg-gray-300 text-gray-700 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-400 dark:hover:bg-gray-600 rounded-r-md">Timeline View</a>
          </div>
        </div>
        <button id="toggle-filters-btn"
                class="p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
                title="Toggle Filters"
                onClick=${()=>{d(!s)}}>
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
          </svg>
        </button>
      </div>

      <div class="recordings-layout flex flex-col md:flex-row gap-4 w-full">
        <!-- Sidebar for filters -->
        <${b}
          filters=${S}
          setFilters=${R}
          pagination=${T}
          setPagination=${P}
          streams=${o}
          filtersVisible=${s}
          applyFilters=${te}
          resetFilters=${()=>{const e={dateRange:"last7days",startDate:"",startTime:"00:00",endDate:"",endTime:"23:59",streamId:"all",recordingType:"all"},t=new Date,r=new Date(t);r.setDate(t.getDate()-7),e.endDate=t.toISOString().split("T")[0],e.startDate=r.toISOString().split("T")[0],R(e),P((e=>({...e,currentPage:1}))),m("start_time"),k("desc");const a=window.location.pathname;window.history.pushState({path:a},"",a)}}
          handleDateRangeChange=${e=>{const t=e.target.value;if(R((e=>({...e,dateRange:t}))),!("custom"!==t||S.startDate&&S.endDate)){const e=new Date,t=new Date(e);t.setDate(e.getDate()-7),R((r=>({...r,endDate:e.toISOString().split("T")[0],startDate:t.toISOString().split("T")[0]})))}}}
          setDefaultDateRange=${N}
        />

        <!-- Main content area -->
        <div class="recordings-content flex-1">
          <${f}
            activeFiltersDisplay=${F}
            removeFilter=${e=>{switch(e){case"dateRange":R((e=>({...e,dateRange:"last7days"})));break;case"streamId":R((e=>({...e,streamId:"all"})));break;case"recordingType":R((e=>({...e,recordingType:"all"})))}te()}}
            hasActiveFilters=${C}
          />

          <${h}
            isLoading=${Q}
            hasData=${J}
            loadingMessage="Loading recordings..."
            emptyMessage="No recordings found matching your criteria"
          >
            <${v}
              recordings=${e}
              sortField=${l}
              sortDirection=${w}
              sortBy=${e=>{l===e?k("asc"===w?"desc":"asc"):(k("start_time"===e?"desc":"asc"),m(e)),P((e=>({...e,currentPage:1}))),x.updateUrlWithFilters(S,{...T,currentPage:1},e,e===l?"asc"===w?"desc":"asc":"start_time"===e?"desc":"asc")}}
              selectedRecordings=${_}
              toggleRecordingSelection=${e=>{B((t=>({...t,[e]:!t[e]})))}}
              selectAll=${M}
              toggleSelectAll=${()=>{const t=!M;z(t);const r={};t&&e.forEach((e=>{r[e.id]=!0})),B(r)}}
              getSelectedCount=${re}
              openDeleteModal=${e=>{H(e),O(!0)}}
              playRecording=${e=>{D.playRecording(e,y)}}
              downloadRecording=${e=>{D.downloadRecording(e)}}
              deleteRecording=${e=>{confirm(`Are you sure you want to delete this recording from ${e.stream}?`)&&ne(e.id)}}
              recordingsTableBodyRef=${U}
              pagination=${T}
            />

            <${$}
              pagination=${T}
              goToPage=${e=>{e<1||e>T.totalPages||(P((t=>({...t,currentPage:e}))),x.updateUrlWithFilters(S,{...T,currentPage:e},l,w))}}
            />
          <//>
        </div>
      </div>

      <${u}
        isOpen=${L}
        onClose=${ae}
        onConfirm=${()=>{if(ae(),"selected"===A){const e=Object.entries(_).filter((([e,t])=>t)).map((([e,t])=>parseInt(e,10)));oe({ids:e}),B({}),z(!1)}else{const e={};if("custom"===S.dateRange)e.start=`${S.startDate}T${S.startTime}:00`,e.end=`${S.endDate}T${S.endTime}:00`;else{const{start:t,end:r}=D.getDateRangeFromPreset(S.dateRange);e.start=t,e.end=r}"all"!==S.streamId&&(e.stream_name=S.streamId),"detection"===S.recordingType&&(e.detection=1),oe({filter:e}),B({}),z(!1)}}}
        mode=${A}
        count=${re()}
      />
    </section>
  `}}}}));
//# sourceMappingURL=RecordingsView-legacy-CO1f03Fs.js.map
