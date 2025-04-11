const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./preact-app-BzntiBeJ.js","../css/preact-app.css"])))=>i.map(i=>d[i]);
var O=Object.freeze,ee=Object.defineProperty;var b=(u,w)=>O(ee(u,"raw",{value:O(w||u.slice())}));import{d as $,A as j,y as I,s as te,a as oe,b as ne,c as v,e as re,h as C,_ as se}from"./preact-app-BzntiBeJ.js";import{s as le,c as ae,t as ie}from"./DetectionOverlay-4KTA9wlk.js";var B,F,V,q,A,z;function ce(){const[u,w]=$([]),[m,U]=$(()=>new URLSearchParams(window.location.search).get("layout")||"4"),[y,P]=$(()=>new URLSearchParams(window.location.search).get("stream")||""),[_,D]=$(!1),[J,T]=$(!0),[p,k]=$(()=>{const a=new URLSearchParams(window.location.search).get("page");return a?Math.max(0,parseInt(a,10)-1):0}),x=j(null),g=j({}),W=j({});I(()=>{te(),oe(),ne();const e=()=>{M()},a=()=>{document.hidden?(console.log("Page hidden, pausing WebRTC streams"),Object.keys(g.current).forEach(n=>{const o=g.current[n];if(o&&o.connectionState!=="closed"){const s="video-".concat(n.replace(/\s+/g,"-")),t=document.getElementById(s);t&&t.pause()}})):(console.log("Page visible, resuming WebRTC streams"),Object.keys(g.current).forEach(n=>{const o=g.current[n];if(o&&o.connectionState!=="closed"){const s="video-".concat(n.replace(/\s+/g,"-")),t=document.getElementById(s);t&&t.play().catch(l=>{console.warn("Could not resume video for ".concat(n,":"),l)})}}))};window.addEventListener("beforeunload",e),document.addEventListener("visibilitychange",a);const r=setInterval(()=>{Object.keys(g.current).forEach(n=>{const o=g.current[n];if(o&&(console.debug("WebRTC connection state for ".concat(n,": ").concat(o.connectionState,", ICE state: ").concat(o.iceConnectionState)),o.iceConnectionState==="failed"||o.iceConnectionState==="disconnected")){console.warn("WebRTC connection for ".concat(n," is in ").concat(o.iceConnectionState," state, will attempt reconnect")),S(n);const s=u.find(t=>t.name===n);s&&(console.log("Attempting to reconnect WebRTC for stream ".concat(n)),R(s))}})},3e4);return()=>{window.removeEventListener("beforeunload",e),document.removeEventListener("visibilitychange",a),clearInterval(r),M()}},[u]),I(()=>{T(!0);const e=setTimeout(()=>{console.warn("Stream loading timed out"),T(!1),v("Loading streams timed out. Please try refreshing the page.")},15e3);G().then(a=>{if(clearTimeout(e),a&&a.length>0){w(a);const n=new URLSearchParams(window.location.search).get("stream");n&&a.some(o=>o.name===n)?P(n):(!y||!a.some(o=>o.name===y))&&P(a[0].name)}else console.warn("No streams returned from API");T(!1)}).catch(a=>{clearTimeout(e),console.error("Error loading streams:",a),v("Error loading streams: "+a.message),T(!1)})},[]),I(()=>{K()},[m,y,u,p]),I(()=>{const e=new URL(window.location);p===0?e.searchParams.delete("page"):e.searchParams.set("page",p+1),m&&m!=="4"?e.searchParams.set("layout",m):m==="4"&&e.searchParams.delete("layout"),m==="1"&&y?e.searchParams.set("stream",y):e.searchParams.delete("stream"),window.history.replaceState({},"",e)},[p,m,y]);const G=async()=>{try{const e=new Promise((d,c)=>{setTimeout(()=>c(new Error("Request timed out")),5e3)}),a=fetch("/api/streams"),r=await Promise.race([a,e]);if(!r.ok)throw new Error("Failed to load streams");const n=new Promise((d,c)=>{setTimeout(()=>c(new Error("JSON parsing timed out")),3e3)}),o=r.json(),t=(await Promise.race([o,n])||[]).map(d=>{const c=new Promise((f,N)=>{setTimeout(()=>N(new Error("Timeout fetching details for stream ".concat(d.name))),3e3)}),h=fetch("/api/streams/".concat(encodeURIComponent(d.id||d.name))).then(f=>{if(!f.ok)throw new Error("Failed to load details for stream ".concat(d.name));return f.json()});return Promise.race([h,c]).catch(f=>(console.error("Error loading details for stream ".concat(d.name,":"),f),d))}),l=await Promise.all(t);console.log("Loaded detailed streams for WebRTC view:",l);const i=l.filter(d=>d.is_deleted?(console.log("Stream ".concat(d.name," is soft deleted, filtering out")),!1):d.enabled?d.streaming_enabled?!0:(console.log("Stream ".concat(d.name," is not configured for HLS, filtering out")),!1):(console.log("Stream ".concat(d.name," is inactive, filtering out")),!1));return console.log("Filtered streams for WebRTC view:",i),i||[]}catch(e){return console.error("Error loading streams for WebRTC view:",e),v("Error loading streams: "+e.message),[]}},E=()=>{switch(m){case"1":return 1;case"2":return 2;case"4":return 4;case"6":return 6;case"9":return 9;case"16":return 16;default:return 4}},K=()=>{if(!x.current)return;const e=x.current.querySelector(".placeholder");if(x.current.innerHTML="",e&&u.length===0){x.current.appendChild(e);return}let a=u;if(m==="1"&&y)a=u.filter(n=>n.name===y);else{const n=E(),o=Math.ceil(u.length/n);if(p>=o){k(Math.max(0,o-1));return}const s=p*n,t=Math.min(s+n,u.length);a=u.slice(s,t)}const r=a.map(n=>n.name);Object.keys(g.current).forEach(n=>{r.includes(n)||(console.log("Cleaning up WebRTC connection for stream ".concat(n," as it's not on the current page")),S(n))}),a.forEach((n,o)=>{Q(n),g.current[n.name]?console.log("WebRTC connection for stream ".concat(n.name," already exists, reusing")):setTimeout(()=>{R(n)},o*500)})},Q=e=>{const a=e.id||e.name,r=document.createElement("div");r.className="video-cell",r.dataset.streamName=e.name,r.style.position="relative";const n=document.createElement("video");n.id="video-".concat(e.name.replace(/\s+/g,"-")),n.className="video-element",n.playsInline=!0,n.autoplay=!0,n.muted=!0,n.style.pointerEvents="none";const o=document.createElement("div");o.className="loading-indicator",o.innerHTML='\n      <div class="spinner"></div>\n      <p>Connecting...</p>\n    ',o.style.position="absolute",o.style.top="0",o.style.left="0",o.style.width="100%",o.style.height="100%",o.style.display="flex",o.style.flexDirection="column",o.style.justifyContent="center",o.style.alignItems="center",o.style.backgroundColor="rgba(0, 0, 0, 0.7)",o.style.color="white",o.style.zIndex="20";const s=document.createElement("div");s.className="error-indicator",s.style.display="none",s.style.position="absolute",s.style.top="0",s.style.left="0",s.style.width="100%",s.style.height="100%",s.style.flexDirection="column",s.style.justifyContent="center",s.style.alignItems="center",s.style.backgroundColor="rgba(0, 0, 0, 0.7)",s.style.color="white",s.style.zIndex="20";const t=document.createElement("div");t.className="stream-name-overlay",t.textContent=e.name,t.style.position="absolute",t.style.top="10px",t.style.left="10px",t.style.padding="5px 10px",t.style.backgroundColor="rgba(0, 0, 0, 0.5)",t.style.color="white",t.style.borderRadius="4px",t.style.fontSize="14px",t.style.zIndex="15";const l=document.createElement("div");l.className="stream-controls",l.innerHTML='\n      <button class="snapshot-btn" title="Take Snapshot" data-id="'.concat(a,'" data-name="').concat(e.name,'">\n        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path><circle cx="12" cy="13" r="4"></circle></svg>\n      </button>\n      <button class="fullscreen-btn" title="Toggle Fullscreen" data-id="').concat(a,'" data-name="').concat(e.name,'">\n        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>\n      </button>\n    '),l.style.position="absolute",l.style.bottom="10px",l.style.right="10px",l.style.display="flex",l.style.gap="10px",l.style.zIndex="30";const i=document.createElement("canvas");i.id="canvas-".concat(e.name.replace(/\s+/g,"-")),i.className="detection-overlay",i.style.position="absolute",i.style.top="0",i.style.left="0",i.style.width="100%",i.style.height="100%",i.style.pointerEvents="none",i.style.zIndex="5",r.appendChild(n),r.appendChild(o),r.appendChild(s),r.appendChild(t),r.appendChild(l),r.appendChild(i),x.current.appendChild(r),r.querySelectorAll("button").forEach(f=>{f.style.position="relative",f.style.zIndex="30",f.style.pointerEvents="auto"});const c=r.querySelector(".snapshot-btn");c&&c.addEventListener("click",f=>{Y(a)});const h=r.querySelector(".fullscreen-btn");h&&h.addEventListener("click",()=>{Z(e.name)})},R=e=>{const a="video-".concat(e.name.replace(/\s+/g,"-")),r=document.getElementById(a),n=r?r.closest(".video-cell"):null;if(!r||!n)return;const o=n.querySelector(".loading-indicator");o&&(o.style.display="flex");const s="canvas-".concat(e.name.replace(/\s+/g,"-"));let t=document.getElementById(s);t||(t=document.createElement("canvas"),t.id=s,t.className="detection-overlay",t.style.position="absolute",t.style.top="0",t.style.left="0",t.style.width="100%",t.style.height="100%",t.style.pointerEvents="none",n.appendChild(t));const l=new RTCPeerConnection({iceServers:[{urls:"stun:stun.l.google.com:19302"}],iceTransportPolicy:"all",bundlePolicy:"balanced",rtcpMuxPolicy:"require",sdpSemantics:"unified-plan"});g.current[e.name]=l,l.ontrack=c=>{console.log("Track received for stream ".concat(e.name,":"),c),c.track.kind==="video"&&(r.srcObject=c.streams[0],r.onloadeddata=()=>{o&&(o.style.display="none")})},l.onicecandidate=c=>{c.candidate&&console.log("ICE candidate for stream ".concat(e.name,":"),c.candidate)},l.oniceconnectionstatechange=()=>{console.log("ICE connection state for stream ".concat(e.name,":"),l.iceConnectionState),(l.iceConnectionState==="failed"||l.iceConnectionState==="disconnected")&&L(e.name,"WebRTC connection failed")},l.addTransceiver("video",{direction:"recvonly"}),l.addTransceiver("audio",{direction:"recvonly"});const i={offerToReceiveAudio:!0,offerToReceiveVideo:!0},d=setTimeout(()=>{console.warn("WebRTC setup timed out for stream ".concat(e.name)),L(e.name,"WebRTC setup timed out"),g.current[e.name]&&S(e.name)},15e3);l.createOffer(i).then(c=>(console.log("Created offer for stream ".concat(e.name,":"),c),console.log("Original SDP for stream ".concat(e.name,":"),c.sdp),(!c.sdp.includes("a=ice-ufrag:")||!c.sdp.includes("a=ice-pwd:"))&&console.warn("SDP for stream ".concat(e.name," is missing ice-ufrag or ice-pwd!")),console.log("Using original offer for stream ".concat(e.name)),l.setLocalDescription(c))).then(()=>(console.log("Set local description for stream ".concat(e.name)),X(e.name,l.localDescription))).then(c=>(console.log("Received answer for stream ".concat(e.name,":"),c),l.setRemoteDescription(new RTCSessionDescription(c)))).then(()=>{console.log("Set remote description for stream ".concat(e.name)),clearTimeout(d),console.log("Stream ".concat(e.name," detection settings:"),{detection_based_recording:e.detection_based_recording,detection_model:e.detection_model,detection_threshold:e.detection_threshold}),e.detection_based_recording&&e.detection_model?(console.log("Starting detection polling for stream ".concat(e.name)),le(e.name,t,r,W.current)):console.log("Detection not enabled for stream ".concat(e.name))}).catch(c=>{clearTimeout(d),console.error("Error setting up WebRTC for stream ".concat(e.name,":"),c),L(e.name,c.message)})},X=async(e,a)=>{try{const r=localStorage.getItem("auth"),n={type:a.type,sdp:a.sdp};console.log("Sending formatted offer for stream ".concat(e,":"),n);const o=new AbortController,s=o.signal,t=setTimeout(()=>{console.warn("Aborting WebRTC offer request for stream ".concat(e," due to timeout")),o.abort()},8e3);try{const l=await fetch("/api/webrtc?src=".concat(encodeURIComponent(e)),{method:"POST",headers:{"Content-Type":"application/json",...r?{Authorization:"Basic "+r}:{}},body:JSON.stringify(n),signal:s});if(clearTimeout(t),!l.ok)throw new Error("Failed to send offer: ".concat(l.status," ").concat(l.statusText));const i=new AbortController,d=i.signal,c=setTimeout(()=>{console.warn("Aborting JSON parsing for stream ".concat(e," due to timeout")),i.abort()},5e3);try{const h=await l.text();clearTimeout(c);try{return JSON.parse(h)}catch(f){throw console.error("Error parsing JSON for stream ".concat(e,":"),f),console.log("Raw response text: ".concat(h)),new Error("Failed to parse WebRTC answer: ".concat(f.message))}}catch(h){throw clearTimeout(c),h.name==="AbortError"?new Error("WebRTC answer parsing timed out for stream ".concat(e)):h}}catch(l){throw clearTimeout(t),l.name==="AbortError"?new Error("WebRTC offer request timed out for stream ".concat(e)):l}}catch(r){throw console.error("Error sending offer for stream ".concat(e,":"),r),r}},L=(e,a)=>{console.error("WebRTC error for stream ".concat(e,":"),a);const r="video-".concat(e.replace(/\s+/g,"-")),n=document.getElementById(r);if(!n)return;const o=n.closest(".video-cell");if(!o)return;const s=o.querySelector(".loading-indicator");s&&(s.style.display="none");let t=o.querySelector(".error-indicator");t||(t=document.createElement("div"),t.className="error-indicator",t.style.position="absolute",t.style.top="0",t.style.left="0",t.style.width="100%",t.style.height="100%",t.style.display="flex",t.style.flexDirection="column",t.style.justifyContent="center",t.style.alignItems="center",t.style.backgroundColor="rgba(0, 0, 0, 0.7)",t.style.color="white",t.style.zIndex="20",o.appendChild(t)),t.innerHTML='\n      <div class="error-icon">!</div>\n      <p>'.concat(a||"WebRTC connection failed",'</p>\n      <button class="retry-button mt-4 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Retry</button>\n    '),t.style.display="flex";const l=t.querySelector(".retry-button");l&&(l.style.position="relative",l.style.zIndex="30",l.style.pointerEvents="auto",l.addEventListener("click",()=>{s&&(s.style.display="flex"),t.style.display="none",S(e),fetch("/api/streams/".concat(encodeURIComponent(e))).then(i=>i.json()).then(i=>{R(i)}).catch(i=>{console.error("Error fetching stream info:",i),t.style.display="flex";const d=t.querySelector("p");d&&(d.textContent="Could not reconnect: "+i.message),s&&(s.style.display="none")})}))},S=e=>{g.current[e]&&(g.current[e].close(),delete g.current[e]);const a="video-".concat(e.replace(/\s+/g,"-")),r=document.getElementById(a);r&&(r.srcObject=null),ae(e,W.current)},M=()=>{Object.keys(g.current).forEach(e=>{S(e)})},Y=e=>{const a=document.querySelector('.snapshot-btn[data-id="'.concat(e,'"]'));let r;if(a)r=a.getAttribute("data-name");else{const i=(event.currentTarget||event.target).closest(".video-cell");i&&(r=i.dataset.streamName)}if(!r){console.error("Stream name not found for snapshot"),v("Cannot take snapshot: Stream not identified");return}const n="video-".concat(r.replace(/\s+/g,"-")),o=document.getElementById(n);if(!o){console.error("Video element not found for stream:",r),v("Cannot take snapshot: Video element not found");return}const s=document.createElement("canvas");if(s.width=o.videoWidth,s.height=o.videoHeight,s.width===0||s.height===0){console.error("Invalid video dimensions:",s.width,s.height),v("Cannot take snapshot: Video not loaded or has invalid dimensions");return}s.getContext("2d").drawImage(o,0,0,s.width,s.height);try{window.__snapshotCanvas=s;const l=new Date().toISOString().replace(/[:.]/g,"-"),i="snapshot-".concat(r.replace(/\s+/g,"-"),"-").concat(l,".jpg");window.__snapshotFileName=i,re(s.toDataURL("image/jpeg",.95),"Snapshot: ".concat(r)),v("Snapshot taken successfully")}catch(l){console.error("Error creating snapshot:",l),v("Failed to create snapshot: "+l.message)}},Z=e=>{const a="video-".concat(e.replace(/\s+/g,"-")),r=document.getElementById(a),n=r?r.closest(".video-cell"):null;if(!n){console.error("Stream not found:",e);return}document.fullscreenElement?document.exitFullscreen():n.requestFullscreen().catch(o=>{console.error("Error attempting to enable fullscreen: ".concat(o.message)),v("Could not enable fullscreen mode: ".concat(o.message))})};return C(z||(z=b(['\n    <section id="live-page" class="page ','">\n      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">\n        <div class="flex items-center space-x-2">\n          <h2 class="text-xl font-bold mr-4">Live View</h2>\n          <div class="flex space-x-2">\n            <button\n              id="hls-toggle-btn"\n              class="px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"\n              onClick=','\n            >\n              HLS View\n            </button>\n          </div>\n        </div>\n        <div class="controls flex items-center space-x-2">\n          <div class="flex items-center">\n            <label for="layout-selector" class="mr-2">Layout:</label>\n            <select\n              id="layout-selector"\n              class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"\n              value=',"\n              onChange=",'\n            >\n              <option value="1">1 Stream</option>\n              <option value="2">2 Streams</option>\n              <option value="4" selected>4 Streams</option>\n              <option value="6">6 Streams</option>\n              <option value="9">9 Streams</option>\n              <option value="16">16 Streams</option>\n            </select>\n          </div>\n\n          ','\n\n          <!-- Fullscreen button -->\n          <button\n            id="fullscreen-btn"\n            class="p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"\n            onClick=','\n            title="Toggle Fullscreen"\n          >\n            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">\n              <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>\n            </svg>\n          </button>\n        </div>\n      </div>\n\n      <div class="flex flex-col space-y-4">\n        <div\n          id="video-grid"\n          class=',"\n          ref=","\n        >\n          ","\n          <!-- Video cells will be dynamically added by the updateVideoGrid function -->\n        </div>\n\n        ","\n      </div>\n    </section>\n  "])),_?"fullscreen-mode":"",()=>window.location.href="/hls.html",m,e=>{const a=e.target.value;U(a),k(0)},m==="1"&&C(F||(F=b(['\n            <div class="flex items-center">\n              <label for="stream-selector" class="mr-2">Stream:</label>\n              <select\n                id="stream-selector"\n                class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"\n                value=',"\n                onChange=","\n              >\n                ","\n              </select>\n            </div>\n          "])),y,e=>{const a=e.target.value;P(a)},u.map(e=>C(B||(B=b(["\n                  <option key="," value=",">","</option>\n                "])),e.name,e.name,e.name))),()=>ie(_,D),"video-container layout-".concat(m),x,J?C(V||(V=b(['\n            <div class="flex justify-center items-center col-span-full row-span-full h-64 w-full">\n              <div class="flex flex-col items-center justify-center py-8">\n                <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>\n                <p class="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>\n              </div>\n            </div>\n          ']))):u.length===0?C(q||(q=b(['\n            <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">\n              <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>\n              <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>\n            </div>\n          ']))):null,m!=="1"&&u.length>E()?C(A||(A=b(['\n          <div class="pagination-controls flex justify-center items-center space-x-4 mt-4">\n            <button\n              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"\n              onClick=',"\n              disabled=",'\n            >\n              Previous\n            </button>\n            <span class="text-gray-700 dark:text-gray-300">\n              Page '," of ",'\n            </span>\n            <button\n              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"\n              onClick=',"\n              disabled=","\n            >\n              Next\n            </button>\n          </div>\n        "])),()=>k(Math.max(0,p-1)),p===0,p+1,Math.ceil(u.length/E()),()=>k(Math.min(Math.ceil(u.length/E())-1,p+1)),p>=Math.ceil(u.length/E())-1):null)}var H;function ge(){const u=document.getElementById("main-content");u&&se(async()=>{const{render:w}=await import("./preact-app-BzntiBeJ.js").then(m=>m.p);return{render:w}},__vite__mapDeps([0,1]),import.meta.url).then(({render:w})=>{u.innerHTML="",w(C(H||(H=b(["<"," />"])),ce),u)})}export{ce as WebRTCView,ge as loadWebRTCView};
//# sourceMappingURL=WebRTCView-Cz-hcMh9.js.map
