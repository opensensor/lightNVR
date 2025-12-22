/**
 * Zone Editor Component
 * Interactive canvas-based zone configuration for detection regions
 */

import { useState, useRef, useEffect } from 'react';
import { getGo2rtcApiPort } from '../../utils/settings-utils.js';

/**
 * ZoneEditor Component
 * Allows users to draw and edit detection zones on a stream preview
 */
export function ZoneEditor({ streamName, zones = [], onZonesChange, onClose }) {
  const canvasRef = useRef(null);
  const imageRef = useRef(null);
  const [currentZone, setCurrentZone] = useState(null);
  const [selectedZoneIndex, setSelectedZoneIndex] = useState(null);
  const [isDrawing, setIsDrawing] = useState(false);
  const [imageLoaded, setImageLoaded] = useState(false);
  const [snapshotUrl, setSnapshotUrl] = useState(null);
  const [snapshotError, setSnapshotError] = useState(false);
  const [editMode, setEditMode] = useState('draw'); // 'draw', 'edit', 'delete'
  const [zoneList, setZoneList] = useState(zones);
  const [hoveredPoint, setHoveredPoint] = useState(null);
  const [draggedPoint, setDraggedPoint] = useState(null); // { zoneIndex, pointIndex }
  const [draggedZone, setDraggedZone] = useState(null); // { zoneIndex, startX, startY }

  // Load zones from the backend API
  useEffect(() => {
    const loadZones = async () => {
      try {
        const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/zones`);
        if (response.ok) {
          const data = await response.json();
          if (data.zones && Array.isArray(data.zones)) {
            console.log('Loaded zones from API:', data.zones);
            setZoneList(data.zones);
          }
        } else {
          console.warn('Failed to load zones:', response.status);
        }
      } catch (error) {
        console.error('Error loading zones:', error);
      }
    };

    if (streamName) {
      loadZones();
    }
  }, [streamName]);

  // Load snapshot for the stream
  useEffect(() => {
    if (streamName) {
      // Use go2rtc's snapshot API endpoint directly
      // Get the port from settings (async)
      const loadSnapshot = async () => {
        try {
          const go2rtcPort = await getGo2rtcApiPort();
          // Add cache-busting parameter to force fresh snapshot
          const timestamp = Date.now();
          const url = `http://${window.location.hostname}:${go2rtcPort}/api/frame.jpeg?src=${encodeURIComponent(streamName)}&t=${timestamp}`;
          console.log('Loading snapshot from:', url);
          setSnapshotUrl(url);
        } catch (err) {
          console.warn('Failed to get go2rtc port from settings, using default 1984:', err);
          const timestamp = Date.now();
          const url = `http://${window.location.hostname}:1984/api/frame.jpeg?src=${encodeURIComponent(streamName)}&t=${timestamp}`;
          setSnapshotUrl(url);
        }
      };

      loadSnapshot();

      // Set a timeout to show canvas anyway after 10 seconds
      const timeout = setTimeout(() => {
        if (!imageLoaded) {
          console.warn('Snapshot not loaded, showing canvas without background');
          setImageLoaded(true);
          setSnapshotError(true);
        }
      }, 10000);

      return () => clearTimeout(timeout);
    }
  }, [streamName, imageLoaded]);

  // Draw zones on canvas
  const drawCanvas = () => {
    const canvas = canvasRef.current;
    const image = imageRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const rect = canvas.getBoundingClientRect();

    // Set canvas size to match display size
    canvas.width = rect.width;
    canvas.height = rect.height;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw the image if loaded
    if (image && imageLoaded && !snapshotError) {
      ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
    } else {
      // Draw a placeholder background
      ctx.fillStyle = '#1a1a1a';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw grid
      ctx.strokeStyle = '#333';
      ctx.lineWidth = 1;
      for (let i = 0; i < canvas.width; i += 50) {
        ctx.beginPath();
        ctx.moveTo(i, 0);
        ctx.lineTo(i, canvas.height);
        ctx.stroke();
      }
      for (let i = 0; i < canvas.height; i += 50) {
        ctx.beginPath();
        ctx.moveTo(0, i);
        ctx.lineTo(canvas.width, i);
        ctx.stroke();
      }
    }

    // Draw existing zones
    zoneList.forEach((zone, index) => {
      const isSelected = index === selectedZoneIndex;
      drawZone(ctx, zone, canvas.width, canvas.height, isSelected, false, index);
    });

    // Draw current zone being drawn
    if (currentZone && currentZone.polygon.length > 0) {
      drawZone(ctx, currentZone, canvas.width, canvas.height, true, true, null);
    }
  };

  // Draw a single zone
  const drawZone = (ctx, zone, width, height, isSelected, isDrawing = false, zoneIndex = null) => {
    if (!zone.polygon || zone.polygon.length === 0) return;

    ctx.save();

    // Draw polygon
    ctx.beginPath();
    zone.polygon.forEach((point, i) => {
      const x = point.x * width;
      const y = point.y * height;
      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });

    if (!isDrawing) {
      ctx.closePath();
    }

    // Fill with semi-transparent color
    const color = zone.color || '#3b82f6';
    ctx.fillStyle = isSelected ? `${color}40` : `${color}20`;
    ctx.fill();

    // Stroke
    ctx.strokeStyle = isSelected ? color : `${color}80`;
    ctx.lineWidth = isSelected ? 3 : 2;
    ctx.stroke();

    // Draw points
    zone.polygon.forEach((point, i) => {
      const x = point.x * width;
      const y = point.y * height;

      // Check if this point is being hovered or dragged
      const isHovered = hoveredPoint && hoveredPoint.zoneIndex === zoneIndex && hoveredPoint.pointIndex === i;
      const isDragged = draggedPoint && draggedPoint.zoneIndex === zoneIndex && draggedPoint.pointIndex === i;

      ctx.beginPath();
      ctx.arc(x, y, (isHovered || isDragged) ? 8 : (isSelected ? 6 : 4), 0, 2 * Math.PI);
      ctx.fillStyle = (isHovered || isDragged) ? '#ffffff' : (isSelected ? color : '#ffffff');
      ctx.fill();
      ctx.strokeStyle = color;
      ctx.lineWidth = (isHovered || isDragged) ? 3 : 2;
      ctx.stroke();
    });

    // Draw zone label
    if (zone.name && zone.polygon.length > 0) {
      const centerX = zone.polygon.reduce((sum, p) => sum + p.x, 0) / zone.polygon.length * width;
      const centerY = zone.polygon.reduce((sum, p) => sum + p.y, 0) / zone.polygon.length * height;

      ctx.fillStyle = color;
      ctx.font = 'bold 14px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';

      // Draw text background
      const textMetrics = ctx.measureText(zone.name);
      ctx.fillStyle = '#ffffff';
      ctx.fillRect(centerX - textMetrics.width / 2 - 4, centerY - 10, textMetrics.width + 8, 20);

      ctx.fillStyle = color;
      ctx.fillText(zone.name, centerX, centerY);
    }

    ctx.restore();
  };

  // Redraw canvas when zones or image changes
  useEffect(() => {
    drawCanvas();
  }, [zoneList, currentZone, selectedZoneIndex, imageLoaded, hoveredPoint, draggedPoint, draggedZone]);

  // Handle mouse down
  const handleMouseDown = (e) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = (e.clientY - rect.top) / rect.height;

    if (editMode === 'edit') {
      // Check if clicking on a point to drag
      const point = findPointNearCursor(x, y);
      if (point) {
        setDraggedPoint(point);
        setSelectedZoneIndex(point.zoneIndex);
        return;
      }

      // Check if clicking inside a zone to drag the whole zone
      const clickedZoneIndex = findZoneAtPoint(x, y);
      if (clickedZoneIndex !== null) {
        setDraggedZone({ zoneIndex: clickedZoneIndex, startX: x, startY: y });
        setSelectedZoneIndex(clickedZoneIndex);
        return;
      }

      // Otherwise deselect
      setSelectedZoneIndex(null);
    }
  };

  // Handle mouse move
  const handleMouseMove = (e) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = (e.clientY - rect.top) / rect.height;

    // Update cursor style based on hover
    if (editMode === 'edit' && !draggedPoint && !draggedZone) {
      const point = findPointNearCursor(x, y);
      if (point) {
        canvas.style.cursor = 'move';
        setHoveredPoint(point);
      } else {
        const zoneIndex = findZoneAtPoint(x, y);
        canvas.style.cursor = zoneIndex !== null ? 'grab' : 'default';
        setHoveredPoint(null);
      }
    }

    // Handle point dragging
    if (draggedPoint) {
      canvas.style.cursor = 'move';
      const newZones = [...zoneList];
      newZones[draggedPoint.zoneIndex].polygon[draggedPoint.pointIndex] = { x, y };
      setZoneList(newZones);
    }

    // Handle zone dragging
    if (draggedZone) {
      canvas.style.cursor = 'grabbing';
      const deltaX = x - draggedZone.startX;
      const deltaY = y - draggedZone.startY;

      const newZones = [...zoneList];
      const zone = newZones[draggedZone.zoneIndex];

      // Move all points by the delta
      zone.polygon = zone.polygon.map(point => ({
        x: Math.max(0, Math.min(1, point.x + deltaX)),
        y: Math.max(0, Math.min(1, point.y + deltaY))
      }));

      setZoneList(newZones);
      // Update start position for next move
      setDraggedZone({ ...draggedZone, startX: x, startY: y });
    }
  };

  // Handle mouse up
  const handleMouseUp = () => {
    setDraggedPoint(null);
    setDraggedZone(null);
    const canvas = canvasRef.current;
    if (canvas) {
      canvas.style.cursor = 'default';
    }
  };

  // Handle canvas click (for draw and delete modes)
  const handleCanvasClick = (e) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = (e.clientY - rect.top) / rect.height;

    if (editMode === 'draw') {
      // Add point to current zone
      if (!currentZone) {
        setCurrentZone({
          id: `zone_${Date.now()}`,
          name: `Zone ${zoneList.length + 1}`,
          polygon: [{ x, y }],
          enabled: true,
          color: '#3b82f6'
        });
      } else {
        setCurrentZone({
          ...currentZone,
          polygon: [...currentZone.polygon, { x, y }]
        });
      }
    } else if (editMode === 'delete') {
      // Delete zone
      const clickedZoneIndex = findZoneAtPoint(x, y);
      if (clickedZoneIndex !== null) {
        const newZones = zoneList.filter((_, i) => i !== clickedZoneIndex);
        setZoneList(newZones);
        setSelectedZoneIndex(null);
      }
    }
  };

  // Find zone at point
  const findZoneAtPoint = (x, y) => {
    for (let i = zoneList.length - 1; i >= 0; i--) {
      const zone = zoneList[i];
      if (isPointInPolygon({ x, y }, zone.polygon)) {
        return i;
      }
    }
    return null;
  };

  // Find point near cursor (for dragging)
  const findPointNearCursor = (x, y, threshold = 0.02) => {
    for (let i = 0; i < zoneList.length; i++) {
      const zone = zoneList[i];
      for (let j = 0; j < zone.polygon.length; j++) {
        const point = zone.polygon[j];
        const distance = Math.sqrt(Math.pow(point.x - x, 2) + Math.pow(point.y - y, 2));
        if (distance < threshold) {
          return { zoneIndex: i, pointIndex: j };
        }
      }
    }
    return null;
  };

  // Check if point is in polygon
  const isPointInPolygon = (point, polygon) => {
    let inside = false;
    for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
      const xi = polygon[i].x, yi = polygon[i].y;
      const xj = polygon[j].x, yj = polygon[j].y;
      
      const intersect = ((yi > point.y) !== (yj > point.y))
        && (point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi);
      if (intersect) inside = !inside;
    }
    return inside;
  };

  // Complete current zone
  const completeZone = () => {
    if (currentZone && currentZone.polygon.length >= 3) {
      setZoneList([...zoneList, currentZone]);
      setCurrentZone(null);
      setSelectedZoneIndex(zoneList.length);
    }
  };

  // Cancel current zone
  const cancelZone = () => {
    setCurrentZone(null);
  };

  // Delete selected zone
  const deleteSelectedZone = () => {
    if (selectedZoneIndex !== null) {
      const newZones = zoneList.filter((_, i) => i !== selectedZoneIndex);
      setZoneList(newZones);
      setSelectedZoneIndex(null);
    }
  };

  // Save zones
  const handleSave = async () => {
    try {
      // Save zones to the backend API
      const response = await fetch(`/api/streams/${encodeURIComponent(streamName)}/zones`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ zones: zoneList }),
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({ error: 'Unknown error' }));
        throw new Error(errorData.error || `HTTP error ${response.status}`);
      }

      const result = await response.json();
      console.log('Zones saved successfully:', result);

      // Update parent component
      onZonesChange(zoneList);
      onClose();
    } catch (error) {
      console.error('Failed to save zones:', error);
      alert(`Failed to save zones: ${error.message}`);
    }
  };

  // Update zone property
  const updateZoneProperty = (index, property, value) => {
    const newZones = [...zoneList];
    newZones[index] = { ...newZones[index], [property]: value };
    setZoneList(newZones);
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-[60] p-4">
      <div className="bg-card text-card-foreground rounded-lg shadow-xl w-full max-w-7xl max-h-[95vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-border">
          <div>
            <h3 className="text-2xl font-bold">Detection Zone Editor</h3>
            <p className="text-sm text-muted-foreground mt-1">
              Draw zones where you want to detect objects for {streamName}
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground transition-colors p-2 rounded-full hover:bg-muted"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden flex">
          {/* Canvas Area */}
          <div className="flex-1 p-6 flex flex-col">
            <div className="flex-1 relative bg-muted rounded-lg overflow-hidden">
              {snapshotUrl && (
                <img
                  ref={imageRef}
                  src={snapshotUrl}
                  alt="Stream preview"
                  className="hidden"
                  crossOrigin="anonymous"
                  onLoad={() => {
                    console.log('✅ Snapshot loaded successfully');
                    setImageLoaded(true);
                    setSnapshotError(false);
                    drawCanvas();
                  }}
                  onError={(e) => {
                    console.error('❌ Failed to load snapshot:', e);
                    console.error('Snapshot URL was:', snapshotUrl);
                    setSnapshotError(true);
                    setImageLoaded(true); // Show canvas anyway
                  }}
                />
              )}
              <canvas
                ref={canvasRef}
                onClick={handleCanvasClick}
                onMouseDown={handleMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                onMouseLeave={handleMouseUp}
                className="w-full h-full cursor-crosshair"
                style={{ display: imageLoaded ? 'block' : 'none', minHeight: '400px' }}
              />
              {!imageLoaded && (
                <div className="absolute inset-0 flex items-center justify-center flex-col space-y-2">
                  <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
                  <p className="text-muted-foreground">Loading stream snapshot...</p>
                  <p className="text-xs text-muted-foreground">Using go2rtc snapshot API</p>
                </div>
              )}
              {imageLoaded && snapshotError && (
                <div className="absolute top-4 left-4 bg-yellow-500/20 border border-yellow-500 text-yellow-200 px-3 py-2 rounded text-sm">
                  ⚠️ Snapshot unavailable - drawing on grid background
                </div>
              )}
            </div>

            {/* Toolbar */}
            <div className="mt-4 flex items-center justify-between bg-muted p-3 rounded-lg">
              <div className="flex space-x-2">
                <button
                  onClick={() => setEditMode('draw')}
                  className={`px-4 py-2 rounded ${editMode === 'draw' ? 'bg-primary text-primary-foreground' : 'bg-background'}`}
                >
                  ✏️ Draw Zone
                </button>
                <button
                  onClick={() => setEditMode('edit')}
                  className={`px-4 py-2 rounded ${editMode === 'edit' ? 'bg-primary text-primary-foreground' : 'bg-background'}`}
                >
                  ✋ Select
                </button>
                <button
                  onClick={() => setEditMode('delete')}
                  className={`px-4 py-2 rounded ${editMode === 'delete' ? 'bg-danger text-white' : 'bg-background'}`}
                >
                  🗑️ Delete
                </button>
              </div>
              
              {currentZone && (
                <div className="flex space-x-2">
                  <button onClick={completeZone} className="btn-success">
                    Complete Zone ({currentZone.polygon.length} points)
                  </button>
                  <button onClick={cancelZone} className="btn-secondary">
                    Cancel
                  </button>
                </div>
              )}
            </div>
          </div>

          {/* Sidebar - Zone List */}
          <div className="w-80 border-l border-border p-6 overflow-y-auto">
            <h4 className="font-semibold mb-4">Zones ({zoneList.length})</h4>
            
            {zoneList.length === 0 && (
              <p className="text-sm text-muted-foreground">
                No zones defined. Click "Draw Zone" and click on the canvas to create a zone.
              </p>
            )}

            {zoneList.map((zone, index) => (
              <div
                key={zone.id}
                className={`mb-3 p-3 rounded border ${
                  index === selectedZoneIndex ? 'border-primary bg-primary/10' : 'border-border'
                }`}
                onClick={() => setSelectedZoneIndex(index)}
              >
                <input
                  type="text"
                  value={zone.name}
                  onChange={(e) => updateZoneProperty(index, 'name', e.target.value)}
                  className="w-full mb-2 px-2 py-1 border border-input rounded bg-background text-foreground"
                  placeholder="Zone name"
                />
                
                <div className="flex items-center space-x-2 mb-2">
                  <label className="text-sm">Color:</label>
                  <input
                    type="color"
                    value={zone.color || '#3b82f6'}
                    onChange={(e) => updateZoneProperty(index, 'color', e.target.value)}
                    className="w-12 h-8 rounded cursor-pointer"
                  />
                </div>

                <div className="flex items-center space-x-2 mb-2">
                  <input
                    type="checkbox"
                    checked={zone.enabled}
                    onChange={(e) => updateZoneProperty(index, 'enabled', e.target.checked)}
                    id={`zone-enabled-${index}`}
                  />
                  <label htmlFor={`zone-enabled-${index}`} className="text-sm">Enabled</label>
                </div>

                <p className="text-xs text-muted-foreground">
                  {zone.polygon.length} points
                </p>
              </div>
            ))}
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end space-x-3 p-6 border-t border-border">
          <button onClick={onClose} className="btn-secondary">
            Cancel
          </button>
          <button onClick={handleSave} className="btn-primary">
            Save Zones
          </button>
        </div>
      </div>
    </div>
  );
}

