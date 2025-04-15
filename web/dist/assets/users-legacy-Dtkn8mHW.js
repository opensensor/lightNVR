System.register(["./query-client-legacy-BQ_U5NnD.js","./layout-legacy-CDxh3f01.js","./Footer-legacy-B5sb7m6V.js"],(function(e,t){"use strict";var r,a,o,s,n,l,d,i,c,u,b,p,g,y;return{setters:[e=>{r=e.d,a=e.h,o=e.e,s=e.y,n=e.c,l=e.f,d=e.u,i=e.E,c=e.q,u=e.Q},null,e=>{b=e.h,p=e.c,g=e.H,y=e.F}],execute:function(){const e={0:"Admin",1:"User",2:"Viewer",3:"API"};function t({users:t,onEdit:r,onDelete:a,onApiKey:o}){return b`
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
          ${t.map((t=>b`
            <tr key=${t.id} class="hover:bg-gray-100 dark:hover:bg-gray-600">
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${t.id}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${t.username}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${t.email||"-"}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${e[t.role]||"Unknown"}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">
                <span class=${"inline-block px-2 py-1 text-xs font-semibold rounded-full "+(t.is_active?"bg-green-100 text-green-800":"bg-red-100 text-red-800")}>
                  ${t.is_active?"Active":"Inactive"}
                </span>
              </td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${t.last_login?new Date(1e3*t.last_login).toLocaleString():"Never"}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">
                <div class="flex space-x-2">
                  <button 
                    class="p-1 text-blue-600 hover:text-blue-800 rounded hover:bg-blue-100 transition-colors"
                    onClick=${e=>((e,t)=>{t.preventDefault(),t.stopPropagation(),r(e)})(t,e)}
                    title="Edit User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button 
                    class="p-1 text-red-600 hover:text-red-800 rounded hover:bg-red-100 transition-colors"
                    onClick=${e=>((e,t)=>{t.preventDefault(),t.stopPropagation(),a(e)})(t,e)}
                    title="Delete User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                  <button 
                    class="p-1 text-gray-600 hover:text-gray-800 rounded hover:bg-gray-100 transition-colors"
                    onClick=${e=>((e,t)=>{t.preventDefault(),t.stopPropagation(),o(e)})(t,e)}
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
  `}function h({formData:t,handleInputChange:r,handleAddUser:a,onClose:o}){return b`
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
              value=${t.username}
              onChange=${r}
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
              value=${t.password}
              onChange=${r}
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
              value=${t.email}
              onChange=${r}
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
              onChange=${r}
            >
              ${Object.entries(e).map((([e,t])=>b`
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
                onChange=${r}
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
  `}function m({currentUser:t,formData:r,handleInputChange:a,handleEditUser:o,onClose:s}){return b`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${s}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">Edit User: ${t.username}</h2>
        
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
              value=${r.username}
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
              value=${r.password}
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
              value=${r.email}
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
              value=${r.role}
              onChange=${a}
            >
              ${Object.entries(e).map((([e,t])=>b`
                <option value=${e}>${t}</option>
              `))}
            </select>
          </div>
          
          <div class="mb-6">
            <label class="flex items-center">
              <input
                type="checkbox"
                name="is_active"
                checked=${r.is_active}
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
  `}function v({currentUser:e,handleDeleteUser:t,onClose:r}){return b`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${r}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">Delete User</h2>
        
        <p class="mb-6">
          Are you sure you want to delete the user "${e.username}"? This action cannot be undone.
        </p>
        
        <div class="flex justify-end">
          <button
            class="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2"
            onClick=${r}
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
  `}function x({currentUser:e,newApiKey:t,handleGenerateApiKey:r,copyApiKey:a,onClose:o}){return console.log("API Key Modal - newApiKey:",t),b`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${e=>{t&&"Generating..."!==t||o(e)}}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${e=>{e.stopPropagation()}}>
        <h2 class="text-xl font-bold mb-4">API Key for ${e.username}</h2>

        <div class="mb-6">
          ${t?b`
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
                  onClick=${a}
                >
                  Copy
                </button>
              </div>
              <p class="text-sm text-gray-600 dark:text-gray-300 mt-2">
                This key will only be shown once. Save it securely.
              </p>
            </div>
          `:b`
            <p class="mb-4">
              Generate a new API key for this user. This will invalidate any existing API key.
            </p>
            <button
              class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
              onClick=${r}
            >
              Generate New API Key
            </button>
          `}
        </div>

        <div class="flex justify-end">
          <button
            class="${t&&"Generating..."!==t?"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700":"px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500"}"
            onClick=${o}
          >
            ${t&&"Generating..."!==t?"Done":"Close"}
          </button>
        </div>
      </div>
    </div>
  `}function f(){const[e,i]=r(null),[c,u]=r(null),[b,g]=r(""),[y,f]=r({username:"",password:"",email:"",role:1,is_active:!0}),w=a((()=>{const e=localStorage.getItem("auth");return e?{Authorization:"Basic "+e}:{}}),[]),{data:k,isLoading:$,error:C,refetch:A}=o(["users"],"/api/auth/users",{headers:w(),cache:"no-store",timeout:15e3,retries:2,retryDelay:1e3}),E=k?.users||[];s((()=>{const e=document.getElementById("add-user-btn");if(e){const t=()=>{f({username:"",password:"",email:"",role:1,is_active:!0}),i("add")};return e.addEventListener("click",t),()=>{e&&e.removeEventListener("click",t)}}}),[]);const U=a((e=>{const{name:t,value:r,type:a,checked:o}=e.target;f((e=>({...e,[t]:"checkbox"===a?o:"role"===t?parseInt(r,10):r})))}),[]),D=n({mutationFn:async e=>await l("/api/auth/users",{method:"POST",headers:{"Content-Type":"application/json",...w()},body:JSON.stringify(e),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{i(null),p("User added successfully","success",5e3),A()},onError:e=>{console.error("Error adding user:",e),p(`Error adding user: ${e.message}`,"error",8e3)}}),P=n({mutationFn:async({userId:e,userData:t})=>await l(`/api/auth/users/${e}`,{method:"PUT",headers:{"Content-Type":"application/json",...w()},body:JSON.stringify(t),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{i(null),p("User updated successfully","success",5e3),A()},onError:e=>{console.error("Error updating user:",e),p(`Error updating user: ${e.message}`,"error",8e3)}}),I=n({mutationFn:async e=>await l(`/api/auth/users/${e}`,{method:"DELETE",headers:w(),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{i(null),p("User deleted successfully","success",5e3),A()},onError:e=>{console.error("Error deleting user:",e),p(`Error deleting user: ${e.message}`,"error",8e3)}}),j=n({mutationFn:async e=>await l(`/api/auth/users/${e}/api-key`,{method:"POST",headers:w(),timeout:2e4,retries:1,retryDelay:2e3}),onMutate:()=>{g("Generating...")},onSuccess:e=>{g(e.api_key),p("API key generated successfully","success"),setTimeout((()=>{A()}),100)},onError:e=>{console.error("Error generating API key:",e),g(""),p(`Error generating API key: ${e.message}`,"error")}}),S=a((e=>{e&&e.preventDefault(),console.log("Adding user:",y.username),D.mutate(y)}),[y]),K=a((e=>{e&&e.preventDefault(),console.log("Editing user:",c.id,c.username),P.mutate({userId:c.id,userData:y})}),[c,y]),T=a((()=>{console.log("Deleting user:",c.id,c.username),I.mutate(c.id)}),[c]),L=a((()=>{console.log("Generating API key for user:",c.id,c.username),j.mutate(c.id)}),[c]),_=a((()=>{navigator.clipboard.writeText(b).then((()=>{window.showSuccessToast?window.showSuccessToast("API key copied to clipboard"):p("API key copied to clipboard","success")})).catch((e=>{console.error("Error copying API key:",e),window.showErrorToast?window.showErrorToast("Failed to copy API key"):p("Failed to copy API key","error")}))}),[b]),N=a((e=>{u(e),f({username:e.username,password:"",email:e.email||"",role:e.role,is_active:e.is_active}),i("edit")}),[]),F=a((e=>{u(e),i("delete")}),[]),G=a((e=>{u(e),g(""),i("apiKey")}),[]),M=a((()=>{i(null)}),[]);return $&&0===E.length?d("div",{className:"flex justify-center items-center p-8",children:[d("div",{className:"w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"}),d("span",{className:"sr-only",children:"Loading..."})]}):0!==E.length||$?d("div",{children:[d(t,{users:E,onEdit:N,onDelete:F,onApiKey:G}),"add"===e&&d(h,{formData:y,handleInputChange:U,handleAddUser:S,onClose:M}),"edit"===e&&d(m,{currentUser:c,formData:y,handleInputChange:U,handleEditUser:K,onClose:M}),"delete"===e&&d(v,{currentUser:c,handleDeleteUser:T,onClose:M}),"apiKey"===e&&d(x,{currentUser:c,newApiKey:b,handleGenerateApiKey:L,copyApiKey:_,onClose:M})]}):d("div",{children:[d("div",{className:"bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4",children:[d("h4",{className:"font-bold mb-2",children:"No Users Found"}),d("p",{children:'Click the "Add User" button to create your first user.'})]}),"add"===e&&d(h,{formData:y,handleInputChange:U,handleAddUser:S,onClose:M})]})}document.addEventListener("DOMContentLoaded",(()=>{const e=document.getElementById("main-content");e&&i(d(u,{client:c,children:[d(g,{}),d(f,{}),d(y,{})]}),e)}))}}}));
//# sourceMappingURL=users-legacy-Dtkn8mHW.js.map
