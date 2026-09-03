/**
 * Pure geometry and URL helpers for the detection-zone editor.
 */

/**
 * Snapshots are ordinary HTTP resources, so always request them through
 * LightNVR's authenticated, same-origin go2rtc proxy. Live WebRTC and MSE
 * signaling still use their dedicated direct-to-go2rtc URL helpers.
 */
export function buildZoneSnapshotUrl(streamName, timestamp = Date.now()) {
  if (!streamName) return null;
  return `/go2rtc/api/frame.jpeg?src=${encodeURIComponent(streamName)}&t=${timestamp}`;
}

/**
 * Return the largest rectangle with the content's aspect ratio that fits in
 * the available canvas.
 */
export function containedPreviewRect(canvasWidth, canvasHeight, contentWidth, contentHeight) {
  const width = Number(canvasWidth);
  const height = Number(canvasHeight);
  const sourceWidth = Number(contentWidth);
  const sourceHeight = Number(contentHeight);

  if (!(width > 0) || !(height > 0)) {
    return { x: 0, y: 0, width: 0, height: 0 };
  }
  if (!(sourceWidth > 0) || !(sourceHeight > 0)) {
    return { x: 0, y: 0, width, height };
  }

  const scale = Math.min(width / sourceWidth, height / sourceHeight);
  const previewWidth = sourceWidth * scale;
  const previewHeight = sourceHeight * scale;

  return {
    x: (width - previewWidth) / 2,
    y: (height - previewHeight) / 2,
    width: previewWidth,
    height: previewHeight,
  };
}

/**
 * Convert a canvas-space point into the normalized coordinate system used by
 * detection zones. Points in letterbox/pillarbox bars are rejected unless the
 * caller asks to clamp a drag to the nearest frame edge.
 */
export function canvasPointToZonePoint(x, y, previewRect, clamp = false) {
  if (!previewRect || !(previewRect.width > 0) || !(previewRect.height > 0)) {
    return null;
  }

  let normalizedX = (x - previewRect.x) / previewRect.width;
  let normalizedY = (y - previewRect.y) / previewRect.height;

  if (!clamp && (normalizedX < 0 || normalizedX > 1 || normalizedY < 0 || normalizedY > 1)) {
    return null;
  }

  normalizedX = Math.max(0, Math.min(1, normalizedX));
  normalizedY = Math.max(0, Math.min(1, normalizedY));
  return { x: normalizedX, y: normalizedY };
}
