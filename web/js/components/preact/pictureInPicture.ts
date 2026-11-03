import type { PresentationModeVideo } from '../../types/live-transport.ts';

export interface PictureInPictureAdapter {
  active: () => boolean;
  enter: () => Promise<unknown> | void;
  exit: () => Promise<unknown> | void;
}

export function pictureInPictureAdapter(
  video: PresentationModeVideo | null | undefined,
  documentObject: Document | null | undefined = globalThis.document
): PictureInPictureAdapter | null {
  if (!video || !documentObject) return null;
  if (documentObject.pictureInPictureEnabled
      && typeof video.requestPictureInPicture === 'function'
      && typeof documentObject.exitPictureInPicture === 'function') {
    return {
      active: () => documentObject.pictureInPictureElement === video,
      enter: () => video.requestPictureInPicture(),
      exit: () => documentObject.exitPictureInPicture(),
    };
  }
  if (typeof video.webkitSupportsPresentationMode === 'function'
      && video.webkitSupportsPresentationMode('picture-in-picture')
      && typeof video.webkitSetPresentationMode === 'function') {
    return {
      active: () => video.webkitPresentationMode === 'picture-in-picture',
      enter: () => video.webkitSetPresentationMode!('picture-in-picture'),
      exit: () => video.webkitSetPresentationMode!('inline'),
    };
  }
  return null;
}
