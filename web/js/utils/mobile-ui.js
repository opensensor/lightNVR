export const MOBILE_CAMERA_PAGE_SIZE = 4;

export function resolveUiMode(preference, viewport, devOverride) {
  const allowed = ['auto', 'mobile', 'desktop'];
  const selected = allowed.includes(devOverride) ? devOverride : allowed.includes(preference) ? preference : 'auto';
  if (selected === 'auto') return viewport === 'mobile' ? 'mobile' : 'desktop';
  return selected;
}

export function uiAccountScopeKey(session) {
  if (!session || session.demo_mode || !session.id) return 'scoped';
  return String(session.id);
}

export function paginateMobileCameras(cameras, page) {
  const offset = Math.max(0, page) * MOBILE_CAMERA_PAGE_SIZE;
  return cameras.slice(offset, offset + MOBILE_CAMERA_PAGE_SIZE);
}

export function compareFavoriteCameras(cameraA, cameraB, favorites) {
  const favoriteA = favorites.find((favorite) => favorite.camera_uuid === cameraA.camera_uuid);
  const favoriteB = favorites.find((favorite) => favorite.camera_uuid === cameraB.camera_uuid);
  const createdAtOrder = Number(favoriteA?.created_at || 0) - Number(favoriteB?.created_at || 0);
  return createdAtOrder || String(cameraA.camera_uuid).localeCompare(String(cameraB.camera_uuid));
}
