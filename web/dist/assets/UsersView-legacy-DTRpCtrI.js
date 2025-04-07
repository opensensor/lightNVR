System.register(["./preact-app-legacy-CfMi5fXv.js","./fetch-utils-legacy-DOz1-Xee.js"],(function(e,t){"use strict";var r,a,s,o,n,l,d,i,c,u;return{setters:[e=>{r=e.h,a=e.d,s=e.A,o=e.q,n=e.y,l=e._,d=e.c},e=>{i=e.c,c=e.f,u=e.e}],execute:function(){e({UsersView:v,loadUsersView:function(){const e=document.getElementById("users-container");e&&l((async()=>{const{render:e}=await t.import("./preact-app-legacy-CfMi5fXv.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:t})=>{t(r`<${v} />`,e)}))}});const b={0:"Admin",1:"User",2:"Viewer",3:"API"};function p({users:e,onEdit:t,onDelete:a,onApiKey:s}){return r`
    <div class="overflow-x-auto">
      <table class="w-full border-collapse">
        <thead class="bg-gray-50 dark:bg-gray-700">
          <tr>
            <th class="py-3 px-6 text-left font-semibold">ID</th>
            <th class="py-3 px-6 text-left font-semibold">Username</th>
            <th class="py-3 px-6 text-left font-semibold">Email</th>
            <th class="py-3 px-6 text-left font-semibold">Role</th>
            <th class="py-3 px-6 text-left font-semibold">Status</th>
            <th class="py-3 px-6 text-left font-semibold">Last Login</th>
            <th class="py-3 px-6 text-left font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
          ${e.map((e=>r`
            <tr key=${e.id} class="hover:bg-gray-100 dark:hover:bg-gray-600">
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${e.id}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${e.username}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${e.email||"-"}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${b[e.role]||"Unknown"}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">
                <span class=${"inline-block px-2 py-1 text-xs font-semibold rounded-full "+(e.is_active?"bg-green-100 text-green-800":"bg-red-100 text-red-800")}>
                  ${e.is_active?"Active":"Inactive"}
                </span>
              </td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${e.last_login?new Date(1e3*e.last_login).toLocaleString():"Never"}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">
                <div class="flex space-x-2">
                  <button 
                    class="p-1 text-blue-600 hover:text-blue-800 rounded hover:bg-blue-100 transition-colors"
                    onClick=${r=>((e,r)=>{r.preventDefault(),r.stopPropagation(),t(e)})(e,r)}
                    title="Edit User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button 
                    class="p-1 text-red-600 hover:text-red-800 rounded hover:bg-red-100 transition-colors"
                    onClick=${t=>((e,t)=>{t.preventDefault(),t.stopPropagation(),a(e)})(e,t)}
                    title="Delete User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                  <button 
                    class="p-1 text-gray-600 hover:text-gray-800 rounded hover:bg-gray-100 transition-colors"
                    onClick=${t=>((e,t)=>{t.preventDefault(),t.stopPropagation(),s(e)})(e,t)}
                    title="Manage API Key"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          `))}
        </tbody>
      </table>
    </div>
  `}function g({formData:e,handleInputChange:t,handleAddUser:a,onClose:s}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${s}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">Add New User</h2>
        
        <form onSubmit=${e=>{e.preventDefault(),e.stopPropagation(),a(e)}}>
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="username">
              Username
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="username"
              type="text"
              name="username"
              value=${e.username}
              onChange=${t}
              required
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="password">
              Password
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="password"
              type="password"
              name="password"
              value=${e.password}
              onChange=${t}
              required
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="email">
              Email
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="email"
              type="email"
              name="email"
              value=${e.email}
              onChange=${t}
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="role">
              Role
            </label>
            <select
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="role"
              name="role"
              value=${e.role}
              onChange=${t}
            >
              ${Object.entries(b).map((([e,t])=>r`
                <option value=${e}>${t}</option>
              `))}
            </select>
          </div>
          
          <div class="mb-6">
            <label class="flex items-center">
              <input
                type="checkbox"
                name="is_active"
                checked=${e.is_active}
                onChange=${t}
                class="mr-2"
              />
              <span class="text-sm font-bold">Active</span>
            </label>
          </div>
          
          <div class="flex justify-end mt-6">
            <button
              type="button"
              class="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2"
              onClick=${s}
            >
              Cancel
            </button>
            <button
              type="submit"
              class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Add User
            </button>
          </div>
        </form>
      </div>
    </div>
  `}function y({currentUser:e,formData:t,handleInputChange:a,handleEditUser:s,onClose:o}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${o}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">Edit User: ${e.username}</h2>
        
        <form onSubmit=${e=>{e.preventDefault(),e.stopPropagation(),s(e)}}>
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="username">
              Username
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="username"
              type="text"
              name="username"
              value=${t.username}
              onChange=${a}
              required
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="password">
              Password (leave blank to keep current)
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="password"
              type="password"
              name="password"
              value=${t.password}
              onChange=${a}
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="email">
              Email
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="email"
              type="email"
              name="email"
              value=${t.email}
              onChange=${a}
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-sm font-bold mb-2" for="role">
              Role
            </label>
            <select
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="role"
              name="role"
              value=${t.role}
              onChange=${a}
            >
              ${Object.entries(b).map((([e,t])=>r`
                <option value=${e}>${t}</option>
              `))}
            </select>
          </div>
          
          <div class="mb-6">
            <label class="flex items-center">
              <input
                type="checkbox"
                name="is_active"
                checked=${t.is_active}
                onChange=${a}
                class="mr-2"
              />
              <span class="text-sm font-bold">Active</span>
            </label>
          </div>
          
          <div class="flex justify-end mt-6">
            <button
              type="button"
              class="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2"
              onClick=${o}
            >
              Cancel
            </button>
            <button
              type="submit"
              class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Update User
            </button>
          </div>
        </form>
      </div>
    </div>
  `}function h({currentUser:e,handleDeleteUser:t,onClose:a}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${a}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">Delete User</h2>
        
        <p class="mb-6">
          Are you sure you want to delete the user "${e.username}"? This action cannot be undone.
        </p>
        
        <div class="flex justify-end">
          <button
            class="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2"
            onClick=${a}
          >
            Cancel
          </button>
          <button
            class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
            onClick=${e=>{e.stopPropagation(),t()}}
          >
            Delete User
          </button>
        </div>
      </div>
    </div>
  `}function m({currentUser:e,newApiKey:t,handleGenerateApiKey:a,copyApiKey:s,onClose:o}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${o}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">API Key for ${e.username}</h2>
        
        <div class="mb-6">
          ${t?r`
            <div class="mb-4">
              <label class="block text-sm font-bold mb-2">
                API Key
              </label>
              <div class="flex">
                <input
                  class="shadow appearance-none border rounded-l w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                  type="text"
                  value=${t}
                  readonly
                />
                <button
                  class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-r"
                  onClick=${s}
                >
                  Copy
                </button>
              </div>
              <p class="text-sm text-gray-600 dark:text-gray-300 mt-2">
                This key will only be shown once. Save it securely.
              </p>
            </div>
          `:r`
            <p class="mb-4">
              Generate a new API key for this user. This will invalidate any existing API key.
            </p>
            <button
              class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
              onClick=${a}
            >
              Generate New API Key
            </button>
          `}
        </div>
        
        <div class="flex justify-end">
          <button
            class="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500"
            onClick=${o}
          >
            Close
          </button>
        </div>
      </div>
    </div>
  `}function v(){const[e,t]=a([]),[l,b]=a(!0),[v,x]=a(null),[f,w]=a(null),[k,$]=a(null),[C,A]=a(""),[U,P]=a({username:"",password:"",email:"",role:1,is_active:!0}),E=s(null),D=o((()=>{const e=localStorage.getItem("auth");return e?{Authorization:"Basic "+e}:{}}),[]),I=o((async()=>{b(!0);try{const e=await c("/api/auth/users",{headers:D(),cache:"no-store",signal:E.current?.signal,timeout:15e3,retries:2,retryDelay:1e3});t(e.users||[]),x(null)}catch(v){"Request was cancelled"!==v.message&&(console.error("Error fetching users:",v),x("Failed to load users. Please try again."))}finally{b(!1)}}),[D]);n((()=>(E.current=i(),I(),()=>{E.current&&E.current.abort()})),[I]),n((()=>{const e=document.getElementById("add-user-btn");if(e){const t=()=>{P({username:"",password:"",email:"",role:1,is_active:!0}),w("add")};return e.addEventListener("click",t),()=>{e&&e.removeEventListener("click",t)}}}),[]);const j=o((e=>{const{name:t,value:r,type:a,checked:s}=e.target;P((e=>({...e,[t]:"checkbox"===a?s:"role"===t?parseInt(r,10):r})))}),[]),K=o((async e=>{e&&e.preventDefault();try{console.log("Adding user:",U.username),await c("/api/auth/users",{method:"POST",headers:{"Content-Type":"application/json",...D()},body:JSON.stringify(U),signal:E.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),w(null),d("User added successfully","success",5e3),await I()}catch(t){console.error("Error adding user:",t),d(`Error adding user: ${t.message}`,"error",8e3)}}),[U,D,I]),T=o((async e=>{e&&e.preventDefault();try{console.log("Editing user:",k.id,k.username),await c(`/api/auth/users/${k.id}`,{method:"PUT",headers:{"Content-Type":"application/json",...D()},body:JSON.stringify(U),signal:E.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),w(null),d("User updated successfully","success",5e3),await I()}catch(t){console.error("Error updating user:",t),d(`Error updating user: ${t.message}`,"error",8e3)}}),[k,U,D,I]),_=o((async()=>{try{console.log("Deleting user:",k.id,k.username),await u(`/api/auth/users/${k.id}`,{method:"DELETE",headers:D(),signal:E.current?.signal,timeout:15e3,retries:1,retryDelay:1e3}),w(null),d("User deleted successfully","success",5e3),await I()}catch(e){console.error("Error deleting user:",e),d(`Error deleting user: ${e.message}`,"error",8e3)}}),[k,D,I]),S=o((async()=>{A("Generating...");try{console.log("Generating API key for user:",k.id,k.username);const e=await c(`/api/auth/users/${k.id}/api-key`,{method:"POST",headers:D(),signal:E.current?.signal,timeout:2e4,retries:1,retryDelay:2e3});A(e.api_key),d("API key generated successfully","success"),await I()}catch(e){console.error("Error generating API key:",e),A(""),d(`Error generating API key: ${e.message}`,"error")}}),[k,D,I]),L=o((()=>{navigator.clipboard.writeText(C).then((()=>{window.showSuccessToast?window.showSuccessToast("API key copied to clipboard"):d("API key copied to clipboard","success")})).catch((e=>{console.error("Error copying API key:",e),window.showErrorToast?window.showErrorToast("Failed to copy API key"):d("Failed to copy API key","error")}))}),[C]),z=o((e=>{$(e),P({username:e.username,password:"",email:e.email||"",role:e.role,is_active:e.is_active}),w("edit")}),[]),B=o((e=>{$(e),w("delete")}),[]),G=o((e=>{$(e),A(""),w("apiKey")}),[]),H=o((()=>{w(null)}),[]);return l&&0===e.length?r`
      <div class="flex justify-center items-center p-8">
        <div class="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
        <span class="sr-only">Loading...</span>
      </div>
    `:0!==e.length||l?r`
    <div>
      <${p} 
        users=${e} 
        onEdit=${z} 
        onDelete=${B} 
        onApiKey=${G} 
      />
      
      ${"add"===f&&r`<${g} 
        formData=${U} 
        handleInputChange=${j} 
        handleAddUser=${K} 
        onClose=${H} 
      />`}
      
      ${"edit"===f&&r`<${y} 
        currentUser=${k} 
        formData=${U} 
        handleInputChange=${j} 
        handleEditUser=${T} 
        onClose=${H} 
      />`}
      
      ${"delete"===f&&r`<${h} 
        currentUser=${k} 
        handleDeleteUser=${_} 
        onClose=${H} 
      />`}
      
      ${"apiKey"===f&&r`<${m} 
        currentUser=${k} 
        newApiKey=${C} 
        handleGenerateApiKey=${S} 
        copyApiKey=${L} 
        onClose=${H} 
      />`}
    </div>
  `:r`
      <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4">
        <h4 class="font-bold mb-2">No Users Found</h4>
        <p>Click the "Add User" button to create your first user.</p>
      </div>
      ${"add"===f&&r`<${g} 
        formData=${U} 
        handleInputChange=${j} 
        handleAddUser=${K} 
        onClose=${H} 
      />`}
    `}}}}));
//# sourceMappingURL=UsersView-legacy-DTRpCtrI.js.map
