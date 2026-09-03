import {
  buildZoneSnapshotUrl,
  containedPreviewRect,
  canvasPointToZonePoint,
} from '../js/components/preact/zoneEditorModel.js';

describe('Detection-zone editor model', () => {
  test('uses the authenticated same-origin proxy for snapshots', () => {
    expect(buildZoneSnapshotUrl('Front Door / east', 1234)).toBe(
      '/go2rtc/api/frame.jpeg?src=Front%20Door%20%2F%20east&t=1234',
    );
    expect(buildZoneSnapshotUrl('', 1234)).toBeNull();
  });

  test('contains a wide frame without changing its aspect ratio', () => {
    expect(containedPreviewRect(800, 600, 1920, 1080)).toEqual({
      x: 0,
      y: 75,
      width: 800,
      height: 450,
    });
  });

  test('contains a tall frame without changing its aspect ratio', () => {
    expect(containedPreviewRect(800, 600, 600, 1200)).toEqual({
      x: 250,
      y: 0,
      width: 300,
      height: 600,
    });
  });

  test('maps zone points against the frame rather than the letterbox bars', () => {
    const preview = containedPreviewRect(800, 600, 1920, 1080);

    expect(canvasPointToZonePoint(400, 300, preview)).toEqual({ x: 0.5, y: 0.5 });
    expect(canvasPointToZonePoint(400, 50, preview)).toBeNull();
    expect(canvasPointToZonePoint(400, 50, preview, true)).toEqual({ x: 0.5, y: 0 });
  });
});
