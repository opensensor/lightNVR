import { useCallback, useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { parseEptzConfig } from '../../utils/eptz-config.js';

const VERTEX_SHADER = `#version 300 es
precision highp float;
out vec2 v_ndc;
void main() {
  vec2 position;
  if (gl_VertexID == 0) position = vec2(-1.0, -1.0);
  else if (gl_VertexID == 1) position = vec2(1.0, -1.0);
  else if (gl_VertexID == 2) position = vec2(-1.0, 1.0);
  else position = vec2(1.0, 1.0);
  v_ndc = position;
  gl_Position = vec4(position, 0.0, 1.0);
}`;

const FRAGMENT_SHADER = `#version 300 es
precision highp float;
in vec2 v_ndc;
out vec4 out_color;
uniform sampler2D u_video;
uniform vec2 u_source_size;
uniform vec2 u_output_size;
uniform vec2 u_center;
uniform float u_radius;
uniform float u_lens_fov;
uniform float u_rotation;
uniform float u_yaw;
uniform float u_tilt;
uniform float u_view_fov;

const float PI = 3.14159265358979323846;

void main() {
  float yaw = radians(u_yaw);
  float polar = radians(90.0 + u_tilt);
  float sin_polar = sin(polar);
  vec3 forward = vec3(sin_polar * cos(yaw), sin_polar * sin(yaw), cos(polar));
  vec3 right = vec3(-sin(yaw), cos(yaw), 0.0);
  vec3 up = normalize(cross(right, forward));

  float aspect = u_output_size.x / max(u_output_size.y, 1.0);
  float tangent = tan(radians(u_view_fov) * 0.5);
  vec3 ray = normalize(forward + right * (v_ndc.x * aspect * tangent) +
                       up * (v_ndc.y * tangent));

  float theta = acos(clamp(ray.z, -1.0, 1.0));
  float lens_half_fov = radians(u_lens_fov) * 0.5;
  float normalized_radius = theta / lens_half_fov;
  float radial_length = length(ray.xy);
  if (normalized_radius > 1.0 || radial_length < 0.000001) {
    if (radial_length < 0.000001 && normalized_radius <= 1.0) {
      out_color = texture(u_video, u_center);
    } else {
      out_color = vec4(0.0, 0.0, 0.0, 1.0);
    }
    return;
  }

  vec2 radial = ray.xy / radial_length;
  float c = cos(radians(u_rotation));
  float s = sin(radians(u_rotation));
  radial = mat2(c, -s, s, c) * radial;
  float minimum_source_dimension = min(u_source_size.x, u_source_size.y);
  vec2 image_scale = vec2(minimum_source_dimension / u_source_size.x,
                          minimum_source_dimension / u_source_size.y);
  vec2 source_uv = u_center + radial * normalized_radius * u_radius * image_scale;
  if (any(lessThan(source_uv, vec2(0.0))) || any(greaterThan(source_uv, vec2(1.0)))) {
    out_color = vec4(0.0, 0.0, 0.0, 1.0);
    return;
  }
  out_color = texture(u_video, source_uv);
}`;

const clamp = (value, minimum, maximum) => Math.min(maximum, Math.max(minimum, value));
const wrapDegrees = (value) => ((value + 180) % 360 + 360) % 360 - 180;

function compileShader(gl, type, source) {
  const shader = gl.createShader(type);
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const message = gl.getShaderInfoLog(shader) || 'Unknown shader compilation error';
    gl.deleteShader(shader);
    throw new Error(message);
  }
  return shader;
}

function createProgram(gl) {
  const vertex = compileShader(gl, gl.VERTEX_SHADER, VERTEX_SHADER);
  const fragment = compileShader(gl, gl.FRAGMENT_SHADER, FRAGMENT_SHADER);
  const program = gl.createProgram();
  gl.attachShader(program, vertex);
  gl.attachShader(program, fragment);
  gl.linkProgram(program);
  gl.deleteShader(vertex);
  gl.deleteShader(fragment);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    const message = gl.getProgramInfoLog(program) || 'Unknown shader link error';
    gl.deleteProgram(program);
    throw new Error(message);
  }
  return program;
}

function storageKey(streamName) {
  return `lightnvr-eptz-view:${streamName || 'camera'}`;
}

function initialView(config, streamName) {
  const fallback = {
    yaw: config.defaultYaw,
    tilt: config.defaultTilt,
    fov: config.defaultViewFov,
  };
  if (typeof localStorage === 'undefined') return fallback;
  try {
    const stored = JSON.parse(localStorage.getItem(storageKey(streamName)) || 'null');
    if (!stored || !Number.isFinite(stored.yaw) || !Number.isFinite(stored.tilt) ||
        !Number.isFinite(stored.fov)) return fallback;
    return {
      yaw: wrapDegrees(stored.yaw),
      tilt: clamp(stored.tilt, -90, 30),
      fov: clamp(stored.fov, 20, 120),
    };
  } catch {
    return fallback;
  }
}

/**
 * GPU ePTZ surface for an already-decoded video element. It never creates a
 * decoder or network connection; live and recorded players keep ownership of
 * the source video, audio, seeking, and transport health.
 */
export function FisheyeEptzCanvas({ videoRef, eptzConfig, streamName }) {
  const config = useMemo(() => parseEptzConfig(eptzConfig), [eptzConfig]);
  const canvasRef = useRef(null);
  const drawRef = useRef(() => {});
  const pointersRef = useRef(new Map());
  const gestureRef = useRef(null);
  const viewRef = useRef(initialView(config, streamName));
  const [rendererReady, setRendererReady] = useState(false);
  const [rendererError, setRendererError] = useState('');
  const [rendererEpoch, setRendererEpoch] = useState(0);
  const [viewLabel, setViewLabel] = useState(viewRef.current);

  const persistView = useCallback(() => {
    if (typeof localStorage === 'undefined') return;
    try {
      localStorage.setItem(storageKey(streamName), JSON.stringify(viewRef.current));
    } catch {
      // Private browsing or storage policy can reject writes; ePTZ still works.
    }
  }, [streamName]);

  const updateView = useCallback((next, persist = false) => {
    const view = {
      yaw: wrapDegrees(next.yaw),
      tilt: clamp(next.tilt, -90, Math.min(30, config.fov / 2 - 2)),
      fov: clamp(next.fov, 20, Math.min(120, config.fov - 2)),
    };
    viewRef.current = view;
    setViewLabel(view);
    drawRef.current();
    if (persist) persistView();
  }, [config.fov, persistView]);

  const resetView = useCallback((event) => {
    event?.preventDefault?.();
    event?.stopPropagation?.();
    updateView({
      yaw: config.defaultYaw,
      tilt: config.defaultTilt,
      fov: config.defaultViewFov,
    }, true);
  }, [config.defaultTilt, config.defaultViewFov, config.defaultYaw, updateView]);

  useEffect(() => {
    viewRef.current = initialView(config, streamName);
    setViewLabel(viewRef.current);
  }, [config.defaultTilt, config.defaultViewFov, config.defaultYaw, streamName]);

  useEffect(() => {
    if (!config.enabled) return undefined;
    const canvas = canvasRef.current;
    const video = videoRef?.current;
    if (!canvas || !video) return undefined;

    setRendererReady(false);
    setRendererError('');

    let gl;
    let program;
    let texture;
    let resizeObserver;
    let frameCallbackId = null;
    let animationFrameId = null;
    let disposed = false;
    let drawQueued = false;

    const handleContextLost = (event) => {
      event.preventDefault();
      setRendererReady(false);
      setRendererError('Graphics context was lost');
    };
    const handleContextRestored = () => {
      setRendererEpoch((value) => value + 1);
    };
    canvas.addEventListener('webglcontextlost', handleContextLost);
    canvas.addEventListener('webglcontextrestored', handleContextRestored);

    try {
      gl = canvas.getContext('webgl2', {
        alpha: false,
        antialias: false,
        depth: false,
        stencil: false,
        powerPreference: 'high-performance',
      });
      if (!gl) throw new Error('WebGL2 is unavailable');
      program = createProgram(gl);
      texture = gl.createTexture();
      if (!texture) throw new Error('Unable to allocate the video texture');
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false);
      gl.useProgram(program);
      gl.uniform1i(gl.getUniformLocation(program, 'u_video'), 0);
    } catch (error) {
      console.warn(`[ePTZ ${streamName}] renderer unavailable:`, error);
      setRendererError(error.message || 'WebGL2 renderer unavailable');
      canvas.removeEventListener('webglcontextlost', handleContextLost);
      canvas.removeEventListener('webglcontextrestored', handleContextRestored);
      if (gl && texture) gl.deleteTexture(texture);
      if (gl && program) gl.deleteProgram(program);
      return undefined;
    }

    const locations = {};
    ['source_size', 'output_size', 'center', 'radius', 'lens_fov', 'rotation',
      'yaw', 'tilt', 'view_fov'].forEach((name) => {
      locations[name] = gl.getUniformLocation(program, `u_${name}`);
    });

    const resize = () => {
      const rect = canvas.getBoundingClientRect();
      const ratio = Math.min(window.devicePixelRatio || 1, 2);
      const width = Math.max(1, Math.round(rect.width * ratio));
      const height = Math.max(1, Math.round(rect.height * ratio));
      if (canvas.width !== width || canvas.height !== height) {
        canvas.width = width;
        canvas.height = height;
      }
      gl.viewport(0, 0, canvas.width, canvas.height);
    };

    const draw = () => {
      drawQueued = false;
      if (disposed || !video.videoWidth || !video.videoHeight ||
          video.readyState < HTMLMediaElement.HAVE_CURRENT_DATA || gl.isContextLost()) return;
      try {
        resize();
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA,
                      gl.UNSIGNED_BYTE, video);
        gl.useProgram(program);
        gl.uniform2f(locations.source_size, video.videoWidth, video.videoHeight);
        gl.uniform2f(locations.output_size, canvas.width, canvas.height);
        gl.uniform2f(locations.center, config.centerX, config.centerY);
        gl.uniform1f(locations.radius, config.radius);
        gl.uniform1f(locations.lens_fov, config.fov);
        gl.uniform1f(locations.rotation, config.rotation);
        gl.uniform1f(locations.yaw, viewRef.current.yaw);
        gl.uniform1f(locations.tilt, viewRef.current.tilt);
        gl.uniform1f(locations.view_fov, viewRef.current.fov);
        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
        setRendererReady(true);
        setRendererError((current) => current ? '' : current);
      } catch (error) {
        console.warn(`[ePTZ ${streamName}] frame upload failed:`, error);
        setRendererReady(false);
        setRendererError(error.message || 'Unable to render this video source');
      }
    };

    const queueDraw = () => {
      if (drawQueued || disposed) return;
      drawQueued = true;
      animationFrameId = requestAnimationFrame(draw);
    };
    drawRef.current = queueDraw;

    const onVideoFrame = () => {
      draw();
      if (!disposed) frameCallbackId = video.requestVideoFrameCallback(onVideoFrame);
    };
    const onMediaEvent = () => queueDraw();
    const onVideoUnavailable = () => setRendererReady(false);
    video.addEventListener('emptied', onVideoUnavailable);
    video.addEventListener('error', onVideoUnavailable);
    if (typeof video.requestVideoFrameCallback === 'function') {
      frameCallbackId = video.requestVideoFrameCallback(onVideoFrame);
    } else {
      video.addEventListener('loadeddata', onMediaEvent);
      video.addEventListener('timeupdate', onMediaEvent);
      video.addEventListener('seeked', onMediaEvent);
      video.addEventListener('playing', onMediaEvent);
      const fallbackLoop = () => {
        if (disposed) return;
        if (!video.paused) draw();
        animationFrameId = requestAnimationFrame(fallbackLoop);
      };
      animationFrameId = requestAnimationFrame(fallbackLoop);
    }

    if (typeof ResizeObserver !== 'undefined') {
      resizeObserver = new ResizeObserver(queueDraw);
      resizeObserver.observe(canvas);
    }
    queueDraw();

    return () => {
      disposed = true;
      drawRef.current = () => {};
      canvas.removeEventListener('webglcontextlost', handleContextLost);
      canvas.removeEventListener('webglcontextrestored', handleContextRestored);
      resizeObserver?.disconnect();
      if (frameCallbackId !== null && typeof video.cancelVideoFrameCallback === 'function') {
        video.cancelVideoFrameCallback(frameCallbackId);
      }
      if (animationFrameId !== null) cancelAnimationFrame(animationFrameId);
      video.removeEventListener('loadeddata', onMediaEvent);
      video.removeEventListener('timeupdate', onMediaEvent);
      video.removeEventListener('seeked', onMediaEvent);
      video.removeEventListener('playing', onMediaEvent);
      video.removeEventListener('emptied', onVideoUnavailable);
      video.removeEventListener('error', onVideoUnavailable);
      if (gl && texture) gl.deleteTexture(texture);
      if (gl && program) gl.deleteProgram(program);
    };
  }, [config.centerX, config.centerY, config.enabled, config.fov, config.radius,
    config.rotation, rendererEpoch, streamName, videoRef]);

  const pointerPosition = (event) => ({ x: event.clientX, y: event.clientY });
  const handlePointerDown = (event) => {
    event.stopPropagation();
    event.currentTarget.focus({ preventScroll: true });
    pointersRef.current.set(event.pointerId, pointerPosition(event));
    event.currentTarget.setPointerCapture?.(event.pointerId);
    const pointers = Array.from(pointersRef.current.values());
    if (pointers.length === 2) {
      gestureRef.current = {
        type: 'pinch',
        distance: Math.hypot(pointers[0].x - pointers[1].x, pointers[0].y - pointers[1].y),
        startFov: viewRef.current.fov,
      };
    } else {
      gestureRef.current = {
        type: 'pan', pointerId: event.pointerId,
        x: event.clientX, y: event.clientY,
        start: { ...viewRef.current },
      };
    }
  };

  const handlePointerMove = (event) => {
    if (!pointersRef.current.has(event.pointerId)) return;
    event.preventDefault();
    event.stopPropagation();
    pointersRef.current.set(event.pointerId, pointerPosition(event));
    const gesture = gestureRef.current;
    const pointers = Array.from(pointersRef.current.values());
    if (gesture?.type === 'pinch' && pointers.length >= 2) {
      const distance = Math.hypot(pointers[0].x - pointers[1].x, pointers[0].y - pointers[1].y);
      if (gesture.distance > 0) {
        updateView({ ...viewRef.current, fov: gesture.startFov * gesture.distance / distance });
      }
      return;
    }
    if (gesture?.type === 'pan' && gesture.pointerId === event.pointerId) {
      const rect = event.currentTarget.getBoundingClientRect();
      const degreesPerHeight = gesture.start.fov / Math.max(rect.height, 1);
      updateView({
        ...gesture.start,
        yaw: gesture.start.yaw - (event.clientX - gesture.x) * degreesPerHeight,
        tilt: gesture.start.tilt + (event.clientY - gesture.y) * degreesPerHeight,
      });
    }
  };

  const handlePointerEnd = (event) => {
    event.stopPropagation();
    pointersRef.current.delete(event.pointerId);
    event.currentTarget.releasePointerCapture?.(event.pointerId);
    const remaining = Array.from(pointersRef.current.entries());
    if (remaining.length === 1) {
      const [pointerId, point] = remaining[0];
      gestureRef.current = {
        type: 'pan', pointerId, x: point.x, y: point.y,
        start: { ...viewRef.current },
      };
    } else if (remaining.length === 0) {
      gestureRef.current = null;
      persistView();
    }
  };

  const handleWheel = (event) => {
    event.preventDefault();
    event.stopPropagation();
    updateView({
      ...viewRef.current,
      fov: viewRef.current.fov * Math.exp(event.deltaY * 0.0015),
    }, true);
  };

  const handleKeyDown = (event) => {
    const view = viewRef.current;
    const step = event.shiftKey ? 10 : 3;
    let next = null;
    if (event.key === 'ArrowLeft') next = { ...view, yaw: view.yaw - step };
    if (event.key === 'ArrowRight') next = { ...view, yaw: view.yaw + step };
    if (event.key === 'ArrowUp') next = { ...view, tilt: view.tilt + step };
    if (event.key === 'ArrowDown') next = { ...view, tilt: view.tilt - step };
    if (event.key === '+' || event.key === '=') next = { ...view, fov: view.fov * 0.9 };
    if (event.key === '-' || event.key === '_') next = { ...view, fov: view.fov * 1.1 };
    if (event.key === 'Home') {
      resetView(event);
      return;
    }
    if (next) {
      event.preventDefault();
      event.stopPropagation();
      updateView(next, true);
    }
  };

  if (!config.enabled) return null;
  const zoom = config.defaultViewFov / viewLabel.fov;

  return (
    <>
      <canvas
        ref={canvasRef}
        className="absolute inset-0 w-full h-full"
        data-testid="fisheye-eptz-canvas"
        aria-hidden="true"
        style={{ zIndex: 2, pointerEvents: 'none', opacity: rendererReady ? 1 : 0 }}
      />
      {rendererReady && (
        <div
          className="absolute inset-0 outline-none"
          role="application"
          tabIndex="0"
          aria-label="Fisheye electronic pan, tilt, and zoom view"
          title="Drag to pan and tilt; wheel or pinch to zoom; Home resets"
          style={{ zIndex: 3, touchAction: 'none', cursor: 'grab' }}
          onPointerDown={handlePointerDown}
          onPointerMove={handlePointerMove}
          onPointerUp={handlePointerEnd}
          onPointerCancel={handlePointerEnd}
          onWheel={handleWheel}
          onKeyDown={handleKeyDown}
          onClick={(event) => event.stopPropagation()}
        />
      )}
      {rendererReady && (
        <button
          type="button"
          className="absolute px-2 py-1 rounded text-white text-xs"
          style={{
            zIndex: 8,
            left: '50%',
            bottom: '10px',
            transform: 'translateX(-50%)',
            background: 'rgba(0, 0, 0, 0.65)',
          }}
          title="Reset ePTZ view"
          aria-label="Reset ePTZ view"
          onClick={resetView}
        >
          ePTZ {zoom.toFixed(1)}× ↺
        </button>
      )}
      {rendererError && (
        <span
          className="absolute bottom-2 left-2 rounded bg-amber-700/90 px-2 py-1 text-xs text-white"
          style={{ zIndex: 8 }}
          title={rendererError}
        >
          ePTZ unavailable
        </span>
      )}
    </>
  );
}

export default FisheyeEptzCanvas;
