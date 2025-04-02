System.register(["./preact-app-legacy-D5_kcW1v.js","./fetch-utils-legacy-DOz1-Xee.js"],(function(e,t){"use strict";var r,o,a,n,s,i;return{setters:[e=>{r=e.d,o=e.A,a=e.h,n=e._},e=>{s=e.c,i=e.e}],execute:function(){function d(){const[e,t]=r(""),[n,d]=r(""),[l,c]=r(!1),[u,g]=r("");r((()=>{const e=new URLSearchParams(window.location.search);e.has("error")?g("Invalid username or password"):e.has("auth_required")&&g("Authentication required. Please log in to continue.")}));const m=o(null);return a`
    <section id="login-page" class="page flex items-center justify-center min-h-screen">
      <div class="login-container w-full max-w-md p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
        <div class="text-center mb-8">
          <h1 class="text-2xl font-bold">LightNVR</h1>
          <p class="text-gray-600 dark:text-gray-400">Please sign in to continue</p>
        </div>
        
        ${u&&a`
          <div class="mb-4 p-3 bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200 rounded-lg">
            ${u}
          </div>
        `}
        
        <form id="login-form" class="space-y-6" action="/api/auth/login" method="POST" onSubmit=${async t=>{if(t.preventDefault(),!e||!n)return void g("Please enter both username and password");c(!0),m.current=s();const r=btoa(`${e}:${n}`);localStorage.setItem("auth",r);try{const t=await i("/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json",Authorization:"Basic "+r,"X-Requested-With":"XMLHttpRequest",Accept:"application/json"},body:JSON.stringify({username:e,password:n}),credentials:"include",mode:"same-origin",signal:m.current?.signal,timeout:5e3,retries:1,retryDelay:1e3});t.ok||302===t.status?window.location.href="/index.html?t="+(new Date).getTime():(c(!1),g("Invalid username or password"),localStorage.removeItem("auth"))}catch(o){console.error("Login error:",o),"Request timed out"===o.message&&localStorage.getItem("auth")?(console.log("Login request timed out, proceeding with stored credentials"),window.location.href="/index.html?t="+(new Date).getTime()):localStorage.getItem("auth")?(console.log("Login API error, but proceeding with stored credentials"),window.location.href="/index.html?t="+(new Date).getTime()):(c(!1),g("Login failed. Please try again."))}}}>
          <div class="form-group">
            <label for="username" class="block text-sm font-medium mb-1">Username</label>
            <input 
              type="text" 
              id="username" 
              name="username"
              class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              placeholder="Enter your username" 
              value=${e}
              onChange=${e=>t(e.target.value)}
              required
              autocomplete="username"
            />
          </div>
          <div class="form-group">
            <label for="password" class="block text-sm font-medium mb-1">Password</label>
            <input 
              type="password" 
              id="password" 
              name="password"
              class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              placeholder="Enter your password" 
              value=${n}
              onChange=${e=>d(e.target.value)}
              required
              autocomplete="current-password"
            />
          </div>
          <div class="form-group">
            <button 
              type="submit" 
              class="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              disabled=${l}
            >
              ${l?"Signing in...":"Sign In"}
            </button>
          </div>
        </form>
        
        <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          <p>Default credentials: admin / admin</p>
          <p class="mt-2">You can change these in Settings after login</p>
        </div>
      </div>
    </section>
  `}e({LoginView:d,loadLoginView:function(){const e=document.getElementById("main-content");e&&n((async()=>{const{render:e}=await t.import("./preact-app-legacy-D5_kcW1v.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(a`<${d} />`,e)}))}})}}}));
//# sourceMappingURL=LoginView-legacy-BjPb07cd.js.map
