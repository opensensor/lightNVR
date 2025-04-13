import"./preact-app-_8hsa7bb.js";function g(){import.meta.url,import("_").catch(()=>1),async function*(){}().next()}function m(e,t=3e3){let d=document.getElementById("status-message-container");d||(d=document.createElement("div"),d.id="status-message-container",d.className="fixed bottom-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col items-center",document.body.appendChild(d));const a=document.createElement("div");a.className="bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg mb-2 transition-all duration-300 opacity-0 transform translate-y-2",a.textContent=e,d.appendChild(a),setTimeout(()=>{a.classList.remove("opacity-0","translate-y-2")},10),setTimeout(()=>{a.classList.add("opacity-0","translate-y-2"),setTimeout(()=>{a.parentNode===d&&d.removeChild(a),d.children.length===0&&document.body.removeChild(d)},300)},t)}function r(){let e=document.getElementById("batch-delete-modal-container");e||(e=document.createElement("div"),e.id="batch-delete-modal-container",document.body.appendChild(e)),e.innerHTML='\n        <div id="batch-delete-modal" class="modal hidden fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center z-50">\n            <div class="modal-content relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md mx-auto p-6 w-full">\n                <div class="modal-header flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">\n                    <h3 id="batch-delete-modal-title" class="text-xl font-bold text-gray-900 dark:text-white">Batch Delete Progress</h3>\n                    <button id="batch-delete-close-btn" class="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 text-2xl font-bold">&times;</button>\n                </div>\n                <div class="modal-body">\n                    <div id="batch-delete-status" class="mb-4 text-gray-700 dark:text-gray-300">\n                        Preparing to delete recordings...\n                    </div>\n                    <div class="progress-container bg-gray-200 dark:bg-gray-700 rounded-full h-4 mb-4">\n                        <div id="batch-delete-progress-bar" class="bg-green-500 h-4 rounded-full text-center text-xs text-white" style="width: 0%"></div>\n                    </div>\n                    <div class="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-6">\n                        <div id="batch-delete-count">0 / 0</div>\n                        <div id="batch-delete-percentage">0%</div>\n                    </div>\n                    <div id="batch-delete-details" class="mb-4">\n                        <div class="flex justify-between mb-2">\n                            <span class="text-gray-700 dark:text-gray-300">Succeeded:</span>\n                            <span id="batch-delete-succeeded" class="font-bold text-green-600 dark:text-green-400">0</span>\n                        </div>\n                        <div class="flex justify-between">\n                            <span class="text-gray-700 dark:text-gray-300">Failed:</span>\n                            <span id="batch-delete-failed" class="font-bold text-red-600 dark:text-red-400">0</span>\n                        </div>\n                    </div>\n                    <div id="batch-delete-message" class="text-sm italic text-gray-600 dark:text-gray-400 mb-4"></div>\n                </div>\n                <div class="modal-footer flex justify-end pt-2 border-t border-gray-200 dark:border-gray-700">\n                    <button id="batch-delete-done-btn" class="btn btn-primary hidden">Done</button>\n                    <button id="batch-delete-cancel-btn" class="btn btn-secondary">Cancel</button>\n                </div>\n            </div>\n        </div>\n    ';const t=document.getElementById("batch-delete-modal"),d=document.getElementById("batch-delete-close-btn"),a=document.getElementById("batch-delete-done-btn"),n=document.getElementById("batch-delete-cancel-btn");d&&d.addEventListener("click",s),a&&a.addEventListener("click",s),n&&n.addEventListener("click",b),window.addEventListener("click",c=>{c.target===t&&s()})}function u(){const e=document.getElementById("batch-delete-modal");e||r(),h(),e.classList.remove("hidden")}function s(){const e=document.getElementById("batch-delete-modal");e&&e.classList.add("hidden")}function h(){const e=document.getElementById("batch-delete-progress-bar");e&&(e.style.width="0%");const t=document.getElementById("batch-delete-status");t&&(t.textContent="Preparing to delete recordings...");const d=document.getElementById("batch-delete-count");d&&(d.textContent="0 / 0");const a=document.getElementById("batch-delete-percentage");a&&(a.textContent="0%");const n=document.getElementById("batch-delete-succeeded");n&&(n.textContent="0");const c=document.getElementById("batch-delete-failed");c&&(c.textContent="0");const l=document.getElementById("batch-delete-message");l&&(l.textContent="");const o=document.getElementById("batch-delete-done-btn");o&&o.classList.add("hidden");const i=document.getElementById("batch-delete-cancel-btn");i&&i.classList.remove("hidden")}window.updateBatchDeleteProgress=function(e){console.log("Updating batch delete progress UI:",e),u();const t=document.getElementById("batch-delete-progress-bar");if(t){if(console.log("Updating progress bar: current=".concat(e.current,", total=").concat(e.total)),e.total>0){const o=Math.round(e.current/e.total*100);console.log("Setting progress bar width to ".concat(o,"%")),t.style.width="".concat(o,"%"),t.classList.remove("animate-pulse"),t.offsetWidth}else if(e.current>0){const o=Math.min(90,e.current/10);console.log("Setting progress bar width to ".concat(o,"% (estimated)")),t.style.width="".concat(o,"%"),t.classList.add("animate-pulse"),t.offsetWidth}else e.complete?(console.log("Setting progress bar to 100% (complete)"),t.style.width="100%",t.classList.remove("animate-pulse"),t.offsetWidth):(console.log("Setting progress bar to 50% (indeterminate)"),t.style.width="50%",t.classList.add("animate-pulse"),t.offsetWidth);e.error?(t.classList.add("bg-red-500"),t.classList.remove("bg-blue-500")):(t.classList.add("bg-blue-500"),t.classList.remove("bg-red-500"))}const d=document.getElementById("batch-delete-status");d&&e.status&&(d.textContent=e.status);const a=document.getElementById("batch-delete-count");a&&(e.total>0?a.textContent="".concat(e.current," / ").concat(e.total):a.textContent="".concat(e.current," / ?"));const n=document.getElementById("batch-delete-percentage");if(n)if(e.total>0){const o=Math.round(e.current/e.total*100);n.textContent="".concat(o,"%")}else e.complete?n.textContent="100%":n.textContent="In progress";const c=document.getElementById("batch-delete-succeeded");c&&(c.textContent=e.succeeded||"0");const l=document.getElementById("batch-delete-failed");if(l&&(l.textContent=e.failed||"0"),e.complete){const o=document.getElementById("batch-delete-done-btn");o&&o.classList.remove("hidden");const i=document.getElementById("batch-delete-cancel-btn");i&&i.classList.add("hidden"),d&&(!e.status||e.status==="Preparing to delete recordings...")&&(d.textContent="Batch delete operation complete"),t&&(t.style.width="100%",t.classList.remove("animate-pulse"))}};function b(){s(),m("Batch delete operation cancelled",5e3)}window.batchDeleteRecordingsByHttpRequest=function(e){return console.log("Using HTTP fallback for batch delete with params:",e),new Promise((t,d)=>{u();let a=0;e.ids?a=e.ids.length:e.filter&&e.totalCount&&(a=e.totalCount),updateBatchDeleteProgress({current:0,total:a,status:"Using HTTP fallback for batch delete operation",succeeded:0,failed:0}),fetch("/api/recordings/batch-delete",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(e)}).then(n=>{if(!n.ok)throw new Error("HTTP error ".concat(n.status,": ").concat(n.statusText));return n.json()}).then(n=>{console.log("HTTP batch delete result:",n);const c=n.total||a||0,l=n.succeeded||0,o=n.failed||0;updateBatchDeleteProgress({current:c,total:c,succeeded:l,failed:o,status:"Batch delete operation complete",complete:!0});const i=n.success?"Successfully deleted ".concat(l," recordings"):"Deleted ".concat(l," recordings with ").concat(o," failures");m(i,5e3),setTimeout(()=>{typeof loadRecordings=="function"&&loadRecordings()},1e3),t(n)}).catch(n=>{console.error("HTTP batch delete error:",n),updateBatchDeleteProgress({current:0,total:0,succeeded:0,failed:0,status:"Error: ".concat(n.message||"Unknown error"),complete:!0,error:!0}),m("Error: ".concat(n.message||"Unknown error"),5e3),d(n)})})};document.addEventListener("DOMContentLoaded",()=>{if(console.log("Initializing batch delete modal"),r(),!document.getElementById("batch-delete-modal-container")){console.error("Batch delete modal container not found, creating it");const t=document.createElement("div");t.id="batch-delete-modal-container",document.body.appendChild(t),r()}typeof window.updateBatchDeleteProgress!="function"&&(console.error("updateBatchDeleteProgress function not available, setting it up"),window.updateBatchDeleteProgress=updateBatchDeleteProgress),typeof window.showBatchDeleteModal!="function"&&(console.error("showBatchDeleteModal function not available, setting it up"),window.showBatchDeleteModal=u)});window.showBatchDeleteModal=u;window.updateBatchDeleteProgress=updateBatchDeleteProgress;window.initBatchDeleteModal=r;export{g as __vite_legacy_guard};
//# sourceMappingURL=recordings-B20rpg9D.js.map
