System.register(["./preact-app-legacy-CTh1xb99.js","./LoadingIndicator-legacy-Db7scq4a.js"],(function(e,t){"use strict";var n,r,i,o,s,l,a,m,c,g;return{setters:[e=>{n=e.d,r=e.y,i=e.h,o=e.s,s=e.A,l=e.u,a=e.E,m=e.Q,c=e.q},e=>{g=e.L}],execute:function(){function t(){const[e,t]=n(!1),[s,l]=n(1);r((()=>{const e=v.subscribe((e=>{t(e.isPlaying),l(e.zoomLevel)}));return()=>e()}),[]);const a=()=>{v.setState({isPlaying:!1});const e=document.querySelector("#video-player video");e&&e.pause()},m=()=>{if(!v.timelineSegments||0===v.timelineSegments.length)return void o("No recordings to play","warning");console.log("TimelineControls: resumePlayback called"),console.log("TimelineControls: Current state:",{segments:v.timelineSegments.length,currentSegmentIndex:v.currentSegmentIndex,currentTime:v.currentTime,selectedDate:v.selectedDate});let e=null,t=-1,n=0;if(null!==v.currentTime){console.log("TimelineControls: Using current time to find segment:",v.currentTime);for(let r=0;r<v.timelineSegments.length;r++){const i=v.timelineSegments[r];if(v.currentTime>=i.start_timestamp&&v.currentTime<=i.end_timestamp){e=i,t=r,n=v.currentTime-i.start_timestamp,console.log(`TimelineControls: Found segment ${r} containing current time, relative time: ${n}s`);break}}if(!e){let r=0,i=1/0;for(let e=0;e<v.timelineSegments.length;e++){const t=v.timelineSegments[e],n=(t.start_timestamp+t.end_timestamp)/2,o=Math.abs(v.currentTime-n);o<i&&(i=o,r=e)}e=v.timelineSegments[r],t=r,v.currentTime,e.start_timestamp,n=0,console.log(`TimelineControls: Using closest segment ${r}, relative time: ${n}s`)}}else v.currentSegmentIndex>=0&&v.currentSegmentIndex<v.timelineSegments.length?(t=v.currentSegmentIndex,e=v.timelineSegments[t],n=0,console.log(`TimelineControls: Using current segment index ${t}`)):(t=0,e=v.timelineSegments[0],n=0,console.log("TimelineControls: Falling back to first segment"));console.log(`TimelineControls: Playing segment ${t} (ID: ${e.id}) at time ${n}s`),v.currentSegmentIndex=t,v.currentTime=e.start_timestamp+n,v.isPlaying=!0,v.directVideoControl=!0,v.setState({}),setTimeout((()=>{console.log("TimelineControls: Resetting directVideoControl flag"),v.directVideoControl=!1,v.setState({})}),3e3);const r=document.querySelector("#video-player video");if(r){r.pause();const t=()=>{console.log(`TimelineControls: Video metadata loaded, setting time to ${n}s`);try{console.log("TimelineControls: Video metadata",{duration:r.duration,width:r.videoWidth,height:r.videoHeight,segment:e.id,segmentDuration:e.end_timestamp-e.start_timestamp});const t=Math.max(0,Math.min(n,r.duration||0));r.currentTime=t,setTimeout((()=>{v.isPlaying&&(console.log("TimelineControls: Starting video playback"),r.play().then((()=>{console.log("TimelineControls: Video playback started successfully");const t=(e=1)=>{e>5||setTimeout((()=>{r.paused&&v.isPlaying&&(console.log(`TimelineControls: Video paused unexpectedly (attempt ${e}), trying to resume`),r.play().catch((t=>{console.error(`Error resuming video (attempt ${e}):`,t)})),t(e+1))}),500*e)};t();const i=e.end_timestamp-e.start_timestamp;if(console.log(`TimelineControls: Segment duration: ${i}s, video duration: ${r.duration}s`),r.duration<i-1){console.log("TimelineControls: Video duration is shorter than segment duration, will monitor playback");const e=setInterval((()=>{v.isPlaying&&r?r.currentTime>r.duration-.5&&n+r.currentTime<i&&(console.log("TimelineControls: Reached end of video but not end of segment, restarting video"),r.currentTime=0,r.play().catch((e=>{console.error("Error restarting video:",e)}))):clearInterval(e)}),500)}})).catch((e=>{console.error("Error playing video:",e),o("Error playing video: "+e.message,"error")})))}),100)}catch(i){console.error("TimelineControls: Error in handleMetadataLoaded:",i)}finally{r.removeEventListener("loadedmetadata",t)}};r.addEventListener("loadedmetadata",t),console.log(`TimelineControls: Loading video from segment ${e.id}`),r.src=`/api/recordings/play/${e.id}?t=${Date.now()}`,r.load()}else console.error("TimelineControls: No video element found"),o("Error: Video player not found","error")};return i`
    <div class="timeline-controls flex justify-between items-center mb-2">
      <div class="flex items-center">
        <button
          id="play-button"
          class="w-10 h-10 rounded-full bg-green-600 hover:bg-green-700 text-white flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-offset-1 transition-colors shadow-sm mr-2"
          onClick=${()=>{console.log("TimelineControls: togglePlayback called"),console.log("TimelineControls: Current state before toggle:",{isPlaying:e,currentTime:v.currentTime,currentSegmentIndex:v.currentSegmentIndex,segmentsCount:v.timelineSegments?v.timelineSegments.length:0}),e?a():m()}}
          title=${e?"Pause":"Play from current position"}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            ${e?i`
                <!-- Pause icon - two vertical bars -->
                <rect x="6" y="6" width="4" height="12" rx="1" fill="white" />
                <rect x="14" y="6" width="4" height="12" rx="1" fill="white" />
              `:i`
                <!-- Play icon - triangle -->
                <path d="M8 5.14v14l11-7-11-7z" fill="white" />
              `}
          </svg>
        </button>
        <span class="text-xs text-gray-600 dark:text-gray-300">Play from current position</span>
      </div>

      <div class="flex items-center gap-1">
        <span class="text-xs text-gray-600 dark:text-gray-300 mr-1">Zoom:</span>
        <button
          id="zoom-out-button"
          class="w-6 h-6 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors"
          onClick=${()=>{if(s>1){const e=s/2;v.setState({zoomLevel:e}),o(`Zoomed out: ${24/e} hours view`,"info")}}}
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
          onClick=${()=>{if(s<8){const e=2*s;v.setState({zoomLevel:e}),o(`Zoomed in: ${24/e} hours view`,"info")}}}
          title="Zoom In (Show less time)"
          disabled=${s>=8}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>
  `}function d(){const[e,t]=n(0),[o,s]=n(24),[l,a]=n(1);return r((()=>{const e=v.subscribe((e=>{console.log("TimelineRuler: State update received",{zoomLevel:e.zoomLevel,segmentsCount:e.timelineSegments?e.timelineSegments.length:0,currentTime:e.currentTime});const n=24/e.zoomLevel;let r=12;if(null!==e.currentTime){const t=new Date(1e3*e.currentTime);r=t.getHours()+t.getMinutes()/60+t.getSeconds()/3600}else if(e.timelineSegments&&e.timelineSegments.length>0){let t=24,n=0;e.timelineSegments.forEach((e=>{const r=new Date(1e3*e.start_timestamp),i=new Date(1e3*e.end_timestamp),o=r.getHours()+r.getMinutes()/60+r.getSeconds()/3600,s=i.getHours()+i.getMinutes()/60+i.getSeconds()/3600;t=Math.min(t,o),n=Math.max(n,s)})),r=(t+n)/2,console.log("TimelineRuler: Calculated center from segments",{earliestHour:t,latestHour:n,centerHour:r})}let i=Math.max(0,r-n/2),o=Math.min(24,i+n);24===o&&n<24?(i=Math.max(0,24-n),o=24):0===i&&n<24&&(o=Math.min(24,n)),t(i),s(o),a(e.zoomLevel),console.log("TimelineRuler: Calculated time range",{newStartHour:i,newEndHour:o,hoursPerView:n,centerHour:r}),v.timelineStartHour===i&&v.timelineEndHour===o||(console.log("TimelineRuler: Updating global state with new time range"),v.setState({timelineStartHour:i,timelineEndHour:o}))}));return()=>e()}),[]),i`
    <div class="timeline-ruler relative w-full h-8 bg-gray-300 dark:bg-gray-800 border-b border-gray-400 dark:border-gray-600">
      ${(()=>{const t=[];for(let n=Math.floor(e);n<=Math.ceil(o);n++)if(n>=0&&n<=24){const r=(n-e)/(o-e)*100;if(t.push(i`
          <div
            key="tick-${n}"
            class="absolute top-0 w-px h-5 bg-gray-500 dark:bg-gray-400"
            style="left: ${r}%;"
          ></div>
        `),t.push(i`
          <div
            key="label-${n}"
            class="absolute top-0 text-xs text-gray-600 dark:text-gray-300 transform -translate-x-1/2"
            style="left: ${r}%;"
          >
            ${n}:00
          </div>
        `),n<24&&l>=2){const r=(n+.5-e)/(o-e)*100;if(t.push(i`
            <div
              key="tick-${n}-30"
              class="absolute top-2 w-px h-3 bg-gray-400 dark:bg-gray-500"
              style="left: ${r}%;"
            ></div>
          `),l>=4){const r=(n+.25-e)/(o-e)*100,s=(n+.75-e)/(o-e)*100;t.push(i`
              <div
                key="tick-${n}-15"
                class="absolute top-3 w-px h-2 bg-gray-400 dark:bg-gray-500"
                style="left: ${r}%;"
              ></div>
            `),t.push(i`
              <div
                key="tick-${n}-45"
                class="absolute top-3 w-px h-2 bg-gray-400 dark:bg-gray-500"
                style="left: ${s}%;"
              ></div>
            `)}}}return t})()}
      <div class="absolute bottom-0 left-0 text-xs text-gray-500 px-1">
        Zoom: ${l}x (${Math.round(24/l)} hours)
      </div>
    </div>
  `}function u({segments:e}){const[t,o]=n(e||[]),[l,a]=n(0),[m,c]=n(24),[g,d]=n(-1);r((()=>{console.log(`TimelineSegments: Received segments from props: ${e?e.length:0}`),e&&e.length>0&&o(e)}),[e]);const u=s(null),p=s(!1),h=s(0),f=s([]);r((()=>{const e=v.subscribe((e=>{console.log(`TimelineSegments: State update received, segments: ${e.timelineSegments?e.timelineSegments.length:0}`),e.timelineSegments&&(!f.current||e.timelineSegments.length!==f.current.length||JSON.stringify(e.timelineSegments)!==JSON.stringify(f.current)||e.forceReload)&&(console.log(`TimelineSegments: Updating segments (${e.timelineSegments.length})`),o(e.timelineSegments),f.current=[...e.timelineSegments],h.current=Date.now());const t=void 0!==e.timelineStartHour?e.timelineStartHour:0,n=void 0!==e.timelineEndHour?e.timelineEndHour:24;console.log(`TimelineSegments: Time range update - startHour: ${t}, endHour: ${n}`),a(t),c(n),d(e.currentSegmentIndex||-1)}));return v.timelineSegments&&v.timelineSegments.length>0&&(console.log(`TimelineSegments: Initial load of segments (${v.timelineSegments.length})`),o(v.timelineSegments),f.current=[...v.timelineSegments],d(v.currentSegmentIndex||0),void 0!==v.timelineStartHour&&a(v.timelineStartHour),void 0!==v.timelineEndHour&&c(v.timelineEndHour),h.current=Date.now()),()=>e()}),[]),r((()=>{const e=u.current;if(!e)return;const t=t=>{(t.target===e||t.target.classList.contains("timeline-clickable-area"))&&(p.current=!0,S(t),document.addEventListener("mousemove",n),document.addEventListener("mouseup",r))},n=e=>{p.current&&S(e)},r=()=>{p.current=!1,document.removeEventListener("mousemove",n),document.removeEventListener("mouseup",r)};return e.addEventListener("mousedown",t),()=>{e.removeEventListener("mousedown",t),document.removeEventListener("mousemove",n),document.removeEventListener("mouseup",r)}}),[l,m,t]);const S=e=>{const n=u.current;if(!n)return;const r=n.getBoundingClientRect(),i=e.clientX-r.left,o=r.width,s=l+i/o*(m-l),a=new Date(v.selectedDate);a.setHours(Math.floor(s)),a.setMinutes(Math.floor(s%1*60)),a.setSeconds(Math.floor(s%1*60%1*60));const c=a.getTime()/1e3;v.setState({currentTime:c,prevCurrentTime:v.currentTime,isPlaying:!1});let g=!1;for(let l=0;l<t.length;l++){const n=t[l],r=n.local_start_timestamp||n.start_timestamp,i=n.local_end_timestamp||n.end_timestamp;if(c>=r&&c<=i){console.log(`TimelineSegments: Found segment ${l} containing timestamp`),v.setState({currentSegmentIndex:l}),e.target.classList.contains("timeline-segment")&&y(l,c-r),g=!0;break}}g||v.setState({currentSegmentIndex:-1})},y=(e,n=null)=>{if(console.log(`TimelineSegments: playSegment(${e}, ${n})`),e<0||e>=t.length)return void console.warn(`TimelineSegments: Invalid segment index: ${e}`);const r=t[e],i=r.local_start_timestamp||r.start_timestamp,o=null!==n?i+n:i;v.setState({isPlaying:!1,currentSegmentIndex:-1}),document.body.offsetHeight,setTimeout((()=>{v.setState({currentSegmentIndex:e,currentTime:o,isPlaying:!0,forceReload:!0}),setTimeout((()=>{const e=document.querySelector("#video-player video");e&&(e.pause(),e.removeAttribute("src"),e.load(),e.src=`/api/recordings/play/${r.id}?t=${Date.now()}`,e.onloadedmetadata=()=>{const t=null!==n?n:0;e.currentTime=t,e.play().catch((e=>console.error("Error playing video:",e)))})}),50)}),50)};return i`
    <div
      class="timeline-segments relative w-full h-16 pt-2"
      ref=${u}
    >
      ${(()=>{if(console.log("TimelineSegments: renderSegments called"),console.log("TimelineSegments: segments:",t),console.log("TimelineSegments: startHour:",l,"endHour:",m),!t||0===t.length)return console.log("TimelineSegments: No segments to render"),i`<div class="text-center text-red-500 font-bold">No segments to display</div>`;console.log("TimelineSegments: Rendering segments:",t.length);const e=[],n=new Map;console.log("TimelineSegments: Starting to process segments");let r=0,o=0;t.forEach(((e,t)=>{const i=e.start_timestamp,s=e.end_timestamp,a=new Date(1e3*i),c=new Date(1e3*s),g=a.getHours()+a.getMinutes()/60+a.getSeconds()/3600,d=c.getHours()+c.getMinutes()/60+c.getSeconds()/3600;if(d<l||g>m)return o++,void(t<5&&console.log(`TimelineSegments: Skipping segment ${t}, startHour=${g}, endHour=${d}, visible range=${l}-${m}`));r++;const u=Math.floor(g),p=Math.min(Math.ceil(d),24);for(let r=u;r<p;r++)r>=l&&r<=m&&(n.has(r)||n.set(r,[]),n.get(r).push(t))}));const s=[];let a=null;[...t].sort(((e,t)=>e.start_timestamp-t.start_timestamp)).forEach(((e,t)=>{a?e.start_timestamp-a.end_timestamp<=1?(a.end_timestamp=e.end_timestamp,a.originalIndices.push(t),e.has_detection&&(a.has_detection=!0)):(s.push(a),a={...e,originalIndices:[t]}):a={...e,originalIndices:[t]}})),a&&s.push(a),s.forEach(((t,n)=>{const r=t.start_timestamp,o=t.end_timestamp,s=new Date(1e3*r),a=new Date(1e3*o),c=s.getHours()+s.getMinutes()/60+s.getSeconds()/3600,g=a.getHours()+a.getMinutes()/60+a.getSeconds()/3600;if(g<l||c>m)return;const d=Math.max(c,l),u=Math.min(g,m),p=(d-l)/(m-l)*100,h=(u-d)/(m-l)*100,f=`${Math.round(o-r)}s`,v=s.toLocaleTimeString(),S=a.toLocaleTimeString();e.push(i`
        <div
          key="segment-${n}"
          class="timeline-segment absolute rounded-sm transition-all duration-200 ${t.has_detection?"bg-red-500":"bg-blue-500"}"
          style="left: ${p}%; width: ${h}%; height: ${80}%; top: 50%; transform: translateY(-50%);"
          title="${v} - ${S} (${f})"
        ></div>
      `)}));for(let t=Math.floor(l);t<Math.ceil(m);t++)if(!n.has(t)){const n=(t-l)/(m-l)*100,r=100/(m-l);e.push(i`
          <div
            key="clickable-${t}"
            class="timeline-clickable-area absolute h-full cursor-pointer"
            style="left: ${n}%; width: ${r}%;"
            data-hour=${t}
          ></div>
        `)}return console.log(`TimelineSegments: Rendering complete. Total: ${t.length}, Visible: ${r}, Skipped: ${o}, Final rendered: ${e.length}`),e})()}
    </div>
  `}function p(){const[e,t]=n(0),[o,l]=n(!1),[a,m]=n(0),[c,g]=n(24),[d,u]=n(null),[p,h]=n(!1),f=s(null);s(null);const S=s(0),y=s(((e,t)=>{let n;return function(...r){n&&clearTimeout(n),n=setTimeout((()=>{e.apply(this,r)}),t)}})(((e,t,n)=>{b(e,t,n)}),100)).current;r((()=>{const e=v.subscribe((e=>{console.log("TimelineCursor: State update received",{currentTime:e.currentTime,startHour:e.timelineStartHour,endHour:e.timelineEndHour,segmentsCount:e.timelineSegments?e.timelineSegments.length:0,isDragging:p,userControllingCursor:e.userControllingCursor}),m(e.timelineStartHour||0),g(e.timelineEndHour||24),p||e.userControllingCursor||(u(e.currentTime),T(e.currentTime),y(e.currentTime,e.timelineStartHour||0,e.timelineEndHour||24))}));return()=>e()}),[p,y]),r((()=>{const e=f.current;if(!e)return;const n=e=>{e.preventDefault(),e.stopPropagation(),console.log("TimelineCursor: Mouse down event"),S.current=e.clientX,h(!0),v.userControllingCursor=!0,v.setState({}),document.addEventListener("mousemove",r),document.addEventListener("mouseup",i)},r=n=>{if(!p)return;const r=e.parentElement;if(!r)return;const i=r.getBoundingClientRect(),o=Math.max(0,Math.min(n.clientX-i.left,i.width))/i.width*100;t(o);const s=a+o/100*(c-a),l=new Date;v.selectedDate&&l.setTime(new Date(v.selectedDate).getTime()),l.setHours(Math.floor(s)),l.setMinutes(Math.floor(s%1*60)),l.setSeconds(Math.floor(s%1*60%1*60)),l.setMilliseconds(0);const m=l.getTime()/1e3;u(m),T(m)},i=t=>{if(!p)return;const n=e.parentElement;if(!n)return;const o=n.getBoundingClientRect(),s=Math.max(0,Math.min(t.clientX-o.left,o.width)),l=o.width,m=s/l*100;console.log("TimelineCursor: Mouse up at position",{positionPercent:m,clickX:s,containerWidth:l});const g=c-a,d=a+m/100*g;console.log("TimelineCursor: Calculated hour",{hour:d,startHour:a,endHour:c,hourRange:g});const u=new Date;v.selectedDate&&u.setTime(new Date(v.selectedDate).getTime()),u.setHours(Math.floor(d)),u.setMinutes(Math.floor(d%1*60)),u.setSeconds(Math.floor(d%1*60%1*60)),u.setMilliseconds(0);const f=u.getTime()/1e3;console.log("TimelineCursor: Converted to timestamp",{timestamp:f,dateTime:u.toLocaleString(),selectedDate:v.selectedDate}),console.log("TimelineCursor: Mouse up event"),h(!1),document.removeEventListener("mousemove",r),document.removeEventListener("mouseup",i),setTimeout((()=>{console.log("TimelineCursor: Releasing cursor control"),v.userControllingCursor=!1,v.setState({})}),100),v.currentTime=f,v.prevCurrentTime=v.currentTime,v.isPlaying=!1,v.setState({});const S=v.timelineSegments||[];console.log("TimelineCursor: Searching for segment containing timestamp",{timestamp:f,segmentsCount:S.length});let y=!1,b=-1,T=1/0;for(let e=0;e<S.length;e++){const t=S[e],n=t.local_start_timestamp||t.start_timestamp,r=t.local_end_timestamp||t.end_timestamp;if(e<3&&console.log(`TimelineCursor: Segment ${e}`,{startTimestamp:n,endTimestamp:r,startTime:new Date(1e3*n).toLocaleTimeString(),endTime:new Date(1e3*r).toLocaleTimeString()}),f>=n&&f<=r){console.log(`TimelineCursor: Found exact match at segment ${e}`),v.currentSegmentIndex=e,v.setState({}),y=!0;break}const i=(n+r)/2,o=Math.abs(f-i);o<T&&(T=o,b=e)}y||(b>=0?(console.log(`TimelineCursor: No exact match, using closest segment ${b}`),v.currentSegmentIndex=b,v.setState({})):(console.log("TimelineCursor: No segments found at all"),v.currentSegmentIndex=-1,v.setState({})))};return e.addEventListener("mousedown",n),()=>{e.removeEventListener("mousedown",n),document.removeEventListener("mousemove",r),document.removeEventListener("mouseup",i)}}),[f.current,a,c,p]);const b=(e,n,r)=>{if(console.log("TimelineCursor: updateCursorPosition called",{time:e,startHr:n,endHr:r}),null===e)return console.log("TimelineCursor: No current time, hiding cursor"),void l(!1);const i=new Date(1e3*e),o=i.getHours()+i.getMinutes()/60+i.getSeconds()/3600;if(console.log("TimelineCursor: Calculated hour",{hour:o,timeString:i.toLocaleTimeString()}),o<n||o>r)return console.log("TimelineCursor: Time outside visible range, hiding cursor"),void l(!1);const s=(o-n)/(r-n)*100;console.log("TimelineCursor: Calculated position",{position:s,hour:o,startHr:n,endHr:r}),t(s),l(!0)},T=e=>{if(null===e)return;const t=document.getElementById("time-display");if(!t)return;const n=new Date(1e3*e),r=n.getHours().toString().padStart(2,"0"),i=n.getMinutes().toString().padStart(2,"0"),o=n.getSeconds().toString().padStart(2,"0");t.textContent=`${r}:${i}:${o}`};return r((()=>{console.log("TimelineCursor: Initializing cursor position"),console.log("TimelineCursor: Initial state",{currentTime:v.currentTime,startHour:v.timelineStartHour,endHour:v.timelineEndHour,segments:v.timelineSegments?v.timelineSegments.length:0});const e=()=>{if(console.log("TimelineCursor: Checking timelineState directly",{currentTime:v.currentTime,segmentsLength:v.timelineSegments?v.timelineSegments.length:0,currentSegmentIndex:v.currentSegmentIndex}),v.currentTime)return console.log("TimelineCursor: Setting initial cursor position with current time"),l(!0),b(v.currentTime,v.timelineStartHour||0,v.timelineEndHour||24),!0;if(v.timelineSegments&&v.timelineSegments.length>0){console.log("TimelineCursor: Using first segment start time for cursor");const e=v.timelineSegments[0].start_timestamp;return console.log("TimelineCursor: Directly setting timelineState properties"),v.currentTime=e,v.currentSegmentIndex=0,v.setState({}),l(!0),b(e,v.timelineStartHour||0,v.timelineEndHour||24),!0}return!1};e()||(console.log("TimelineCursor: Initial initialization failed, will retry after delay"),[100,300,500,1e3].forEach(((t,n)=>{setTimeout((()=>{o||(console.log(`TimelineCursor: Retry initialization attempt ${n+1}`),e())}),t)})))}),[]),i`
    <div
      ref=${f}
      class="timeline-cursor absolute top-0 h-full z-50 transition-all duration-100 cursor-ew-resize"
      style="left: ${e}%; display: ${o?"block":"none"}; pointer-events: auto; width: 7px; margin-left: -3.5px;"
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
  `}function h(){const[e,t]=n(1);return r((()=>{const e=v.subscribe((e=>{t(e.playbackSpeed)}));return()=>e()}),[]),i`
    <div class="mt-2 mb-4 p-2 border border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-sm">
      <div class="flex flex-col items-center">
        <div class="text-sm font-semibold mb-2 text-gray-700 dark:text-gray-300">Playback Speed</div>
        
        <div class="flex flex-wrap justify-center gap-1">
          ${[.25,.5,1,1.5,2,4].map((t=>i`
            <button 
              key=${`speed-${t}`}
              class=${`speed-btn px-2 py-1 text-sm rounded-full ${t===e?"bg-green-500 text-white":"bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"} \n                font-medium transition-all focus:outline-none focus:ring-1 focus:ring-green-500 focus:ring-opacity-50`}
              data-speed=${t}
              onClick=${()=>(e=>{const t=document.querySelector("#video-player video");t&&(t.playbackRate=e),v.setState({playbackSpeed:e}),o(`Playback speed: ${e}x`,"info")})(t)}
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
  `}function f(){const[e,t]=n(-1),[l,a]=n(!1),[m,c]=n([]),[g,d]=n(1),u=s(null),p=s(null),f=s(null);r((()=>{const e=v.subscribe((e=>{t(e.currentSegmentIndex),a(e.isPlaying),c(e.timelineSegments||[]),d(e.playbackSpeed),S(e)}));return()=>e()}),[]);const S=e=>{const t=u.current;if(!t)return;if(!e.timelineSegments||0===e.timelineSegments.length||e.currentSegmentIndex<0||e.currentSegmentIndex>=e.timelineSegments.length)return;const n=e.timelineSegments[e.currentSegmentIndex];if(!n)return;const r=f.current!==n.id,i=r,o=null!==e.currentTime&&e.currentTime>=n.start_timestamp?e.currentTime-n.start_timestamp:0,s=null!==e.prevCurrentTime&&Math.abs(e.currentTime-e.prevCurrentTime)>1;r&&(console.log(`Segment changed from ${f.current} to ${n.id}`),f.current=n.id),i?(console.log(`Loading new segment ${n.id} (segmentChanged: ${r})`),y(n,o,e.isPlaying)):s?(console.log(`Seeking to ${o}s within current segment`),t.currentTime=o):e.isPlaying&&t.paused?t.play().catch((e=>{console.error("Error playing video:",e)})):e.isPlaying||t.paused||t.pause(),t.playbackRate!==e.playbackSpeed&&(t.playbackRate=e.playbackSpeed)},y=(e,t=0,n=!1)=>{const r=u.current;if(!r)return;console.log(`Loading segment ${e.id} at time ${t}s, autoplay: ${n}`),r.pause();const i=`/api/recordings/play/${e.id}?t=${Date.now()}`,s=()=>{console.log("Video metadata loaded"),r.currentTime=t,r.playbackRate=g,n&&r.play().catch((e=>{console.error("Error playing video:",e),o("Error playing video: "+e.message,"error")})),r.removeEventListener("loadedmetadata",s)};r.addEventListener("loadedmetadata",s),r.src=i,r.load()},b=e=>{if(null===e)return;const t=document.getElementById("time-display");if(!t)return;const n=new Date(1e3*e),r=n.getHours().toString().padStart(2,"0"),i=n.getMinutes().toString().padStart(2,"0"),o=n.getSeconds().toString().padStart(2,"0");t.textContent=`${r}:${i}:${o}`};return i`
    <div class="timeline-player-container mb-2" id="video-player">
      <div class="relative w-full bg-black rounded-lg shadow-md" style="aspect-ratio: 16/9;">
        <video
            ref=${u}
            class="w-full h-full object-contain"
            controls
            autoplay=${!1}
            muted=${!1}
            playsInline
            onended=${()=>{if(console.log("Video ended"),e<m.length-1){const t=e+1;console.log(`Playing next segment ${t}`),v.setState({currentSegmentIndex:t,currentTime:m[t].start_timestamp,isPlaying:!0,forceReload:!0})}else console.log("End of all segments"),v.setState({isPlaying:!1})}}
            ontimeupdate=${()=>{const t=u.current;if(!t)return;if(e<0||!m||0===m.length||e>=m.length)return;const n=m[e];if(!n)return;const r=n.start_timestamp+t.currentTime;b(r),v.setState({currentTime:r,prevCurrentTime:p.current}),p.current=r}}
        ></video>
        
        <!-- Add a message for invalid segments -->
        <div 
          class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-70 text-white text-center p-4 ${e>=0&&m.length>0?"hidden":""}"
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
  `}e("loadTimelineView",(function(){const e=document.getElementById("main-content");e&&(e.innerHTML="",a(i`<${m} client=${c}>
      <${S} />
    <//>`,e))}));const v={streams:[],timelineSegments:[],selectedStream:null,selectedDate:null,isPlaying:!1,currentSegmentIndex:-1,zoomLevel:1,timelineStartHour:0,timelineEndHour:24,currentTime:null,prevCurrentTime:null,playbackSpeed:1,showOnlySegments:!0,forceReload:!1,userControllingCursor:!1,listeners:new Set,lastUpdateTime:0,pendingUpdates:{},setState(e){const t=Date.now();console.log("timelineState: setState called with",e),console.log("timelineState: current state before update",{currentTime:this.currentTime,currentSegmentIndex:this.currentSegmentIndex,segmentsLength:this.timelineSegments.length}),void 0!==e.currentTime&&!e.currentSegmentIndex&&!e.isPlaying&&t-this.lastUpdateTime<250?console.log("timelineState: Skipping frequent time update"):(Object.assign(this,e),e.forceReload&&(this.forceReload=!1),this.lastUpdateTime=t,console.log("timelineState: state after update",{currentTime:this.currentTime,currentSegmentIndex:this.currentSegmentIndex,segmentsLength:this.timelineSegments.length}),this.notifyListeners())},subscribe(e){return this.listeners.add(e),()=>this.listeners.delete(e)},notifyListeners(){this.listeners.forEach((e=>e(this)))},flushPendingUpdates(){Object.keys(this.pendingUpdates).length>0&&(Object.assign(this,this.pendingUpdates),this.pendingUpdates={},this.lastUpdateTime=Date.now(),this.notifyListeners())}};function S(){const e=function(){const e=new URLSearchParams(window.location.search);return{stream:e.get("stream")||"",date:e.get("date")||(t=new Date,`${t.getFullYear()}-${String(t.getMonth()+1).padStart(2,"0")}-${String(t.getDate()).padStart(2,"0")}`)};var t}(),[a,m]=n(!1),[c,h]=n([]),[S,y]=n(e.stream),[b,T]=n(e.date),[x,$]=n([]),w=s(null),k=s(!1),C=s(null);r((()=>(C.current=setInterval((()=>{v.flushPendingUpdates()}),200),()=>{C.current&&clearInterval(C.current)})),[]);const{data:H,isLoading:I,error:E}=l("streams","/api/streams",{timeout:15e3,retries:2,retryDelay:1e3});r((()=>{if(H&&Array.isArray(H)&&H.length>0&&!k.current)if(console.log("TimelinePage: Streams loaded, initializing data"),k.current=!0,h(H),v.setState({streams:H}),H.some((e=>e.name===S))&&S)console.log(`TimelinePage: Using stream from URL: ${S}`);else if(H.length>0){const e=H[0].name;console.log(`TimelinePage: Using first stream: ${e}`),y(e)}}),[H]),r((()=>{E&&(console.error("TimelinePage: Error loading streams:",E),o("Error loading streams: "+E.message,"error"))}),[E]),r((()=>{S&&(function(e,t){if(!e)return;const n=new URL(window.location.href);n.searchParams.set("stream",e),n.searchParams.set("date",t),window.history.replaceState({},"",n)}(S,b),v.setState({selectedStream:S,selectedDate:b}))}),[S,b]);const{startTime:M,endTime:L}=(e=>{const t=new Date(e);t.setHours(0,0,0,0);const n=new Date(e);return n.setHours(23,59,59,999),{startTime:t.toISOString(),endTime:n.toISOString()}})(b),{data:P,isLoading:D,error:_,refetch:R}=l(["timeline-segments",S,b],S?`/api/timeline/segments?stream=${encodeURIComponent(S)}&start=${encodeURIComponent(M)}&end=${encodeURIComponent(L)}`:null,{timeout:3e4,retries:2,retryDelay:1e3},{enabled:!!S,onSuccess:e=>{console.log("TimelinePage: Timeline data received:",e);const t=e.segments||[];if(console.log(`TimelinePage: Received ${t.length} segments`),0===t.length)return console.log("TimelinePage: No segments found"),$([]),v.setState({timelineSegments:[],currentSegmentIndex:-1,currentTime:null,isPlaying:!1}),void o("No recordings found for the selected date","warning");const n=JSON.parse(JSON.stringify(t));n.slice(0,3).forEach(((e,t)=>{const n=new Date(1e3*e.start_timestamp),r=new Date(1e3*e.end_timestamp);console.log(`TimelinePage: Segment ${t} - Start: ${n.toLocaleTimeString()}, End: ${r.toLocaleTimeString()}`)})),console.log("TimelinePage: Setting segments"),$(n),document.body.offsetHeight;const r=n[0].start_timestamp;console.log("TimelinePage: Setting initial segment and time",{firstSegmentId:n[0].id,startTime:new Date(1e3*r).toLocaleTimeString()}),console.log("TimelinePage: Directly setting timelineState properties"),v.timelineSegments=n,v.currentSegmentIndex=0,v.currentTime=r,v.prevCurrentTime=r,v.isPlaying=!1,v.forceReload=!0,v.zoomLevel=1,v.selectedDate=b,v.setState({}),console.log("TimelinePage: Updated timelineState with segments"),setTimeout((()=>{console.log("TimelinePage: State after update (delayed check):",{segmentsLength:v.timelineSegments.length,currentSegmentIndex:v.currentSegmentIndex,currentTime:v.currentTime}),v.currentTime&&-1!==v.currentSegmentIndex||(console.log("TimelinePage: State not properly updated, forcing update"),v.setState({currentSegmentIndex:0,currentTime:r,prevCurrentTime:r}))}),100);const i=document.querySelector("#video-player video");i&&(i.src=`/api/recordings/play/${n[0].id}?t=${Date.now()}`,i.load()),o(`Loaded ${n.length} recording segments`,"success")},onError:e=>{console.error("TimelinePage: Error loading timeline data:",e),o("Error loading timeline data: "+e.message,"error"),$([])}});return i`
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
              value=${S||""}
              onChange=${e=>{const t=e.target.value;console.log(`TimelinePage: Stream changed to ${t}`),y(t)}}
          >
            <option value="" disabled>Select a stream (${c.length} available)</option>
            ${c.map((e=>i`
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
              value=${b}
              onChange=${e=>{const t=e.target.value;console.log(`TimelinePage: Date changed to ${t}`),T(t)}}
          />
        </div>
      </div>

      <!-- Auto-load message -->
      <div class="mb-4 text-sm text-gray-500 dark:text-gray-400 italic">
        ${D?"Loading...":"Recordings auto-load when stream or date changes"}
      </div>

      <!-- Current time display -->
      <div class="flex justify-between items-center mb-2">
        <div id="time-display" class="timeline-time-display bg-gray-200 dark:bg-gray-700 px-3 py-1 rounded font-mono text-base">00:00:00</div>
      </div>

      <!-- Debug info -->
      <div class="mb-2 text-xs text-gray-500">
        Debug - isLoading: ${D?"true":"false"},
        Streams: ${c.length},
        Segments: ${x.length}
      </div>

      <!-- Content -->
      ${D?i`<${g} message="Loading timeline data..." />`:0===x.length?i`
        <div class="flex flex-col items-center justify-center py-12 text-center">
          <svg class="w-16 h-16 text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p class="text-gray-600 dark:text-gray-400 text-lg">No recordings found for the selected date and stream</p>
        </div>
      `:i`
      <!-- Video player -->
      <${f} />

      <!-- Playback controls -->
      <${t} />

        <!-- Timeline -->
        <div
            id="timeline-container"
            class="relative w-full h-24 bg-gray-200 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg mb-6 overflow-hidden"
            ref=${w}
        >
          <${d} />
          <${u} segments=${x} />
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
          <li>Click on the timeline to position the cursor at a specific time</li>
          <li>Drag the orange cursor to navigate precisely</li>
          <li>Click on a segment (blue bar) to play that recording</li>
          <li>Use the play button to start playback from the current cursor position</li>
          <li>Use the zoom buttons to adjust the timeline scale</li>
        </ul>
      </div>
    </div>
  `}}}}));
//# sourceMappingURL=TimelineView-legacy-CSyZqYAg.js.map
