import {
  MOBILE_CAMERA_PAGE_SIZE,
  compareFavoriteCameras,
  paginateMobileCameras,
  resolveUiMode,
  uiAccountScopeKey,
} from '../js/utils/mobile-ui.js';
import { QueryClient } from '@tanstack/query-core';

describe('мобильный интерфейс Live View', () => {
  test('автоматический режим меняется по результату определения окна, явный выбор имеет приоритет', () => {
    expect(resolveUiMode('auto', 'mobile')).toBe('mobile');
    expect(resolveUiMode('auto', 'desktop')).toBe('desktop');
    expect(resolveUiMode('mobile', 'desktop')).toBe('mobile');
    expect(resolveUiMode('desktop', 'mobile')).toBe('desktop');
    expect(resolveUiMode('unknown', 'mobile')).toBe('mobile');
    expect(resolveUiMode('desktop', 'mobile', 'mobile')).toBe('mobile');
  });

  test('query scope отличается для двух серверных пользователей и изолирует demo', () => {
    const userA = uiAccountScopeKey({ id: 17, demo_mode: false });
    const userB = uiAccountScopeKey({ id: 29, demo_mode: false });
    expect(userA).not.toBe(userB);
    expect(uiAccountScopeKey({ id: 17, demo_mode: true })).toBe('scoped');
    expect(uiAccountScopeKey({})).toBe('scoped');

    const cache = new QueryClient();
    cache.setQueryData(['ui-preferences', userA], { ui_mode: 'mobile' });
    cache.setQueryData(['ui-preferences', userB], { ui_mode: 'desktop' });
    expect(cache.getQueryData(['ui-preferences', userA]).ui_mode).toBe('mobile');
    expect(cache.getQueryData(['ui-preferences', userB]).ui_mode).toBe('desktop');
  });

  test('при 80 камерах в DOM-странице находятся максимум четыре карточки', () => {
    const cameras = Array.from({ length: 80 }, (_, index) => ({ camera_uuid: `camera-${index}` }));
    expect(MOBILE_CAMERA_PAGE_SIZE).toBe(4);
    expect(paginateMobileCameras(cameras, 0)).toHaveLength(4);
    expect(paginateMobileCameras(cameras, 19).map((camera) => camera.camera_uuid))
      .toEqual(['camera-76', 'camera-77', 'camera-78', 'camera-79']);
    expect(paginateMobileCameras(cameras, 20)).toHaveLength(0);
  });

  test('избранное сортируется по числовым timestamp в секундах, затем по UUID', () => {
    const cameraA = { camera_uuid: 'z-camera' };
    const cameraB = { camera_uuid: 'a-camera' };
    const favorites = [
      { camera_uuid: cameraA.camera_uuid, created_at: 1800000000 },
      { camera_uuid: cameraB.camera_uuid, created_at: 1900000000 },
    ];
    expect([cameraA, cameraB].sort((a, b) => compareFavoriteCameras(a, b, favorites)))
      .toEqual([cameraA, cameraB]);
    expect(compareFavoriteCameras(
      { camera_uuid: 'z-camera' }, { camera_uuid: 'a-camera' },
      [{ camera_uuid: 'z-camera', created_at: 42 }, { camera_uuid: 'a-camera', created_at: 42 }],
    )).toBeGreaterThan(0);
  });
});
