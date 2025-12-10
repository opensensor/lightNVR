/**
 * PTZ Controls Component
 * Pan-Tilt-Zoom controls for PTZ-enabled cameras
 */

import { h } from 'preact';
import { useState, useEffect, useRef, useCallback } from 'preact/hooks';

/**
 * PTZ API functions
 */
const ptzApi = {
  async move(streamName, pan, tilt, zoom) {
    const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/ptz/move`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ pan, tilt, zoom })
    });
    return response.json();
  },

  async stop(streamName) {
    const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/ptz/stop`, {
      method: 'POST'
    });
    return response.json();
  },

  async home(streamName) {
    const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/ptz/home`, {
      method: 'POST'
    });
    return response.json();
  },

  async getPresets(streamName) {
    const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/ptz/presets`);
    return response.json();
  },

  async gotoPreset(streamName, token) {
    const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/ptz/preset`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token })
    });
    return response.json();
  },

  async getCapabilities(streamName) {
    const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/ptz/capabilities`);
    return response.json();
  }
};

/**
 * Direction button component
 */
function DirectionButton({ direction, onMouseDown, onMouseUp, onMouseLeave, disabled }) {
  const icons = {
    up: '▲',
    down: '▼',
    left: '◀',
    right: '▶',
    'zoom-in': '+',
    'zoom-out': '−'
  };

  return (
    <button
      className={`ptz-btn ptz-btn-${direction}`}
      onMouseDown={onMouseDown}
      onMouseUp={onMouseUp}
      onMouseLeave={onMouseLeave}
      onTouchStart={onMouseDown}
      onTouchEnd={onMouseUp}
      disabled={disabled}
      style={{
        width: '40px',
        height: '40px',
        borderRadius: '50%',
        border: 'none',
        backgroundColor: 'rgba(59, 130, 246, 0.8)',
        color: 'white',
        fontSize: '16px',
        cursor: disabled ? 'not-allowed' : 'pointer',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        transition: 'background-color 0.2s',
        opacity: disabled ? 0.5 : 1
      }}
    >
      {icons[direction]}
    </button>
  );
}

/**
 * PTZ Controls component
 */
export function PTZControls({ stream, isVisible = true, onClose }) {
  const [speed, setSpeed] = useState(0.5);
  const [presets, setPresets] = useState([]);
  const [capabilities, setCapabilities] = useState(null);
  const [isMoving, setIsMoving] = useState(false);
  const [error, setError] = useState(null);
  const moveTimeoutRef = useRef(null);

  // Load capabilities and presets
  useEffect(() => {
    if (!stream?.name || !stream?.ptz_enabled) return;

    ptzApi.getCapabilities(stream.name)
      .then(setCapabilities)
      .catch(err => console.error('Failed to get PTZ capabilities:', err));

    ptzApi.getPresets(stream.name)
      .then(data => setPresets(data.presets || []))
      .catch(err => console.error('Failed to get PTZ presets:', err));
  }, [stream?.name, stream?.ptz_enabled]);

  // Handle continuous move
  const handleMoveStart = useCallback((pan, tilt, zoom) => {
    if (!stream?.name) return;
    setIsMoving(true);
    setError(null);
    
    ptzApi.move(stream.name, pan * speed, tilt * speed, zoom * speed)
      .catch(err => {
        setError('Move failed');
        console.error('PTZ move error:', err);
      });
  }, [stream?.name, speed]);

  const handleMoveStop = useCallback(() => {
    if (!stream?.name || !isMoving) return;
    setIsMoving(false);
    
    ptzApi.stop(stream.name)
      .catch(err => console.error('PTZ stop error:', err));
  }, [stream?.name, isMoving]);

  const handleHome = useCallback(() => {
    if (!stream?.name) return;
    setError(null);
    
    ptzApi.home(stream.name)
      .catch(err => {
        setError('Home failed');
        console.error('PTZ home error:', err);
      });
  }, [stream?.name]);

  const handlePreset = useCallback((token) => {
    if (!stream?.name) return;
    setError(null);
    
    ptzApi.gotoPreset(stream.name, token)
      .catch(err => {
        setError('Preset failed');
        console.error('PTZ preset error:', err);
      });
  }, [stream?.name]);

  if (!isVisible || !stream?.ptz_enabled) return null;

  return (
    <div
      className="ptz-controls"
      style={{
        position: 'absolute',
        bottom: '60px',
        left: '10px',
        backgroundColor: 'rgba(0, 0, 0, 0.75)',
        borderRadius: '8px',
        padding: '12px',
        zIndex: 10,
        display: 'flex',
        flexDirection: 'column',
        gap: '8px',
        minWidth: '160px'
      }}
    >
      {/* Header with close button */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '4px' }}>
        <span style={{ color: 'white', fontSize: '12px', fontWeight: 'bold' }}>PTZ Control</span>
        {onClose && (
          <button
            onClick={onClose}
            style={{
              background: 'none',
              border: 'none',
              color: 'white',
              cursor: 'pointer',
              fontSize: '16px',
              padding: '0 4px'
            }}
          >
            ×
          </button>
        )}
      </div>

      {/* Direction pad */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 40px)', gap: '4px', justifyContent: 'center' }}>
        <div></div>
        <DirectionButton
          direction="up"
          onMouseDown={() => handleMoveStart(0, 1, 0)}
          onMouseUp={handleMoveStop}
          onMouseLeave={handleMoveStop}
        />
        <div></div>
        <DirectionButton
          direction="left"
          onMouseDown={() => handleMoveStart(-1, 0, 0)}
          onMouseUp={handleMoveStop}
          onMouseLeave={handleMoveStop}
        />
        <button
          onClick={handleHome}
          style={{
            width: '40px',
            height: '40px',
            borderRadius: '50%',
            border: 'none',
            backgroundColor: 'rgba(107, 114, 128, 0.8)',
            color: 'white',
            fontSize: '10px',
            cursor: 'pointer'
          }}
          title="Go to Home Position"
        >
          ⌂
        </button>
        <DirectionButton
          direction="right"
          onMouseDown={() => handleMoveStart(1, 0, 0)}
          onMouseUp={handleMoveStop}
          onMouseLeave={handleMoveStop}
        />
        <div></div>
        <DirectionButton
          direction="down"
          onMouseDown={() => handleMoveStart(0, -1, 0)}
          onMouseUp={handleMoveStop}
          onMouseLeave={handleMoveStop}
        />
        <div></div>
      </div>

      {/* Zoom controls */}
      <div style={{ display: 'flex', justifyContent: 'center', gap: '8px', marginTop: '4px' }}>
        <DirectionButton
          direction="zoom-out"
          onMouseDown={() => handleMoveStart(0, 0, -1)}
          onMouseUp={handleMoveStop}
          onMouseLeave={handleMoveStop}
        />
        <span style={{ color: 'white', fontSize: '11px', alignSelf: 'center' }}>Zoom</span>
        <DirectionButton
          direction="zoom-in"
          onMouseDown={() => handleMoveStart(0, 0, 1)}
          onMouseUp={handleMoveStop}
          onMouseLeave={handleMoveStop}
        />
      </div>

      {/* Speed slider */}
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginTop: '4px' }}>
        <span style={{ color: 'white', fontSize: '11px' }}>Speed:</span>
        <input
          type="range"
          min="0.1"
          max="1"
          step="0.1"
          value={speed}
          onChange={(e) => setSpeed(parseFloat(e.target.value))}
          style={{ flex: 1 }}
        />
        <span style={{ color: 'white', fontSize: '11px', width: '30px' }}>{Math.round(speed * 100)}%</span>
      </div>

      {/* Presets dropdown */}
      {presets.length > 0 && (
        <div style={{ marginTop: '4px' }}>
          <select
            onChange={(e) => e.target.value && handlePreset(e.target.value)}
            style={{
              width: '100%',
              padding: '6px',
              borderRadius: '4px',
              border: 'none',
              backgroundColor: 'rgba(59, 130, 246, 0.8)',
              color: 'white',
              fontSize: '12px',
              cursor: 'pointer'
            }}
          >
            <option value="">Go to Preset...</option>
            {presets.map(preset => (
              <option key={preset.token} value={preset.token}>
                {preset.name || `Preset ${preset.token}`}
              </option>
            ))}
          </select>
        </div>
      )}

      {/* Error message */}
      {error && (
        <div style={{ color: '#ef4444', fontSize: '11px', textAlign: 'center' }}>
          {error}
        </div>
      )}
    </div>
  );
}

