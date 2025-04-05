System.register(["./preact-app-legacy-BbvXHy7Y.js"],(function(e,t){"use strict";var r;return{setters:[e=>{r=e.h}],execute:function(){function t({message:e="Loading...",size:t="md",fullPage:a=!1}){const s={sm:"w-6 h-6",md:"w-10 h-10",lg:"w-16 h-16"},d=s[t]||s.md;return a?r`
      <div class="fixed inset-0 bg-white bg-opacity-75 dark:bg-gray-900 dark:bg-opacity-75 flex items-center justify-center z-50">
        <div class="text-center">
          <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 ${d}"></div>
          ${e&&r`<p class="mt-4 text-gray-700 dark:text-gray-300 text-lg">${e}</p>`}
        </div>
      </div>
    `:r`
    <div class="flex flex-col items-center justify-center py-8">
      <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 ${d}"></div>
      ${e&&r`<p class="mt-4 text-gray-700 dark:text-gray-300">${e}</p>`}
    </div>
  `}e({C:function({isLoading:e,hasData:a,children:s,loadingMessage:d="Loading data...",emptyMessage:i="No data available"}){return e?r`<${t} message=${d} />`:a?s:r`
      <div class="flex flex-col items-center justify-center py-12 text-center">
        <svg class="w-16 h-16 text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <p class="text-gray-600 dark:text-gray-400 text-lg">${i}</p>
      </div>
    `},L:t})}}}));
//# sourceMappingURL=LoadingIndicator-legacy--uSbwK_F.js.map
