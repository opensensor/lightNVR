System.register(["./preact-app-legacy-PB7hD_5g.js","./DetectionOverlay-legacy-be7vwIRv.js"],(function(e,t){"use strict";var o,n,r,s,a,i,c,l,d,u,m,g,f,p;return{setters:[e=>{o=e.d,n=e.A,r=e.y,s=e.h,a=e._,i=e.s,c=e.a,l=e.b,d=e.c,u=e.e},e=>{m=e.t,g=e.e,f=e.s,p=e.c}],execute:function(){function h(){const[e,t]=o([]),[a,h]=o("4"),[b,v]=o(""),[y,w]=o(!1),[$,x]=o(!0),[C,E]=o(0),S=n(null),T=n({}),k=n({});r((()=>{i(),c(),l();const t=e=>{if("Escape"===e.key){console.log("Escape key pressed, current fullscreen state:",y);const e=document.getElementById("live-page");e&&e.classList.contains("fullscreen-mode")&&(console.log("Detected fullscreen mode via DOM, exiting fullscreen"),g(null,w))}};document.addEventListener("keydown",t);const o=()=>{_()},n=()=>{document.hidden?(console.log("Page hidden, pausing WebRTC streams"),Object.keys(T.current).forEach((e=>{const t=T.current[e];if(t&&"closed"!==t.connectionState){const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t);o&&o.pause()}}))):(console.log("Page visible, resuming WebRTC streams"),Object.keys(T.current).forEach((e=>{const t=T.current[e];if(t&&"closed"!==t.connectionState){const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t);o&&o.play().catch((t=>{console.warn(`Could not resume video for ${e}:`,t)}))}})))};window.addEventListener("beforeunload",o),document.addEventListener("visibilitychange",n);const r=setInterval((()=>{Object.keys(T.current).forEach((t=>{const o=T.current[t];if(o&&(console.debug(`WebRTC connection state for ${t}: ${o.connectionState}, ICE state: ${o.iceConnectionState}`),"failed"===o.iceConnectionState||"disconnected"===o.iceConnectionState)){console.warn(`WebRTC connection for ${t} is in ${o.iceConnectionState} state, will attempt reconnect`),O(t);const n=e.find((e=>e.name===t));n&&(console.log(`Attempting to reconnect WebRTC for stream ${t}`),j(n))}}))}),3e4);return()=>{document.removeEventListener("keydown",t),window.removeEventListener("beforeunload",o),document.removeEventListener("visibilitychange",n),clearInterval(r),_()}}),[e]),r((()=>{x(!0);const e=setTimeout((()=>{console.warn("Stream loading timed out"),x(!1),d("Loading streams timed out. Please try refreshing the page.")}),15e3);R().then((o=>{clearTimeout(e),o&&o.length>0?(t(o),v(o[0].name)):console.warn("No streams returned from API"),x(!1)})).catch((t=>{clearTimeout(e),console.error("Error loading streams:",t),d("Error loading streams: "+t.message),x(!1)}))}),[]),r((()=>{W()}),[a,b,e,C]);const R=async()=>{try{const e=new Promise(((e,t)=>{setTimeout((()=>t(new Error("Request timed out"))),5e3)})),t=fetch("/api/streams"),o=await Promise.race([t,e]);if(!o.ok)throw new Error("Failed to load streams");const n=new Promise(((e,t)=>{setTimeout((()=>t(new Error("JSON parsing timed out"))),3e3)})),r=o.json(),s=(await Promise.race([r,n])||[]).map((e=>{const t=new Promise(((t,o)=>{setTimeout((()=>o(new Error(`Timeout fetching details for stream ${e.name}`))),3e3)})),o=fetch(`/api/streams/${encodeURIComponent(e.id||e.name)}`).then((t=>{if(!t.ok)throw new Error(`Failed to load details for stream ${e.name}`);return t.json()}));return Promise.race([o,t]).catch((t=>(console.error(`Error loading details for stream ${e.name}:`,t),e)))})),a=await Promise.all(s);console.log("Loaded detailed streams for WebRTC view:",a);const i=a.filter((e=>e.is_deleted?(console.log(`Stream ${e.name} is soft deleted, filtering out`),!1):e.enabled?!!e.streaming_enabled||(console.log(`Stream ${e.name} is not configured for HLS, filtering out`),!1):(console.log(`Stream ${e.name} is inactive, filtering out`),!1)));return console.log("Filtered streams for WebRTC view:",i),i||[]}catch(e){return console.error("Error loading streams for WebRTC view:",e),d("Error loading streams: "+e.message),[]}},I=()=>{switch(a){case"1":return 1;case"2":return 2;case"4":default:return 4;case"6":return 6;case"9":return 9;case"16":return 16}},W=()=>{if(!S.current)return;const t=S.current.querySelector(".placeholder");if(S.current.innerHTML="",t&&0===e.length)return void S.current.appendChild(t);let o=e;if("1"===a&&b)o=e.filter((e=>e.name===b));else{const t=I(),n=Math.ceil(e.length/t);if(C>=n)return void E(Math.max(0,n-1));const r=C*t,s=Math.min(r+t,e.length);o=e.slice(r,s)}const n=o.map((e=>e.name));Object.keys(T.current).forEach((e=>{n.includes(e)||(console.log(`Cleaning up WebRTC connection for stream ${e} (not visible in current view)`),O(e))})),o.forEach((e=>{const t=e.id||e.name,o=document.createElement("div");o.className="video-cell",o.innerHTML=`\n        <video id="video-${e.name.replace(/\s+/g,"-")}" autoplay muted></video>\n        <div class="stream-info">\n          <span>${e.name}</span>\n          <span>${e.width}x${e.height} · ${e.fps}fps</span>\n          <div class="stream-controls">\n            <button class="snapshot-btn" data-id="${t}" data-name="${e.name}">\n              <span>📷</span> Snapshot\n            </button>\n            <button class="fullscreen-btn" data-id="${t}" data-name="${e.name}">\n              <span>⛶</span> Fullscreen\n            </button>\n          </div>\n        </div>\n        <div class="loading-indicator">\n          <div class="loading-spinner"></div>\n          <span>Connecting WebRTC...</span>\n        </div>\n      `,S.current.appendChild(o),j(e);const n=o.querySelector(".snapshot-btn");n&&n.addEventListener("click",(()=>{q(t)}));const r=o.querySelector(".fullscreen-btn");r&&r.addEventListener("click",(()=>{D(e.name)}))}))},j=e=>{const t=`video-${e.name.replace(/\s+/g,"-")}`,o=document.getElementById(t),n=o?o.closest(".video-cell"):null;if(!o||!n)return;const r=n.querySelector(".loading-indicator");r&&(r.style.display="flex");const s=`canvas-${e.name.replace(/\s+/g,"-")}`;let a=document.getElementById(s);a||(a=document.createElement("canvas"),a.id=s,a.className="detection-overlay",a.style.position="absolute",a.style.top="0",a.style.left="0",a.style.width="100%",a.style.height="100%",a.style.pointerEvents="none",n.appendChild(a));const i=new RTCPeerConnection({iceServers:[{urls:"stun:stun.l.google.com:19302"}],iceTransportPolicy:"all",bundlePolicy:"balanced",rtcpMuxPolicy:"require",sdpSemantics:"unified-plan"});T.current[e.name]=i,i.ontrack=t=>{console.log(`Track received for stream ${e.name}:`,t),"video"===t.track.kind&&(o.srcObject=t.streams[0],o.onloadeddata=()=>{r&&(r.style.display="none")})},i.onicecandidate=t=>{t.candidate&&console.log(`ICE candidate for stream ${e.name}:`,t.candidate)},i.oniceconnectionstatechange=()=>{console.log(`ICE connection state for stream ${e.name}:`,i.iceConnectionState),"failed"!==i.iceConnectionState&&"disconnected"!==i.iceConnectionState||P(e.name,"WebRTC connection failed")},i.addTransceiver("video",{direction:"recvonly"}),i.addTransceiver("audio",{direction:"recvonly"});const c=setTimeout((()=>{console.warn(`WebRTC setup timed out for stream ${e.name}`),P(e.name,"WebRTC setup timed out"),T.current[e.name]&&O(e.name)}),15e3);i.createOffer({offerToReceiveAudio:!0,offerToReceiveVideo:!0}).then((t=>(console.log(`Created offer for stream ${e.name}:`,t),console.log(`Original SDP for stream ${e.name}:`,t.sdp),t.sdp.includes("a=ice-ufrag:")&&t.sdp.includes("a=ice-pwd:")||console.warn(`SDP for stream ${e.name} is missing ice-ufrag or ice-pwd!`),console.log(`Using original offer for stream ${e.name}`),i.setLocalDescription(t)))).then((()=>(console.log(`Set local description for stream ${e.name}`),L(e.name,i.localDescription)))).then((t=>(console.log(`Received answer for stream ${e.name}:`,t),i.setRemoteDescription(new RTCSessionDescription(t))))).then((()=>{console.log(`Set remote description for stream ${e.name}`),clearTimeout(c),console.log(`Stream ${e.name} detection settings:`,{detection_based_recording:e.detection_based_recording,detection_model:e.detection_model,detection_threshold:e.detection_threshold}),e.detection_based_recording&&e.detection_model?(console.log(`Starting detection polling for stream ${e.name}`),f(e.name,a,o,k.current)):console.log(`Detection not enabled for stream ${e.name}`)})).catch((t=>{clearTimeout(c),console.error(`Error setting up WebRTC for stream ${e.name}:`,t),P(e.name,t.message)}))},L=async(e,t)=>{try{const s=localStorage.getItem("auth"),a={type:t.type,sdp:t.sdp};console.log(`Sending formatted offer for stream ${e}:`,a);const i=new AbortController,c=i.signal,l=setTimeout((()=>{console.warn(`Aborting WebRTC offer request for stream ${e} due to timeout`),i.abort()}),8e3);try{const t=await fetch(`/api/webrtc?src=${encodeURIComponent(e)}`,{method:"POST",headers:{"Content-Type":"application/json",...s?{Authorization:"Basic "+s}:{}},body:JSON.stringify(a),signal:c});if(clearTimeout(l),!t.ok)throw new Error(`Failed to send offer: ${t.status} ${t.statusText}`);const r=new AbortController,i=(r.signal,setTimeout((()=>{console.warn(`Aborting JSON parsing for stream ${e} due to timeout`),r.abort()}),5e3));try{const n=await t.text();clearTimeout(i);try{return JSON.parse(n)}catch(o){throw console.error(`Error parsing JSON for stream ${e}:`,o),console.log(`Raw response text: ${n}`),new Error(`Failed to parse WebRTC answer: ${o.message}`)}}catch(n){if(clearTimeout(i),"AbortError"===n.name)throw new Error(`WebRTC answer parsing timed out for stream ${e}`);throw n}}catch(r){if(clearTimeout(l),"AbortError"===r.name)throw new Error(`WebRTC offer request timed out for stream ${e}`);throw r}}catch(s){throw console.error(`Error sending offer for stream ${e}:`,s),s}},P=(e,t)=>{const o=`video-${e.replace(/\s+/g,"-")}`,n=document.getElementById(o),r=n?n.closest(".video-cell"):null;if(!r)return;const s=r.querySelector(".loading-indicator");s&&(s.style.display="none");let a=r.querySelector(".error-indicator");a||(a=document.createElement("div"),a.className="error-indicator",r.appendChild(a)),a.innerHTML=`\n      <div class="error-icon">!</div>\n      <p>${t||"WebRTC connection failed"}</p>\n      <button class="retry-button mt-4 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Retry</button>\n    `;const i=a.querySelector(".retry-button");i&&i.addEventListener("click",(()=>{s&&(s.style.display="flex"),a.style.display="none",O(e),fetch(`/api/streams/${encodeURIComponent(e)}`).then((e=>e.json())).then((e=>{j(e)})).catch((e=>{console.error("Error fetching stream info:",e),a.style.display="flex";const t=a.querySelector("p");t&&(t.textContent="Could not reconnect: "+e.message),s&&(s.style.display="none")}))}))},O=e=>{T.current[e]&&(T.current[e].close(),delete T.current[e]);const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t);o&&(o.srcObject=null),p(e,k.current)},_=()=>{Object.keys(T.current).forEach((e=>{O(e)}))},q=e=>{const t=document.querySelector(`.snapshot-btn[data-id="${e}"]`);if(!t)return void console.error("Stream element not found for ID:",e);const o=t.getAttribute("data-name");if(!o)return void console.error("Stream name not found for ID:",e);const n=`video-${o.replace(/\s+/g,"-")}`,r=document.getElementById(n);if(!r)return void console.error("Video element not found for stream:",o);const s=document.createElement("canvas");if(s.width=r.videoWidth,s.height=r.videoHeight,0===s.width||0===s.height)return console.error("Invalid video dimensions:",s.width,s.height),void d("Cannot take snapshot: Video not loaded or has invalid dimensions");s.getContext("2d").drawImage(r,0,0,s.width,s.height);try{window.__snapshotCanvas=s;const e=(new Date).toISOString().replace(/[:.]/g,"-"),t=`snapshot-${o.replace(/\s+/g,"-")}-${e}.jpg`;window.__snapshotFileName=t,u(s.toDataURL("image/jpeg",.95),`Snapshot: ${o}`),d("Snapshot taken successfully")}catch(a){console.error("Error creating snapshot:",a),d("Failed to create snapshot: "+a.message)}},D=e=>{const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t),n=o?o.closest(".video-cell"):null;n?document.fullscreenElement?document.exitFullscreen():n.requestFullscreen().catch((e=>{console.error(`Error attempting to enable fullscreen: ${e.message}`),d(`Could not enable fullscreen mode: ${e.message}`)})):console.error("Stream not found:",e)};return s`
    <section id="live-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div class="flex items-center space-x-2">
          <h2 class="text-xl font-bold mr-4">Live View</h2>
          <div class="flex space-x-2">
            <button 
              id="hls-toggle-btn" 
              class="px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${()=>window.location.href="/hls.html"}
            >
              HLS View
            </button>
            <button 
              id="fullscreen-btn" 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${()=>m(y,w)}
            >
              Fullscreen
            </button>
          </div>
        </div>
        <div class="controls flex items-center space-x-2">
          <select 
            id="layout-selector" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
            value=${a}
            onChange=${e=>{h(e.target.value),E(0)}}
          >
            <option value="1">Single View</option>
            <option value="2">2x1 Grid</option>
            <option value="4" selected>2x2 Grid</option>
            <option value="6">2x3 Grid</option>
            <option value="9">3x3 Grid</option>
            <option value="16">4x4 Grid</option>
          </select>
          
          ${"1"===a&&s`
            <select 
              id="stream-selector" 
              class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
              value=${b}
              onChange=${e=>v(e.target.value)}
            >
              ${e.map((e=>s`
                <option key=${e.name} value=${e.name}>${e.name}</option>
              `))}
            </select>
          `}
        </div>
      </div>
      
      <div class="flex flex-col space-y-4">
        <div 
          id="video-grid" 
          class=${`video-container layout-${a}`}
          ref=${S}
        >
          ${$?s`
            <div class="flex justify-center items-center col-span-full row-span-full h-64 w-full">
              <div class="flex flex-col items-center justify-center py-8">
                <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>
                <p class="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>
              </div>
            </div>
          `:0===e.length?s`
            <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
              <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
              <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
            </div>
          `:null}
          <!-- Video cells will be dynamically added by the updateVideoGrid function -->
        </div>
        
        ${"1"!==a&&e.length>I()?s`
          <div class="pagination-controls flex justify-center items-center space-x-4 mt-4">
            <button 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick=${()=>E(Math.max(0,C-1))}
              disabled=${0===C}
            >
              Previous
            </button>
            <span class="text-gray-700 dark:text-gray-300">
              Page ${C+1} of ${Math.ceil(e.length/I())}
            </span>
            <button 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick=${()=>E(Math.min(Math.ceil(e.length/I())-1,C+1))}
              disabled=${C>=Math.ceil(e.length/I())-1}
            >
              Next
            </button>
          </div>
        `:null}
      </div>
    </section>
  `}e({WebRTCView:h,loadWebRTCView:function(){const e=document.getElementById("main-content");e&&a((async()=>{const{render:e}=await t.import("./preact-app-legacy-PB7hD_5g.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(s`<${h} />`,e)}))}})}}}));
//# sourceMappingURL=WebRTCView-legacy-n5e8ClTy.js.map
