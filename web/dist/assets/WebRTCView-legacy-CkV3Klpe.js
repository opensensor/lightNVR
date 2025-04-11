System.register(["./preact-app-legacy-Bb2RSFTE.js","./DetectionOverlay-legacy-DdI-O9YA.js"],(function(e,t){"use strict";var o,n,r,s,a,l,i,c,d,m,u,g,p;return{setters:[e=>{o=e.d,n=e.A,r=e.y,s=e.s,a=e.a,l=e.b,i=e.c,c=e.e,d=e.h,m=e._},e=>{u=e.s,g=e.c,p=e.t}],execute:function(){function f(){const[e,t]=o([]),[m,f]=o((()=>new URLSearchParams(window.location.search).get("layout")||"4")),[h,y]=o((()=>new URLSearchParams(window.location.search).get("stream")||"")),[b,v]=o(!1),[w,$]=o(!0),[x,C]=o((()=>{const e=new URLSearchParams(window.location.search).get("page");return e?Math.max(0,parseInt(e,10)-1):0})),S=n(null),E=n({}),T=n({});r((()=>{s(),a(),l();const t=()=>{N()},o=()=>{document.hidden?(console.log("Page hidden, pausing WebRTC streams"),Object.keys(E.current).forEach((e=>{const t=E.current[e];if(t&&"closed"!==t.connectionState){const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t);o&&o.pause()}}))):(console.log("Page visible, resuming WebRTC streams"),Object.keys(E.current).forEach((e=>{const t=E.current[e];if(t&&"closed"!==t.connectionState){const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t);o&&o.play().catch((t=>{console.warn(`Could not resume video for ${e}:`,t)}))}})))};window.addEventListener("beforeunload",t),document.addEventListener("visibilitychange",o);const n=setInterval((()=>{Object.keys(E.current).forEach((t=>{const o=E.current[t];if(o&&(console.debug(`WebRTC connection state for ${t}: ${o.connectionState}, ICE state: ${o.iceConnectionState}`),"failed"===o.iceConnectionState||"disconnected"===o.iceConnectionState)){console.warn(`WebRTC connection for ${t} is in ${o.iceConnectionState} state, will attempt reconnect`),M(t);const n=e.find((e=>e.name===t));n&&(console.log(`Attempting to reconnect WebRTC for stream ${t}`),j(n))}}))}),3e4);return()=>{window.removeEventListener("beforeunload",t),document.removeEventListener("visibilitychange",o),clearInterval(n),N()}}),[e]),r((()=>{$(!0);const e=setTimeout((()=>{console.warn("Stream loading timed out"),$(!1),i("Loading streams timed out. Please try refreshing the page.")}),15e3);k().then((o=>{if(clearTimeout(e),o&&o.length>0){t(o);const e=new URLSearchParams(window.location.search).get("stream");e&&o.some((t=>t.name===e))?y(e):h&&o.some((e=>e.name===h))||y(o[0].name)}else console.warn("No streams returned from API");$(!1)})).catch((t=>{clearTimeout(e),console.error("Error loading streams:",t),i("Error loading streams: "+t.message),$(!1)}))}),[]),r((()=>{I()}),[m,h,e,x]),r((()=>{const e=new URL(window.location);0===x?e.searchParams.delete("page"):e.searchParams.set("page",x+1),m&&"4"!==m?e.searchParams.set("layout",m):"4"===m&&e.searchParams.delete("layout"),"1"===m&&h?e.searchParams.set("stream",h):e.searchParams.delete("stream"),window.history.replaceState({},"",e)}),[x,m,h]);const k=async()=>{try{const e=new Promise(((e,t)=>{setTimeout((()=>t(new Error("Request timed out"))),5e3)})),t=fetch("/api/streams"),o=await Promise.race([t,e]);if(!o.ok)throw new Error("Failed to load streams");const n=new Promise(((e,t)=>{setTimeout((()=>t(new Error("JSON parsing timed out"))),3e3)})),r=o.json(),s=(await Promise.race([r,n])||[]).map((e=>{const t=new Promise(((t,o)=>{setTimeout((()=>o(new Error(`Timeout fetching details for stream ${e.name}`))),3e3)})),o=fetch(`/api/streams/${encodeURIComponent(e.id||e.name)}`).then((t=>{if(!t.ok)throw new Error(`Failed to load details for stream ${e.name}`);return t.json()}));return Promise.race([o,t]).catch((t=>(console.error(`Error loading details for stream ${e.name}:`,t),e)))})),a=await Promise.all(s);console.log("Loaded detailed streams for WebRTC view:",a);const l=a.filter((e=>e.is_deleted?(console.log(`Stream ${e.name} is soft deleted, filtering out`),!1):e.enabled?!!e.streaming_enabled||(console.log(`Stream ${e.name} is not configured for HLS, filtering out`),!1):(console.log(`Stream ${e.name} is inactive, filtering out`),!1)));return console.log("Filtered streams for WebRTC view:",l),l||[]}catch(e){return console.error("Error loading streams for WebRTC view:",e),i("Error loading streams: "+e.message),[]}},R=()=>{switch(m){case"1":return 1;case"2":return 2;case"4":default:return 4;case"6":return 6;case"9":return 9;case"16":return 16}},I=()=>{if(!S.current)return;const t=S.current.querySelector(".placeholder");if(S.current.innerHTML="",t&&0===e.length)return void S.current.appendChild(t);let o=e;if("1"===m&&h)o=e.filter((e=>e.name===h));else{const t=R(),n=Math.ceil(e.length/t);if(x>=n)return void C(Math.max(0,n-1));const r=x*t,s=Math.min(r+t,e.length);o=e.slice(r,s)}const n=o.map((e=>e.name));Object.keys(E.current).forEach((e=>{n.includes(e)||(console.log(`Cleaning up WebRTC connection for stream ${e} as it's not on the current page`),M(e))})),o.forEach(((e,t)=>{P(e),E.current[e.name]?console.log(`WebRTC connection for stream ${e.name} already exists, reusing`):setTimeout((()=>{j(e)}),500*t)}))},P=e=>{const t=e.id||e.name,o=document.createElement("div");o.className="video-cell",o.dataset.streamName=e.name,o.style.position="relative";const n=document.createElement("video");n.id=`video-${e.name.replace(/\s+/g,"-")}`,n.className="video-element",n.playsInline=!0,n.autoplay=!0,n.muted=!0,n.style.pointerEvents="none";const r=document.createElement("div");r.className="loading-indicator",r.innerHTML='\n      <div class="spinner"></div>\n      <p>Connecting...</p>\n    ',r.style.position="absolute",r.style.top="0",r.style.left="0",r.style.width="100%",r.style.height="100%",r.style.display="flex",r.style.flexDirection="column",r.style.justifyContent="center",r.style.alignItems="center",r.style.backgroundColor="rgba(0, 0, 0, 0.7)",r.style.color="white",r.style.zIndex="20";const s=document.createElement("div");s.className="error-indicator",s.style.display="none",s.style.position="absolute",s.style.top="0",s.style.left="0",s.style.width="100%",s.style.height="100%",s.style.flexDirection="column",s.style.justifyContent="center",s.style.alignItems="center",s.style.backgroundColor="rgba(0, 0, 0, 0.7)",s.style.color="white",s.style.zIndex="20";const a=document.createElement("div");a.className="stream-name-overlay",a.textContent=e.name,a.style.position="absolute",a.style.top="10px",a.style.left="10px",a.style.padding="5px 10px",a.style.backgroundColor="rgba(0, 0, 0, 0.5)",a.style.color="white",a.style.borderRadius="4px",a.style.fontSize="14px",a.style.zIndex="15";const l=document.createElement("div");l.className="stream-controls",l.innerHTML=`\n      <button class="snapshot-btn" title="Take Snapshot" data-id="${t}" data-name="${e.name}">\n        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path><circle cx="12" cy="13" r="4"></circle></svg>\n      </button>\n      <button class="fullscreen-btn" title="Toggle Fullscreen" data-id="${t}" data-name="${e.name}">\n        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>\n      </button>\n    `,l.style.position="absolute",l.style.bottom="10px",l.style.right="10px",l.style.display="flex",l.style.gap="10px",l.style.zIndex="30";const i=document.createElement("canvas");i.id=`canvas-${e.name.replace(/\s+/g,"-")}`,i.className="detection-overlay",i.style.position="absolute",i.style.top="0",i.style.left="0",i.style.width="100%",i.style.height="100%",i.style.pointerEvents="none",i.style.zIndex="5",o.appendChild(n),o.appendChild(r),o.appendChild(s),o.appendChild(a),o.appendChild(l),o.appendChild(i),S.current.appendChild(o),o.querySelectorAll("button").forEach((e=>{e.style.position="relative",e.style.zIndex="30",e.style.pointerEvents="auto"}));const c=o.querySelector(".snapshot-btn");c&&c.addEventListener("click",(e=>{_(t)}));const d=o.querySelector(".fullscreen-btn");d&&d.addEventListener("click",(()=>{O(e.name)}))},j=e=>{const t=`video-${e.name.replace(/\s+/g,"-")}`,o=document.getElementById(t),n=o?o.closest(".video-cell"):null;if(!o||!n)return;const r=n.querySelector(".loading-indicator");r&&(r.style.display="flex");const s=`canvas-${e.name.replace(/\s+/g,"-")}`;let a=document.getElementById(s);a||(a=document.createElement("canvas"),a.id=s,a.className="detection-overlay",a.style.position="absolute",a.style.top="0",a.style.left="0",a.style.width="100%",a.style.height="100%",a.style.pointerEvents="none",n.appendChild(a));const l=new RTCPeerConnection({iceServers:[{urls:"stun:stun.l.google.com:19302"}],iceTransportPolicy:"all",bundlePolicy:"balanced",rtcpMuxPolicy:"require",sdpSemantics:"unified-plan"});E.current[e.name]=l,l.ontrack=t=>{console.log(`Track received for stream ${e.name}:`,t),"video"===t.track.kind&&(o.srcObject=t.streams[0],o.onloadeddata=()=>{r&&(r.style.display="none")})},l.onicecandidate=t=>{t.candidate&&console.log(`ICE candidate for stream ${e.name}:`,t.candidate)},l.oniceconnectionstatechange=()=>{console.log(`ICE connection state for stream ${e.name}:`,l.iceConnectionState),"failed"!==l.iceConnectionState&&"disconnected"!==l.iceConnectionState||W(e.name,"WebRTC connection failed")},l.addTransceiver("video",{direction:"recvonly"}),l.addTransceiver("audio",{direction:"recvonly"});const i=setTimeout((()=>{console.warn(`WebRTC setup timed out for stream ${e.name}`),W(e.name,"WebRTC setup timed out"),E.current[e.name]&&M(e.name)}),15e3);l.createOffer({offerToReceiveAudio:!0,offerToReceiveVideo:!0}).then((t=>(console.log(`Created offer for stream ${e.name}:`,t),console.log(`Original SDP for stream ${e.name}:`,t.sdp),t.sdp.includes("a=ice-ufrag:")&&t.sdp.includes("a=ice-pwd:")||console.warn(`SDP for stream ${e.name} is missing ice-ufrag or ice-pwd!`),console.log(`Using original offer for stream ${e.name}`),l.setLocalDescription(t)))).then((()=>(console.log(`Set local description for stream ${e.name}`),L(e.name,l.localDescription)))).then((t=>(console.log(`Received answer for stream ${e.name}:`,t),l.setRemoteDescription(new RTCSessionDescription(t))))).then((()=>{console.log(`Set remote description for stream ${e.name}`),clearTimeout(i),console.log(`Stream ${e.name} detection settings:`,{detection_based_recording:e.detection_based_recording,detection_model:e.detection_model,detection_threshold:e.detection_threshold}),e.detection_based_recording&&e.detection_model?(console.log(`Starting detection polling for stream ${e.name}`),u(e.name,a,o,T.current)):console.log(`Detection not enabled for stream ${e.name}`)})).catch((t=>{clearTimeout(i),console.error(`Error setting up WebRTC for stream ${e.name}:`,t),W(e.name,t.message)}))},L=async(e,t)=>{try{const s=localStorage.getItem("auth"),a={type:t.type,sdp:t.sdp};console.log(`Sending formatted offer for stream ${e}:`,a);const l=new AbortController,i=l.signal,c=setTimeout((()=>{console.warn(`Aborting WebRTC offer request for stream ${e} due to timeout`),l.abort()}),8e3);try{const t=await fetch(`/api/webrtc?src=${encodeURIComponent(e)}`,{method:"POST",headers:{"Content-Type":"application/json",...s?{Authorization:"Basic "+s}:{}},body:JSON.stringify(a),signal:i});if(clearTimeout(c),!t.ok)throw new Error(`Failed to send offer: ${t.status} ${t.statusText}`);const r=new AbortController,l=(r.signal,setTimeout((()=>{console.warn(`Aborting JSON parsing for stream ${e} due to timeout`),r.abort()}),5e3));try{const n=await t.text();clearTimeout(l);try{return JSON.parse(n)}catch(o){throw console.error(`Error parsing JSON for stream ${e}:`,o),console.log(`Raw response text: ${n}`),new Error(`Failed to parse WebRTC answer: ${o.message}`)}}catch(n){if(clearTimeout(l),"AbortError"===n.name)throw new Error(`WebRTC answer parsing timed out for stream ${e}`);throw n}}catch(r){if(clearTimeout(c),"AbortError"===r.name)throw new Error(`WebRTC offer request timed out for stream ${e}`);throw r}}catch(s){throw console.error(`Error sending offer for stream ${e}:`,s),s}},W=(e,t)=>{console.error(`WebRTC error for stream ${e}:`,t);const o=`video-${e.replace(/\s+/g,"-")}`,n=document.getElementById(o);if(!n)return;const r=n.closest(".video-cell");if(!r)return;const s=r.querySelector(".loading-indicator");s&&(s.style.display="none");let a=r.querySelector(".error-indicator");a||(a=document.createElement("div"),a.className="error-indicator",a.style.position="absolute",a.style.top="0",a.style.left="0",a.style.width="100%",a.style.height="100%",a.style.display="flex",a.style.flexDirection="column",a.style.justifyContent="center",a.style.alignItems="center",a.style.backgroundColor="rgba(0, 0, 0, 0.7)",a.style.color="white",a.style.zIndex="20",r.appendChild(a)),a.innerHTML=`\n      <div class="error-icon">!</div>\n      <p>${t||"WebRTC connection failed"}</p>\n      <button class="retry-button mt-4 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Retry</button>\n    `,a.style.display="flex";const l=a.querySelector(".retry-button");l&&(l.style.position="relative",l.style.zIndex="30",l.style.pointerEvents="auto",l.addEventListener("click",(()=>{s&&(s.style.display="flex"),a.style.display="none",M(e),fetch(`/api/streams/${encodeURIComponent(e)}`).then((e=>e.json())).then((e=>{j(e)})).catch((e=>{console.error("Error fetching stream info:",e),a.style.display="flex";const t=a.querySelector("p");t&&(t.textContent="Could not reconnect: "+e.message),s&&(s.style.display="none")}))})))},M=e=>{E.current[e]&&(E.current[e].close(),delete E.current[e]);const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t);o&&(o.srcObject=null),g(e,T.current)},N=()=>{Object.keys(E.current).forEach((e=>{M(e)}))},_=e=>{const t=document.querySelector(`.snapshot-btn[data-id="${e}"]`);let o;if(t)o=t.getAttribute("data-name");else{const e=(event.currentTarget||event.target).closest(".video-cell");e&&(o=e.dataset.streamName)}if(!o)return console.error("Stream name not found for snapshot"),void i("Cannot take snapshot: Stream not identified");const n=`video-${o.replace(/\s+/g,"-")}`,r=document.getElementById(n);if(!r)return console.error("Video element not found for stream:",o),void i("Cannot take snapshot: Video element not found");const s=document.createElement("canvas");if(s.width=r.videoWidth,s.height=r.videoHeight,0===s.width||0===s.height)return console.error("Invalid video dimensions:",s.width,s.height),void i("Cannot take snapshot: Video not loaded or has invalid dimensions");s.getContext("2d").drawImage(r,0,0,s.width,s.height);try{window.__snapshotCanvas=s;const e=(new Date).toISOString().replace(/[:.]/g,"-"),t=`snapshot-${o.replace(/\s+/g,"-")}-${e}.jpg`;window.__snapshotFileName=t,c(s.toDataURL("image/jpeg",.95),`Snapshot: ${o}`),i("Snapshot taken successfully")}catch(a){console.error("Error creating snapshot:",a),i("Failed to create snapshot: "+a.message)}},O=e=>{const t=`video-${e.replace(/\s+/g,"-")}`,o=document.getElementById(t),n=o?o.closest(".video-cell"):null;n?document.fullscreenElement?document.exitFullscreen():n.requestFullscreen().catch((e=>{console.error(`Error attempting to enable fullscreen: ${e.message}`),i(`Could not enable fullscreen mode: ${e.message}`)})):console.error("Stream not found:",e)};return d`
    <section id="live-page" class="page ${b?"fullscreen-mode":""}">
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
          </div>
        </div>
        <div class="controls flex items-center space-x-2">
          <div class="flex items-center">
            <label for="layout-selector" class="mr-2">Layout:</label>
            <select
              id="layout-selector"
              class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
              value=${m}
              onChange=${e=>{const t=e.target.value;f(t),C(0)}}
            >
              <option value="1">1 Stream</option>
              <option value="2">2 Streams</option>
              <option value="4" selected>4 Streams</option>
              <option value="6">6 Streams</option>
              <option value="9">9 Streams</option>
              <option value="16">16 Streams</option>
            </select>
          </div>

          ${"1"===m&&d`
            <div class="flex items-center">
              <label for="stream-selector" class="mr-2">Stream:</label>
              <select
                id="stream-selector"
                class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
                value=${h}
                onChange=${e=>{const t=e.target.value;y(t)}}
              >
                ${e.map((e=>d`
                  <option key=${e.name} value=${e.name}>${e.name}</option>
                `))}
              </select>
            </div>
          `}

          <!-- Fullscreen button -->
          <button
            id="fullscreen-btn"
            class="p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
            onClick=${()=>p(b,v)}
            title="Toggle Fullscreen"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
            </svg>
          </button>
        </div>
      </div>

      <div class="flex flex-col space-y-4">
        <div
          id="video-grid"
          class=${`video-container layout-${m}`}
          ref=${S}
        >
          ${w?d`
            <div class="flex justify-center items-center col-span-full row-span-full h-64 w-full">
              <div class="flex flex-col items-center justify-center py-8">
                <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>
                <p class="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>
              </div>
            </div>
          `:0===e.length?d`
            <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
              <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
              <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
            </div>
          `:null}
          <!-- Video cells will be dynamically added by the updateVideoGrid function -->
        </div>

        ${"1"!==m&&e.length>R()?d`
          <div class="pagination-controls flex justify-center items-center space-x-4 mt-4">
            <button
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick=${()=>C(Math.max(0,x-1))}
              disabled=${0===x}
            >
              Previous
            </button>
            <span class="text-gray-700 dark:text-gray-300">
              Page ${x+1} of ${Math.ceil(e.length/R())}
            </span>
            <button
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick=${()=>C(Math.min(Math.ceil(e.length/R())-1,x+1))}
              disabled=${x>=Math.ceil(e.length/R())-1}
            >
              Next
            </button>
          </div>
        `:null}
      </div>
    </section>
  `}e({WebRTCView:f,loadWebRTCView:function(){const e=document.getElementById("main-content");e&&m((async()=>{const{render:e}=await t.import("./preact-app-legacy-Bb2RSFTE.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{e.innerHTML="",t(d`<${f} />`,e)}))}})}}}));
//# sourceMappingURL=WebRTCView-legacy-CkV3Klpe.js.map
