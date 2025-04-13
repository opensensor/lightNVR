System.register(["./preact-app-legacy-CG_QZm6V.js"],(function(e,t){"use strict";var o,r,n,a,s,i;return{setters:[e=>{o=e.d,r=e.A,n=e.y,a=e.h,s=e._,i=e.s}],execute:function(){function l(){const[e,t]=o(""),[s,l]=o(""),[c,d]=o(!1),[u,g]=o(""),[m,h]=o(0),p=r(null);n((()=>{const e=new URLSearchParams(window.location.search);e.has("error")?g("Invalid username or password"):e.has("auth_required")&&e.has("logout")?g("You have been successfully logged out."):e.has("auth_required")?g("Authentication required. Please log in to continue."):e.has("logout")&&g("You have been successfully logged out.")}),[]),r(null),n((()=>()=>{p.current&&clearTimeout(p.current)}),[]);const b=()=>{const e=new URLSearchParams(window.location.search).get("redirect"),t=(new Date).getTime(),o=e?`${e}${e.includes("?")?"&":"?"}t=${t}`:`/index.html?t=${t}`;console.log(`Login successful, redirecting to: ${o}`);try{window.location.href=o}catch(r){console.error("Error redirecting via location.href:",r)}p.current=setTimeout((()=>{window.location.pathname.includes("login.html")&&(console.log("Still on login page, trying alternate redirection method"),h((e=>{const t=e+1;if(t<=3)try{window.location.assign(o),p.current=setTimeout((()=>{window.location.pathname.includes("login.html")&&(console.log("Still on login page after assign, trying location.replace"),window.location.replace(o))}),1e3)}catch(r){if(console.error("Error during alternate redirection:",r),t>=3){i('Please click the "Go to Dashboard" button to continue',"info",1e4),g("Login successful! Click the button below to continue.");const e=document.createElement("button");e.textContent="Go to Dashboard",e.className="w-full px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors",e.onclick=()=>{window.location=o};const t=document.getElementById("login-form");t&&t.parentNode&&t.parentNode.appendChild(e)}}return t})))}),500)};return a`
    <section id="login-page" class="page flex items-center justify-center min-h-screen">
      <div class="login-container w-full max-w-md p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
        <div class="text-center mb-8">
          <h1 class="text-2xl font-bold">LightNVR</h1>
          <p class="text-gray-600 dark:text-gray-400">Please sign in to continue</p>
        </div>

        ${u&&a`
          <div class=${"mb-4 p-3 rounded-lg "+(u.includes("successfully logged out")||u.includes("Click the button below")?"bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200":"bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200")}>
            ${u}
          </div>
        `}

        <form id="login-form" class="space-y-6" action="/api/auth/login" method="POST" onSubmit=${async t=>{if(t.preventDefault(),e&&s){d(!0),g("");try{const t=btoa(`${e}:${s}`);localStorage.setItem("auth",t);const o=await fetch("/api/auth/login",{method:"POST",headers:{"Content-Type":"application/json",Authorization:`Basic ${t}`},body:JSON.stringify({username:e,password:s}),timeout:1e4});o.ok||302===o.status?(console.log("Login successful, proceeding to redirect"),sessionStorage.setItem("auth_confirmed","true"),b()):(d(!1),g("Invalid username or password"),localStorage.removeItem("auth"))}catch(o){console.error("Login error:",o),"Request timed out"===o.message&&localStorage.getItem("auth")?(console.log("Login request timed out, proceeding with stored credentials"),b()):localStorage.getItem("auth")?(console.log("Login API error, but proceeding with stored credentials"),b()):(d(!1),g("Login failed. Please try again."))}}else g("Please enter both username and password")}}>
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
                onChange=${e=>l(e.target.value)}
                required
                autocomplete="current-password"
            />
          </div>
          <div class="form-group">
            <button
                type="submit"
                class="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                disabled=${c}
            >
              ${c?"Signing in...":"Sign In"}
            </button>
          </div>
        </form>

        <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          <p>Default credentials: admin / admin</p>
          <p class="mt-2">You can change these in Settings after login</p>
        </div>
      </div>
    </section>
  `}e({LoginView:l,loadLoginView:function(){const e=document.getElementById("main-content");e&&s((async()=>{const{render:e}=await t.import("./preact-app-legacy-CG_QZm6V.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(a`<${l} />`,e)}))}})}}}));
//# sourceMappingURL=LoginView-legacy-DSwuU7D-.js.map
