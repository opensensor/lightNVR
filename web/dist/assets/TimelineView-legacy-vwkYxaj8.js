System.register(["./preact-app-legacy-C7F2kZGB.js","./LoadingIndicator-legacy-DtGf-R_9.js"],(function(e,t){"use strict";var n,o,r,i,s,a,l;return{setters:[e=>{n=e.d,o=e.y,r=e.h,i=e.c,s=e.A,a=e.g},e=>{l=e.L}],execute:function(){function t(){const[e,t]=n(!1),[s,a]=n(1);o((()=>{console.log("TimelineControls: Setting up subscription to timelineState");const e=p.subscribe((e=>{console.log("TimelineControls: Received state update:",e),console.log("TimelineControls: Is playing:",e.isPlaying),console.log("TimelineControls: Zoom level:",e.zoomLevel),console.log("TimelineControls: Segments count:",e.timelineSegments?.length||0),t(e.isPlaying),a(e.zoomLevel)}));return console.log("TimelineControls: Initial timelineState:",p),()=>e()}),[]);const l=()=>{p.setState({isPlaying:!1});const e=document.querySelector("#video-player video");e&&e.pause()},c=()=>{if(!p.timelineSegments||0===p.timelineSegments.length)return void i("No recordings to play","warning");let e=0,t=Number.MAX_SAFE_INTEGER;p.timelineSegments.forEach(((n,o)=>{n.start_timestamp<t&&(t=n.start_timestamp,e=o)})),console.log(`Starting from earliest segment (index ${e})`),p.setState({currentSegmentIndex:e,currentTime:p.timelineSegments[e].start_timestamp,isPlaying:!0,forceReload:!0});const n=p.timelineSegments[e],o=document.querySelector("#video-player video");o&&(console.log("Loading earliest segment video:",n),o.pause(),o.removeAttribute("src"),o.load(),o.src=`/api/recordings/play/${n.id}?t=${Date.now()}`,o.onloadedmetadata=()=>{o.currentTime=0,o.play().catch((e=>{console.error("Error playing video:",e),i("Error playing video: "+e.message,"error")}))})};return r`
    <div class="timeline-controls flex justify-between items-center mb-2">
      <div class="flex items-center">
        <button 
          id="play-button" 
          class="w-10 h-10 rounded-full bg-green-600 hover:bg-green-700 text-white flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-offset-1 transition-colors shadow-sm mr-2"
          onClick=${()=>{e?l():c()}}
          title=${e?"Pause":"Play from earliest recording"}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            ${e?r`
                <!-- Pause icon - two vertical bars -->
                <rect x="6" y="6" width="4" height="12" rx="1" fill="white" />
                <rect x="14" y="6" width="4" height="12" rx="1" fill="white" />
              `:r`
                <!-- Play icon - triangle -->
                <path d="M8 5.14v14l11-7-11-7z" fill="white" />
              `}
          </svg>
        </button>
        <span class="text-xs text-gray-600 dark:text-gray-300">Play from earliest recording</span>
      </div>
      
      <div class="flex items-center gap-1">
        <span class="text-xs text-gray-600 dark:text-gray-300 mr-1">Zoom:</span>
        <button 
          id="zoom-out-button" 
          class="w-6 h-6 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors"
          onClick=${()=>{if(s>1){const e=s/2;p.setState({zoomLevel:e}),i(`Zoomed out: ${24/e} hours view`,"info")}}}
          title="Zoom Out (Show more time)"
          disabled=${s<=1}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
        <button 
          id="zoom-in-button" 
          class="w-6 h-6 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors"
          onClick=${()=>{if(s<8){const e=2*s;p.setState({zoomLevel:e}),i(`Zoomed in: ${24/e} hours view`,"info")}}}
          title="Zoom In (Show less time)"
          disabled=${s>=8}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>
  `}function c(){const[e,t]=n(0),[i,s]=n(24),[a,l]=n(1);return o((()=>{console.log("TimelineRuler: Setting up subscription to timelineState");const e=p.subscribe((e=>{console.log("TimelineRuler: Received state update:",e),console.log("TimelineRuler: Zoom level:",e.zoomLevel);const n=24/e.zoomLevel;console.log("TimelineRuler: Hours per view:",n);let o=12;if(null!==e.currentTime){const t=new Date(1e3*e.currentTime);o=t.getHours()+t.getMinutes()/60+t.getSeconds()/3600}else if(e.timelineSegments&&e.timelineSegments.length>0){let t=24,n=0;e.timelineSegments.forEach((e=>{const o=new Date(1e3*e.start_timestamp),r=new Date(1e3*e.end_timestamp),i=o.getHours()+o.getMinutes()/60+o.getSeconds()/3600,s=r.getHours()+r.getMinutes()/60+r.getSeconds()/3600;t=Math.min(t,i),n=Math.max(n,s)})),o=(t+n)/2}let r=Math.max(0,o-n/2),i=Math.min(24,r+n);24===i&&n<24?(r=Math.max(0,24-n),i=24):0===r&&n<24&&(i=Math.min(24,n)),console.log("TimelineRuler: New hour range:",{newStartHour:r,newEndHour:i}),t(r),s(i),l(e.zoomLevel),p.timelineStartHour===r&&p.timelineEndHour===i||p.setState({timelineStartHour:r,timelineEndHour:i})}));return()=>e()}),[]),r`
    <div class="timeline-ruler relative w-full h-8 bg-gray-300 dark:bg-gray-800 border-b border-gray-400 dark:border-gray-600">
      ${(()=>{const t=[];for(let n=Math.floor(e);n<=Math.ceil(i);n++)if(n>=0&&n<=24){const o=(n-e)/(i-e)*100;if(t.push(r`
          <div 
            key="tick-${n}" 
            class="absolute top-0 w-px h-5 bg-gray-500 dark:bg-gray-400" 
            style="left: ${o}%;"
          ></div>
        `),t.push(r`
          <div 
            key="label-${n}" 
            class="absolute top-0 text-xs text-gray-600 dark:text-gray-300 transform -translate-x-1/2" 
            style="left: ${o}%;"
          >
            ${n}:00
          </div>
        `),n<24&&a>=2){const o=(n+.5-e)/(i-e)*100;if(t.push(r`
            <div 
              key="tick-${n}-30" 
              class="absolute top-2 w-px h-3 bg-gray-400 dark:bg-gray-500" 
              style="left: ${o}%;"
            ></div>
          `),a>=4){const o=(n+.25-e)/(i-e)*100,s=(n+.75-e)/(i-e)*100;t.push(r`
              <div 
                key="tick-${n}-15" 
                class="absolute top-3 w-px h-2 bg-gray-400 dark:bg-gray-500" 
                style="left: ${o}%;"
              ></div>
            `),t.push(r`
              <div 
                key="tick-${n}-45" 
                class="absolute top-3 w-px h-2 bg-gray-400 dark:bg-gray-500" 
                style="left: ${s}%;"
              ></div>
            `)}}}return t})()}
      <div class="absolute bottom-0 left-0 text-xs text-gray-500 px-1">
        Zoom: ${a}x (${Math.round(24/a)} hours)
      </div>
    </div>
  `}function m(){const[e,t]=n([]),[i,a]=n(0),[l,c]=n(24),[m,d]=n(-1),g=s(null),u=s(!1);o((()=>{console.log("TimelineSegments: Setting up subscription to timelineState");const e=p.subscribe((e=>{console.log("TimelineSegments: Received state update"),e.timelineSegments&&(console.log(`TimelineSegments: Updating segments (${e.timelineSegments.length})`),t(e.timelineSegments)),a(e.timelineStartHour||0),c(e.timelineEndHour||24),d(e.currentSegmentIndex||-1)}));return p.timelineSegments&&p.timelineSegments.length>0&&(console.log(`TimelineSegments: Initial segments available (${p.timelineSegments.length})`),t(p.timelineSegments),d(p.currentSegmentIndex||0)),()=>e()}),[]),o((()=>{const e=g.current;if(!e)return;const t=t=>{(t.target===e||t.target.classList.contains("timeline-clickable-area"))&&(u.current=!0,h(t),document.addEventListener("mousemove",n),document.addEventListener("mouseup",o))},n=e=>{u.current&&h(e)},o=()=>{u.current=!1,document.removeEventListener("mousemove",n),document.removeEventListener("mouseup",o)};return e.addEventListener("mousedown",t),()=>{e.removeEventListener("mousedown",t),document.removeEventListener("mousemove",n),document.removeEventListener("mouseup",o)}}),[i,l,e]);const h=t=>{const n=g.current;if(!n)return;const o=n.getBoundingClientRect(),r=t.clientX-o.left,s=o.width,a=i+r/s*(l-i),c=new Date(p.selectedDate);c.setHours(Math.floor(a)),c.setMinutes(Math.floor(a%1*60)),c.setSeconds(Math.floor(a%1*60%1*60));const m=c.getTime()/1e3;let u=!1;for(let i=0;i<e.length;i++){const t=e[i];if(m>=t.start_timestamp&&m<=t.end_timestamp){console.log(`TimelineSegments: Found segment ${i} containing timestamp`);const e=m-t.start_timestamp;d(i),v(i,e),u=!0;break}}if(!u)if(e.length>0){console.log("TimelineSegments: No segment contains the timestamp, finding closest segment");let t=-1,n=1/0;for(let o=0;o<e.length;o++){const r=e[o],i=Math.abs(r.start_timestamp-m),s=Math.abs(r.end_timestamp-m),a=Math.min(i,s);a<n&&(n=a,t=o)}t>=0&&(console.log(`TimelineSegments: Playing closest segment ${t}`),v(t))}else console.log("TimelineSegments: No segments found, just updating currentTime"),p.setState({currentTime:m,prevCurrentTime:p.currentTime})},v=(t,n=null)=>{if(console.log(`TimelineSegments: playSegment(${t}, ${n})`),t<0||t>=e.length)return void console.warn(`TimelineSegments: Invalid segment index: ${t}`);const o=e[t],r=null!==n?o.start_timestamp+n:o.start_timestamp;p.setState({isPlaying:!1,currentSegmentIndex:-1}),document.body.offsetHeight,setTimeout((()=>{p.setState({currentSegmentIndex:t,currentTime:r,isPlaying:!0,forceReload:!0}),setTimeout((()=>{const e=document.querySelector("#video-player video");e&&(e.pause(),e.removeAttribute("src"),e.load(),e.src=`/api/recordings/play/${o.id}?t=${Date.now()}`,e.onloadedmetadata=()=>{const t=null!==n?n:0;e.currentTime=t,e.play().catch((e=>console.error("Error playing video:",e)))})}),50)}),50)};return r`
    <div 
      class="timeline-segments relative w-full h-16 pt-2"
      ref=${g}
    >
      ${(()=>{if(console.log(`TimelineSegments: Rendering ${e.length} segments`),!e||0===e.length)return null;const t=[],n=new Map;e.forEach(((e,t)=>{const o=new Date(1e3*e.start_timestamp),r=new Date(1e3*e.end_timestamp),s=o.getHours()+o.getMinutes()/60+o.getSeconds()/3600,a=r.getHours()+r.getMinutes()/60+r.getSeconds()/3600;if(a<i||s>l)return;const c=Math.floor(s),m=Math.min(Math.ceil(a),24);for(let d=c;d<m;d++)d>=i&&d<=l&&(n.has(d)||n.set(d,[]),n.get(d).push(t))}));const o=[];let s=null;[...e].sort(((e,t)=>e.start_timestamp-t.start_timestamp)).forEach(((e,t)=>{s?e.start_timestamp-s.end_timestamp<=1?(s.end_timestamp=e.end_timestamp,s.originalIndices.push(t),e.has_detection&&(s.has_detection=!0)):(o.push(s),s={...e,originalIndices:[t]}):s={...e,originalIndices:[t]}})),s&&o.push(s),console.log(`TimelineSegments: Merged ${e.length} segments into ${o.length} segments`),o.forEach(((e,n)=>{const o=new Date(1e3*e.start_timestamp),s=new Date(1e3*e.end_timestamp),a=o.getHours()+o.getMinutes()/60+o.getSeconds()/3600,c=s.getHours()+s.getMinutes()/60+s.getSeconds()/3600;if(c<i||a>l)return;const m=Math.max(a,i),d=Math.min(c,l),g=(m-i)/(l-i)*100,u=(d-m)/(l-i)*100,p=`${Math.round(e.end_timestamp-e.start_timestamp)}s`,h=o.toLocaleTimeString(),v=s.toLocaleTimeString();t.push(r`
        <div 
          key="segment-${n}"
          class="timeline-segment absolute rounded-sm transition-all duration-200 ${e.has_detection?"bg-red-500":"bg-blue-500"}"
          style="left: ${g}%; width: ${u}%; height: ${80}%; top: 50%; transform: translateY(-50%);"
          title="${h} - ${v} (${p})"
        ></div>
      `)}));for(let e=Math.floor(i);e<Math.ceil(l);e++)if(!n.has(e)){const n=(e-i)/(l-i)*100,o=100/(l-i);t.push(r`
          <div 
            key="clickable-${e}"
            class="timeline-clickable-area absolute h-full cursor-pointer"
            style="left: ${n}%; width: ${o}%;"
            data-hour=${e}
          ></div>
        `)}return t})()}
    </div>
  `}function d(){const[e,t]=n(0),[i,a]=n(!1),[l,c]=n(0),[m,d]=n(24),[g,u]=n(null),[h,v]=n(!1),f=s(null);s(null);const b=s(0);o((()=>{const e=p.subscribe((e=>{c(e.timelineStartHour||0),d(e.timelineEndHour||24),u(e.currentTime),S(e.currentTime),h||y(e.currentTime,e.timelineStartHour||0,e.timelineEndHour||24)}));return()=>e()}),[h]),o((()=>{const e=f.current;if(!e)return;const n=e=>{e.preventDefault(),e.stopPropagation(),b.current=e.clientX,v(!0),document.addEventListener("mousemove",o),document.addEventListener("mouseup",r)},o=n=>{if(!h)return;const o=e.parentElement;if(!o)return;const r=o.getBoundingClientRect(),i=Math.max(0,Math.min(n.clientX-r.left,r.width))/r.width*100;t(i);const s=l+i/100*(m-l),a=new Date(p.selectedDate);a.setHours(Math.floor(s)),a.setMinutes(Math.floor(s%1*60)),a.setSeconds(Math.floor(s%1*60%1*60));const c=a.getTime()/1e3;S(c)},r=t=>{if(!h)return;const n=e.parentElement;if(!n)return;const i=n.getBoundingClientRect(),s=Math.max(0,Math.min(t.clientX-i.left,i.width)),a=i.width,c=l+s/a*100/100*(m-l),d=new Date(p.selectedDate);d.setHours(Math.floor(c)),d.setMinutes(Math.floor(c%1*60)),d.setSeconds(Math.floor(c%1*60%1*60));const g=d.getTime()/1e3;v(!1),document.removeEventListener("mousemove",o),document.removeEventListener("mouseup",r);const u=p.timelineSegments||[];let f=!1;for(let e=0;e<u.length;e++){const t=u[e];if(g>=t.start_timestamp&&g<=t.end_timestamp){t.start_timestamp,p.setState({currentSegmentIndex:e,currentTime:g,prevCurrentTime:p.currentTime,isPlaying:!0}),f=!0;break}}if(!f&&u.length>0){let e=0,t=1/0;for(let n=0;n<u.length;n++){const o=u[n],r=Math.abs(o.start_timestamp-g),i=Math.abs(o.end_timestamp-g),s=Math.min(r,i);s<t&&(t=s,e=n)}p.setState({currentSegmentIndex:e,currentTime:u[e].start_timestamp,prevCurrentTime:p.currentTime,isPlaying:!0})}};return e.addEventListener("mousedown",n),()=>{e.removeEventListener("mousedown",n),document.removeEventListener("mousemove",o),document.removeEventListener("mouseup",r)}}),[f.current,l,m,h]);const y=(e,n,o)=>{if(null===e)return void a(!1);const r=new Date(1e3*e),i=r.getHours()+r.getMinutes()/60+r.getSeconds()/3600;i<n||i>o?a(!1):(t((i-n)/(o-n)*100),a(!0))},S=e=>{if(null===e)return;const t=document.getElementById("time-display");if(!t)return;const n=new Date(1e3*e),o=n.getHours().toString().padStart(2,"0"),r=n.getMinutes().toString().padStart(2,"0"),i=n.getSeconds().toString().padStart(2,"0");t.textContent=`${o}:${r}:${i}`};return o((()=>{setTimeout((()=>{p.currentTime&&(a(!0),y(p.currentTime,p.timelineStartHour||0,p.timelineEndHour||24))}),500)}),[]),r`
    <div 
      ref=${f}
      class="timeline-cursor absolute top-0 h-full z-50 transition-all duration-100 cursor-ew-resize"
      style="left: ${e}%; display: ${i?"block":"none"}; pointer-events: auto; width: 7px; margin-left: -3.5px;"
    >
      <!-- Invisible wider clickable area -->
      <div class="absolute top-0 bottom-0 left-0 w-full"></div>
      
      <!-- Skinnier needle with no middle chunk -->
      <div class="absolute top-0 bottom-0 w-0.5 bg-orange-500 left-1/2 transform -translate-x-1/2 pointer-events-none"></div>
      
      <!-- Top handle (black) -->
      <div class="absolute top-0 left-1/2 w-4 h-4 bg-black rounded-full transform -translate-x-1/2 -translate-y-1/2 shadow-md pointer-events-none"></div>
      
      <!-- Bottom handle (black) -->
      <div class="absolute bottom-0 left-1/2 w-4 h-4 bg-black rounded-full transform -translate-x-1/2 translate-y-1/2 shadow-md pointer-events-none"></div>
    </div>
  `}function g(){const[e,t]=n(1);return o((()=>{console.log("SpeedControls: Setting up subscription to timelineState");const e=p.subscribe((e=>{console.log("SpeedControls: Received state update:",e),console.log("SpeedControls: Playback speed:",e.playbackSpeed),t(e.playbackSpeed)}));return console.log("SpeedControls: Initial timelineState:",p),()=>e()}),[]),r`
    <div class="mt-2 mb-4 p-2 border border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
      <div class="flex flex-col items-center">
        <div class="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">Playback Speed</div>
        
        <div class="flex flex-wrap justify-center gap-1">
          ${[.25,.5,1,1.5,2,4].map((t=>r`
            <button 
              key=${`speed-${t}`}
              class=${`speed-btn px-2 py-1 text-sm rounded-full ${t===e?"bg-green-500 text-white":"bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"} \n                font-medium transition-all focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-opacity-50`}
              data-speed=${t}
              onClick=${()=>(e=>{const t=document.querySelector("#video-player video");if(t){const n=t.playbackRate;t.playbackRate=e,console.log(`Setting video playback rate from ${n}x to ${e}x`,t),console.log(`Actual playback rate after setting: ${t.playbackRate}x`),setTimeout((()=>{t.playbackRate=e,console.log(`Re-setting playback rate to ${e}x, actual rate: ${t.playbackRate}x`)}),100)}else console.warn("Video player element not found");p.setState({playbackSpeed:e}),i(`Playback speed: ${e}x`,"info")})(t)}
            >
              ${1===t?"1× (Normal)":`${t}×`}
            </button>
          `))}
        </div>
        
        <div class="mt-1 text-xs font-medium text-green-600 dark:text-green-400">
          Current: ${e}× ${1===e?"(Normal)":""}
        </div>
      </div>
    </div>
  `}function u(){const[e,t]=n(-1),[a,l]=n(!1),[c,m]=n([]),[d,u]=n(1),h=s(null),v=s(null),f=s(null);o((()=>{const e=p.subscribe((e=>{t(e.currentSegmentIndex),l(e.isPlaying),m(e.timelineSegments||[]),u(e.playbackSpeed),b(e)}));return()=>e()}),[]);const b=e=>{const t=h.current;if(!t)return;if(!e.timelineSegments||0===e.timelineSegments.length||e.currentSegmentIndex<0||e.currentSegmentIndex>=e.timelineSegments.length)return;const n=e.timelineSegments[e.currentSegmentIndex];if(!n)return;const o=f.current!==n.id,r=o,i=null!==e.currentTime&&e.currentTime>=n.start_timestamp?e.currentTime-n.start_timestamp:0,s=null!==e.prevCurrentTime&&Math.abs(e.currentTime-e.prevCurrentTime)>1;o&&(console.log(`Segment changed from ${f.current} to ${n.id}`),f.current=n.id),r?(console.log(`Loading new segment ${n.id} (segmentChanged: ${o})`),y(n,i,e.isPlaying)):s?(console.log(`Seeking to ${i}s within current segment`),t.currentTime=i):e.isPlaying&&t.paused?t.play().catch((e=>{console.error("Error playing video:",e)})):e.isPlaying||t.paused||t.pause(),t.playbackRate!==e.playbackSpeed&&(t.playbackRate=e.playbackSpeed)},y=(e,t=0,n=!1)=>{const o=h.current;if(!o)return;console.log(`Loading segment ${e.id} at time ${t}s, autoplay: ${n}`),o.pause();const r=`/api/recordings/play/${e.id}?t=${Date.now()}`,s=()=>{console.log("Video metadata loaded"),o.currentTime=t,o.playbackRate=d,n&&o.play().catch((e=>{console.error("Error playing video:",e),i("Error playing video: "+e.message,"error")})),o.removeEventListener("loadedmetadata",s)};o.addEventListener("loadedmetadata",s),o.src=r,o.load()},S=e=>{if(null===e)return;const t=document.getElementById("time-display");if(!t)return;const n=new Date(1e3*e),o=n.getHours().toString().padStart(2,"0"),r=n.getMinutes().toString().padStart(2,"0"),i=n.getSeconds().toString().padStart(2,"0");t.textContent=`${o}:${r}:${i}`};return r`
    <div class="timeline-player-container mb-2" id="video-player">
      <div class="relative w-full bg-black rounded-lg shadow-md" style="aspect-ratio: 16/9;">
        <video
            ref=${h}
            class="w-full h-full object-contain"
            controls
            autoplay=${!1}
            muted=${!1}
            playsInline
            onended=${()=>{if(console.log("Video ended"),e<c.length-1){const t=e+1;console.log(`Playing next segment ${t}`),p.setState({currentSegmentIndex:t,currentTime:c[t].start_timestamp,isPlaying:!0,forceReload:!0})}else console.log("End of all segments"),p.setState({isPlaying:!1})}}
            ontimeupdate=${()=>{const t=h.current;if(!t)return;if(e<0||!c||0===c.length||e>=c.length)return;const n=c[e];if(!n)return;const o=n.start_timestamp+t.currentTime;S(o),p.setState({currentTime:o,prevCurrentTime:v.current}),v.current=o}}
        ></video>
        
        <!-- Add a message for invalid segments -->
        <div 
          class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-70 text-white text-center p-4 ${e>=0&&c.length>0?"hidden":""}"
        >
          <div>
            <p class="mb-2">No valid segment selected.</p>
            <p class="text-sm">Click on a segment in the timeline or use the play button to start playback.</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Playback speed controls -->
    <${g} />
  `}e("loadTimelineView",(function(){const e=document.getElementById("main-content");e&&(e.innerHTML="",a(r`<${h} />`,e))}));const p={streams:[],timelineSegments:[],selectedStream:null,selectedDate:null,isPlaying:!1,currentSegmentIndex:-1,zoomLevel:1,timelineStartHour:0,timelineEndHour:24,currentTime:null,prevCurrentTime:null,playbackSpeed:1,showOnlySegments:!0,forceReload:!1,listeners:new Set,setState(e){Object.assign(this,e),this.notifyListeners()},subscribe(e){return this.listeners.add(e),()=>this.listeners.delete(e)},notifyListeners(){this.listeners.forEach((e=>e(this)))}};function h(){const e=function(){const e=new URLSearchParams(window.location.search);return{stream:e.get("stream")||"",date:e.get("date")||(t=new Date,`${t.getFullYear()}-${String(t.getMonth()+1).padStart(2,"0")}-${String(t.getDate()).padStart(2,"0")}`)};var t}(),[a,g]=n(!1),[h,v]=n([]),[f,b]=n(e.stream),[y,S]=n(e.date),[x,w]=n([]),$=s(null),k=s(!1);o((()=>{console.log("TimelinePage: Initial mount, loading streams"),T()}),[]),o((()=>{if(h.length>0&&!k.current)if(console.log("TimelinePage: Streams loaded, initializing data"),k.current=!0,h.some((e=>e.name===f))&&f)console.log(`TimelinePage: Using stream from URL: ${f}`),M(f,y);else if(h.length>0){const e=h[0].name;console.log(`TimelinePage: Using first stream: ${e}`),b(e),M(e,y)}}),[h]);const T=()=>{console.log("TimelinePage: Loading streams"),g(!0),fetch("/api/streams").then((e=>{if(!e.ok)throw new Error("Failed to load streams");return e.json()})).then((e=>{console.log("TimelinePage: Streams data received:",e);const t=Array.isArray(e)?e:[];console.log(`TimelinePage: Loaded ${t.length} streams`),v(t),g(!1),p.setState({streams:t}),t.length>0&&console.log("TimelinePage: First stream:",t[0])})).catch((e=>{console.error("TimelinePage: Error loading streams:",e),i("Error loading streams: "+e.message,"error"),g(!1)}))},M=(e,t)=>{if(!e)return void i("Please select a stream","error");console.log(`TimelinePage: Loading timeline data for ${e} on ${t}`),g(!0),w([]),i("Loading timeline data...","info"),function(e,t){if(!e)return;const n=new URL(window.location.href);n.searchParams.set("stream",e),n.searchParams.set("date",t),window.history.replaceState({},"",n)}(e,t);const n=new Date(t);n.setHours(0,0,0,0);const o=new Date(t);o.setHours(23,59,59,999);const r=n.toISOString(),s=o.toISOString();p.setState({selectedStream:e,selectedDate:t,timelineSegments:[],currentSegmentIndex:-1,currentTime:null,isPlaying:!1}),fetch(`/api/timeline/segments?stream=${encodeURIComponent(e)}&start=${encodeURIComponent(r)}&end=${encodeURIComponent(s)}`).then((e=>{if(!e.ok)throw new Error("Failed to load timeline data");return e.json()})).then((e=>{console.log("TimelinePage: Timeline data received:",e);const t=e.segments||[];if(console.log(`TimelinePage: Received ${t.length} segments`),g(!1),0===t.length)return console.log("TimelinePage: No segments found"),w([]),p.setState({timelineSegments:[],currentSegmentIndex:-1,currentTime:null,isPlaying:!1}),void i("No recordings found for the selected date","warning");console.log("TimelinePage: Setting segments"),w(t);const n=t[0].start_timestamp;p.setState({timelineSegments:t,currentSegmentIndex:0,currentTime:n,prevCurrentTime:n,isPlaying:!1});const o=document.querySelector("#video-player video");o&&(o.src=`/api/recordings/play/${t[0].id}`,o.load()),i(`Loaded ${t.length} recording segments`,"success")})).catch((e=>{console.error("TimelinePage: Error loading timeline data:",e),i("Error loading timeline data: "+e.message,"error"),g(!1),w([])}))};return r`
    <div class="timeline-page">
      <div class="flex items-center mb-4">
        <h1 class="text-2xl font-bold">Timeline Playback</h1>
        <div class="ml-4 flex">
          <a href="recordings.html" class="px-3 py-1 bg-gray-300 text-gray-700 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-400 dark:hover:bg-gray-600 rounded-l-md">Table View</a>
          <a href="timeline.html" class="px-3 py-1 bg-blue-500 text-white rounded-r-md">Timeline View</a>
        </div>
      </div>

      <!-- Stream selector and date picker -->
      <div class="flex flex-wrap gap-4 mb-2">
        <div class="stream-selector flex-grow">
          <div class="flex justify-between items-center mb-2">
            <label for="stream-selector">Stream</label>
            <button 
              class="text-xs bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 px-2 py-1 rounded"
              onClick=${()=>T()}
            >
              Reload Streams
            </button>
          </div>
          <select
              id="stream-selector"
              class="w-full p-2 border rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${f||""}
              onChange=${e=>{const t=e.target.value;console.log(`TimelinePage: Stream changed to ${t}`),b(t),t&&M(t,y)}}
          >
            <option value="" disabled>Select a stream (${h.length} available)</option>
            ${h.map((e=>r`
              <option key=${e.name} value=${e.name}>${e.name}</option>
            `))}
          </select>
        </div>

        <div class="date-selector flex-grow">
          <label for="timeline-date" class="block mb-2">Date</label>
          <input
              type="date"
              id="timeline-date"
              class="w-full p-2 border rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${y}
              onChange=${e=>{const t=e.target.value;console.log(`TimelinePage: Date changed to ${t}`),S(t),f&&M(f,t)}}
          />
        </div>
      </div>
      
      <!-- Auto-load message -->
      <div class="mb-4 text-sm text-gray-500 dark:text-gray-400 italic">
        ${a?"Loading...":"Recordings auto-load when stream or date changes"}
      </div>

      <!-- Current time display -->
      <div class="flex justify-between items-center mb-2">
        <div id="time-display" class="timeline-time-display bg-gray-200 dark:bg-gray-700 px-3 py-1 rounded font-mono text-base">00:00:00</div>
      </div>
      
      <!-- Debug info -->
      <div class="mb-2 text-xs text-gray-500">
        Debug - isLoading: ${a?"true":"false"}, 
        Streams: ${h.length},
        Segments: ${x.length}
      </div>
      
      <!-- Content -->
      ${a?r`<${l} message="Loading timeline data..." />`:0===x.length?r`
        <div class="flex flex-col items-center justify-center py-12 text-center">
          <svg class="w-16 h-16 text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p class="text-gray-600 dark:text-gray-400 text-lg">No recordings found for the selected date and stream</p>
        </div>
      `:r`
      <!-- Video player -->
      <${u} />

      <!-- Playback controls -->
      <${t} />

        <!-- Timeline -->
        <div
            id="timeline-container"
            class="relative w-full h-24 bg-gray-200 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg mb-6 overflow-hidden"
            ref=${$}
        >
          <${c} />
          <${m} />
          <${d} />
          
          <!-- Instructions for cursor -->
          <div class="absolute bottom-1 right-2 text-xs text-gray-500 dark:text-gray-400 bg-white dark:bg-gray-800 bg-opacity-75 dark:bg-opacity-75 px-2 py-1 rounded">
            Drag the orange dial to navigate
          </div>
        </div>
    `}

      <!-- Instructions -->
      <div class="mt-6 p-4 bg-gray-200 dark:bg-gray-800 rounded">
        <h3 class="text-lg font-semibold mb-2">How to use the timeline:</h3>
        <ul class="list-disc pl-5">
          <li>Select a stream and date to load recordings</li>
          <li>Click on the timeline to seek to a specific time</li>
          <li>Click on a segment (blue bar) to play that recording</li>
          <li>Use the play/pause button to control playback</li>
          <li>Use the zoom buttons to adjust the timeline scale</li>
        </ul>
      </div>
    </div>
  `}}}}));
//# sourceMappingURL=TimelineView-legacy-vwkYxaj8.js.map
