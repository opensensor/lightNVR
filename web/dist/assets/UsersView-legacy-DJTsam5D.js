System.register(["./preact-app-legacy-D-e9SRIZ.js"],(function(e,t){"use strict";var r,a,o,s,n,l,d,i,c;return{setters:[e=>{r=e.h,a=e.d,o=e.q,s=e.u,n=e.y,l=e.j,d=e.c,i=e._,c=e.f}],execute:function(){e({UsersView:m,loadUsersView:function(){const e=document.getElementById("users-container");e&&i((async()=>{const{render:e}=await t.import("./preact-app-legacy-D-e9SRIZ.js").then((e=>e.p));return{render:e}}),void 0,t.meta.url).then((({render:a})=>{i((async()=>{const{QueryClientProvider:e,queryClient:r}=await t.import("./preact-app-legacy-D-e9SRIZ.js").then((e=>e.m));return{QueryClientProvider:e,queryClient:r}}),void 0,t.meta.url).then((({QueryClientProvider:t,queryClient:o})=>{a(r`<${t} client=${o}><${m} /></${t}>`,e)}))}))}});const u={0:"Admin",1:"User",2:"Viewer",3:"API"};function b({users:e,onEdit:t,onDelete:a,onApiKey:o}){return r`
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
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${u[e.role]||"Unknown"}</td>
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
                    onClick=${t=>((e,t)=>{t.preventDefault(),t.stopPropagation(),o(e)})(e,t)}
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
  `}function p({formData:e,handleInputChange:t,handleAddUser:a,onClose:o}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${o}>
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
              ${Object.entries(u).map((([e,t])=>r`
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
              onClick=${o}
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
  `}function g({currentUser:e,formData:t,handleInputChange:a,handleEditUser:o,onClose:s}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${s}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">Edit User: ${e.username}</h2>
        
        <form onSubmit=${e=>{e.preventDefault(),e.stopPropagation(),o(e)}}>
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
              ${Object.entries(u).map((([e,t])=>r`
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
              onClick=${s}
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
  `}function y({currentUser:e,handleDeleteUser:t,onClose:a}){return r`
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
  `}function h({currentUser:e,newApiKey:t,handleGenerateApiKey:a,copyApiKey:o,onClose:s}){return r`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${s}>
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
                  onClick=${o}
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
            onClick=${s}
          >
            Close
          </button>
        </div>
      </div>
    </div>
  `}function m(){const[e,t]=a(null),[i,u]=a(null),[m,v]=a(""),[x,f]=a({username:"",password:"",email:"",role:1,is_active:!0}),w=o((()=>{const e=localStorage.getItem("auth");return e?{Authorization:"Basic "+e}:{}}),[]),{data:k,isLoading:$,error:C,refetch:A}=s(["users"],"/api/auth/users",{headers:w(),cache:"no-store",timeout:15e3,retries:2,retryDelay:1e3}),U=k?.users||[];n((()=>{const e=document.getElementById("add-user-btn");if(e){const r=()=>{f({username:"",password:"",email:"",role:1,is_active:!0}),t("add")};return e.addEventListener("click",r),()=>{e&&e.removeEventListener("click",r)}}}),[]);const P=o((e=>{const{name:t,value:r,type:a,checked:o}=e.target;f((e=>({...e,[t]:"checkbox"===a?o:"role"===t?parseInt(r,10):r})))}),[]),E=l({mutationFn:async e=>await c("/api/auth/users",{method:"POST",headers:{"Content-Type":"application/json",...w()},body:JSON.stringify(e),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{t(null),d("User added successfully","success",5e3),A()},onError:e=>{console.error("Error adding user:",e),d(`Error adding user: ${e.message}`,"error",8e3)}}),D=l({mutationFn:async({userId:e,userData:t})=>await c(`/api/auth/users/${e}`,{method:"PUT",headers:{"Content-Type":"application/json",...w()},body:JSON.stringify(t),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{t(null),d("User updated successfully","success",5e3),A()},onError:e=>{console.error("Error updating user:",e),d(`Error updating user: ${e.message}`,"error",8e3)}}),I=l({mutationFn:async e=>await c(`/api/auth/users/${e}`,{method:"DELETE",headers:w(),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{t(null),d("User deleted successfully","success",5e3),A()},onError:e=>{console.error("Error deleting user:",e),d(`Error deleting user: ${e.message}`,"error",8e3)}}),j=l({mutationFn:async e=>await c(`/api/auth/users/${e}/api-key`,{method:"POST",headers:w(),timeout:2e4,retries:1,retryDelay:2e3}),onMutate:()=>{v("Generating...")},onSuccess:e=>{v(e.api_key),d("API key generated successfully","success"),A()},onError:e=>{console.error("Error generating API key:",e),v(""),d(`Error generating API key: ${e.message}`,"error")}}),S=o((e=>{e&&e.preventDefault(),console.log("Adding user:",x.username),E.mutate(x)}),[x]),K=o((e=>{e&&e.preventDefault(),console.log("Editing user:",i.id,i.username),D.mutate({userId:i.id,userData:x})}),[i,x]),T=o((()=>{console.log("Deleting user:",i.id,i.username),I.mutate(i.id)}),[i]),_=o((()=>{console.log("Generating API key for user:",i.id,i.username),j.mutate(i.id)}),[i]),L=o((()=>{navigator.clipboard.writeText(m).then((()=>{window.showSuccessToast?window.showSuccessToast("API key copied to clipboard"):d("API key copied to clipboard","success")})).catch((e=>{console.error("Error copying API key:",e),window.showErrorToast?window.showErrorToast("Failed to copy API key"):d("Failed to copy API key","error")}))}),[m]),q=o((e=>{u(e),f({username:e.username,password:"",email:e.email||"",role:e.role,is_active:e.is_active}),t("edit")}),[]),z=o((e=>{u(e),t("delete")}),[]),F=o((e=>{u(e),v(""),t("apiKey")}),[]),B=o((()=>{t(null)}),[]);return $&&0===U.length?r`
      <div class="flex justify-center items-center p-8">
        <div class="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
        <span class="sr-only">Loading...</span>
      </div>
    `:0!==U.length||$?r`
    <div>
      <${b} 
        users=${U} 
        onEdit=${q} 
        onDelete=${z} 
        onApiKey=${F} 
      />
      
      ${"add"===e&&r`<${p} 
        formData=${x} 
        handleInputChange=${P} 
        handleAddUser=${S} 
        onClose=${B} 
      />`}
      
      ${"edit"===e&&r`<${g} 
        currentUser=${i} 
        formData=${x} 
        handleInputChange=${P} 
        handleEditUser=${K} 
        onClose=${B} 
      />`}
      
      ${"delete"===e&&r`<${y} 
        currentUser=${i} 
        handleDeleteUser=${T} 
        onClose=${B} 
      />`}
      
      ${"apiKey"===e&&r`<${h} 
        currentUser=${i} 
        newApiKey=${m} 
        handleGenerateApiKey=${_} 
        copyApiKey=${L} 
        onClose=${B} 
      />`}
    </div>
  `:r`
      <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4">
        <h4 class="font-bold mb-2">No Users Found</h4>
        <p>Click the "Add User" button to create your first user.</p>
      </div>
      ${"add"===e&&r`<${p} 
        formData=${x} 
        handleInputChange=${P} 
        handleAddUser=${S} 
        onClose=${B} 
      />`}
    `}}}}));
//# sourceMappingURL=UsersView-legacy-DJTsam5D.js.map
