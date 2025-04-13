System.register(["./preact-app-legacy-BE1jw--t.js"],(function(e,t){"use strict";var r,o,a,s;return{setters:[e=>{r=e.d,o=e.A,a=e.h,s=e._}],execute:function(){function n(){const[e,t]=r(""),[s,n]=r(""),[i,d]=r(!1),[l,u]=r("");r((()=>{const e=new URLSearchParams(window.location.search);e.has("error")?u("Invalid username or password"):e.has("auth_required")&&e.has("logout")?u("You have been successfully logged out."):e.has("auth_required")?u("Authentication required. Please log in to continue."):e.has("logout")&&u("You have been successfully logged out.")})),o(null);const c=()=>{const e=new URLSearchParams(window.location.search).get("redirect");window.location.href=e||"/index.html?t="+(new Date).getTime()};return a`
    <section id="login-page" class="page flex items-center justify-center min-h-screen">
      <div class="login-container w-full max-w-md p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
        <div class="text-center mb-8">
          <h1 class="text-2xl font-bold">LightNVR</h1>
          <p class="text-gray-600 dark:text-gray-400">Please sign in to continue</p>
        </div>
        
        ${l&&a`
          <div class=${"mb-4 p-3 rounded-lg "+(l.includes("successfully logged out")?"bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200":"bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200")}>
            ${l}
          </div>
        `}
        
        <form id="login-form" class="space-y-6" action="/api/auth/login" method="POST" onSubmit=${async t=>{if(t.preventDefault(),e&&s){d(!0),u("");try{const t=btoa(`${e}:${s}`);localStorage.setItem("auth",t);const r=await fetch("/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json",Authorization:`Basic ${t}`},body:JSON.stringify({username:e,password:s}),timeout:1e4});r.ok||302===r.status?c():(d(!1),u("Invalid username or password"),localStorage.removeItem("auth"))}catch(r){console.error("Login error:",r),"Request timed out"===r.message&&localStorage.getItem("auth")?(console.log("Login request timed out, proceeding with stored credentials"),c()):localStorage.getItem("auth")?(console.log("Login API error, but proceeding with stored credentials"),c()):(d(!1),u("Login failed. Please try again."))}}else u("Please enter both username and password")}}>
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
              value=${s}
              onChange=${e=>n(e.target.value)}
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
  `}e({LoginView:n,loadLoginView:function(){const e=document.getElementById("main-content");e&&s((async()=>{const{render:e}=await t.import("./preact-app-legacy-BE1jw--t.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(a`<${n} />`,e)}))}})}}}));
//# sourceMappingURL=LoginView-legacy-dXnvRBIH.js.map
