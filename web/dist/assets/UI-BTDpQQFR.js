const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./query-client-BSpadhku.js","../css/query-client.css"])))=>i.map(i=>d[i]);
import{d as E,q as M,y as N,s as y,u as e,K as X,A as L,$ as F}from"./query-client-BSpadhku.js";const Z="modulepreload",ee=function(a,s){return new URL(a,s).href},W={},B=function(s,i,h){let r=Promise.resolve();if(i&&i.length>0){const d=document.getElementsByTagName("link"),t=document.querySelector("meta[property=csp-nonce]"),m=(t==null?void 0:t.nonce)||(t==null?void 0:t.getAttribute("nonce"));r=Promise.allSettled(i.map(c=>{if(c=ee(c,h),c in W)return;W[c]=!0;const l=c.endsWith(".css"),v=l?'[rel="stylesheet"]':"";if(!!h)for(let w=d.length-1;w>=0;w--){const C=d[w];if(C.href===c&&(!l||C.rel==="stylesheet"))return}else if(document.querySelector('link[href="'.concat(c,'"]').concat(v)))return;const g=document.createElement("link");if(g.rel=l?"stylesheet":Z,l||(g.as="script"),g.crossOrigin="",g.href=c,m&&g.setAttribute("nonce",m),document.head.appendChild(g),l)return new Promise((w,C)=>{g.addEventListener("load",w),g.addEventListener("error",()=>C(new Error("Unable to preload CSS for ".concat(c))))})}))}function o(d){const t=new Event("vite:preloadError",{cancelable:!0});if(t.payload=d,window.dispatchEvent(t),!t.defaultPrevented)throw d}return r.then(d=>{for(const t of d||[])t.status==="rejected"&&o(t.reason);return s().catch(o)})};function V(){const[a,s]=E({canvas:null,fileName:null}),i=M((r,o)=>{console.log("Taking snapshot of stream with ID: ".concat(r));const d=document.querySelector('.snapshot-btn[data-id="'.concat(r,'"]'));let t;if(d)t=d.getAttribute("data-name");else if(o){const u=o.currentTarget||o.target,g=u?u.closest(".video-cell"):null;g&&(t=g.dataset.streamName)}if(!t){console.error("Stream name not found for ID:",r),y("Cannot take snapshot: Stream not identified","error");return}const m="video-".concat(t.replace(/\s+/g,"-")),c=document.getElementById(m);if(!c){console.error("Video element not found for stream:",t);return}const l=document.createElement("canvas");if(l.width=c.videoWidth,l.height=c.videoHeight,l.width===0||l.height===0){console.error("Invalid video dimensions:",l.width,l.height),y("Cannot take snapshot: Video not loaded or has invalid dimensions");return}l.getContext("2d").drawImage(c,0,0,l.width,l.height);try{const u=new Date().toISOString().replace(/[:.]/g,"-"),g="snapshot-".concat(t.replace(/\s+/g,"-"),"-").concat(u,".jpg");s({canvas:l,fileName:g}),re(l.toDataURL("image/jpeg",.95),"Snapshot: ".concat(t)),y("Snapshot taken successfully")}catch(u){console.error("Error creating snapshot:",u),y("Failed to create snapshot: "+u.message)}},[]),h=M(()=>{const{canvas:r,fileName:o}=a;if(!r){console.error("No snapshot canvas available"),y("Download failed: No snapshot data available");return}r.toBlob(function(d){if(!d){console.error("Failed to create blob from canvas"),y("Download failed: Unable to create image data");return}console.log("Created blob:",d.size,"bytes");const t=URL.createObjectURL(d);console.log("Created blob URL:",t);const m=document.createElement("a");m.href=t,m.download=o||"snapshot.jpg",document.body.appendChild(m),console.log("Added download link to document"),m.click(),setTimeout(()=>{document.body.contains(m)&&document.body.removeChild(m),URL.revokeObjectURL(t),console.log("Cleaned up download resources")},1e3),y("Download started")},"image/jpeg",.95)},[a]);return{takeSnapshot:i,downloadSnapshot:h,hasSnapshot:!!a.canvas}}function te(){const{takeSnapshot:a,downloadSnapshot:s}=V();return N(()=>(window.takeSnapshot=a,()=>{delete window.takeSnapshot}),[a]),null}function oe({streamId:a,streamName:s,onSnapshot:i}){const{takeSnapshot:h}=V();return e("button",{className:"snapshot-btn",title:"Take Snapshot","data-id":a,"data-name":s,onClick:o=>{o.preventDefault(),o.stopPropagation(),typeof i=="function"?i(o):h(a,o)},children:e("svg",{xmlns:"http://www.w3.org/2000/svg",width:"24",height:"24",viewBox:"0 0 24 24",fill:"none",stroke:"white","stroke-width":"2","stroke-linecap":"round","stroke-linejoin":"round",children:[e("path",{d:"M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"}),e("circle",{cx:"12",cy:"13",r:"4"})]})})}const ne=Object.freeze(Object.defineProperty({__proto__:null,SnapshotButton:oe,SnapshotManager:te,useSnapshotManager:V},Symbol.toStringTag,{value:"Module"})),ae=X({showVideoModal:()=>{},showSnapshotPreview:()=>{}});function le(a){const{isOpen:s,onClose:i,onConfirm:h,mode:r,count:o}=a;if(!s)return null;let d="Confirm Delete",t="Are you sure you want to delete this item?";return r==="selected"?(d="Delete Selected Recordings",t="Are you sure you want to delete ".concat(o," selected recording").concat(o!==1?"s":"","?")):r==="all"&&(d="Delete All Filtered Recordings",t="Are you sure you want to delete all recordings matching the current filters? This action cannot be undone."),e("div",{class:"fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",onClick:l=>{l.target===l.currentTarget&&i()},onKeyDown:l=>{l.key==="Escape"&&i()},children:e("div",{class:"bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 max-w-md mx-auto",children:[e("div",{class:"mb-4",children:e("h3",{class:"text-lg font-semibold text-gray-900 dark:text-white",children:d})}),e("p",{class:"text-gray-600 dark:text-gray-300 mb-6",children:t}),e("div",{class:"flex justify-end space-x-3",children:[e("button",{class:"px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600",onClick:i,children:"Cancel"}),e("button",{class:"px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors",onClick:h,children:"Delete"})]})]})})}function q({isOpen:a,onClose:s,imageData:i,streamName:h,onDownload:r}){const o=L(null);N(()=>{const t=m=>{m.key==="Escape"&&a&&s()};return document.addEventListener("keydown",t),a&&o.current&&setTimeout(()=>{const m=o.current.querySelector(".modal-content");m&&(m.classList.remove("scale-95","opacity-0"),m.classList.add("scale-100","opacity-100"))},10),()=>{document.removeEventListener("keydown",t)}},[a,s]);const d=t=>{t.target===t.currentTarget&&s()};return a?F(e("div",{ref:o,id:"snapshot-preview-modal",className:"fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50",onClick:d,children:e("div",{className:"modal-content bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out scale-95 opacity-0",style:{width:"90%"},children:[e("div",{className:"flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700",children:[e("h3",{id:"snapshot-preview-title",className:"text-lg font-semibold text-gray-900 dark:text-white",children:h?"Snapshot: ".concat(h):"Snapshot"}),e("button",{className:"close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200",onClick:s,children:"✕"})]}),e("div",{className:"p-4 overflow-auto flex-grow",children:e("img",{id:"snapshot-preview-image",className:"max-w-full max-h-[70vh] mx-auto",src:i,alt:"Snapshot"})}),e("div",{className:"p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-2",children:[e("button",{id:"snapshot-download-btn",className:"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",onClick:r,children:"Download"}),e("button",{id:"snapshot-close-btn",className:"px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600",onClick:s,children:"Close"})]})]})}),document.body):null}function z({isOpen:a,onClose:s,videoUrl:i,title:h,downloadUrl:r}){console.log("VideoModal rendered with props:",{isOpen:a,videoUrl:i,title:h});const[o,d]=E(!1),[t,m]=E(2),[c,l]=E([]),[v,u]=E(null),[g,w]=E("No detections loaded"),[C,T]=E(1),b=L(null),_=L(null),P=L(null);N(()=>{const n=p=>{p.key==="Escape"&&a&&s()};return a&&(console.log("VideoModal opened, setting up event listeners"),document.addEventListener("keydown",n),b.current&&b.current.load()),()=>{if(console.log("VideoModal cleanup"),document.removeEventListener("keydown",n),b.current)try{b.current.pause(),b.current.removeAttribute("src"),b.current.load()}catch(p){console.error("Error cleaning up video element:",p)}}},[a,s]),N(()=>{if(!a||!i)return;const n=i.match(/\/play\/(\d+)/);if(!n||!n[1])return;const p=parseInt(n[1],10);(async()=>{try{const k=await fetch("/api/recordings/".concat(p));if(!k.ok)throw new Error("Failed to fetch recording data");const f=await k.json();if(u(f),f&&f.stream&&f.start_time&&f.end_time){const j=Math.floor(new Date(f.start_time).getTime()/1e3),I=Math.floor(new Date(f.end_time).getTime()/1e3),R=await fetch("/api/detection/results/".concat(f.stream,"?start=").concat(j,"&end=").concat(I));if(!R.ok)throw new Error("Failed to fetch detections");const x=(await R.json()).detections||[];l(x),x.length>0?w("".concat(x.length," detection").concat(x.length!==1?"s":""," available")):w("No detections found for this recording")}}catch(k){console.error("Error fetching data:",k),w("Error loading detections")}})()},[a,i]);const D=M(()=>{if(!o||!b.current||!_.current||c.length===0)return;const n=b.current,p=_.current,S=n.videoWidth,k=n.videoHeight;if(S===0||k===0){requestAnimationFrame(D);return}p.width=S,p.height=k;const f=p.getContext("2d");f.clearRect(0,0,p.width,p.height);const j=n.currentTime;if(!v||!v.start_time)return;const R=Math.floor(new Date(v.start_time).getTime()/1e3)+Math.floor(j),$=c.filter(x=>x.timestamp?Math.abs(x.timestamp-R)<=t:!1);$.length>0?w("Showing ".concat($.length," detection").concat($.length!==1?"s":""," at current time")):w("No detections at current time (".concat(c.length," total)")),$.forEach(x=>{const A=x.x*S,O=x.y*k,G=x.width*S,J=x.height*k;f.strokeStyle="rgba(0, 255, 0, 0.8)",f.lineWidth=2,f.strokeRect(A,O,G,J),f.fillStyle="rgba(0, 0, 0, 0.7)";const U="".concat(x.label," (").concat(Math.round(x.confidence*100),"%)"),Q=f.measureText(U).width+10;f.fillRect(A,O-20,Q,20),f.fillStyle="white",f.font="12px Arial",f.fillText(U,A+5,O-5)}),!n.paused&&!n.ended&&requestAnimationFrame(D)},[o,c,v,t]);N(()=>{if(!a||!b.current)return;const n=b.current,p=()=>{o&&D()},S=()=>{o&&D()},k=()=>{if(o){const f=Math.floor(n.currentTime*2)/2;n.lastDrawnTime!==f&&(n.lastDrawnTime=f,D())}};return n.addEventListener("play",p),n.addEventListener("seeked",S),n.addEventListener("timeupdate",k),()=>{n.removeEventListener("play",p),n.removeEventListener("seeked",S),n.removeEventListener("timeupdate",k)}},[a,o,D]),N(()=>{a&&i&&b.current&&(console.log("Video URL changed, loading new video"),b.current.load())},[a,i]),N(()=>{o?(_.current&&(_.current.style.display="block"),D()):_.current&&(_.current.style.display="none")},[o,D]);const K=n=>{n.target===n.currentTarget&&s()},H=n=>{b.current&&(b.current.playbackRate=n,T(n))},Y=n=>{const p=parseInt(n.target.value,10);m(p),o&&D()};return a?(N(()=>{let n;return a&&P.current&&(n=setTimeout(()=>{var S;const p=(S=P.current)==null?void 0:S.querySelector(".modal-content");p&&(p.classList.remove("scale-95","opacity-0"),p.classList.add("scale-100","opacity-100"),console.log("Modal animation applied"))},10)),()=>{n&&clearTimeout(n)}},[a]),F(e("div",{ref:P,id:"video-preview-modal",className:"fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50",onClick:K,children:e("div",{className:"modal-content bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out ".concat(a?"scale-100 opacity-100":"scale-95 opacity-0"," w-full md:w-[90%]"),children:[e("div",{className:"flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700",children:[e("h3",{id:"video-preview-title",className:"text-lg font-semibold text-gray-900 dark:text-white",children:h||"Video"}),e("button",{className:"close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200",onClick:s,children:"✕"})]}),e("div",{className:"p-4 flex-grow",children:e("div",{className:"relative",children:[e("video",{ref:b,className:"w-full h-auto max-w-full object-contain mx-auto",controls:!0,autoPlay:!0,onError:n=>{console.error("Video error:",n),y("Error loading video. Please try again.","error")},onLoadStart:()=>console.log("Video load started"),onLoadedData:()=>console.log("Video data loaded"),children:i&&e("source",{src:i,type:"video/mp4"})},i),e("canvas",{ref:_,className:"absolute top-0 left-0 w-full h-full pointer-events-none",style:{display:"none"}})]})}),e("div",{id:"recordings-controls",className:"mx-4 mb-4 p-4 border border-green-500 rounded-lg bg-white dark:bg-gray-700 shadow-md relative z-10",children:[e("h3",{className:"text-lg font-bold text-center mb-4 text-gray-800 dark:text-white",children:"PLAYBACK CONTROLS"}),e("div",{className:"grid grid-cols-1 md:grid-cols-2 gap-4 mb-2",children:[e("div",{className:"border-b pb-4 md:border-b-0 md:border-r md:pr-4 md:pb-0",children:[e("h4",{className:"font-bold text-center mb-3 text-gray-700 dark:text-gray-300",children:"Playback Speed"}),e("div",{className:"flex flex-wrap justify-center gap-2",children:[.25,.5,1,1.5,2,4].map(n=>e("button",{className:"speed-btn px-3 py-2 rounded-full ".concat(n===C?"bg-green-500 text-white":"bg-gray-200 hover:bg-gray-300"," text-sm font-medium transition-all focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50"),"data-speed":n,onClick:()=>H(n),children:n===1?"1× (Normal)":"".concat(n,"×")},n))}),e("div",{id:"current-speed-indicator",className:"mt-3 text-center font-medium text-green-600 dark:text-green-400 text-sm",children:["Current Speed: ",C,"× ",C===1?"(Normal)":""]})]}),e("div",{className:"pt-4 md:pt-0 md:pl-4",children:[e("h4",{className:"font-bold text-center mb-2 text-gray-700 dark:text-gray-300",children:"Detection Overlays"}),e("div",{className:"flex flex-col items-center gap-2",children:[e("div",{className:"flex items-center gap-2 mb-2",children:[e("input",{type:"checkbox",id:"detection-overlay-checkbox",className:"w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",checked:o,onChange:n=>d(n.target.checked),disabled:c.length===0}),e("label",{htmlFor:"detection-overlay-checkbox",className:"text-sm font-medium text-gray-700 dark:text-gray-300",children:"Show Detection Overlays"})]}),e("div",{className:"flex flex-col w-full mt-2 mb-2",children:[e("label",{htmlFor:"detection-sensitivity-slider",className:"text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",children:"Detection Sensitivity"}),e("input",{type:"range",id:"detection-sensitivity-slider",className:"w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700",min:"1",max:"10",step:"1",value:t,onChange:Y}),e("div",{id:"detection-sensitivity-value",className:"text-xs text-gray-600 dark:text-gray-400 text-center mb-1",children:["Time Window: ",t," second",t!==1?"s":""]})]}),e("div",{id:"detection-status-indicator",className:"text-center text-sm ".concat(c.length>0?"font-medium text-green-600 dark:text-green-400":"text-gray-600 dark:text-gray-400"),children:g})]})]})]}),r&&e("div",{className:"flex justify-center mt-4 pt-2 border-t border-gray-200 dark:border-gray-700",children:e("a",{className:"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center text-sm",href:r,download:"video-".concat(Date.now(),".mp4"),children:[e("svg",{xmlns:"http://www.w3.org/2000/svg",className:"h-4 w-4 mr-2",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor",children:e("path",{strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:"2",d:"M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"})}),"Download Video"]})})]}),e("div",{className:"p-2"})]})}),document.body)):null}function ie({children:a}){const[s,i]=E({isOpen:!1,imageData:"",streamName:""}),[h,r]=E({isOpen:!1,videoUrl:"",title:"",downloadUrl:""}),o=M((u,g)=>{i({isOpen:!0,imageData:u,streamName:g})},[]),d=M((u,g,w)=>{console.log("ModalProvider.showVideoModal called with:",{videoUrl:u,title:g,downloadUrl:w}),r({isOpen:!1,videoUrl:"",title:"",downloadUrl:""}),setTimeout(()=>{r({isOpen:!0,videoUrl:u,title:g,downloadUrl:w}),console.log("Video modal state updated with new content")},50)},[]),{downloadSnapshot:t}=V(),m=M(()=>{try{const{imageData:u,streamName:g}=s;if(!u){console.error("No image data available for download"),y("Download failed: No image data available","error",5e3);return}if(t){t();return}const w=new Date().toISOString().replace(/[:.]/g,"-"),C="snapshot_".concat(g,"_").concat(w,".jpg"),T=document.createElement("a");T.href=u,T.download=C,document.body.appendChild(T),T.click(),setTimeout(()=>{document.body.removeChild(T)},100),y("Snapshot downloaded successfully","success",3e3)}catch(u){console.error("Error in snapshot download process:",u),y("Download failed: "+u.message,"error",5e3)}},[s,t]),c=M(()=>{i(u=>({...u,isOpen:!1}))},[]),l=M(()=>{console.log("Closing video modal"),r(u=>({...u,isOpen:!1})),setTimeout(()=>{r({isOpen:!1,videoUrl:"",title:"",downloadUrl:""}),console.log("Video modal state fully reset")},300)},[]),v={showSnapshotPreview:o,showVideoModal:d};return e(ae.Provider,{value:v,children:[a,e(q,{isOpen:s.isOpen,onClose:c,imageData:s.imageData,streamName:s.streamName,onDownload:m}),e(z,{isOpen:h.isOpen,onClose:l,videoUrl:h.videoUrl,title:h.title,downloadUrl:h.downloadUrl})]})}function de(){console.warn("setupModals() is deprecated. Use <ModalProvider> component instead.")}function ce(){console.warn("addModalStyles() is deprecated. Modal styles are now included in components.css")}function me(a,s,i){if(console.warn("Direct showVideoModal() is deprecated. Use ModalContext.showVideoModal instead."),window.__modalContext&&window.__modalContext.showVideoModal){console.log("Using modal context to show video modal"),window.__modalContext.showVideoModal(a,s,i);return}if(console.log("Falling back to direct modal rendering"),!document.getElementById("modal-root")){console.log("Creating modal root element");const o=document.createElement("div");o.id="modal-root",document.body.appendChild(o)}const r=document.createElement("div");r.id="temp-modal-container",document.body.appendChild(r),console.log("Dynamically importing preact to render modal"),B(async()=>{const{render:o,h:d}=await import("./query-client-BSpadhku.js").then(t=>t.p);return{render:o,h:d}},__vite__mapDeps([0,1]),import.meta.url).then(({render:o,h:d})=>{console.log("Rendering VideoModal component"),o(d(z,{isOpen:!0,onClose:()=>{console.log("Closing modal"),o(null,r),document.body.removeChild(r)},videoUrl:a,title:s,downloadUrl:i}),r)}).catch(o=>{console.error("Error rendering video modal:",o),y("Error showing video modal: "+o.message,"error")})}function re(a,s){if(console.warn("Direct showSnapshotPreview() is deprecated. Use ModalContext.showSnapshotPreview instead."),window.__modalContext&&window.__modalContext.showSnapshotPreview){window.__modalContext.showSnapshotPreview(a,s);return}if(!document.getElementById("modal-root")){const r=document.createElement("div");r.id="modal-root",document.body.appendChild(r)}const h=document.createElement("div");h.id="temp-snapshot-container",document.body.appendChild(h),B(async()=>{const{render:r,h:o}=await import("./query-client-BSpadhku.js").then(d=>d.p);return{render:r,h:o}},__vite__mapDeps([0,1]),import.meta.url).then(({render:r,h:o})=>{r(o(q,{isOpen:!0,onClose:()=>{r(null,h),document.body.removeChild(h)},imageData:a,streamName:s,onDownload:()=>{try{B(async()=>{const{useSnapshotManager:t}=await Promise.resolve().then(()=>ne);return{useSnapshotManager:t}},void 0,import.meta.url).then(({useSnapshotManager:t})=>{const{downloadSnapshot:m}=t();if(m)m();else{const c=new Date().toISOString().replace(/[:.]/g,"-"),l="snapshot_".concat(s,"_").concat(c,".jpg"),v=document.createElement("a");v.href=a,v.download=l,document.body.appendChild(v),v.click(),setTimeout(()=>{document.body.removeChild(v)},100),y("Snapshot downloaded successfully","success",3e3)}}).catch(t=>{console.error("Error importing SnapshotManager:",t);const m=new Date().toISOString().replace(/[:.]/g,"-"),c="snapshot_".concat(s,"_").concat(m,".jpg"),l=document.createElement("a");l.href=a,l.download=c,document.body.appendChild(l),l.click(),setTimeout(()=>{document.body.removeChild(l)},100),y("Snapshot downloaded successfully","success",3e3)})}catch(t){console.error("Error downloading snapshot:",t),y("Download failed: "+t.message,"error",5e3)}}}),h)}).catch(r=>{console.error("Error rendering snapshot modal:",r),y("Error showing snapshot preview: "+r.message,"error")})}export{le as D,ae as M,oe as S,de as a,ce as b,te as c,me as d,ie as e,re as s,V as u};
//# sourceMappingURL=UI-BTDpQQFR.js.map
