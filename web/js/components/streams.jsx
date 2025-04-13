/**
 * LightNVR Web Interface Stream Management
 * Contains functionality for managing streams (add, edit, delete, test)
 * Using our custom query client for data fetching
 */
import { h, Fragment } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
import {
  useQuery,
  useMutation,
  useQueryClient,
  fetchJSON
} from '../query-client.js';

// API functions
const fetchStreams = async () => {
  return await fetchJSON('/api/streams');
};

const fetchStreamDetails = async (streamId) => {
  return await fetchJSON(`/api/streams/${encodeURIComponent(streamId)}`);
};

const fetchDetectionModels = async () => {
  return await fetchJSON('/api/detection/models');
};

const testStreamConnection = async (url) => {
  return await fetchJSON('/api/streams/test', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url })
  });
};

// Stream components
export function StreamsTable() {
  const queryClient = useQueryClient();
  const { data: streams, isLoading, error } = useQuery({
    queryKey: ['streams'],
    queryFn: fetchStreams
  });

  if (isLoading) return <div className="loading">Loading streams...</div>;
  if (error) return <div className="error">Error loading streams: {error.message}</div>;
  if (!streams || streams.length === 0) {
    return <div className="empty-message">No streams configured</div>;
  }

  return (
    <table className="streams-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>URL</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {streams.map(stream => (
          <tr key={stream.id || stream.name}>
            <td>{stream.name}</td>
            <td className="url-cell">{stream.url}</td>
            <td>
              <span className={`status-indicator ${stream.enabled ? 'active' : 'inactive'}`}>
                {stream.enabled ? 'Active' : 'Inactive'}
              </span>
            </td>
            <td className="actions-cell">
              <button className="btn-icon edit-btn" onClick={() => onEditStream(stream.id || stream.name)}>
                <span className="icon">✎</span>
              </button>
              <DeleteStreamButton streamId={stream.id || stream.name} />
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export function StreamForm({ streamId, onClose }) {
  const queryClient = useQueryClient();
  const [formData, setFormData] = useState({
    name: '',
    url: '',
    enabled: true,
    streaming_enabled: true,
    width: 1280,
    height: 720,
    fps: 15,
    codec: 'h264',
    protocol: '0',
    priority: '5',
    segment_duration: 30,
    record: true,
    record_audio: true,
    detection_enabled: false,
    detection_model: '',
    detection_threshold: 50,
    detection_interval: 10,
    pre_buffer: 10,
    post_buffer: 30
  });

  // Fetch stream details if editing
  const { data: streamDetails, isLoading: isLoadingDetails } = useQuery({
    queryKey: ['stream', streamId],
    queryFn: () => fetchStreamDetails(streamId),
    enabled: !!streamId,
    onSuccess: (data) => {
      setFormData({
        name: data.name,
        url: data.url,
        enabled: data.enabled !== false,
        streaming_enabled: data.streaming_enabled !== false,
        width: data.width || 1280,
        height: data.height || 720,
        fps: data.fps || 15,
        codec: data.codec || 'h264',
        protocol: data.protocol?.toString() || '0',
        priority: data.priority?.toString() || '5',
        segment_duration: data.segment_duration || 30,
        record: data.record !== false,
        record_audio: data.record_audio !== false,
        detection_enabled: data.detection_based_recording || false,
        detection_model: data.detection_model || '',
        detection_threshold: data.detection_threshold || 50,
        detection_interval: data.detection_interval || 10,
        pre_buffer: data.pre_detection_buffer || 10,
        post_buffer: data.post_detection_buffer || 30
      });
    }
  });

  // Fetch detection models
  const { data: modelsData } = useQuery({
    queryKey: ['detectionModels'],
    queryFn: fetchDetectionModels
  });

  // Save stream mutation
  const saveStreamMutation = useMutation({
    mutationFn: async (data) => {
      const method = streamId ? 'PUT' : 'POST';
      const url = streamId ? `/api/streams/${encodeURIComponent(streamId)}` : '/api/streams';

      return await fetchJSON(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['streams'] });
      onClose();
    }
  });

  // Test stream mutation
  const testStreamMutation = useMutation({
    mutationFn: (url) => testStreamConnection(url)
  });

  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    saveStreamMutation.mutate(formData);
  };

  const handleTestConnection = () => {
    testStreamMutation.mutate(formData.url);
  };

  if (isLoadingDetails) return <div className="loading">Loading stream details...</div>;

  return (
    <form onSubmit={handleSubmit} className="stream-form">
      <div className="form-group">
        <label htmlFor="name">Stream Name</label>
        <input
          type="text"
          id="name"
          name="name"
          value={formData.name}
          onChange={handleInputChange}
          required
        />
      </div>

      <div className="form-group">
        <label htmlFor="url">Stream URL</label>
        <input
          type="text"
          id="url"
          name="url"
          value={formData.url}
          onChange={handleInputChange}
          required
        />
        <button type="button" onClick={handleTestConnection} className="test-btn">
          Test Connection
        </button>
        {testStreamMutation.isPending && <span>Testing...</span>}
        {testStreamMutation.isSuccess && <span className="success">Connection successful!</span>}
        {testStreamMutation.isError && <span className="error">Connection failed: {testStreamMutation.error.message}</span>}
      </div>

      <div className="form-group checkbox">
        <input
          type="checkbox"
          id="enabled"
          name="enabled"
          checked={formData.enabled}
          onChange={handleInputChange}
        />
        <label htmlFor="enabled">Enabled</label>
      </div>

      <div className="form-group checkbox">
        <input
          type="checkbox"
          id="streaming_enabled"
          name="streaming_enabled"
          checked={formData.streaming_enabled}
          onChange={handleInputChange}
        />
        <label htmlFor="streaming_enabled">Streaming Enabled</label>
      </div>

      <div className="form-row">
        <div className="form-group">
          <label htmlFor="width">Width</label>
          <input
            type="number"
            id="width"
            name="width"
            value={formData.width}
            onChange={handleInputChange}
          />
        </div>

        <div className="form-group">
          <label htmlFor="height">Height</label>
          <input
            type="number"
            id="height"
            name="height"
            value={formData.height}
            onChange={handleInputChange}
          />
        </div>

        <div className="form-group">
          <label htmlFor="fps">FPS</label>
          <input
            type="number"
            id="fps"
            name="fps"
            value={formData.fps}
            onChange={handleInputChange}
          />
        </div>
      </div>

      <div className="form-group">
        <label htmlFor="detection_enabled">Detection Based Recording</label>
        <input
          type="checkbox"
          id="detection_enabled"
          name="detection_enabled"
          checked={formData.detection_enabled}
          onChange={handleInputChange}
        />
      </div>

      {formData.detection_enabled && (
        <>
          <div className="form-group">
            <label htmlFor="detection_model">Detection Model</label>
            <select
              id="detection_model"
              name="detection_model"
              value={formData.detection_model}
              onChange={handleInputChange}
            >
              <option value="">Select a model</option>
              {modelsData?.map(model => (
                <option key={model.id} value={model.id}>{model.name}</option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label htmlFor="detection_threshold">Detection Threshold (%)</label>
            <input
              type="range"
              id="detection_threshold"
              name="detection_threshold"
              min="1"
              max="100"
              value={formData.detection_threshold}
              onChange={handleInputChange}
            />
            <span>{formData.detection_threshold}%</span>
          </div>
        </>
      )}

      <div className="form-actions">
        <button type="submit" className="btn primary">
          {streamId ? 'Update Stream' : 'Add Stream'}
        </button>
        <button type="button" className="btn secondary" onClick={onClose}>
          Cancel
        </button>
      </div>
    </form>
  );
}

export function LiveViewGrid() {
  const [layout, setLayout] = useState('4');
  const [selectedStream, setSelectedStream] = useState('');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const videoGridRef = useRef(null);
  const videoPlayers = useRef({});
  const detectionIntervals = useRef({});

  // Fetch streams with details for live view
  const { data: streams, isLoading, error } = useQuery({
    queryKey: ['streamsWithDetails'],
    queryFn: async () => {
      const streams = await fetchStreams();
      const detailedStreams = await Promise.all(
        streams.map(async (stream) => {
          try {
            return await fetchStreamDetails(stream.id || stream.name);
          } catch (error) {
            console.error(`Error loading details for stream ${stream.name}:`, error);
            return stream;
          }
        })
      );
      return detailedStreams.filter(stream => stream.enabled && stream.streaming_enabled);
    }
  });

  // Handle layout change
  const handleLayoutChange = (newLayout) => {
    setLayout(newLayout);
  };

  // Handle stream selection
  const handleStreamSelect = (streamName) => {
    setSelectedStream(streamName);
    setLayout('1'); // Switch to single view when selecting a stream
  };

  // Toggle fullscreen mode
  const toggleFullscreen = () => {
    if (videoGridRef.current) {
      if (!isFullscreen) {
        if (videoGridRef.current.requestFullscreen) {
          videoGridRef.current.requestFullscreen();
        } else if (videoGridRef.current.webkitRequestFullscreen) {
          videoGridRef.current.webkitRequestFullscreen();
        } else if (videoGridRef.current.msRequestFullscreen) {
          videoGridRef.current.msRequestFullscreen();
        }
      } else {
        if (document.exitFullscreen) {
          document.exitFullscreen();
        } else if (document.webkitExitFullscreen) {
          document.webkitExitFullscreen();
        } else if (document.msExitFullscreen) {
          document.msExitFullscreen();
        }
      }
    }
  };

  // Listen for fullscreen changes
  useEffect(() => {
    const handleFullscreenChange = () => {
      setIsFullscreen(
        document.fullscreenElement ||
        document.webkitFullscreenElement ||
        document.msFullscreenElement
      );
    };

    document.addEventListener('fullscreenchange', handleFullscreenChange);
    document.addEventListener('webkitfullscreenchange', handleFullscreenChange);
    document.addEventListener('msfullscreenchange', handleFullscreenChange);

    // Cleanup function
    return () => {
      document.removeEventListener('fullscreenchange', handleFullscreenChange);
      document.removeEventListener('webkitfullscreenchange', handleFullscreenChange);
      document.removeEventListener('msfullscreenchange', handleFullscreenChange);

      // Stop all video players when component unmounts
      if (streams) {
        streams.forEach(stream => {
          const streamId = stream.id || stream.name;
          if (videoPlayers.current[streamId]) {
            videoPlayers.current[streamId].destroy();
            delete videoPlayers.current[streamId];
          }
          if (detectionIntervals.current[streamId]) {
            clearInterval(detectionIntervals.current[streamId]);
            delete detectionIntervals.current[streamId];
          }
        });
      }
    };
  }, []);

  if (isLoading) return <div className="loading">Loading streams...</div>;
  if (error) return <div className="error">Error loading streams: {error.message}</div>;
  if (!streams || streams.length === 0) {
    return <div className="empty-message">No active streams available</div>;
  }

  return (
    <div className="live-view-container">
      <div className="controls-bar">
        <div className="layout-controls">
          <button
            className={`layout-btn ${layout === '1' ? 'active' : ''}`}
            onClick={() => handleLayoutChange('1')}
          >
            1×1
          </button>
          <button
            className={`layout-btn ${layout === '4' ? 'active' : ''}`}
            onClick={() => handleLayoutChange('4')}
          >
            2×2
          </button>
          <button
            className={`layout-btn ${layout === '9' ? 'active' : ''}`}
            onClick={() => handleLayoutChange('9')}
          >
            3×3
          </button>
          <button
            className={`layout-btn ${layout === '16' ? 'active' : ''}`}
            onClick={() => handleLayoutChange('16')}
          >
            4×4
          </button>
        </div>

        <div className="stream-selector">
          <select
            value={selectedStream}
            onChange={(e) => handleStreamSelect(e.target.value)}
            disabled={layout !== '1'}
          >
            <option value="">All Streams</option>
            {streams.map(stream => (
              <option key={stream.id || stream.name} value={stream.name}>
                {stream.name}
              </option>
            ))}
          </select>
        </div>

        <button
          className={`fullscreen-btn ${isFullscreen ? 'active' : ''}`}
          onClick={toggleFullscreen}
        >
          {isFullscreen ? 'Exit Fullscreen' : 'Fullscreen'}
        </button>
      </div>

      <div
        ref={videoGridRef}
        className={`video-grid layout-${layout} ${isFullscreen ? 'fullscreen' : ''}`}
      >
        {/* Video cells will be dynamically added here by updateVideoGrid */}
      </div>
    </div>
  );
}

export function DeleteStreamButton({ streamId }) {
  const queryClient = useQueryClient();

  const deleteStreamMutation = useMutation({
    mutationFn: async () => {
      return await fetchJSON(`/api/streams/${encodeURIComponent(streamId)}`, {
        method: 'DELETE'
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    }
  });

  const handleDelete = () => {
    if (confirm(`Are you sure you want to delete this stream?`)) {
      deleteStreamMutation.mutate();
    }
  };

  return (
    <button
      className="btn-icon delete-btn"
      onClick={handleDelete}
      disabled={deleteStreamMutation.isPending}
    >
      <span className="icon">×</span>
    </button>
  );
}

export function StreamFilter({ onFilterChange }) {
  const { data: streams, isLoading } = useQuery({
    queryKey: ['streams'],
    queryFn: fetchStreams
  });

  const [selectedStream, setSelectedStream] = useState('all');

  // Handle stream selection change
  const handleStreamChange = (e) => {
    const streamValue = e.target.value;
    setSelectedStream(streamValue);

    // Call the parent component's filter change handler if provided
    if (onFilterChange) {
      onFilterChange({ stream: streamValue });
    }

    // Update URL parameters if needed
    const urlParams = new URLSearchParams(window.location.search);
    if (streamValue === 'all') {
      urlParams.delete('stream');
    } else {
      urlParams.set('stream', streamValue);
    }

    // Update URL without page reload
    const newUrl = `${window.location.pathname}?${urlParams.toString()}`;
    window.history.pushState({ path: newUrl }, '', newUrl);
  };

  // Set initial value from URL if present
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.has('stream')) {
      const streamFromUrl = urlParams.get('stream');
      setSelectedStream(streamFromUrl);

      // Call the parent component's filter change handler if provided
      if (onFilterChange) {
        onFilterChange({ stream: streamFromUrl });
      }
    }
  }, [onFilterChange]);

  return (
    <div className="stream-filter">
      <label htmlFor="stream-filter" className="block text-sm font-medium mb-1">Stream:</label>
      <select
        id="stream-filter"
        value={selectedStream}
        onChange={handleStreamChange}
        className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
      >
        <option value="all">All Streams</option>
        {streams && streams.map(stream => (
          <option key={stream.id || stream.name} value={stream.name}>
            {stream.name}
          </option>
        ))}
      </select>
    </div>
  );
}
