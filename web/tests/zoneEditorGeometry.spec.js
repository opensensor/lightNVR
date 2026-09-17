import {
  computeImageLetterbox,
  canvasPointToImageFraction,
  imageFractionToCanvasPoint,
  resolveImageAspect,
} from '../js/utils/zone-editor-geometry.js';

describe('computeImageLetterbox', () => {
  it('adds top/bottom bars when the image is wider than the canvas (letterboxing)', () => {
    // 1920x1080 (16:9) image in a 800x800 (1:1) canvas.
    const lb = computeImageLetterbox(800, 800, 1920 / 1080);
    expect(lb.drawWidth).toBeCloseTo(800);
    expect(lb.drawHeight).toBeCloseTo(450);
    expect(lb.offsetX).toBeCloseTo(0);
    expect(lb.offsetY).toBeCloseTo(175);
  });

  it('adds side bars when the image is taller/narrower than the canvas (pillarboxing)', () => {
    // FrontDoor: 2560x1920 (4:3) image in a wide 1200x600 canvas.
    const lb = computeImageLetterbox(1200, 600, 2560 / 1920);
    expect(lb.drawHeight).toBeCloseTo(600);
    expect(lb.drawWidth).toBeCloseTo(800);
    expect(lb.offsetY).toBeCloseTo(0);
    expect(lb.offsetX).toBeCloseTo(200);
  });

  it('has no offset when the image and canvas share an aspect ratio', () => {
    const lb = computeImageLetterbox(1280, 960, 1280 / 960);
    expect(lb.offsetX).toBeCloseTo(0);
    expect(lb.offsetY).toBeCloseTo(0);
    expect(lb.drawWidth).toBeCloseTo(1280);
    expect(lb.drawHeight).toBeCloseTo(960);
  });

  it('falls back to filling the canvas when dimensions are unavailable', () => {
    const lb = computeImageLetterbox(800, 600, 0);
    expect(lb).toEqual({ offsetX: 0, offsetY: 0, drawWidth: 800, drawHeight: 600 });
  });
});

describe('canvasPointToImageFraction', () => {
  // FrontDoor-shaped: 4:3 image pillarboxed into an 1200x600 canvas.
  const letterbox = { offsetX: 200, offsetY: 0, drawWidth: 800, drawHeight: 600 };

  it('maps the top-left corner of the visible image to (0, 0)', () => {
    expect(canvasPointToImageFraction(200, 0, letterbox)).toEqual({ x: 0, y: 0 });
  });

  it('maps the center of the visible image to (0.5, 0.5)', () => {
    const p = canvasPointToImageFraction(600, 300, letterbox);
    expect(p.x).toBeCloseTo(0.5);
    expect(p.y).toBeCloseTo(0.5);
  });

  it('maps the bottom-right corner of the visible image to (1, 1)', () => {
    expect(canvasPointToImageFraction(1000, 600, letterbox)).toEqual({ x: 1, y: 1 });
  });

  it('clamps a click inside the left pillarbox bar to the image edge (x=0)', () => {
    // This is the bug's failure mode: a click at raw canvas x=50 is inside
    // the black bar, well left of the image (which starts at x=200).
    const p = canvasPointToImageFraction(50, 300, letterbox);
    expect(p.x).toBe(0);
  });

  it('clamps a click inside the right pillarbox bar to the image edge (x=1)', () => {
    const p = canvasPointToImageFraction(1150, 300, letterbox);
    expect(p.x).toBe(1);
  });
});

describe('imageFractionToCanvasPoint', () => {
  const letterbox = { offsetX: 200, offsetY: 0, drawWidth: 800, drawHeight: 600 };

  it('matches the DetectionOverlay.jsx convention: fraction * drawWidth + offset', () => {
    const p = imageFractionToCanvasPoint(0.25, 0.5, letterbox);
    expect(p.x).toBeCloseTo(0.25 * 800 + 200);
    expect(p.y).toBeCloseTo(0.5 * 600 + 0);
  });

  it('is the exact inverse of canvasPointToImageFraction for points inside the image', () => {
    const original = { x: 0.3, y: 0.7 };
    const canvasPoint = imageFractionToCanvasPoint(original.x, original.y, letterbox);
    const roundTripped = canvasPointToImageFraction(canvasPoint.x, canvasPoint.y, letterbox);
    expect(roundTripped.x).toBeCloseTo(original.x);
    expect(roundTripped.y).toBeCloseTo(original.y);
  });
});

describe('resolveImageAspect', () => {
  const loadedImage = { naturalWidth: 2560, naturalHeight: 1920 };

  it('prefers the loaded snapshot image over the configured fallback', () => {
    const aspect = resolveImageAspect(loadedImage, true, false, 1920, 1080);
    expect(aspect).toBeCloseTo(2560 / 1920);
  });

  it('falls back to the stream\'s configured resolution while the snapshot is still loading', () => {
    // imageLoaded=false: the <img> hasn't fired onLoad yet.
    const aspect = resolveImageAspect(loadedImage, false, false, 1920, 1080);
    expect(aspect).toBeCloseTo(1920 / 1080);
  });

  it('falls back to the configured resolution when the snapshot failed to load', () => {
    const aspect = resolveImageAspect(loadedImage, true, true, 1920, 1080);
    expect(aspect).toBeCloseTo(1920 / 1080);
  });

  it('returns null when there is no image and no valid configured resolution', () => {
    expect(resolveImageAspect(null, false, false, 0, 0)).toBeNull();
    expect(resolveImageAspect(null, false, false, undefined, undefined)).toBeNull();
  });

  it('treats a zero or missing configured height as invalid rather than dividing by zero', () => {
    expect(resolveImageAspect(null, false, false, 1920, 0)).toBeNull();
  });
});
