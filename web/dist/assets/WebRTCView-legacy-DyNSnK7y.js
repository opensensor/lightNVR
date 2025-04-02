System.register(["./preact-app-legacy-DGNEAXcV.js","./DetectionOverlay-legacy-Q18gFRPV.js"],(function(e,t){"use strict";var n,o,r,s,a,i,c,l,d,u,m,g,f,p;return{setters:[e=>{n=e.d,o=e.A,r=e.y,s=e.h,a=e._,i=e.s,c=e.a,l=e.b,d=e.c,u=e.e},e=>{m=e.t,g=e.e,f=e.s,p=e.c}],execute:function(){function h(){const[e,t]=n([]),[a,h]=n("4"),[b,y]=n(""),[v,w]=n(!1),[$,x]=n(!0),[C,E]=n(0),T=o(null),S=o({}),k=o({});r((()=>{i(),c(),l();const t=e=>{if("Escape"===e.key){console.log("Escape key pressed, current fullscreen state:",v);const e=document.getElementById("live-page");e&&e.classList.contains("fullscreen-mode")&&(console.log("Detected fullscreen mode via DOM, exiting fullscreen"),g(null,w))}};document.addEventListener("keydown",t);const n=()=>{_()},o=()=>{document.hidden?(console.log("Page hidden, pausing WebRTC streams"),Object.keys(S.current).forEach((e=>{const t=S.current[e];if(t&&"closed"!==t.connectionState){const t=`video-${e.replace(/\s+/g,"-")}`,n=document.getElementById(t);n&&n.pause()}}))):(console.log("Page visible, resuming WebRTC streams"),Object.keys(S.current).forEach((e=>{const t=S.current[e];if(t&&"closed"!==t.connectionState){const t=`video-${e.replace(/\s+/g,"-")}`,n=document.getElementById(t);n&&n.play().catch((t=>{console.warn(`Could not resume video for ${e}:`,t)}))}})))};window.addEventListener("beforeunload",n),document.addEventListener("visibilitychange",o);const r=setInterval((()=>{Object.keys(S.current).forEach((t=>{const n=S.current[t];if(n&&(console.debug(`WebRTC connection state for ${t}: ${n.connectionState}, ICE state: ${n.iceConnectionState}`),"failed"===n.iceConnectionState||"disconnected"===n.iceConnectionState)){console.warn(`WebRTC connection for ${t} is in ${n.iceConnectionState} state, will attempt reconnect`),O(t);const o=e.find((e=>e.name===t));o&&(console.log(`Attempting to reconnect WebRTC for stream ${t}`),W(o))}}))}),3e4);return()=>{document.removeEventListener("keydown",t),window.removeEventListener("beforeunload",n),document.removeEventListener("visibilitychange",o),clearInterval(r),_()}}),[e]),r((()=>{x(!0);const e=setTimeout((()=>{console.warn("Stream loading timed out"),x(!1),d("Loading streams timed out. Please try refreshing the page.")}),15e3);R().then((n=>{clearTimeout(e),n&&n.length>0?(t(n),y(n[0].name)):console.warn("No streams returned from API"),x(!1)})).catch((t=>{clearTimeout(e),console.error("Error loading streams:",t),d("Error loading streams: "+t.message),x(!1)}))}),[]),r((()=>{j()}),[a,b,e,C]);const R=async()=>{try{const e=new Promise(((e,t)=>{setTimeout((()=>t(new Error("Request timed out"))),5e3)})),t=fetch("/api/streams"),n=await Promise.race([t,e]);if(!n.ok)throw new Error("Failed to load streams");const o=new Promise(((e,t)=>{setTimeout((()=>t(new Error("JSON parsing timed out"))),3e3)})),r=n.json(),s=(await Promise.race([r,o])||[]).map((e=>{const t=new Promise(((t,n)=>{setTimeout((()=>n(new Error(`Timeout fetching details for stream ${e.name}`))),3e3)})),n=fetch(`/api/streams/${encodeURIComponent(e.id||e.name)}`).then((t=>{if(!t.ok)throw new Error(`Failed to load details for stream ${e.name}`);return t.json()}));return Promise.race([n,t]).catch((t=>(console.error(`Error loading details for stream ${e.name}:`,t),e)))})),a=await Promise.all(s);return console.log("Loaded detailed streams for WebRTC view:",a),a||[]}catch(e){return console.error("Error loading streams for WebRTC view:",e),d("Error loading streams: "+e.message),[]}},I=()=>{switch(a){case"1":return 1;case"2":return 2;case"4":default:return 4;case"6":return 6;case"9":return 9;case"16":return 16}},j=()=>{if(!T.current)return;const t=T.current.querySelector(".placeholder");if(T.current.innerHTML="",t&&0===e.length)return void T.current.appendChild(t);let n=e;if("1"===a&&b)n=e.filter((e=>e.name===b));else{const t=I(),o=Math.ceil(e.length/t);if(C>=o)return void E(Math.max(0,o-1));const r=C*t,s=Math.min(r+t,e.length);n=e.slice(r,s)}const o=n.map((e=>e.name));Object.keys(S.current).forEach((e=>{o.includes(e)||(console.log(`Cleaning up WebRTC connection for stream ${e} (not visible in current view)`),O(e))})),n.forEach((e=>{const t=e.id||e.name,n=document.createElement("div");n.className="video-cell",n.innerHTML=`\n        <video id="video-${e.name.replace(/\s+/g,"-")}" autoplay muted></video>\n        <div class="stream-info">\n          <span>${e.name}</span>\n          <span>${e.width}x${e.height} · ${e.fps}fps</span>\n          <div class="stream-controls">\n            <button class="snapshot-btn" data-id="${t}" data-name="${e.name}">\n              <span>📷</span> Snapshot\n            </button>\n            <button class="fullscreen-btn" data-id="${t}" data-name="${e.name}">\n              <span>⛶</span> Fullscreen\n            </button>\n          </div>\n        </div>\n        <div class="loading-indicator">\n          <div class="loading-spinner"></div>\n          <span>Connecting WebRTC...</span>\n        </div>\n      `,T.current.appendChild(n),W(e);const o=n.querySelector(".snapshot-btn");o&&o.addEventListener("click",(()=>{q(t)}));const r=n.querySelector(".fullscreen-btn");r&&r.addEventListener("click",(()=>{D(e.name)}))}))},W=e=>{const t=`video-${e.name.replace(/\s+/g,"-")}`,n=document.getElementById(t),o=n?n.closest(".video-cell"):null;if(!n||!o)return;const r=o.querySelector(".loading-indicator");r&&(r.style.display="flex");const s=`canvas-${e.name.replace(/\s+/g,"-")}`;let a=document.getElementById(s);a||(a=document.createElement("canvas"),a.id=s,a.className="detection-overlay",a.style.position="absolute",a.style.top="0",a.style.left="0",a.style.width="100%",a.style.height="100%",a.style.pointerEvents="none",o.appendChild(a));const i=new RTCPeerConnection({iceServers:[{urls:"stun:stun.l.google.com:19302"}],iceTransportPolicy:"all",bundlePolicy:"balanced",rtcpMuxPolicy:"require",sdpSemantics:"unified-plan"});S.current[e.name]=i,i.ontrack=t=>{console.log(`Track received for stream ${e.name}:`,t),"video"===t.track.kind&&(n.srcObject=t.streams[0],n.onloadeddata=()=>{r&&(r.style.display="none")})},i.onicecandidate=t=>{t.candidate&&console.log(`ICE candidate for stream ${e.name}:`,t.candidate)},i.oniceconnectionstatechange=()=>{console.log(`ICE connection state for stream ${e.name}:`,i.iceConnectionState),"failed"!==i.iceConnectionState&&"disconnected"!==i.iceConnectionState||P(e.name,"WebRTC connection failed")},i.addTransceiver("video",{direction:"recvonly"}),i.addTransceiver("audio",{direction:"recvonly"});const c=setTimeout((()=>{console.warn(`WebRTC setup timed out for stream ${e.name}`),P(e.name,"WebRTC setup timed out"),S.current[e.name]&&O(e.name)}),15e3);i.createOffer({offerToReceiveAudio:!0,offerToReceiveVideo:!0}).then((t=>(console.log(`Created offer for stream ${e.name}:`,t),console.log(`Original SDP for stream ${e.name}:`,t.sdp),t.sdp.includes("a=ice-ufrag:")&&t.sdp.includes("a=ice-pwd:")||console.warn(`SDP for stream ${e.name} is missing ice-ufrag or ice-pwd!`),console.log(`Using original offer for stream ${e.name}`),i.setLocalDescription(t)))).then((()=>(console.log(`Set local description for stream ${e.name}`),L(e.name,i.localDescription)))).then((t=>(console.log(`Received answer for stream ${e.name}:`,t),i.setRemoteDescription(new RTCSessionDescription(t))))).then((()=>{console.log(`Set remote description for stream ${e.name}`),clearTimeout(c),console.log(`Stream ${e.name} detection settings:`,{detection_based_recording:e.detection_based_recording,detection_model:e.detection_model,detection_threshold:e.detection_threshold}),e.detection_based_recording&&e.detection_model?(console.log(`Starting detection polling for stream ${e.name}`),f(e.name,a,n,k.current)):console.log(`Detection not enabled for stream ${e.name}`)})).catch((t=>{clearTimeout(c),console.error(`Error setting up WebRTC for stream ${e.name}:`,t),P(e.name,t.message)}))},L=async(e,t)=>{try{const s=localStorage.getItem("auth"),a={type:t.type,sdp:t.sdp};console.log(`Sending formatted offer for stream ${e}:`,a);const i=new AbortController,c=i.signal,l=setTimeout((()=>{console.warn(`Aborting WebRTC offer request for stream ${e} due to timeout`),i.abort()}),8e3);try{const t=await fetch(`/api/webrtc?src=${encodeURIComponent(e)}`,{method:"POST",headers:{"Content-Type":"application/json",...s?{Authorization:"Basic "+s}:{}},body:JSON.stringify(a),signal:c});if(clearTimeout(l),!t.ok)throw new Error(`Failed to send offer: ${t.status} ${t.statusText}`);const r=new AbortController,i=(r.signal,setTimeout((()=>{console.warn(`Aborting JSON parsing for stream ${e} due to timeout`),r.abort()}),5e3));try{const o=await t.text();clearTimeout(i);try{return JSON.parse(o)}catch(n){throw console.error(`Error parsing JSON for stream ${e}:`,n),console.log(`Raw response text: ${o}`),new Error(`Failed to parse WebRTC answer: ${n.message}`)}}catch(o){if(clearTimeout(i),"AbortError"===o.name)throw new Error(`WebRTC answer parsing timed out for stream ${e}`);throw o}}catch(r){if(clearTimeout(l),"AbortError"===r.name)throw new Error(`WebRTC offer request timed out for stream ${e}`);throw r}}catch(s){throw console.error(`Error sending offer for stream ${e}:`,s),s}},P=(e,t)=>{const n=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(n),r=o?o.closest(".video-cell"):null;if(!r)return;const s=r.querySelector(".loading-indicator");s&&(s.style.display="none");let a=r.querySelector(".error-indicator");a||(a=document.createElement("div"),a.className="error-indicator",r.appendChild(a)),a.innerHTML=`\n      <div class="error-icon">!</div>\n      <p>${t||"WebRTC connection failed"}</p>\n      <button class="retry-button mt-4 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Retry</button>\n    `;const i=a.querySelector(".retry-button");i&&i.addEventListener("click",(()=>{s&&(s.style.display="flex"),a.style.display="none",O(e),fetch(`/api/streams/${encodeURIComponent(e)}`).then((e=>e.json())).then((e=>{W(e)})).catch((e=>{console.error("Error fetching stream info:",e),a.style.display="flex";const t=a.querySelector("p");t&&(t.textContent="Could not reconnect: "+e.message),s&&(s.style.display="none")}))}))},O=e=>{S.current[e]&&(S.current[e].close(),delete S.current[e]);const t=`video-${e.replace(/\s+/g,"-")}`,n=document.getElementById(t);n&&(n.srcObject=null),p(e,k.current)},_=()=>{Object.keys(S.current).forEach((e=>{O(e)}))},q=e=>{const t=document.querySelector(`.snapshot-btn[data-id="${e}"]`);if(!t)return void console.error("Stream element not found for ID:",e);const n=t.getAttribute("data-name");if(!n)return void console.error("Stream name not found for ID:",e);const o=`video-${n.replace(/\s+/g,"-")}`,r=document.getElementById(o);if(!r)return void console.error("Video element not found for stream:",n);const s=document.createElement("canvas");if(s.width=r.videoWidth,s.height=r.videoHeight,0===s.width||0===s.height)return console.error("Invalid video dimensions:",s.width,s.height),void d("Cannot take snapshot: Video not loaded or has invalid dimensions");s.getContext("2d").drawImage(r,0,0,s.width,s.height);try{window.__snapshotCanvas=s;const e=(new Date).toISOString().replace(/[:.]/g,"-"),t=`snapshot-${n.replace(/\s+/g,"-")}-${e}.jpg`;window.__snapshotFileName=t,u(s.toDataURL("image/jpeg",.95),`Snapshot: ${n}`),d("Snapshot taken successfully")}catch(a){console.error("Error creating snapshot:",a),d("Failed to create snapshot: "+a.message)}},D=e=>{const t=`video-${e.replace(/\s+/g,"-")}`,n=document.getElementById(t),o=n?n.closest(".video-cell"):null;o?document.fullscreenElement?document.exitFullscreen():o.requestFullscreen().catch((e=>{console.error(`Error attempting to enable fullscreen: ${e.message}`),d(`Could not enable fullscreen mode: ${e.message}`)})):console.error("Stream not found:",e)};return s`
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
              onClick=${()=>m(v,w)}
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
              onChange=${e=>y(e.target.value)}
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
          ref=${T}
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
  `}e({WebRTCView:h,loadWebRTCView:function(){const e=document.getElementById("main-content");e&&a((async()=>{const{render:e}=await t.import("./preact-app-legacy-DGNEAXcV.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(s`<${h} />`,e)}))}})}}}));
//# sourceMappingURL=WebRTCView-legacy-DyNSnK7y.js.map
