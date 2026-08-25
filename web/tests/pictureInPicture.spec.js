import { pictureInPictureAdapter } from '../js/components/preact/pictureInPicture.js';

describe('picture-in-picture adapter', () => {
  test('uses the standard API when available', async () => {
    const video = { requestPictureInPicture: jest.fn(async () => {}) };
    const documentObject = {
      pictureInPictureEnabled: true,
      pictureInPictureElement: video,
      exitPictureInPicture: jest.fn(async () => {}),
    };
    const adapter = pictureInPictureAdapter(video, documentObject);
    expect(adapter.active()).toBe(true);
    await adapter.exit();
    expect(documentObject.exitPictureInPicture).toHaveBeenCalledTimes(1);
  });

  test('falls back to Safari presentation mode', () => {
    const video = {
      webkitPresentationMode: 'inline',
      webkitSupportsPresentationMode: jest.fn(() => true),
      webkitSetPresentationMode: jest.fn(),
    };
    const adapter = pictureInPictureAdapter(video, {});
    adapter.enter();
    expect(video.webkitSetPresentationMode).toHaveBeenCalledWith('picture-in-picture');
  });
});
