/** Complete fleet responses for full-page browser scenarios. */
import type {components} from '../../../web/generated/api/schema';
import {
  validateCameraCollectionsResponse, validateFleetQueryResponse,
  type ContractValidator,
} from '../../../web/generated/api/validators.js';

type Collection = components['schemas']['CameraCollection'];
type Camera = components['schemas']['FleetCamera'];

export function contractFixture<T>(guard: ContractValidator<T>, value: T): T {
  if (!guard(value)) throw new Error(`Invalid fleet fixture: ${JSON.stringify(guard.errors)}`);
  return value;
}

export function collectionFixture(uuid: string, name: string, count: number): Collection {
  return {
    uuid, name, description: '', type: 'static', shared: true, owner_user_id: 1,
    member_count: count, effective_count: count, created_at: 1, updated_at: 1,
    selector: null, selector_redacted: false,
  };
}

export function collectionsFixture(collections: Collection[]) {
  return contractFixture(validateCameraCollectionsResponse, {collections, count: collections.length});
}

export function cameraQueryFixture(cameras: Pick<Camera, 'camera_uuid' | 'name'>[]) {
  return contractFixture(validateFleetQueryResponse, {
    selector_version: 1, queue_version: 1, preview: false, page: 1, page_size: 200,
    total: cameras.length, total_pages: cameras.length ? 1 : 0,
    sort_by: 'camera_uuid', sort_order: 'asc', queues: [],
    cameras: cameras.map((camera): Camera => ({
      camera_uuid: camera.camera_uuid, name: camera.name, address: 'fixture.invalid',
      enabled: true, recording_mode: 'off', health: 'up', availability: 'live',
      health_changed_at: 1, last_frame_ts: 1, first_video_at: 1, last_recording_at: 0,
      current_fps: 1, recording_active: false, can_configure: true,
      manufacturer: '', model: '', capabilities: [], location: {uuid: '', name: '', path: ''}, tags: [],
    })),
  });
}
