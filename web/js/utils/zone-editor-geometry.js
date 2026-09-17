/**
 * Pure coordinate math for ZoneEditor.jsx.
 *
 * A camera image is drawn into the editor's canvas letterboxed/pillarboxed
 * to preserve its aspect ratio (the canvas element's own aspect ratio is
 * whatever its container gives it, which rarely matches the camera's).
 * Zone points are stored as a [0,1] fraction of the actual image content,
 * not of the padded canvas -- the same convention DetectionOverlay.jsx
 * already uses to render zones over the live view. These helpers keep
 * ZoneEditor's own mouse capture and redraw consistent with that.
 */

/**
 * Computes where an image is drawn within a canvas of a possibly different
 * aspect ratio, preserving the image's aspect ratio ("contain" fit).
 *
 * @param {number} canvasWidth
 * @param {number} canvasHeight
 * @param {number} imageAspect - image.naturalWidth / image.naturalHeight
 * @returns {{offsetX: number, offsetY: number, drawWidth: number, drawHeight: number}}
 */
export function computeImageLetterbox(canvasWidth, canvasHeight, imageAspect) {
  if (!canvasWidth || !canvasHeight || !imageAspect) {
    return { offsetX: 0, offsetY: 0, drawWidth: canvasWidth || 0, drawHeight: canvasHeight || 0 };
  }

  const canvasAspect = canvasWidth / canvasHeight;
  let drawWidth, drawHeight, offsetX = 0, offsetY = 0;

  if (imageAspect > canvasAspect) {
    // Image is wider than the canvas: letterboxing (bars on top/bottom).
    drawWidth = canvasWidth;
    drawHeight = canvasWidth / imageAspect;
    offsetY = (canvasHeight - drawHeight) / 2;
  } else {
    // Image is taller than the canvas: pillarboxing (bars on the sides).
    drawHeight = canvasHeight;
    drawWidth = canvasHeight * imageAspect;
    offsetX = (canvasWidth - drawWidth) / 2;
  }

  return { offsetX, offsetY, drawWidth, drawHeight };
}

/**
 * Converts a canvas-relative pixel position (e.g. from a mouse event, via
 * getBoundingClientRect()) into a [0,1] fraction of the image content,
 * undoing the letterbox offset. A position inside the letterbox bars
 * clamps to the nearest image edge rather than falling outside [0,1].
 *
 * @param {number} canvasX
 * @param {number} canvasY
 * @param {{offsetX: number, offsetY: number, drawWidth: number, drawHeight: number}} letterbox
 * @returns {{x: number, y: number}}
 */
export function canvasPointToImageFraction(canvasX, canvasY, letterbox) {
  const { offsetX, offsetY, drawWidth, drawHeight } = letterbox;
  if (!drawWidth || !drawHeight) return { x: 0, y: 0 };

  const x = (canvasX - offsetX) / drawWidth;
  const y = (canvasY - offsetY) / drawHeight;

  return {
    x: Math.max(0, Math.min(1, x)),
    y: Math.max(0, Math.min(1, y)),
  };
}

/**
 * Picks the aspect ratio to letterbox the editor's canvas against: the
 * loaded snapshot image when one is available, otherwise the stream's
 * configured resolution (so the placeholder shown while the snapshot is
 * loading, or after it fails, still letterboxes consistently with the real
 * image -- rather than falling back to a plain 1:1 canvas fill that would
 * make an already-saved zone appear to shift when the snapshot isn't
 * available). Returns null when neither source is usable, meaning the
 * caller has no aspect ratio to letterbox against at all.
 *
 * @param {{naturalWidth: number, naturalHeight: number}|null} image
 * @param {boolean} imageLoaded
 * @param {boolean} snapshotError
 * @param {number} [fallbackWidth] - the stream's configured width
 * @param {number} [fallbackHeight] - the stream's configured height
 * @returns {number|null}
 */
export function resolveImageAspect(image, imageLoaded, snapshotError, fallbackWidth, fallbackHeight) {
  if (image && imageLoaded && !snapshotError && image.naturalWidth > 0 && image.naturalHeight > 0) {
    return image.naturalWidth / image.naturalHeight;
  }
  if (fallbackWidth > 0 && fallbackHeight > 0) {
    return fallbackWidth / fallbackHeight;
  }
  return null;
}

/**
 * Inverse of canvasPointToImageFraction: converts a stored [0,1] zone
 * point back into canvas pixel coordinates for drawing, matching the
 * convention DetectionOverlay.jsx already uses.
 *
 * @param {number} fractionX
 * @param {number} fractionY
 * @param {{offsetX: number, offsetY: number, drawWidth: number, drawHeight: number}} letterbox
 * @returns {{x: number, y: number}}
 */
export function imageFractionToCanvasPoint(fractionX, fractionY, letterbox) {
  const { offsetX, offsetY, drawWidth, drawHeight } = letterbox;
  return {
    x: fractionX * drawWidth + offsetX,
    y: fractionY * drawHeight + offsetY,
  };
}
