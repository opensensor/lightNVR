import{d as x,A as N,y as C,u as e,a as j,b as O,s as L,q as M,T as D,E as z,c as V,e as A,Q as U,k as H}from"./query-client-BSpadhku.js";/* empty css               */import{D as B,t as q,u as Q,F as J,L as K}from"./LiveView-CCi0A_l2.js";import{S as G,s as $,u as X,a as Y,b as Z,c as ee}from"./UI-BTDpQQFR.js";import{L as te}from"./LoadingIndicator-DEzjv51C.js";import{H as re,F as oe}from"./Footer-CMMNA2Vx.js";function ie(){import.meta.url,import("_").catch(()=>1),async function*(){}().next()}function ne({stream:a,streamId:b,onToggleFullscreen:k}){const[S,s]=x(!0),[h,w]=x(null),[I,c]=x(!1),u=N(null),g=N(null),p=N(null),i=N(null),y=N(null);C(()=>{if(!a||!a.name||!u.current)return;console.log("Initializing WebRTC connection for stream ".concat(a.name)),s(!0),w(null);const o=new RTCPeerConnection({iceServers:[{urls:"stun:stun.l.google.com:19302"},{urls:"stun:stun1.l.google.com:19302"}]});return p.current=o,o.ontrack=n=>{if(console.log("Track received for stream ".concat(a.name)),n.track.kind==="video"){const l=u.current;if(!l)return;l.srcObject=n.streams[0],l.onloadeddata=()=>{console.log("Video data loaded for stream ".concat(a.name))},l.onplaying=()=>{console.log("Video playing for stream ".concat(a.name)),s(!1),c(!0)},l.onerror=R=>{console.error("Video error for stream ".concat(a.name,":"),R),w("Video playback error"),s(!1)}}},o.onicecandidate=n=>{n.candidate&&(n.candidate.candidate!==""?console.log("ICE candidate for stream ".concat(a.name)):console.log("Ignoring empty ICE candidate for stream ".concat(a.name)))},o.oniceconnectionstatechange=()=>{console.log("ICE connection state for stream ".concat(a.name,": ").concat(o.iceConnectionState)),o.iceConnectionState==="failed"&&(w("WebRTC ICE connection failed"),s(!1))},o.addTransceiver("video",{direction:"recvonly"}),o.addTransceiver("audio",{direction:"recvonly"}),o.createOffer().then(n=>o.setLocalDescription(n)).then(()=>{y.current=new AbortController;const n={type:o.localDescription.type,sdp:o.localDescription.sdp},l=localStorage.getItem("auth");return fetch("/api/webrtc?src=".concat(encodeURIComponent(a.name)),{method:"POST",headers:{"Content-Type":"application/json",...l?{Authorization:"Basic "+l}:{}},body:JSON.stringify(n),signal:y.current.signal})}).then(n=>{if(!n.ok)throw new Error("Failed to send offer: ".concat(n.status," ").concat(n.statusText));return n.text()}).then(n=>{try{return JSON.parse(n)}catch(l){throw console.error("Error parsing JSON for stream ".concat(a.name,":"),l),new Error("Failed to parse WebRTC answer")}}).then(n=>o.setRemoteDescription(new RTCSessionDescription(n))).catch(n=>{console.error("Error setting up WebRTC for stream ".concat(a.name,":"),n),w(n.message||"Failed to establish WebRTC connection"),s(!1)}),()=>{console.log("Cleaning up WebRTC connection for stream ".concat(a.name)),y.current&&(y.current.abort(),y.current=null),u.current&&u.current.srcObject&&(u.current.srcObject.getTracks().forEach(l=>l.stop()),u.current.srcObject=null),p.current&&(p.current.close(),p.current=null)}},[a]);const P=()=>{w(null),s(!0),p.current&&(p.current.close(),p.current=null),u.current&&u.current.srcObject&&(u.current.srcObject.getTracks().forEach(n=>n.stop()),u.current.srcObject=null),c(!1)};return e("div",{className:"video-cell","data-stream-name":a.name,"data-stream-id":b,ref:g,style:{position:"relative",pointerEvents:"auto",zIndex:1},children:[e("video",{id:"video-".concat(b.replace(/\s+/g,"-")),className:"video-element",ref:u,playsInline:!0,autoPlay:!0,muted:!0,disablePictureInPicture:!0,style:{width:"100%",height:"100%",objectFit:"contain"}}),a.detection_based_recording&&a.detection_model&&e(B,{ref:i,streamName:a.name,videoRef:u,enabled:I,detectionModel:a.detection_model}),e("div",{className:"stream-name-overlay",style:{position:"absolute",top:"10px",left:"10px",padding:"5px 10px",backgroundColor:"rgba(0, 0, 0, 0.5)",color:"white",borderRadius:"4px",fontSize:"14px",zIndex:3},children:a.name}),e("div",{className:"stream-controls",style:{position:"absolute",bottom:"10px",right:"10px",display:"flex",gap:"10px",zIndex:5,backgroundColor:"rgba(0, 0, 0, 0.5)",padding:"5px",borderRadius:"4px"},children:[e("div",{style:{backgroundColor:"transparent",padding:"5px",borderRadius:"4px"},onMouseOver:o=>o.currentTarget.style.backgroundColor="rgba(255, 255, 255, 0.2)",onMouseOut:o=>o.currentTarget.style.backgroundColor="transparent",children:e(G,{streamId:b,streamName:a.name,onSnapshot:()=>{if(u.current){let o=null;if(i.current&&typeof i.current.getCanvasRef=="function"&&(o=i.current.getCanvasRef()),o){const n=q(u,o,a.name);n&&$(n.canvas.toDataURL("image/jpeg",.95),"Snapshot: ".concat(a.name))}else{const n=u.current,l=document.createElement("canvas");l.width=n.videoWidth,l.height=n.videoHeight,l.width>0&&l.height>0&&(l.getContext("2d").drawImage(n,0,0,l.width,l.height),$(l.toDataURL("image/jpeg",.95),"Snapshot: ".concat(a.name)))}}}})}),e("button",{className:"fullscreen-btn",title:"Toggle Fullscreen","data-id":b,"data-name":a.name,onClick:o=>k(a.name,o,g.current),style:{backgroundColor:"transparent",border:"none",padding:"5px",borderRadius:"4px",color:"white",cursor:"pointer"},onMouseOver:o=>o.currentTarget.style.backgroundColor="rgba(255, 255, 255, 0.2)",onMouseOut:o=>o.currentTarget.style.backgroundColor="transparent",children:e("svg",{xmlns:"http://www.w3.org/2000/svg",width:"24",height:"24",viewBox:"0 0 24 24",fill:"none",stroke:"white","stroke-width":"2","stroke-linecap":"round","stroke-linejoin":"round",children:e("path",{d:"M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"})})})]}),S&&e("div",{style:{position:"absolute",top:0,left:0,right:0,bottom:0,zIndex:5,pointerEvents:"none"},children:e(te,{message:"Connecting..."})}),h&&e("div",{className:"error-indicator",style:{position:"absolute",top:0,left:0,right:0,bottom:0,width:"100%",height:"100%",display:"flex",flexDirection:"column",justifyContent:"center",alignItems:"center",backgroundColor:"rgba(0, 0, 0, 0.7)",color:"white",zIndex:5,textAlign:"center"},children:e("div",{className:"error-content",style:{display:"flex",flexDirection:"column",justifyContent:"center",alignItems:"center",width:"80%",maxWidth:"300px",padding:"20px",borderRadius:"8px",backgroundColor:"rgba(0, 0, 0, 0.5)"},children:[e("div",{className:"error-icon",style:{fontSize:"28px",marginBottom:"15px",fontWeight:"bold",width:"40px",height:"40px",lineHeight:"40px",borderRadius:"50%",backgroundColor:"rgba(220, 38, 38, 0.8)",textAlign:"center"},children:"!"}),e("p",{style:{marginBottom:"20px",textAlign:"center",width:"100%",fontSize:"14px",lineHeight:"1.4"},children:h}),e("button",{className:"retry-button",onClick:P,style:{padding:"8px 20px",backgroundColor:"#2563eb",color:"white",borderRadius:"4px",border:"none",cursor:"pointer",fontWeight:"bold",fontSize:"14px",boxShadow:"0 2px 4px rgba(0, 0, 0, 0.2)",transition:"background-color 0.2s ease"},onMouseOver:o=>o.currentTarget.style.backgroundColor="#1d4ed8",onMouseOut:o=>o.currentTarget.style.backgroundColor="#2563eb",children:"Retry"})]})})]})}function ae(){const{takeSnapshot:a}=X(),{isFullscreen:b,setIsFullscreen:k,toggleFullscreen:S}=Q(),[s,h]=x([]),[w,I]=x(!0),[c,u]=x(()=>{const t=new URLSearchParams(window.location.search).get("layout");return t||sessionStorage.getItem("webrtc_layout")||"4"}),[g,p]=x(()=>{const t=new URLSearchParams(window.location.search).get("stream");return t||sessionStorage.getItem("webrtc_selected_stream")||""}),[i,y]=x(()=>{const t=new URLSearchParams(window.location.search).get("page");if(t)return Math.max(0,parseInt(t,10)-1);const m=sessionStorage.getItem("webrtc_current_page");return m?Math.max(0,parseInt(m,10)-1):0}),P=j();C(()=>{Y(),Z()},[]);const{data:o,isLoading:n,error:l}=O("streams","/api/streams",{timeout:15e3,retries:2,retryDelay:1e3});C(()=>{I(n)},[n]),C(()=>{o&&Array.isArray(o)&&(async()=>{try{const t=await R(o);if(t.length>0){h(t);const f=new URLSearchParams(window.location.search).get("stream");f&&t.some(d=>d.name===f)?p(f):(!g||!t.some(d=>d.name===g))&&p(t[0].name)}else console.warn("No streams available for WebRTC view after filtering")}catch(t){console.error("Error processing streams:",t),L("Error processing streams: "+t.message)}})()},[o,g]),C(()=>{if(s.length===0)return;console.log("Updating URL parameters");const r=new URL(window.location);i===0?r.searchParams.delete("page"):r.searchParams.set("page",i+1),c!=="4"?r.searchParams.set("layout",c):r.searchParams.delete("layout"),c==="1"&&g?r.searchParams.set("stream",g):r.searchParams.delete("stream"),window.history.replaceState({},"",r),i>0?sessionStorage.setItem("webrtc_current_page",(i+1).toString()):sessionStorage.removeItem("webrtc_current_page"),c!=="4"?sessionStorage.setItem("webrtc_layout",c):sessionStorage.removeItem("webrtc_layout"),c==="1"&&g?sessionStorage.setItem("webrtc_selected_stream",g):sessionStorage.removeItem("webrtc_selected_stream")},[i,c,g,s.length]);const R=async r=>{try{if(!r||!Array.isArray(r))return console.warn("No streams data provided to filter"),[];const t=r.map(async d=>{try{const T=d.id||d.name;return await P.fetchQuery({queryKey:["stream-details",T],queryFn:async()=>{const E=await fetch("/api/streams/".concat(encodeURIComponent(T)));if(!E.ok)throw new Error("Failed to load details for stream ".concat(d.name));return E.json()},staleTime:3e4})}catch(T){return console.error("Error loading details for stream ".concat(d.name,":"),T),d}}),m=await Promise.all(t);console.log("Loaded detailed streams for WebRTC view:",m);const f=m.filter(d=>d.is_deleted?(console.log("Stream ".concat(d.name," is soft deleted, filtering out")),!1):d.enabled?d.streaming_enabled?!0:(console.log("Stream ".concat(d.name," is not configured for streaming, filtering out")),!1):(console.log("Stream ".concat(d.name," is inactive, filtering out")),!1));return console.log("Filtered streams for WebRTC view:",f),f||[]}catch(t){return console.error("Error filtering streams for WebRTC view:",t),L("Error processing streams: "+t.message),[]}},v=M(()=>{switch(c){case"1":return 1;case"2":return 2;case"4":return 4;case"6":return 6;case"9":return 9;case"16":return 16;default:return 4}},[c]),W=M(()=>{let r=s;if(c==="1"&&g)r=s.filter(t=>t.name===g);else{const t=v(),m=Math.ceil(s.length/t);if(i>=m&&m>0)return[];const f=i*t,d=Math.min(f+t,s.length);r=s.slice(f,d)}return r},[s,c,g,i]);C(()=>{if(s.length===0)return;const r=v(),t=Math.ceil(s.length/r);i>=t&&y(Math.max(0,t-1))},[s,c,i,v]);const F=(r,t,m)=>{if(t&&(t.preventDefault(),t.stopPropagation()),!r){console.error("Stream name not provided for fullscreen toggle");return}if(console.log("Toggling fullscreen for stream: ".concat(r)),!m){console.error("Video cell element not provided for fullscreen toggle");return}document.fullscreenElement?(console.log("Exiting fullscreen mode"),document.exitFullscreen()):(console.log("Entering fullscreen mode for video cell"),m.requestFullscreen().catch(f=>{console.error("Error attempting to enable fullscreen: ".concat(f.message)),L("Could not enable fullscreen mode: ".concat(f.message))}))},_=D(()=>W(),[s,c,g,i,v]);return e("section",{id:"live-page",className:"page ".concat(b?"fullscreen-mode":""),children:[e(ee,{}),e(J,{isFullscreen:b,setIsFullscreen:k,targetId:"live-page"}),e("div",{className:"page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow",style:{position:"relative",zIndex:10,pointerEvents:"auto"},children:[e("div",{className:"flex items-center space-x-2",children:[e("h2",{className:"text-xl font-bold mr-4",children:"Live View"}),e("div",{className:"flex space-x-2",children:e("button",{id:"hls-toggle-btn",className:"px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 inline-block text-center",style:{position:"relative",zIndex:50},onClick:()=>{window.location.href="/hls.html"},children:"HLS View"})})]}),e("div",{className:"controls flex items-center space-x-2",children:[e("div",{className:"flex items-center",children:[e("label",{htmlFor:"layout-selector",className:"mr-2",children:"Layout:"}),e("select",{id:"layout-selector",className:"px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600",value:c,onChange:r=>{const t=r.target.value;u(t),y(0)},children:[e("option",{value:"1",children:"1 Stream"}),e("option",{value:"2",children:"2 Streams"}),e("option",{value:"4",children:"4 Streams"}),e("option",{value:"6",children:"6 Streams"}),e("option",{value:"9",children:"9 Streams"}),e("option",{value:"16",children:"16 Streams"})]})]}),c==="1"&&e("div",{className:"flex items-center",children:[e("label",{htmlFor:"stream-selector",className:"mr-2",children:"Stream:"}),e("select",{id:"stream-selector",className:"px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600",value:g,onChange:r=>{const t=r.target.value;p(t)},children:s.map(r=>e("option",{value:r.name,children:r.name},r.name))})]}),e("button",{id:"fullscreen-btn",className:"p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none",onClick:()=>S(),title:"Toggle Fullscreen",children:e("svg",{xmlns:"http://www.w3.org/2000/svg",width:"24",height:"24",viewBox:"0 0 24 24",fill:"none",stroke:"currentColor",strokeWidth:"2",strokeLinecap:"round",strokeLinejoin:"round",children:e("path",{d:"M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"})})})]})]}),e("div",{className:"flex flex-col space-y-4 h-full",children:[e("div",{id:"video-grid",className:"video-container layout-".concat(c),children:n?e("div",{className:"flex justify-center items-center col-span-full row-span-full h-64 w-full",style:{pointerEvents:"none",zIndex:1},children:e("div",{className:"flex flex-col items-center justify-center py-8",children:[e("div",{className:"inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"}),e("p",{className:"mt-4 text-gray-700 dark:text-gray-300",children:"Loading streams..."})]})}):w&&!n?e("div",{className:"flex justify-center items-center col-span-full row-span-full h-64 w-full",style:{pointerEvents:"none",position:"relative",zIndex:1},children:e("div",{className:"flex flex-col items-center justify-center py-8",children:[e("div",{className:"inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"}),e("p",{className:"mt-4 text-gray-700 dark:text-gray-300",children:"Loading streams..."})]})}):l?e("div",{className:"placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8",children:[e("p",{className:"mb-6 text-gray-600 dark:text-gray-300 text-lg",children:["Error loading streams: ",l.message]}),e("button",{onClick:()=>window.location.reload(),className:"btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",children:"Retry"})]}):s.length===0?e("div",{className:"placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8",children:[e("p",{className:"mb-6 text-gray-600 dark:text-gray-300 text-lg",children:"No streams configured"}),e("a",{href:"streams.html",className:"btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",children:"Configure Streams"})]}):_.map(r=>e(ne,{stream:r,onToggleFullscreen:F,streamId:r.name},r.name))}),c!=="1"&&s.length>v()?e("div",{className:"pagination-controls flex justify-center items-center space-x-4 mt-4",children:[e("button",{className:"px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",onClick:()=>{console.log("Changing to previous page"),y(Math.max(0,i-1))},disabled:i===0,children:"Previous"}),e("span",{className:"text-gray-700 dark:text-gray-300",children:["Page ",i+1," of ",Math.ceil(s.length/v())]}),e("button",{className:"px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",onClick:()=>{console.log("Changing to next page");const r=Math.ceil(s.length/v());y(Math.min(r-1,i+1))},disabled:i>=Math.ceil(s.length/v())-1,children:"Next"})]}):null]})]})}function se(){const[a,b]=x(!1),[k,S]=x(!0);return C(()=>{async function s(){try{const h=await fetch("/api/settings");if(!h.ok){console.error("Failed to fetch settings:",h.status,h.statusText),S(!1);return}(await h.json()).webrtc_disabled?(console.log("WebRTC is disabled, using HLS view"),b(!0)):(console.log("WebRTC is enabled, using WebRTC view"),b(!1))}catch(h){console.error("Error checking WebRTC status:",h)}finally{S(!1)}}s()},[]),k?e("div",{className:"loading",children:"Loading..."}):e(H,{children:a?e(K,{isWebRTCDisabled:!0}):e(ae,{})})}document.addEventListener("DOMContentLoaded",()=>{const a=document.getElementById("main-content");a&&z(e(U,{client:V,children:[e(re,{}),e(A,{}),e(se,{}),e(oe,{})]}),a)});export{ie as __vite_legacy_guard};
//# sourceMappingURL=index-CUj9-k1I.js.map
