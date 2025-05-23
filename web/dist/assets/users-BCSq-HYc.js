import{u as e,k as $,d as g,q as u,b as z,h as y,s as m,i as x,E as q,c as B,e as R,Q as J}from"./query-client-DIfHb7sS.js";/* empty css               */import{H as Q,F as V}from"./Footer-C_MeC-4D.js";function re(){import.meta.url,import("_").catch(()=>1),async function*(){}().next()}const v={0:"Admin",1:"User",2:"Viewer",3:"API"};function W({users:n,onEdit:a,onDelete:o,onApiKey:d}){const c=(r,s)=>{s.preventDefault(),s.stopPropagation(),a(r)},i=(r,s)=>{s.preventDefault(),s.stopPropagation(),o(r)},l=(r,s)=>{s.preventDefault(),s.stopPropagation(),d(r)};return e("div",{className:"overflow-x-auto",children:e("table",{className:"w-full border-collapse",children:[e("thead",{className:"bg-gray-50 dark:bg-gray-700",children:e("tr",{children:[e("th",{className:"py-3 px-6 text-left font-semibold",children:"ID"}),e("th",{className:"py-3 px-6 text-left font-semibold",children:"Username"}),e("th",{className:"py-3 px-6 text-left font-semibold",children:"Email"}),e("th",{className:"py-3 px-6 text-left font-semibold",children:"Role"}),e("th",{className:"py-3 px-6 text-left font-semibold",children:"Status"}),e("th",{className:"py-3 px-6 text-left font-semibold",children:"Last Login"}),e("th",{className:"py-3 px-6 text-left font-semibold",children:"Actions"})]})}),e("tbody",{className:"divide-y divide-gray-200 dark:divide-gray-700",children:n.map(r=>e("tr",{className:"hover:bg-gray-100 dark:hover:bg-gray-600",children:[e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:r.id}),e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:r.username}),e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:r.email||"-"}),e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:v[r.role]||"Unknown"}),e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:e("span",{className:"inline-block px-2 py-1 text-xs font-semibold rounded-full ".concat(r.is_active?"bg-green-100 text-green-800":"bg-red-100 text-red-800"),children:r.is_active?"Active":"Inactive"})}),e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:r.last_login?new Date(r.last_login*1e3).toLocaleString():"Never"}),e("td",{className:"py-3 px-6 border-b border-gray-200 dark:border-gray-700",children:e("div",{className:"flex space-x-2",children:[e("button",{className:"p-1 text-blue-600 hover:text-blue-800 rounded hover:bg-blue-100 transition-colors",onClick:s=>c(r,s),title:"Edit User",children:e("svg",{xmlns:"http://www.w3.org/2000/svg",className:"h-5 w-5",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor",children:e("path",{strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:"2",d:"M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"})})}),e("button",{className:"p-1 text-red-600 hover:text-red-800 rounded hover:bg-red-100 transition-colors",onClick:s=>i(r,s),title:"Delete User",children:e("svg",{xmlns:"http://www.w3.org/2000/svg",className:"h-5 w-5",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor",children:e("path",{strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:"2",d:"M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"})})}),e("button",{className:"p-1 text-gray-600 hover:text-gray-800 rounded hover:bg-gray-100 transition-colors",onClick:s=>l(r,s),title:"Manage API Key",children:e("svg",{xmlns:"http://www.w3.org/2000/svg",className:"h-5 w-5",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor",children:e("path",{strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:"2",d:"M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"})})})]})})]},r.id))})]})})}function A({formData:n,handleInputChange:a,handleAddUser:o,onClose:d}){return e("div",{className:"fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50",onClick:d,children:e("div",{className:"bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white",onClick:l=>{l.stopPropagation()},children:[e("h2",{className:"text-xl font-bold mb-4",children:"Add New User"}),e("form",{onSubmit:l=>{l.preventDefault(),l.stopPropagation(),o(l)},children:[e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"username",children:"Username"}),e("input",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"username",type:"text",name:"username",value:n.username,onChange:a,required:!0})]}),e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"password",children:"Password"}),e("input",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"password",type:"password",name:"password",value:n.password,onChange:a,required:!0})]}),e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"email",children:"Email"}),e("input",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"email",type:"email",name:"email",value:n.email,onChange:a})]}),e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"role",children:"Role"}),e("select",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"role",name:"role",value:n.role,onChange:a,children:Object.entries(v).map(([l,r])=>e("option",{value:l,children:r},l))})]}),e("div",{className:"mb-6",children:e("label",{className:"flex items-center",children:[e("input",{type:"checkbox",name:"is_active",checked:n.is_active,onChange:a,className:"mr-2"}),e("span",{className:"text-sm font-bold",children:"Active"})]})}),e("div",{className:"flex justify-end mt-6",children:[e("button",{type:"button",className:"px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2",onClick:d,children:"Cancel"}),e("button",{type:"submit",className:"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",children:"Add User"})]})]})]})})}function X({currentUser:n,formData:a,handleInputChange:o,handleEditUser:d,onClose:c}){const i=r=>{r.preventDefault(),r.stopPropagation(),d(r)};return e("div",{className:"fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50",onClick:c,children:e("div",{className:"bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white",onClick:r=>{r.stopPropagation()},children:[e("h2",{className:"text-xl font-bold mb-4",children:["Edit User: ",n.username]}),e("form",{onSubmit:i,children:[e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"username",children:"Username"}),e("input",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"username",type:"text",name:"username",value:a.username,onChange:o,required:!0})]}),e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"password",children:"Password (leave blank to keep current)"}),e("input",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"password",type:"password",name:"password",value:a.password,onChange:o})]}),e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"email",children:"Email"}),e("input",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"email",type:"email",name:"email",value:a.email,onChange:o})]}),e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",htmlFor:"role",children:"Role"}),e("select",{className:"shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",id:"role",name:"role",value:a.role,onChange:o,children:Object.entries(v).map(([r,s])=>e("option",{value:r,children:s},r))})]}),e("div",{className:"mb-6",children:e("label",{className:"flex items-center",children:[e("input",{type:"checkbox",name:"is_active",checked:a.is_active,onChange:o,className:"mr-2"}),e("span",{className:"text-sm font-bold",children:"Active"})]})}),e("div",{className:"flex justify-end mt-6",children:[e("button",{type:"button",className:"px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2",onClick:c,children:"Cancel"}),e("button",{type:"submit",className:"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",children:"Update User"})]})]})]})})}function Y({currentUser:n,handleDeleteUser:a,onClose:o}){const d=i=>{i.stopPropagation(),a()};return e("div",{className:"fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50",onClick:o,children:e("div",{className:"bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white",onClick:i=>{i.stopPropagation()},children:[e("h2",{className:"text-xl font-bold mb-4",children:"Delete User"}),e("p",{className:"mb-6",children:['Are you sure you want to delete the user "',n.username,'"? This action cannot be undone.']}),e("div",{className:"flex justify-end",children:[e("button",{className:"px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2",onClick:o,children:"Cancel"}),e("button",{className:"px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700",onClick:d,children:"Delete User"})]})]})})}function Z({currentUser:n,newApiKey:a,handleGenerateApiKey:o,copyApiKey:d,onClose:c}){const i=r=>{r.stopPropagation()};return console.log("API Key Modal - newApiKey:",a),e("div",{className:"fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50",onClick:r=>{a&&a!=="Generating..."||c(r)},children:e("div",{className:"bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white",onClick:i,children:[e("h2",{className:"text-xl font-bold mb-4",children:["API Key for ",n.username]}),e("div",{className:"mb-6",children:a?e("div",{className:"mb-4",children:[e("label",{className:"block text-sm font-bold mb-2",children:"API Key"}),e("div",{className:"flex",children:[e("input",{className:"shadow appearance-none border rounded-l w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline",type:"text",value:a,readOnly:!0}),e("button",{className:"bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-r",onClick:d,children:"Copy"})]}),e("p",{className:"text-sm text-gray-600 dark:text-gray-300 mt-2",children:"This key will only be shown once. Save it securely."})]}):e($,{children:[e("p",{className:"mb-4",children:"Generate a new API key for this user. This will invalidate any existing API key."}),e("button",{className:"w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4",onClick:o,children:"Generate New API Key"})]})}),e("div",{className:"flex justify-end",children:e("button",{className:a&&a!=="Generating..."?"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700":"px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500",onClick:c,children:a&&a!=="Generating..."?"Done":"Close"})})]})})}function ee(){const[n,a]=g(null),[o,d]=g(null),[c,i]=g(""),[l,r]=g({username:"",password:"",email:"",role:1,is_active:!0}),s=u(()=>{const t=localStorage.getItem("auth");return t?{Authorization:"Basic "+t}:{}},[]),{data:f,isLoading:N,error:te,refetch:b}=z(["users"],"/api/auth/users",{headers:s(),cache:"no-store",timeout:15e3,retries:2,retryDelay:1e3}),w=(f==null?void 0:f.users)||[],C=u(()=>{r({username:"",password:"",email:"",role:1,is_active:!0}),a("add")},[]),k=u(t=>{const{name:p,value:P,type:O,checked:H}=t.target;r(G=>({...G,[p]:O==="checkbox"?H:p==="role"?parseInt(P,10):P}))},[]),E=y({mutationFn:async t=>await x("/api/auth/users",{method:"POST",headers:{"Content-Type":"application/json",...s()},body:JSON.stringify(t),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{a(null),m("User added successfully","success",5e3),b()},onError:t=>{console.error("Error adding user:",t),m("Error adding user: ".concat(t.message),"error",8e3)}}),M=y({mutationFn:async({userId:t,userData:p})=>await x("/api/auth/users/".concat(t),{method:"PUT",headers:{"Content-Type":"application/json",...s()},body:JSON.stringify(p),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{a(null),m("User updated successfully","success",5e3),b()},onError:t=>{console.error("Error updating user:",t),m("Error updating user: ".concat(t.message),"error",8e3)}}),S=y({mutationFn:async t=>await x("/api/auth/users/".concat(t),{method:"DELETE",headers:s(),timeout:15e3,retries:1,retryDelay:1e3}),onSuccess:()=>{a(null),m("User deleted successfully","success",5e3),b()},onError:t=>{console.error("Error deleting user:",t),m("Error deleting user: ".concat(t.message),"error",8e3)}}),D=y({mutationFn:async t=>await x("/api/auth/users/".concat(t,"/api-key"),{method:"POST",headers:s(),timeout:2e4,retries:1,retryDelay:2e3}),onMutate:()=>{i("Generating...")},onSuccess:t=>{i(t.api_key),m("API key generated successfully","success"),setTimeout(()=>{b()},100)},onError:t=>{console.error("Error generating API key:",t),i(""),m("Error generating API key: ".concat(t.message),"error")}}),U=u(t=>{t&&t.preventDefault(),console.log("Adding user:",l.username),E.mutate(l)},[l]),j=u(t=>{t&&t.preventDefault(),console.log("Editing user:",o.id,o.username),M.mutate({userId:o.id,userData:l})},[o,l]),F=u(()=>{console.log("Deleting user:",o.id,o.username),S.mutate(o.id)},[o]),L=u(()=>{console.log("Generating API key for user:",o.id,o.username),D.mutate(o.id)},[o]),_=u(()=>{navigator.clipboard.writeText(c).then(()=>{window.showSuccessToast?window.showSuccessToast("API key copied to clipboard"):m("API key copied to clipboard","success")}).catch(t=>{console.error("Error copying API key:",t),window.showErrorToast?window.showErrorToast("Failed to copy API key"):m("Failed to copy API key","error")})},[c]),T=u(t=>{d(t),r({username:t.username,password:"",email:t.email||"",role:t.role,is_active:t.is_active}),a("edit")},[]),I=u(t=>{d(t),a("delete")},[]),K=u(t=>{d(t),i(""),a("apiKey")},[]),h=u(()=>{a(null)},[]);return N&&w.length===0?e("div",{className:"flex justify-center items-center p-8",children:[e("div",{className:"w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"}),e("span",{className:"sr-only",children:"Loading..."})]}):w.length===0&&!N?e("div",{children:[e("div",{className:"mb-4 flex justify-between items-center",children:[e("h2",{className:"text-xl font-semibold",children:"User Management"}),e("button",{className:"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",onClick:C,children:"Add User"})]}),e("div",{className:"bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4",children:[e("h4",{className:"font-bold mb-2",children:"No Users Found"}),e("p",{children:'Click the "Add User" button to create your first user.'})]}),n==="add"&&e(A,{formData:l,handleInputChange:k,handleAddUser:U,onClose:h})]}):e("div",{children:[e("div",{className:"mb-4 flex justify-between items-center",children:[e("h2",{className:"text-xl font-semibold",children:"User Management"}),e("button",{className:"px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors",onClick:C,children:"Add User"})]}),e(W,{users:w,onEdit:T,onDelete:I,onApiKey:K}),n==="add"&&e(A,{formData:l,handleInputChange:k,handleAddUser:U,onClose:h}),n==="edit"&&e(X,{currentUser:o,formData:l,handleInputChange:k,handleEditUser:j,onClose:h}),n==="delete"&&e(Y,{currentUser:o,handleDeleteUser:F,onClose:h}),n==="apiKey"&&e(Z,{currentUser:o,newApiKey:c,handleGenerateApiKey:L,copyApiKey:_,onClose:h})]})}document.addEventListener("DOMContentLoaded",()=>{const n=document.getElementById("main-content");n&&q(e(J,{client:B,children:[e(Q,{}),e(R,{}),e(ee,{}),e(V,{})]}),n)});export{re as __vite_legacy_guard};
//# sourceMappingURL=users-BCSq-HYc.js.map
