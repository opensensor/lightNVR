export const PSEUDO_FULLSCREEN_EVENT = 'lightnvr:pseudo-fullscreenchange';

let pseudoFullscreenElement = null;

export function getNativeFullscreenElement() {
  return document.fullscreenElement || document.webkitFullscreenElement || pseudoFullscreenElement || null;
}

function enterPseudoFullscreen(element) {
  pseudoFullscreenElement?.classList.remove('pseudo-native-fullscreen');
  pseudoFullscreenElement = element;
  element.classList.add('pseudo-native-fullscreen');
  document.body.classList.add('pseudo-native-fullscreen-active');
  document.dispatchEvent(new Event(PSEUDO_FULLSCREEN_EVENT));
}

function exitPseudoFullscreen() {
  if (!pseudoFullscreenElement) return;
  pseudoFullscreenElement.classList.remove('pseudo-native-fullscreen');
  pseudoFullscreenElement = null;
  document.body.classList.remove('pseudo-native-fullscreen-active');
  document.dispatchEvent(new Event(PSEUDO_FULLSCREEN_EVENT));
}

export function isPseudoFullscreenActive() {
  return Boolean(pseudoFullscreenElement);
}

export function requestNativeFullscreen(element) {
  const request = element?.requestFullscreen || element?.webkitRequestFullscreen;
  if (!request) {
    enterPseudoFullscreen(element);
    return Promise.resolve();
  }
  try {
    return Promise.resolve(request.call(element)).catch(() => {
      enterPseudoFullscreen(element);
    });
  } catch {
    enterPseudoFullscreen(element);
    return Promise.resolve();
  }
}

export function exitNativeFullscreen() {
  if (pseudoFullscreenElement) {
    exitPseudoFullscreen();
    return Promise.resolve();
  }
  const exit = document.exitFullscreen || document.webkitExitFullscreen;
  if (!exit) {
    exitPseudoFullscreen();
    return Promise.resolve();
  }
  try {
    return Promise.resolve(exit.call(document));
  } catch (error) {
    return Promise.reject(error);
  }
}
