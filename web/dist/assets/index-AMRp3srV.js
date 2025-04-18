import{d as y,A as C,y as x,u as r,a as G,b as K,c as X,s as M,q as Y,E as Z,e as ee,Q as oe,T as re,k as ne}from"./query-client-CKvJ69Lv.js";/* empty css               */import{s as A,u as te,c as ae,F as se,L as ie}from"./LiveView-DD_Rl5VA.js";import{s as le,a as ce}from"./UI-QFITwjX7.js";import{SnapshotButton as de,useSnapshotManager as ue,SnapshotManager as ge}from"./SnapshotManager-aeto7uun.js";import{L as fe}from"./LoadingIndicator-DNuPz-3P.js";import{H as me,F as pe}from"./Footer-BGfTnlCZ.js";function we(){import.meta.url,import("_").catch(()=>1),async function*(){}().next()}function he({stream:n,onTakeSnapshot:v,onToggleFullscreen:L,webrtcConnections:S,detectionIntervals:T,initializeWebRTCPlayer:i,cleanupWebRTCPlayer:W}){const[l,p]=y(!0),[d,w]=y(null),b=C(null),h=C(null),c=C(null);x(()=>{if(!n)return;if(console.log("WebRTCVideoCell: Initializing player for stream ".concat(n.name)),S.current[n.name]){console.log("WebRTCVideoCell: Stream ".concat(n.name," already has a connection, skipping initialization")),p(!1);return}console.log("WebRTCVideoCell: Will initialize stream ".concat(n.name," after a short delay"));const u=setTimeout(()=>{b.current&&h.current&&(console.log("WebRTCVideoCell: Now initializing stream ".concat(n.name)),i(n,b.current,h.current,{onLoadedData:()=>{console.log("Video data loaded for stream ".concat(n.name)),p(!1)},onPlaying:()=>{console.log("Video playing for stream ".concat(n.name)),p(!1),n.detection_based_recording&&n.detection_model&&h.current&&(console.log("Starting detection polling for stream ".concat(n.name)),A(n.name,h.current,b.current,T))},onError:$=>{console.error("Video error for stream ".concat(n.name,":"),$),w($||"Video playback error"),p(!1)}}))},100);return()=>{clearTimeout(u),n&&(console.log("WebRTCVideoCell: Cleaning up player for stream ".concat(n.name)),W(n.name))}},[n.name]);const R=()=>{n&&(console.log("Retrying connection for stream ".concat(n.name)),p(!0),w(null),W(n.name),setTimeout(()=>{b.current&&h.current&&(console.log("Reinitializing WebRTC player for stream ".concat(n.name)),i(n,b.current,h.current,{onLoadedData:()=>{console.log("Video data loaded for stream ".concat(n.name)),p(!1)},onPlaying:()=>{console.log("Video playing for stream ".concat(n.name)),p(!1)},onError:g=>{console.error("Video error for stream ".concat(n.name,":"),g),console.log("Trying one more time for stream ".concat(n.name," with a longer delay")),setTimeout(()=>{b.current&&h.current&&i(n,b.current,h.current,{onLoadedData:()=>{console.log("Video data loaded for stream ".concat(n.name," on second attempt")),p(!1)},onPlaying:()=>{console.log("Video playing for stream ".concat(n.name," on second attempt")),p(!1)},onError:u=>{console.error("Video error for stream ".concat(n.name," on second attempt:"),u),w(u||"Video playback error"),p(!1)}})},1e3)}}))},200))};return r("div",{className:"video-cell","data-stream-name":n.name,"data-stream-id":n.id||n.name,ref:c,style:{position:"relative",pointerEvents:l?"none":"auto"},children:[r("style",{children:"\n          @keyframes spin {\n            0% { transform: rotate(0deg); }\n            100% { transform: rotate(360deg); }\n          }\n        "}),r("video",{id:"video-".concat(n.name.replace(/\s+/g,"-")),className:"video-element",ref:b,playsInline:!0,autoPlay:!0,muted:!0,style:{pointerEvents:"none",width:"100%",height:"100%",objectFit:"contain",zIndex:1}}),r("canvas",{id:"canvas-".concat(n.name.replace(/\s+/g,"-")),className:"detection-overlay",ref:h,style:{position:"absolute",top:0,left:0,width:"100%",height:"100%",pointerEvents:"none",zIndex:2}}),r("div",{className:"stream-name-overlay",style:{position:"absolute",top:"10px",left:"10px",padding:"5px 10px",backgroundColor:"rgba(0, 0, 0, 0.5)",color:"white",borderRadius:"4px",fontSize:"14px",zIndex:3,pointerEvents:"none"},children:n.name}),r("div",{className:"stream-controls",style:{position:"absolute",bottom:"10px",right:"10px",display:"flex",gap:"10px",zIndex:5,backgroundColor:"rgba(0, 0, 0, 0.5)",padding:"5px",borderRadius:"4px",pointerEvents:"auto"},children:[r("div",{style:{backgroundColor:"transparent",padding:"5px",borderRadius:"4px",position:"relative",zIndex:1},onMouseOver:g=>g.currentTarget.style.backgroundColor="rgba(255, 255, 255, 0.2)",onMouseOut:g=>g.currentTarget.style.backgroundColor="transparent",children:r(de,{streamId:n.id||n.name,streamName:n.name})}),r("button",{className:"fullscreen-btn",title:"Toggle Fullscreen","data-id":n.id||n.name,"data-name":n.name,onClick:g=>L(n.name,g,c.current),style:{backgroundColor:"transparent",border:"none",padding:"5px",borderRadius:"4px",color:"white",cursor:"pointer",position:"relative",zIndex:1},onMouseOver:g=>g.currentTarget.style.backgroundColor="rgba(255, 255, 255, 0.2)",onMouseOut:g=>g.currentTarget.style.backgroundColor="transparent",children:r("svg",{xmlns:"http://www.w3.org/2000/svg",width:"24",height:"24",viewBox:"0 0 24 24",fill:"none",stroke:"white","stroke-width":"2","stroke-linecap":"round","stroke-linejoin":"round",children:r("path",{d:"M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"})})})]}),l&&r(fe,{message:"Connecting..."}),d&&r("div",{className:"error-indicator",style:{position:"absolute",top:0,left:0,right:0,bottom:0,width:"100%",height:"100%",display:"flex",flexDirection:"column",justifyContent:"center",alignItems:"center",backgroundColor:"rgba(0, 0, 0, 0.7)",color:"white",zIndex:10,pointerEvents:"auto",textAlign:"center",overflow:"hidden"},children:r("div",{className:"error-content",style:{display:"flex",flexDirection:"column",justifyContent:"center",alignItems:"center",width:"80%",maxWidth:"300px",padding:"20px",borderRadius:"8px",backgroundColor:"rgba(0, 0, 0, 0.5)"},children:[r("div",{className:"error-icon",style:{fontSize:"28px",marginBottom:"15px",fontWeight:"bold",width:"40px",height:"40px",lineHeight:"40px",borderRadius:"50%",backgroundColor:"rgba(220, 38, 38, 0.8)",textAlign:"center"},children:"!"}),r("p",{style:{marginBottom:"20px",textAlign:"center",width:"100%",fontSize:"14px",lineHeight:"1.4"},children:d}),r("button",{className:"retry-button",onClick:R,style:{padding:"8px 20px",backgroundColor:"#2563eb",color:"white",borderRadius:"4px",border:"none",cursor:"pointer",fontWeight:"bold",fontSize:"14px",boxShadow:"0 2px 4px rgba(0, 0, 0, 0.2)",transition:"background-color 0.2s ease"},onMouseOver:g=>g.currentTarget.style.backgroundColor="#1d4ed8",onMouseOut:g=>g.currentTarget.style.backgroundColor="#2563eb",children:"Retry"})]})})]})}function be(){const{takeSnapshot:n}=ue(),{isFullscreen:v,setIsFullscreen:L,toggleFullscreen:S}=te(),T=G({mutationFn:async e=>{const{streamName:o,...s}=e,t=localStorage.getItem("auth"),a=await fetch("/api/webrtc?src=".concat(encodeURIComponent(o)),{method:"POST",headers:{"Content-Type":"application/json",...t?{Authorization:"Basic "+t}:{}},body:JSON.stringify(s),signal:e.signal});if(!a.ok)throw new Error("Failed to send offer: ".concat(a.status," ").concat(a.statusText));const m=await a.text();try{return JSON.parse(m)}catch(I){throw console.error("Error parsing JSON for stream ".concat(o,":"),I),console.log("Raw response text: ".concat(m)),new Error("Failed to parse WebRTC answer: ".concat(I.message))}},onError:(e,o)=>{console.error("Error sending WebRTC offer for stream ".concat(o.streamName,":"),e)}}),[i,W]=y([]),[l,p]=y(()=>{const o=new URLSearchParams(window.location.search).get("layout");return o||sessionStorage.getItem("webrtc_layout")||"4"}),[d,w]=y(()=>{const o=new URLSearchParams(window.location.search).get("stream");return o||sessionStorage.getItem("webrtc_selected_stream")||""}),[b,h]=y(!0),[c,R]=y(()=>{const o=new URLSearchParams(window.location.search).get("page");if(o)return Math.max(0,parseInt(o,10)-1);const s=sessionStorage.getItem("webrtc_current_page");return s?Math.max(0,parseInt(s,10)-1):0}),g=C(null),u=C({}),$=C({});x(()=>{le(),ce();const e=()=>{console.log("Preserving URL parameters before page reload");const s=new URL(window.location);c>0?s.searchParams.set("page",c+1):s.searchParams.delete("page"),l!=="4"?s.searchParams.set("layout",l):s.searchParams.delete("layout"),l==="1"&&d?s.searchParams.set("stream",d):s.searchParams.delete("stream"),window.history.replaceState({},"",s),c>0?sessionStorage.setItem("webrtc_current_page",(c+1).toString()):sessionStorage.removeItem("webrtc_current_page"),l!=="4"?sessionStorage.setItem("webrtc_layout",l):sessionStorage.removeItem("webrtc_layout"),l==="1"&&d?sessionStorage.setItem("webrtc_selected_stream",d):sessionStorage.removeItem("webrtc_selected_stream")};window.addEventListener("beforeunload",e);const o=setInterval(()=>{Object.keys(u.current).forEach(s=>{const t=u.current[s];if(t&&(console.debug("WebRTC connection state for ".concat(s,": ").concat(t.connectionState,", ICE state: ").concat(t.iceConnectionState)),t.iceConnectionState==="failed"||t.iceConnectionState==="disconnected")){console.warn("WebRTC connection for ".concat(s," is in ").concat(t.iceConnectionState," state, will attempt reconnect")),P(s);const a=i.find(m=>m.name===s);a&&(console.log("Attempting to reconnect WebRTC for stream ".concat(s)),O(a))}})},3e4);return()=>{window.removeEventListener("beforeunload",e),clearInterval(o),H()}},[i,c,l,d]);const j=K(),{data:N,isLoading:V,error:D}=X("streams","/api/streams",{timeout:15e3,retries:2,retryDelay:1e3});x(()=>{h(V)},[V]),x(()=>{N&&Array.isArray(N)&&(async()=>{try{const o=await U(N);if(o.length>0){W(o);const t=new URLSearchParams(window.location.search).get("stream");t&&o.some(a=>a.name===t)?w(t):(!d||!o.some(a=>a.name===d))&&w(o[0].name)}else console.warn("No streams available for WebRTC view after filtering")}catch(o){console.error("Error processing streams:",o),M("Error processing streams: "+o.message)}})()},[N,d,j]);const F=C({layout:l,selectedStream:d,currentPage:c,streamsLength:i.length});x(()=>{const e=F.current;(e.layout!==l||e.selectedStream!==d||e.currentPage!==c||e.streamsLength!==i.length)&&(console.log("Layout, selectedStream, currentPage, or streams changed, updating video grid"),q(),F.current={layout:l,selectedStream:d,currentPage:c,streamsLength:i.length})},[l,d,i,c]),x(()=>{if(i.length===0)return;const e=setTimeout(()=>{console.log("Updating URL parameters");const o=new URL(window.location);c===0?o.searchParams.delete("page"):o.searchParams.set("page",c+1),l!=="4"?o.searchParams.set("layout",l):o.searchParams.delete("layout"),l==="1"&&d?o.searchParams.set("stream",d):o.searchParams.delete("stream"),window.history.replaceState({},"",o),c>0?sessionStorage.setItem("webrtc_current_page",(c+1).toString()):sessionStorage.removeItem("webrtc_current_page"),l!=="4"?sessionStorage.setItem("webrtc_layout",l):sessionStorage.removeItem("webrtc_layout"),l==="1"&&d?sessionStorage.setItem("webrtc_selected_stream",d):sessionStorage.removeItem("webrtc_selected_stream")},300);return()=>clearTimeout(e)},[c,l,d,i.length]);const U=async e=>{try{if(!e||!Array.isArray(e))return console.warn("No streams data provided to filter"),[];const o=e.map(async a=>{try{const m=a.id||a.name;return await j.fetchQuery({queryKey:["stream-details",m],queryFn:async()=>{const E=await fetch("/api/streams/".concat(encodeURIComponent(m)));if(!E.ok)throw new Error("Failed to load details for stream ".concat(a.name));return E.json()},staleTime:3e4})}catch(m){return console.error("Error loading details for stream ".concat(a.name,":"),m),a}}),s=await Promise.all(o);console.log("Loaded detailed streams for WebRTC view:",s);const t=s.filter(a=>a.is_deleted?(console.log("Stream ".concat(a.name," is soft deleted, filtering out")),!1):a.enabled?a.streaming_enabled?!0:(console.log("Stream ".concat(a.name," is not configured for streaming, filtering out")),!1):(console.log("Stream ".concat(a.name," is inactive, filtering out")),!1));return console.log("Filtered streams for WebRTC view:",t),t||[]}catch(o){return console.error("Error filtering streams for WebRTC view:",o),M("Error processing streams: "+o.message),[]}},k=()=>{switch(l){case"1":return 1;case"2":return 2;case"4":return 4;case"6":return 6;case"9":return 9;case"16":return 16;default:return 4}},z=()=>{let e=i;if(l==="1"&&d)e=i.filter(o=>o.name===d);else{const o=k(),s=Math.ceil(i.length/o);if(c>=s&&s>0)return[];const t=c*o,a=Math.min(t+o,i.length);e=i.slice(t,a)}return e},q=()=>{if(!g.current)return;let e=z();if(e.length===0&&i.length>0){const t=k(),a=Math.ceil(i.length/t);if(c>=a){R(Math.max(0,a-1));return}}const o=e.map(t=>t.name);console.log("Updating video grid for page ".concat(c+1,", showing streams:"),o);const s=Object.keys(u.current).filter(t=>!o.includes(t));s.length>0&&(console.log("Cleaning up ".concat(s.length," WebRTC connections that are no longer visible:"),s),s.forEach(t=>{P(t)}))},O=(e,o,s,t={})=>{if(!e||!o){console.error("Cannot initialize WebRTC player: missing stream or video element");return}u.current[e.name]&&(console.log("WebRTC connection for stream ".concat(e.name," already exists, cleaning up first")),P(e.name)),console.log("Initializing WebRTC player for stream ".concat(e.name));const a=new RTCPeerConnection({iceServers:[{urls:"stun:stun.l.google.com:19302"}],iceTransportPolicy:"all",bundlePolicy:"balanced",rtcpMuxPolicy:"require",sdpSemantics:"unified-plan"});u.current[e.name]=a,a.ontrack=f=>{console.log("Track received for stream ".concat(e.name,":"),f),f.track.kind==="video"&&(o.srcObject=f.streams[0],o.onloadeddata=()=>{console.log("Video data loaded for stream ".concat(e.name)),t.onLoadedData&&t.onLoadedData()},o.onplaying=()=>{console.log("Video playing for stream ".concat(e.name)),t.onPlaying&&t.onPlaying(),e.detection_based_recording&&e.detection_model&&s?(console.log("Starting detection polling for stream ".concat(e.name," now that video is playing")),A(e.name,s,o,$.current)):console.log("Detection not enabled for stream ".concat(e.name))},o.onerror=J=>{console.error("Video error for stream ".concat(e.name,":"),J),t.onError&&t.onError("Video playback error")})},a.onicecandidate=f=>{f.candidate&&console.log("ICE candidate for stream ".concat(e.name,":"),f.candidate)},a.oniceconnectionstatechange=()=>{console.log("ICE connection state for stream ".concat(e.name,":"),a.iceConnectionState),a.iceConnectionState==="failed"?(console.warn("ICE failed for stream ".concat(e.name)),t.onError&&t.onError("WebRTC ICE connection failed")):a.iceConnectionState==="disconnected"&&console.warn("ICE disconnected for stream ".concat(e.name))},a.onconnectionstatechange=()=>{console.log("Connection state changed for stream ".concat(e.name,":"),a.connectionState),a.connectionState==="failed"&&(console.warn("Connection failed for stream ".concat(e.name)),t.onError&&t.onError("WebRTC connection failed"))},a.addTransceiver("video",{direction:"recvonly"}),a.addTransceiver("audio",{direction:"recvonly"});const m={offerToReceiveAudio:!0,offerToReceiveVideo:!0},I=setTimeout(()=>{console.warn("WebRTC setup timed out for stream ".concat(e.name)),t.onError&&t.onError("WebRTC setup timed out"),u.current[e.name]&&P(e.name)},3e4),E=setTimeout(()=>{u.current[e.name]&&(!o.srcObject||o.readyState<2)&&(console.warn("Video playback timed out for stream ".concat(e.name)),t.onError&&t.onError("Video playback timed out"))},2e4),_=()=>u.current[e.name]===a;a.createOffer(m).then(f=>{if(!_())throw new Error("Connection was cleaned up during offer creation");return console.log("Created offer for stream ".concat(e.name)),a.setLocalDescription(f)}).then(()=>{if(!_())throw new Error("Connection was cleaned up after setting local description");return console.log("Set local description for stream ".concat(e.name)),B(e.name,a.localDescription)}).then(f=>{if(!_())throw new Error("Connection was cleaned up after receiving answer");return console.log("Received answer for stream ".concat(e.name)),a.setRemoteDescription(new RTCSessionDescription(f))}).then(()=>{if(!_())throw new Error("Connection was cleaned up after setting remote description");console.log("Set remote description for stream ".concat(e.name)),clearTimeout(I),clearTimeout(E)}).catch(f=>{clearTimeout(I),clearTimeout(E),_()?(console.error("Error setting up WebRTC for stream ".concat(e.name,":"),f),t.onError&&t.onError(f.message)):console.log("WebRTC setup for stream ".concat(e.name," was cancelled: ").concat(f.message))}),o.addEventListener("playing",()=>{clearTimeout(E)},{once:!0})},B=Y(async(e,o)=>{try{const s={type:o.type,sdp:o.sdp};console.log("Sending formatted offer for stream ".concat(e));const t=new AbortController,a=t.signal;if(u.current[e])u.current[e].abortController=t;else return console.log("Connection for stream ".concat(e," no longer exists, aborting offer")),t.abort(),Promise.reject(new Error("Connection no longer exists"));const m=await T.mutateAsync({...s,streamName:e,signal:a});return u.current[e]?m:(console.log("Connection for stream ".concat(e," was cleaned up during offer, rejecting result")),Promise.reject(new Error("Connection was cleaned up during offer")))}catch(s){if(s.name==="AbortError")return console.log("WebRTC offer request for stream ".concat(e," was aborted")),Promise.reject(new Error("Request aborted"));throw console.error("Error sending offer for stream ".concat(e,":"),s),s}},[T]),P=e=>{if(console.log("Cleaning up WebRTC player for stream ".concat(e)),u.current[e]){const o=u.current[e];if(o.abortController){console.log("Aborting pending WebRTC requests for stream ".concat(e));try{o.abortController.abort()}catch(s){console.error("Error aborting WebRTC request for stream ".concat(e,":"),s)}}o.onicecandidate&&(o.onicecandidate=null),o.oniceconnectionstatechange&&(o.oniceconnectionstatechange=null),o.onconnectionstatechange&&(o.onconnectionstatechange=null),o.ontrack&&(o.ontrack=null),o.close(),delete u.current[e],console.log("Closed WebRTC connection for stream ".concat(e))}ae(e,$.current)},H=()=>{console.log("Stopping all WebRTC streams"),Object.keys(u.current).forEach(e=>{P(e)}),console.log("All WebRTC streams stopped")},Q=(e,o,s)=>{if(o&&(o.preventDefault(),o.stopPropagation()),!e){console.error("Stream name not provided for fullscreen toggle");return}if(console.log("Toggling fullscreen for stream: ".concat(e)),!s){console.error("Video cell element not provided for fullscreen toggle");return}document.fullscreenElement?(console.log("Exiting fullscreen mode"),document.exitFullscreen()):(console.log("Entering fullscreen mode for video cell"),s.requestFullscreen().catch(t=>{console.error("Error attempting to enable fullscreen: ".concat(t.message)),M("Could not enable fullscreen mode: ".concat(t.message))}))};return r("section",{id:"live-page",className:"page ".concat(v?"fullscreen-mode":""),style:{position:"relative",zIndex:1},children:[r(ge,{}),r(se,{isFullscreen:v,setIsFullscreen:L,targetId:"live-page"}),r("div",{className:"page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow",children:[r("div",{className:"flex items-center space-x-2",children:[r("h2",{className:"text-xl font-bold mr-4",children:"Live View"}),r("div",{className:"flex space-x-2",children:r("button",{id:"hls-toggle-btn",className:"px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800",onClick:()=>window.location.href="/hls.html",children:"HLS View"})})]}),r("div",{className:"controls flex items-center space-x-2",children:[r("div",{className:"flex items-center",children:[r("label",{for:"layout-selector",className:"mr-2",children:"Layout:"}),r("select",{id:"layout-selector",className:"px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600",value:l,onChange:e=>{const o=e.target.value;p(o),R(0)},children:[r("option",{value:"1",children:"1 Stream"}),r("option",{value:"2",children:"2 Streams"}),r("option",{value:"4",selected:!0,children:"4 Streams"}),r("option",{value:"6",children:"6 Streams"}),r("option",{value:"9",children:"9 Streams"}),r("option",{value:"16",children:"16 Streams"})]})]}),l==="1"&&r("div",{className:"flex items-center",children:[r("label",{for:"stream-selector",className:"mr-2",children:"Stream:"}),r("select",{id:"stream-selector",className:"px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600",value:d,onChange:e=>{const o=e.target.value;w(o)},children:i.map(e=>r("option",{value:e.name,children:e.name},e.name))})]}),r("button",{id:"fullscreen-btn",className:"p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none",onClick:()=>S(),title:"Toggle Fullscreen",children:r("svg",{xmlns:"http://www.w3.org/2000/svg",width:"24",height:"24",viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":"2","stroke-linecap":"round","stroke-linejoin":"round",children:r("path",{d:"M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"})})})]})]}),r("div",{className:"flex flex-col space-y-4",children:[r("div",{id:"video-grid",className:"video-container layout-".concat(l),ref:g,style:{position:"relative",zIndex:1},children:V?r("div",{className:"flex justify-center items-center col-span-full row-span-full h-64 w-full",children:r("div",{className:"flex flex-col items-center justify-center py-8",children:[r("div",{className:"inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"}),r("p",{className:"mt-4 text-gray-700 dark:text-gray-300",children:"Loading streams..."})]})}):b&&!V?r("div",{className:"flex justify-center items-center col-span-full row-span-full h-64 w-full",style:{pointerEvents:"none",position:"relative",zIndex:1},children:r("div",{className:"flex flex-col items-center justify-center py-8",children:[r("div",{className:"inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"}),r("p",{className:"mt-4 text-gray-700 dark:text-gray-300",children:"Loading streams..."})]})}):D?r("div",{className:"placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8",children:[r("p",{className:"mb-6 text-gray-600 dark:text-gray-300 text-lg",children:["Error loading streams: ",D.message]}),r("button",{onClick:()=>window.location.reload(),className:"btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",children:"Retry"})]}):i.length===0?r("div",{className:"placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8",children:[r("p",{className:"mb-6 text-gray-600 dark:text-gray-300 text-lg",children:"No streams configured"}),r("a",{href:"streams.html",className:"btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",children:"Configure Streams"})]}):z().map(e=>r(he,{stream:e,onTakeSnapshot:n,onToggleFullscreen:Q,webrtcConnections:u,detectionIntervals:$,initializeWebRTCPlayer:O,cleanupWebRTCPlayer:P},e.name))}),l!=="1"&&i.length>k()?r("div",{className:"pagination-controls flex justify-center items-center space-x-4 mt-4",children:[r("button",{className:"px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",onClick:()=>{console.log("Changing to previous page"),R(Math.max(0,c-1));const e=new URL(window.location),o=c-1;o>0?(e.searchParams.set("page",o+1),sessionStorage.setItem("webrtc_current_page",(o+1).toString())):(e.searchParams.delete("page"),sessionStorage.removeItem("webrtc_current_page")),window.history.replaceState({},"",e)},disabled:c===0,children:"Previous"}),r("span",{className:"text-gray-700 dark:text-gray-300",children:["Page ",c+1," of ",Math.ceil(i.length/k())]}),r("button",{className:"px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",onClick:()=>{console.log("Changing to next page");const e=Math.min(Math.ceil(i.length/k())-1,c+1);R(e);const o=new URL(window.location);o.searchParams.set("page",e+1),sessionStorage.setItem("webrtc_current_page",(e+1).toString()),window.history.replaceState({},"",o)},disabled:c>=Math.ceil(i.length/k())-1,children:"Next"})]}):null]})]})}function ye(){const[n,v]=y(!1),[L,S]=y(!0);return x(()=>{async function T(){try{const i=await fetch("/api/settings");if(!i.ok){console.error("Failed to fetch settings:",i.status,i.statusText),S(!1);return}(await i.json()).webrtc_disabled?(console.log("WebRTC is disabled, using HLS view"),v(!0)):(console.log("WebRTC is enabled, using WebRTC view"),v(!1))}catch(i){console.error("Error checking WebRTC status:",i)}finally{S(!1)}}T()},[]),L?r("div",{className:"loading",children:"Loading..."}):r(ne,{children:[r(me,{}),r(re,{}),n?r(ie,{isWebRTCDisabled:!0}):r(be,{}),r(pe,{})]})}document.addEventListener("DOMContentLoaded",()=>{const n=document.getElementById("main-content");n&&Z(r(oe,{client:ee,children:r(ye,{})}),n)});export{we as __vite_legacy_guard};
//# sourceMappingURL=index-AMRp3srV.js.map
