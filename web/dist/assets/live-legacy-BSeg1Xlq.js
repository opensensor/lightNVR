System.register([],(function(e,t){"use strict";return{execute:function(){var e=document.createElement("style");e.textContent=".video-container{display:grid;gap:.5rem;height:calc(100vh - 180px);min-height:450px;transition:all .3s ease;width:100%;padding:.25rem}.video-container.layout-1{grid-template-columns:1fr}.video-container.layout-2{grid-template-columns:repeat(auto-fit,minmax(min(100%,450px),1fr));grid-auto-rows:1fr}.fullscreen-mode .video-container.layout-2{grid-template-columns:repeat(2,1fr)}.video-container.layout-4{grid-template-columns:repeat(auto-fit,minmax(min(100%,450px),1fr));grid-auto-rows:1fr}.video-container.layout-6{grid-template-columns:repeat(auto-fit,minmax(min(100%,400px),1fr));grid-auto-rows:1fr}.video-container.layout-9{grid-template-columns:repeat(auto-fit,minmax(min(100%,350px),1fr));grid-auto-rows:1fr}.video-container.layout-16{grid-template-columns:repeat(auto-fit,minmax(min(100%,250px),1fr));grid-auto-rows:1fr}@media (min-width: 1200px){.video-container.layout-2{grid-template-columns:repeat(2,1fr);grid-template-rows:1fr}.video-container.layout-4{grid-template-columns:repeat(2,1fr);grid-template-rows:repeat(2,1fr)}.video-container.layout-6{grid-template-columns:repeat(2,1fr);grid-template-rows:repeat(3,1fr)}.video-container.layout-9{grid-template-columns:repeat(3,1fr);grid-template-rows:repeat(3,1fr)}.video-container.layout-16{grid-template-columns:repeat(4,1fr);grid-template-rows:repeat(4,1fr)}}.video-cell{position:relative;width:100%;height:100%;overflow:hidden;border-radius:.5rem;background-color:#000;box-shadow:0 4px 6px rgba(0,0,0,.1)}.video-cell:hover{box-shadow:0 8px 15px rgba(0,0,0,.15);transform:translateY(-3px);z-index:2}.video-element{position:absolute;width:100%;height:100%;-o-object-fit:cover;object-fit:cover;z-index:1}.detection-overlay{position:absolute;top:0;left:0;width:100%;height:100%;z-index:2;pointer-events:none}.fullscreen-mode .video-cell video{padding:0}.video-cell .stream-info-bar{position:absolute;bottom:0;left:0;right:0;padding:.75rem;background-color:rgba(0,0,0,.7);color:#fff;font-size:.9rem;display:flex;justify-content:space-between;align-items:center;opacity:0;transform:translateY(100%);transition:all .3s ease;backdrop-filter:blur(4px);-webkit-backdrop-filter:blur(4px);z-index:20}.video-cell .stream-details{display:flex;flex-direction:column;justify-content:center;flex-grow:1;flex-shrink:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.video-cell .stream-name{font-weight:700;margin-bottom:.25rem;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.video-cell .stream-resolution{font-size:.8rem;opacity:.8;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.video-cell:hover .stream-info-bar{opacity:1;transform:translateY(0)}.video-cell .stream-controls{display:flex;justify-content:flex-end;gap:.5rem;margin-left:auto;flex-shrink:0;min-width:85px}.video-cell .stream-controls button{background-color:rgba(255,255,255,.15);border:none;color:#fff;cursor:pointer;padding:.5rem;font-size:.9rem;border-radius:4px;transition:all .2s ease;display:flex;align-items:center;justify-content:center;width:36px;height:36px;flex-shrink:0;position:relative}.video-cell .stream-controls button:hover{background-color:rgba(255,255,255,.3);transform:scale(1.1)}.placeholder{grid-column:1 / -1;grid-row:1 / -1;display:flex;height:100%;flex-direction:column;align-items:center;justify-content:center;border-radius:.5rem;--tw-bg-opacity: 1;background-color:rgb(243 244 246 / var(--tw-bg-opacity, 1));padding:2rem;text-align:center;--tw-shadow: 0 1px 3px 0 rgb(0 0 0 / .1), 0 1px 2px -1px rgb(0 0 0 / .1);--tw-shadow-colored: 0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color);box-shadow:var(--tw-ring-offset-shadow, 0 0 #0000),var(--tw-ring-shadow, 0 0 #0000),var(--tw-shadow)}@media (prefers-color-scheme: dark){.placeholder{--tw-bg-opacity: 1;background-color:rgb(31 41 55 / var(--tw-bg-opacity, 1))}}.placeholder p{margin-bottom:1.5rem;font-size:1.125rem;line-height:1.75rem;--tw-text-opacity: 1;color:rgb(75 85 99 / var(--tw-text-opacity, 1))}@media (prefers-color-scheme: dark){.placeholder p{--tw-text-opacity: 1;color:rgb(209 213 219 / var(--tw-text-opacity, 1))}}.fullscreen-mode{position:fixed;top:0;left:0;width:100%;height:100%;z-index:1000;background-color:#000;padding:0}.fullscreen-mode .video-container{height:100vh;padding:.5rem}.loading-indicator{position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);display:flex;flex-direction:column;align-items:center;color:#fff;z-index:15}.loading-spinner{width:40px;height:40px;border:3px solid rgba(255,255,255,.2);border-radius:50%;border-top-color:#1e88e5;animation:spin 1s ease-in-out infinite;margin-bottom:.75rem}@keyframes spin{to{transform:rotate(360deg)}}.error-indicator{position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);display:flex;flex-direction:column;align-items:center;color:#fff;text-align:center;padding:1.5rem;background-color:rgba(0,0,0,.5);border-radius:8px;backdrop-filter:blur(4px);-webkit-backdrop-filter:blur(4px);z-index:15}.error-icon{color:#f44336;font-size:2rem;margin-bottom:.75rem}@media (max-width: 992px){.video-container{gap:.75rem}}@media (max-width: 768px){.video-container{height:auto;min-height:auto}.video-cell{aspect-ratio:16 / 9;margin-bottom:.5rem}.video-cell .stream-info-bar{opacity:1;transform:translateY(0)}.video-cell .stream-controls{gap:.75rem}.video-cell .stream-info-bar{padding:.5rem;font-size:.8rem}.video-cell .stream-controls{padding:.5rem}.video-cell .stream-controls button{padding:.35rem .5rem;font-size:.8rem}.play-button{width:5rem;height:5rem}.video-cell .stream-controls button{min-height:44px;min-width:44px}.loading-indicator{background-color:rgba(0,0,0,.5);padding:1rem;border-radius:8px}.error-indicator{width:80%;max-width:300px}.retry-button{min-height:44px;min-width:100px;font-size:1rem}}@supports (-webkit-touch-callout: none){.video-cell video{position:relative;z-index:1}.video-cell .stream-info-bar{background-color:rgba(0,0,0,.8)}}.status-message{position:fixed;top:1.25rem;left:50%;z-index:50;max-width:80%;--tw-translate-x: -50%;--tw-translate-y: -1.25rem;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skew(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));border-radius:.25rem;--tw-bg-opacity: 1;background-color:rgb(34 197 94 / var(--tw-bg-opacity, 1));padding:.625rem 1rem;text-align:center;font-size:.875rem;line-height:1.25rem;--tw-text-opacity: 1;color:rgb(255 255 255 / var(--tw-text-opacity, 1));opacity:0;--tw-shadow: 0 10px 15px -3px rgb(0 0 0 / .1), 0 4px 6px -4px rgb(0 0 0 / .1);--tw-shadow-colored: 0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color);box-shadow:var(--tw-ring-offset-shadow, 0 0 #0000),var(--tw-ring-shadow, 0 0 #0000),var(--tw-shadow);transition-property:all;transition-timing-function:cubic-bezier(.4,0,.2,1);transition-duration:.3s}.status-message.visible{--tw-translate-y: -0px;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skew(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));opacity:1}.modal.block{display:flex!important}.modal.hidden{display:none!important}#snapshot-preview-modal{z-index:1050}#snapshot-preview-modal .modal-content{max-width:90%;margin:0 auto}#snapshot-preview-image{max-height:70vh;-o-object-fit:contain;object-fit:contain;width:100%}.play-overlay{position:absolute;top:0;right:0;bottom:0;left:0;background-color:rgba(0,0,0,.6);backdrop-filter:blur(4px);-webkit-backdrop-filter:blur(4px);display:flex;flex-direction:column;align-items:center;justify-content:center;cursor:pointer;transition:all .2s ease;z-index:15;touch-action:manipulation}.play-overlay:hover,.play-overlay:active{background-color:rgba(0,0,0,.5)}.play-button{width:4rem;height:4rem;background-color:rgba(255,255,255,.2);border-radius:50%;display:flex;flex-direction:column;align-items:center;justify-content:center;transition:transform .2s ease}.play-overlay:hover .play-button,.play-overlay:active .play-button{transform:scale(1.1)}.tap-message{margin-top:10px;color:#fff;font-size:14px;text-align:center}\n/*$vite$:1*/",document.head.appendChild(e)}}}));
//# sourceMappingURL=live-legacy-BSeg1Xlq.js.map
