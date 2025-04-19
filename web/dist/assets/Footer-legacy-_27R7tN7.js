System.register(["./query-client-legacy-BljCN6fF.js"],(function(e,t){"use strict";var n,l,r;return{setters:[e=>{n=e.d,l=e.y,r=e.u}],execute:function(){e({F:function(){const[e]=n((new Date).getFullYear());return r("footer",{class:"bg-gray-800 text-white py-3 px-4 mt-4 shadow-inner",children:r("div",{class:"container mx-auto flex flex-col sm:flex-row justify-between items-center",children:r("div",{class:"text-center sm:text-left mb-2 sm:mb-0",children:[r("p",{class:"text-sm",children:["© ",e," LightNVR ·",r("a",{href:"https://github.com/opensensor/lightnvr",class:"text-blue-300 hover:text-blue-100 text-sm",target:"_blank",rel:"noopener noreferrer",children:"GitHub"})]}),r("p",{class:"text-xs text-gray-400 mt-1 hidden sm:block",children:"Lightweight Network Video Recorder"})]})})})},H:function({version:e=t}){const i=document.getElementById("header-container"),o=i?.dataset?.activeNav||"",[s,a]=n(""),[c,d]=n(!1);l((()=>{const e=localStorage.getItem("auth");if(e)try{const t=atob(e).split(":")[0];a(t)}catch(t){console.error("Error decoding auth token:",t),a("User")}else a("User")}),[]);const h=e=>{e.preventDefault(),localStorage.removeItem("auth"),document.cookie="auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict",document.cookie="session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict",fetch("/api/auth/logout",{method:"POST"}).then((()=>{window.location.href="login.html?auth_required=true&logout=true"})).catch((()=>{window.location.href="login.html?auth_required=true&logout=true"}))},m=()=>{d(!c)},u=[{id:"nav-live",href:"/"===window.location.pathname||window.location.pathname.endsWith("/")?"./":"index.html",label:"Live View"},{id:"nav-recordings",href:"recordings.html",label:"Recordings"},{id:"nav-streams",href:"streams.html",label:"Streams"},{id:"nav-settings",href:"settings.html",label:"Settings"},{id:"nav-users",href:"users.html",label:"Users"},{id:"nav-system",href:"system.html",label:"System"}],x=e=>{const t=o===e.id?"bg-blue-600":"hover:bg-blue-700";return r("li",{className:c?"w-full":"mx-1",children:r("a",{href:e.href,id:e.id,className:`text-white no-underline rounded transition-colors ${c?"block w-full px-4 py-3 text-left":"px-3 py-2"} ${t}`,onClick:t=>{((e,t)=>{t&&(t.preventDefault(),t.stopPropagation()),setTimeout((()=>{window.location.href=e}),0)})(e.href,t),c&&m()},children:e.label})})};return r("header",{className:"bg-gray-800 text-white py-2 shadow-md mb-4 w-full",style:{position:"relative",zIndex:20},children:[r("div",{className:"container mx-auto px-4 flex justify-between items-center",children:[r("div",{className:"logo flex items-center",children:[r("h1",{className:"text-xl font-bold m-0",children:"LightNVR"}),r("span",{className:"version text-blue-200 text-xs ml-2",children:["v",e]})]}),r("nav",{className:"hidden md:block",style:{position:"relative",zIndex:20},children:r("ul",{className:"flex list-none m-0 p-0",children:u.map(x)})}),r("div",{className:"user-menu hidden md:flex items-center",children:[r("span",{className:"mr-2",children:s}),r("a",{href:"#",onClick:h,className:"logout-link text-white no-underline hover:bg-blue-700 px-3 py-1 rounded transition-colors",children:"Logout"})]}),r("button",{className:"md:hidden text-white p-2 focus:outline-none",onClick:m,"aria-label":"Toggle menu",children:r("svg",{xmlns:"http://www.w3.org/2000/svg",className:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor",children:r("path",{strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:"2",d:c?"M6 18L18 6M6 6l12 12":"M4 6h16M4 12h16M4 18h16"})})})]}),c&&r("div",{className:"md:hidden mt-2 border-t border-gray-700 pt-2 container mx-auto px-4",children:r("ul",{className:"list-none m-0 p-0 flex flex-col w-full",children:[u.map(x),r("li",{className:"w-full mt-2 pt-2 border-t border-gray-700",children:r("div",{className:"flex justify-between items-center px-4 py-2",children:[r("span",{children:s}),r("a",{href:"#",onClick:h,className:"logout-link text-white no-underline hover:bg-blue-700 px-3 py-1 rounded transition-colors",children:"Logout"})]})})]})})]})}});const t="0.11.19"}}}));
//# sourceMappingURL=Footer-legacy-_27R7tN7.js.map
