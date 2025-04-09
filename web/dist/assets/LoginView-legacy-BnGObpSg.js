System.register(["./preact-app-legacy-BfZz7QfI.js"],(function(e,t){"use strict";var r,a,o,n;return{setters:[e=>{r=e.d,a=e.A,o=e.h,n=e._}],execute:function(){function s(){const[e,t]=r(""),[n,s]=r(""),[i,d]=r(!1),[l,c]=r("");r((()=>{const e=new URLSearchParams(window.location.search);e.has("error")?c("Invalid username or password"):e.has("auth_required")&&c("Authentication required. Please log in to continue.")})),a(null);const u=()=>{const e=new URLSearchParams(window.location.search).get("redirect");window.location.href=e||"/index.html?t="+(new Date).getTime()};return o`
    <section id="login-page" class="page flex items-center justify-center min-h-screen">
      <div class="login-container w-full max-w-md p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
        <div class="text-center mb-8">
          <h1 class="text-2xl font-bold">LightNVR</h1>
          <p class="text-gray-600 dark:text-gray-400">Please sign in to continue</p>
        </div>
        
        ${l&&o`
          <div class="mb-4 p-3 bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200 rounded-lg">
            ${l}
          </div>
        `}
        
        <form id="login-form" class="space-y-6" action="/api/auth/login" method="POST" onSubmit=${async t=>{if(t.preventDefault(),e&&n){d(!0),c("");try{const t=btoa(`${e}:${n}`);localStorage.setItem("auth",t);const r=await fetch("/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json",Authorization:`Basic ${t}`},body:JSON.stringify({username:e,password:n}),timeout:1e4});r.ok||302===r.status?u():(d(!1),c("Invalid username or password"),localStorage.removeItem("auth"))}catch(r){console.error("Login error:",r),"Request timed out"===r.message&&localStorage.getItem("auth")?(console.log("Login request timed out, proceeding with stored credentials"),u()):localStorage.getItem("auth")?(console.log("Login API error, but proceeding with stored credentials"),u()):(d(!1),c("Login failed. Please try again."))}}else c("Please enter both username and password")}}>
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
              onChange=${e=>s(e.target.value)}
              required
              autocomplete="current-password"
            />
          </div>
          <div class="form-group">
            <button 
              type="submit" 
              class="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              disabled=${i}
            >
              ${i?"Signing in...":"Sign In"}
            </button>
          </div>
        </form>
        
        <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          <p>Default credentials: admin / admin</p>
          <p class="mt-2">You can change these in Settings after login</p>
        </div>
      </div>
    </section>
  `}e({LoginView:s,loadLoginView:function(){const e=document.getElementById("main-content");e&&n((async()=>{const{render:e}=await t.import("./preact-app-legacy-BfZz7QfI.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(o`<${s} />`,e)}))}})}}}));
//# sourceMappingURL=LoginView-legacy-BnGObpSg.js.map
