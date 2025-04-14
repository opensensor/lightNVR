System.register(["./preact-app-legacy-B5xuyY7_.js","./LoadingIndicator-legacy-CYmYLc0b.js"],(function(e,t){"use strict";var n,o,r,s,i,a,l,c,m,d;return{setters:[e=>{n=e.d,o=e.y,r=e.h,s=e.s,i=e.A,a=e.u,l=e.E,c=e.Q,m=e.q},e=>{d=e.L}],execute:function(){function t(){const[e,t]=n(!1),[i,a]=n(1);o((()=>{console.log("TimelineControls: Setting up subscription to timelineState");const e=f.subscribe((e=>{console.log("TimelineControls: Received state update:",e),console.log("TimelineControls: Is playing:",e.isPlaying),console.log("TimelineControls: Zoom level:",e.zoomLevel),console.log("TimelineControls: Segments count:",e.timelineSegments?.length||0),t(e.isPlaying),a(e.zoomLevel)}));return console.log("TimelineControls: Initial timelineState:",f),()=>e()}),[]);const l=()=>{f.setState({isPlaying:!1});const e=document.querySelector("#video-player video");e&&e.pause()},c=()=>{if(!f.timelineSegments||0===f.timelineSegments.length)return void s("No recordings to play","warning");let e=0,t=Number.MAX_SAFE_INTEGER;f.timelineSegments.forEach(((n,o)=>{n.start_timestamp<t&&(t=n.start_timestamp,e=o)})),console.log(`Starting from earliest segment (index ${e})`),f.setState({currentSegmentIndex:e,currentTime:f.timelineSegments[e].start_timestamp,isPlaying:!0,forceReload:!0});const n=f.timelineSegments[e],o=document.querySelector("#video-player video");o&&(console.log("Loading earliest segment video:",n),o.pause(),o.removeAttribute("src"),o.load(),o.src=`/api/recordings/play/${n.id}?t=${Date.now()}`,o.onloadedmetadata=()=>{o.currentTime=0,o.play().catch((e=>{console.error("Error playing video:",e),s("Error playing video: "+e.message,"error")}))})};return r`
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
          onClick=${()=>{if(i>1){const e=i/2;f.setState({zoomLevel:e}),s(`Zoomed out: ${24/e} hours view`,"info")}}}
          title="Zoom Out (Show more time)"
          disabled=${i<=1}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
        <button 
          id="zoom-in-button" 
          class="w-6 h-6 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors"
          onClick=${()=>{if(i<8){const e=2*i;f.setState({zoomLevel:e}),s(`Zoomed in: ${24/e} hours view`,"info")}}}
          title="Zoom In (Show less time)"
          disabled=${i>=8}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>
  `}function g(){const[e,t]=n(0),[s,i]=n(24),[a,l]=n(1);return o((()=>{console.log("TimelineRuler: Setting up subscription to timelineState");const e=f.subscribe((e=>{console.log("TimelineRuler: Received state update:",e),console.log("TimelineRuler: Zoom level:",e.zoomLevel);const n=24/e.zoomLevel;console.log("TimelineRuler: Hours per view:",n);let o=12;if(null!==e.currentTime){const t=new Date(1e3*e.currentTime);o=t.getHours()+t.getMinutes()/60+t.getSeconds()/3600}else if(e.timelineSegments&&e.timelineSegments.length>0){let t=24,n=0;e.timelineSegments.forEach((e=>{const o=new Date(1e3*e.start_timestamp),r=new Date(1e3*e.end_timestamp),s=o.getHours()+o.getMinutes()/60+o.getSeconds()/3600,i=r.getHours()+r.getMinutes()/60+r.getSeconds()/3600;t=Math.min(t,s),n=Math.max(n,i)})),o=(t+n)/2}let r=Math.max(0,o-n/2),s=Math.min(24,r+n);24===s&&n<24?(r=Math.max(0,24-n),s=24):0===r&&n<24&&(s=Math.min(24,n)),console.log("TimelineRuler: New hour range:",{newStartHour:r,newEndHour:s}),t(r),i(s),l(e.zoomLevel),f.timelineStartHour===r&&f.timelineEndHour===s||f.setState({timelineStartHour:r,timelineEndHour:s})}));return()=>e()}),[]),r`
    <div class="timeline-ruler relative w-full h-8 bg-gray-300 dark:bg-gray-800 border-b border-gray-400 dark:border-gray-600">
      ${(()=>{const t=[];for(let n=Math.floor(e);n<=Math.ceil(s);n++)if(n>=0&&n<=24){const o=(n-e)/(s-e)*100;if(t.push(r`
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
        `),n<24&&a>=2){const o=(n+.5-e)/(s-e)*100;if(t.push(r`
            <div 
              key="tick-${n}-30" 
              class="absolute top-2 w-px h-3 bg-gray-400 dark:bg-gray-500" 
              style="left: ${o}%;"
            ></div>
          `),a>=4){const o=(n+.25-e)/(s-e)*100,i=(n+.75-e)/(s-e)*100;t.push(r`
              <div 
                key="tick-${n}-15" 
                class="absolute top-3 w-px h-2 bg-gray-400 dark:bg-gray-500" 
                style="left: ${o}%;"
              ></div>
            `),t.push(r`
              <div 
                key="tick-${n}-45" 
                class="absolute top-3 w-px h-2 bg-gray-400 dark:bg-gray-500" 
                style="left: ${i}%;"
              ></div>
            `)}}}return t})()}
      <div class="absolute bottom-0 left-0 text-xs text-gray-500 px-1">
        Zoom: ${a}x (${Math.round(24/a)} hours)
      </div>
    </div>
  `}function u(){const[e,t]=n([]),[s,a]=n(0),[l,c]=n(24),[m,d]=n(-1),g=i(null),u=i(!1);o((()=>{console.log("TimelineSegments: Setting up subscription to timelineState");const e=f.subscribe((e=>{console.log("TimelineSegments: Received state update"),e.timelineSegments&&(console.log(`TimelineSegments: Updating segments (${e.timelineSegments.length})`),t(e.timelineSegments)),a(e.timelineStartHour||0),c(e.timelineEndHour||24),d(e.currentSegmentIndex||-1)}));return f.timelineSegments&&f.timelineSegments.length>0&&(console.log(`TimelineSegments: Initial segments available (${f.timelineSegments.length})`),t(f.timelineSegments),d(f.currentSegmentIndex||0)),()=>e()}),[]),o((()=>{const e=g.current;if(!e)return;const t=t=>{(t.target===e||t.target.classList.contains("timeline-clickable-area"))&&(u.current=!0,p(t),document.addEventListener("mousemove",n),document.addEventListener("mouseup",o))},n=e=>{u.current&&p(e)},o=()=>{u.current=!1,document.removeEventListener("mousemove",n),document.removeEventListener("mouseup",o)};return e.addEventListener("mousedown",t),()=>{e.removeEventListener("mousedown",t),document.removeEventListener("mousemove",n),document.removeEventListener("mouseup",o)}}),[s,l,e]);const p=t=>{const n=g.current;if(!n)return;const o=n.getBoundingClientRect(),r=t.clientX-o.left,i=o.width,a=s+r/i*(l-s),c=new Date(f.selectedDate);c.setHours(Math.floor(a)),c.setMinutes(Math.floor(a%1*60)),c.setSeconds(Math.floor(a%1*60%1*60));const m=c.getTime()/1e3;let u=!1;for(let s=0;s<e.length;s++){const t=e[s],n=t.local_start_timestamp||t.start_timestamp,o=t.local_end_timestamp||t.end_timestamp;if(m>=n&&m<=o){console.log(`TimelineSegments: Found segment ${s} containing timestamp`);const e=m-n;d(s),h(s,e),u=!0;break}}if(!u)if(e.length>0){console.log("TimelineSegments: No segment contains the timestamp, finding closest segment");let t=-1,n=1/0;for(let o=0;o<e.length;o++){const r=e[o],s=r.local_start_timestamp||r.start_timestamp,i=r.local_end_timestamp||r.end_timestamp,a=Math.abs(s-m),l=Math.abs(i-m),c=Math.min(a,l);c<n&&(n=c,t=o)}t>=0&&(console.log(`TimelineSegments: Playing closest segment ${t}`),h(t))}else console.log("TimelineSegments: No segments found, just updating currentTime"),f.setState({currentTime:m,prevCurrentTime:f.currentTime})},h=(t,n=null)=>{if(console.log(`TimelineSegments: playSegment(${t}, ${n})`),t<0||t>=e.length)return void console.warn(`TimelineSegments: Invalid segment index: ${t}`);const o=e[t],r=o.local_start_timestamp||o.start_timestamp,s=null!==n?r+n:r;f.setState({isPlaying:!1,currentSegmentIndex:-1}),document.body.offsetHeight,setTimeout((()=>{f.setState({currentSegmentIndex:t,currentTime:s,isPlaying:!0,forceReload:!0}),setTimeout((()=>{const e=document.querySelector("#video-player video");e&&(e.pause(),e.removeAttribute("src"),e.load(),e.src=`/api/recordings/play/${o.id}?t=${Date.now()}`,e.onloadedmetadata=()=>{const t=null!==n?n:0;e.currentTime=t,e.play().catch((e=>console.error("Error playing video:",e)))})}),50)}),50)};return r`
    <div 
      class="timeline-segments relative w-full h-16 pt-2"
      ref=${g}
    >
      ${(()=>{if(console.log(`TimelineSegments: Rendering ${e.length} segments`),!e||0===e.length)return null;const t=[],n=new Map;e.forEach(((e,t)=>{const o=e.local_start_timestamp||e.start_timestamp,r=e.local_end_timestamp||e.end_timestamp,i=new Date(1e3*o),a=new Date(1e3*r),c=i.getHours()+i.getMinutes()/60+i.getSeconds()/3600,m=a.getHours()+a.getMinutes()/60+a.getSeconds()/3600;if(m<s||c>l)return;const d=Math.floor(c),g=Math.min(Math.ceil(m),24);for(let u=d;u<g;u++)u>=s&&u<=l&&(n.has(u)||n.set(u,[]),n.get(u).push(t))}));const o=[];let i=null;[...e].sort(((e,t)=>(e.local_start_timestamp||e.start_timestamp)-(t.local_start_timestamp||t.start_timestamp))).forEach(((e,t)=>{i?(e.local_start_timestamp||e.start_timestamp)-(i.local_end_timestamp||i.end_timestamp)<=1?(i.end_timestamp=e.end_timestamp,e.local_end_timestamp&&(i.local_end_timestamp=e.local_end_timestamp),i.originalIndices.push(t),e.has_detection&&(i.has_detection=!0)):(o.push(i),i={...e,originalIndices:[t]}):i={...e,originalIndices:[t]}})),i&&o.push(i),console.log(`TimelineSegments: Merged ${e.length} segments into ${o.length} segments`),o.forEach(((e,n)=>{const o=e.local_start_timestamp||e.start_timestamp,i=e.local_end_timestamp||e.end_timestamp,a=new Date(1e3*o),c=new Date(1e3*i),m=a.getHours()+a.getMinutes()/60+a.getSeconds()/3600,d=c.getHours()+c.getMinutes()/60+c.getSeconds()/3600;if(d<s||m>l)return;const g=Math.max(m,s),u=Math.min(d,l),p=(g-s)/(l-s)*100,h=(u-g)/(l-s)*100,v=`${Math.round(i-o)}s`,f=a.toLocaleTimeString(),y=c.toLocaleTimeString();t.push(r`
        <div 
          key="segment-${n}"
          class="timeline-segment absolute rounded-sm transition-all duration-200 ${e.has_detection?"bg-red-500":"bg-blue-500"}"
          style="left: ${p}%; width: ${h}%; height: ${80}%; top: 50%; transform: translateY(-50%);"
          title="${f} - ${y} (${v})"
        ></div>
      `)}));for(let e=Math.floor(s);e<Math.ceil(l);e++)if(!n.has(e)){const n=(e-s)/(l-s)*100,o=100/(l-s);t.push(r`
          <div 
            key="clickable-${e}"
            class="timeline-clickable-area absolute h-full cursor-pointer"
            style="left: ${n}%; width: ${o}%;"
            data-hour=${e}
          ></div>
        `)}return t})()}
    </div>
  `}function p(){const[e,t]=n(0),[s,a]=n(!1),[l,c]=n(0),[m,d]=n(24),[g,u]=n(null),[p,h]=n(!1),v=i(null);i(null);const y=i(0);o((()=>{const e=f.subscribe((e=>{c(e.timelineStartHour||0),d(e.timelineEndHour||24),u(e.currentTime),S(e.currentTime),p||b(e.currentTime,e.timelineStartHour||0,e.timelineEndHour||24)}));return()=>e()}),[p]),o((()=>{const e=v.current;if(!e)return;const n=e=>{e.preventDefault(),e.stopPropagation(),y.current=e.clientX,h(!0),document.addEventListener("mousemove",o),document.addEventListener("mouseup",r)},o=n=>{if(!p)return;const o=e.parentElement;if(!o)return;const r=o.getBoundingClientRect(),s=Math.max(0,Math.min(n.clientX-r.left,r.width))/r.width*100;t(s);const i=l+s/100*(m-l),a=new Date(f.selectedDate);a.setHours(Math.floor(i)),a.setMinutes(Math.floor(i%1*60)),a.setSeconds(Math.floor(i%1*60%1*60));const c=a.getTime()/1e3;S(c)},r=t=>{if(!p)return;const n=e.parentElement;if(!n)return;const s=n.getBoundingClientRect(),i=Math.max(0,Math.min(t.clientX-s.left,s.width)),a=s.width,c=l+i/a*100/100*(m-l),d=new Date(f.selectedDate);d.setHours(Math.floor(c)),d.setMinutes(Math.floor(c%1*60)),d.setSeconds(Math.floor(c%1*60%1*60));const g=d.getTime()/1e3;h(!1),document.removeEventListener("mousemove",o),document.removeEventListener("mouseup",r);const u=f.timelineSegments||[];let v=!1;for(let e=0;e<u.length;e++){const t=u[e],n=t.local_start_timestamp||t.start_timestamp,o=t.local_end_timestamp||t.end_timestamp;if(g>=n&&g<=o){f.setState({currentSegmentIndex:e,currentTime:g,prevCurrentTime:f.currentTime,isPlaying:!0}),v=!0;break}}if(!v&&u.length>0){let e=0,t=1/0;for(let r=0;r<u.length;r++){const n=u[r],o=n.local_start_timestamp||n.start_timestamp,s=n.local_end_timestamp||n.end_timestamp,i=Math.abs(o-g),a=Math.abs(s-g),l=Math.min(i,a);l<t&&(t=l,e=r)}const n=u[e],o=n.local_start_timestamp||n.start_timestamp;f.setState({currentSegmentIndex:e,currentTime:o,prevCurrentTime:f.currentTime,isPlaying:!0})}};return e.addEventListener("mousedown",n),()=>{e.removeEventListener("mousedown",n),document.removeEventListener("mousemove",o),document.removeEventListener("mouseup",r)}}),[v.current,l,m,p]);const b=(e,n,o)=>{if(null===e)return void a(!1);const r=new Date(1e3*e),s=r.getHours()+r.getMinutes()/60+r.getSeconds()/3600;s<n||s>o?a(!1):(t((s-n)/(o-n)*100),a(!0))},S=e=>{if(null===e)return;const t=document.getElementById("time-display");if(!t)return;const n=new Date(1e3*e),o=n.getHours().toString().padStart(2,"0"),r=n.getMinutes().toString().padStart(2,"0"),s=n.getSeconds().toString().padStart(2,"0");t.textContent=`${o}:${r}:${s}`};return o((()=>{setTimeout((()=>{f.currentTime&&(a(!0),b(f.currentTime,f.timelineStartHour||0,f.timelineEndHour||24))}),500)}),[]),r`
    <div 
      ref=${v}
      class="timeline-cursor absolute top-0 h-full z-50 transition-all duration-100 cursor-ew-resize"
      style="left: ${e}%; display: ${s?"block":"none"}; pointer-events: auto; width: 7px; margin-left: -3.5px;"
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
  `}function h(){const[e,t]=n(1);return o((()=>{console.log("SpeedControls: Setting up subscription to timelineState");const e=f.subscribe((e=>{console.log("SpeedControls: Received state update:",e),console.log("SpeedControls: Playback speed:",e.playbackSpeed),t(e.playbackSpeed)}));return console.log("SpeedControls: Initial timelineState:",f),()=>e()}),[]),r`
    <div class="mt-2 mb-4 p-2 border border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
      <div class="flex flex-col items-center">
        <div class="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">Playback Speed</div>
        
        <div class="flex flex-wrap justify-center gap-1">
          ${[.25,.5,1,1.5,2,4].map((t=>r`
            <button 
              key=${`speed-${t}`}
              class=${`speed-btn px-2 py-1 text-sm rounded-full ${t===e?"bg-green-500 text-white":"bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"} \n                font-medium transition-all focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-opacity-50`}
              data-speed=${t}
              onClick=${()=>(e=>{const t=document.querySelector("#video-player video");if(t){const n=t.playbackRate;t.playbackRate=e,console.log(`Setting video playback rate from ${n}x to ${e}x`,t),console.log(`Actual playback rate after setting: ${t.playbackRate}x`),setTimeout((()=>{t.playbackRate=e,console.log(`Re-setting playback rate to ${e}x, actual rate: ${t.playbackRate}x`)}),100)}else console.warn("Video player element not found");f.setState({playbackSpeed:e}),s(`Playback speed: ${e}x`,"info")})(t)}
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
  `}function v(){const[e,t]=n(-1),[a,l]=n(!1),[c,m]=n([]),[d,g]=n(1),u=i(null),p=i(null),v=i(null);o((()=>{const e=f.subscribe((e=>{t(e.currentSegmentIndex),l(e.isPlaying),m(e.timelineSegments||[]),g(e.playbackSpeed),y(e)}));return()=>e()}),[]);const y=e=>{const t=u.current;if(!t)return;if(!e.timelineSegments||0===e.timelineSegments.length||e.currentSegmentIndex<0||e.currentSegmentIndex>=e.timelineSegments.length)return;const n=e.timelineSegments[e.currentSegmentIndex];if(!n)return;const o=v.current!==n.id,r=o,s=n.local_start_timestamp||n.start_timestamp,i=null!==e.currentTime&&e.currentTime>=s?e.currentTime-s:0,a=null!==e.prevCurrentTime&&Math.abs(e.currentTime-e.prevCurrentTime)>1;o&&(console.log(`Segment changed from ${v.current} to ${n.id}`),v.current=n.id),r?(console.log(`Loading new segment ${n.id} (segmentChanged: ${o})`),b(n,i,e.isPlaying)):a?(console.log(`Seeking to ${i}s within current segment`),t.currentTime=i):e.isPlaying&&t.paused?t.play().catch((e=>{console.error("Error playing video:",e)})):e.isPlaying||t.paused||t.pause(),t.playbackRate!==e.playbackSpeed&&(t.playbackRate=e.playbackSpeed)},b=(e,t=0,n=!1)=>{const o=u.current;if(!o)return;console.log(`Loading segment ${e.id} at time ${t}s, autoplay: ${n}`),o.pause();const r=`/api/recordings/play/${e.id}?t=${Date.now()}`,i=()=>{console.log("Video metadata loaded"),o.currentTime=t,o.playbackRate=d,n&&o.play().catch((e=>{console.error("Error playing video:",e),s("Error playing video: "+e.message,"error")})),o.removeEventListener("loadedmetadata",i)};o.addEventListener("loadedmetadata",i),o.src=r,o.load()},S=e=>{if(null===e)return;const t=document.getElementById("time-display");if(!t)return;const n=new Date(1e3*e),o=n.getHours().toString().padStart(2,"0"),r=n.getMinutes().toString().padStart(2,"0"),s=n.getSeconds().toString().padStart(2,"0");t.textContent=`${o}:${r}:${s}`};return r`
    <div class="timeline-player-container mb-2" id="video-player">
      <div class="relative w-full bg-black rounded-lg shadow-md" style="aspect-ratio: 16/9;">
        <video
            ref=${u}
            class="w-full h-full object-contain"
            controls
            autoplay=${!1}
            muted=${!1}
            playsInline
            onended=${()=>{if(console.log("Video ended"),e<c.length-1){const t=e+1;console.log(`Playing next segment ${t}`);const n=c[t],o=n.local_start_timestamp||n.start_timestamp;f.setState({currentSegmentIndex:t,currentTime:o,isPlaying:!0,forceReload:!0})}else console.log("End of all segments"),f.setState({isPlaying:!1})}}
            ontimeupdate=${()=>{const t=u.current;if(!t)return;if(e<0||!c||0===c.length||e>=c.length)return;const n=c[e];if(!n)return;const o=(n.local_start_timestamp||n.start_timestamp)+t.currentTime;S(o),f.setState({currentTime:o,prevCurrentTime:p.current}),p.current=o}}
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
    <${h} />
  `}e("loadTimelineView",(function(){const e=document.getElementById("main-content");e&&(e.innerHTML="",l(r`<${c} client=${m}>
      <${y} />
    <//>`,e))}));const f={streams:[],timelineSegments:[],selectedStream:null,selectedDate:null,isPlaying:!1,currentSegmentIndex:-1,zoomLevel:1,timelineStartHour:0,timelineEndHour:24,currentTime:null,prevCurrentTime:null,playbackSpeed:1,showOnlySegments:!0,forceReload:!1,listeners:new Set,setState(e){Object.assign(this,e),this.notifyListeners()},subscribe(e){return this.listeners.add(e),()=>this.listeners.delete(e)},notifyListeners(){this.listeners.forEach((e=>e(this)))}};function y(){const e=function(){const e=new URLSearchParams(window.location.search);return{stream:e.get("stream")||"",date:e.get("date")||(t=new Date,`${t.getFullYear()}-${String(t.getMonth()+1).padStart(2,"0")}-${String(t.getDate()).padStart(2,"0")}`)};var t}(),[l,c]=n(!1),[m,h]=n([]),[y,b]=n(e.stream),[S,x]=n(e.date),[w,$]=n([]),k=i(null),T=i(!1),{data:_,isLoading:M,error:E}=a("streams","/api/streams",{timeout:15e3,retries:2,retryDelay:1e3});o((()=>{if(_&&Array.isArray(_)&&_.length>0&&!T.current)if(console.log("TimelinePage: Streams loaded, initializing data"),T.current=!0,h(_),f.setState({streams:_}),_.some((e=>e.name===y))&&y)console.log(`TimelinePage: Using stream from URL: ${y}`);else if(_.length>0){const e=_[0].name;console.log(`TimelinePage: Using first stream: ${e}`),b(e)}}),[_]),o((()=>{E&&(console.error("TimelinePage: Error loading streams:",E),s("Error loading streams: "+E.message,"error"))}),[E]),o((()=>{y&&(function(e,t){if(!e)return;const n=new URL(window.location.href);n.searchParams.set("stream",e),n.searchParams.set("date",t),window.history.replaceState({},"",n)}(y,S),f.setState({selectedStream:y,selectedDate:S}))}),[y,S]);const{startTime:L,endTime:P}=(e=>{const t=new Date(e);t.setHours(0,0,0,0);const n=new Date(e);return n.setHours(23,59,59,999),{startTime:t.toISOString(),endTime:n.toISOString()}})(S),{data:C,isLoading:I,error:H,refetch:R}=a(["timeline-segments",y,S],y?`/api/timeline/segments?stream=${encodeURIComponent(y)}&start=${encodeURIComponent(L)}&end=${encodeURIComponent(P)}`:null,{timeout:3e4,retries:2,retryDelay:1e3},{enabled:!!y,onSuccess:e=>{console.log("TimelinePage: Timeline data received:",e);const t=e.segments||[];if(console.log(`TimelinePage: Received ${t.length} segments`),0===t.length)return console.log("TimelinePage: No segments found"),$([]),f.setState({timelineSegments:[],currentSegmentIndex:-1,currentTime:null,isPlaying:!1}),void s("No recordings found for the selected date","warning");console.log("TimelinePage: Setting segments"),$(t);const n=t[0].start_timestamp;f.setState({timelineSegments:t,currentSegmentIndex:0,currentTime:n,prevCurrentTime:n,isPlaying:!1});const o=document.querySelector("#video-player video");o&&(o.src=`/api/recordings/play/${t[0].id}`,o.load()),s(`Loaded ${t.length} recording segments`,"success")},onError:e=>{console.error("TimelinePage: Error loading timeline data:",e),s("Error loading timeline data: "+e.message,"error"),$([])}});return r`
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
              onClick=${()=>R()}
            >
              Reload Data
            </button>
          </div>
          <select
              id="stream-selector"
              class="w-full p-2 border rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${y||""}
              onChange=${e=>{const t=e.target.value;console.log(`TimelinePage: Stream changed to ${t}`),b(t)}}
          >
            <option value="" disabled>Select a stream (${m.length} available)</option>
            ${m.map((e=>r`
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
              value=${S}
              onChange=${e=>{const t=e.target.value;console.log(`TimelinePage: Date changed to ${t}`),x(t)}}
          />
        </div>
      </div>

      <!-- Auto-load message -->
      <div class="mb-4 text-sm text-gray-500 dark:text-gray-400 italic">
        ${I?"Loading...":"Recordings auto-load when stream or date changes"}
      </div>

      <!-- Current time display -->
      <div class="flex justify-between items-center mb-2">
        <div id="time-display" class="timeline-time-display bg-gray-200 dark:bg-gray-700 px-3 py-1 rounded font-mono text-base">00:00:00</div>
      </div>

      <!-- Debug info -->
      <div class="mb-2 text-xs text-gray-500">
        Debug - isLoading: ${I?"true":"false"},
        Streams: ${m.length},
        Segments: ${w.length}
      </div>

      <!-- Content -->
      ${I?r`<${d} message="Loading timeline data..." />`:0===w.length?r`
        <div class="flex flex-col items-center justify-center py-12 text-center">
          <svg class="w-16 h-16 text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p class="text-gray-600 dark:text-gray-400 text-lg">No recordings found for the selected date and stream</p>
        </div>
      `:r`
      <!-- Video player -->
      <${v} />

      <!-- Playback controls -->
      <${t} />

        <!-- Timeline -->
        <div
            id="timeline-container"
            class="relative w-full h-24 bg-gray-200 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg mb-6 overflow-hidden"
            ref=${k}
        >
          <${g} />
          <${u} />
          <${p} />

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
//# sourceMappingURL=TimelineView-legacy-DQdiYq4n.js.map
