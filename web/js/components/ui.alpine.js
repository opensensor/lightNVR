/**
 * LightNVR Web Interface UI Components for Alpine.js
 * Contains functionality for modals, status messages, and other UI elements
 */

document.addEventListener('alpine:init', () => {
    // Status message component
    Alpine.store('statusMessage', {
        message: '',
        visible: false,
        timeout: null,
        
        show(message, duration = 3000) {
            this.message = message;
            this.visible = true;
            
            // Clear any existing timeout
            if (this.timeout) {
                clearTimeout(this.timeout);
            }
            
            // Set timeout to hide the message
            this.timeout = setTimeout(() => {
                this.visible = false;
            }, duration);
        },
        
        hide() {
            this.visible = false;
        }
    });
    
    // Status message component data
    Alpine.data('statusMessage', () => ({
        get visible() {
            return Alpine.store('statusMessage').visible;
        },
        
        get message() {
            return Alpine.store('statusMessage').message;
        }
    }));
    
    // Modal component
    Alpine.data('modal', (id) => ({
        id: id,
        isOpen: false,
        
        init() {
            // Close modal when clicking outside
            document.addEventListener('click', (e) => {
                if (e.target.classList.contains('modal') && this.isOpen) {
                    this.close();
                }
            });
            
            // Close modal when pressing Escape
            document.addEventListener('keydown', (e) => {
                if (e.key === 'Escape' && this.isOpen) {
                    this.close();
                }
            });
        },
        
        open() {
            this.isOpen = true;
            document.body.style.overflow = 'hidden';
        },
        
        close() {
            this.isOpen = false;
            document.body.style.overflow = '';
        },
        
        toggle() {
            this.isOpen = !this.isOpen;
            document.body.style.overflow = this.isOpen ? 'hidden' : '';
        }
    }));
    
    // Stream modal store
    Alpine.store('streamModal', {
        stream: {
            id: null,
            name: '',
            url: '',
            enabled: true,
            streamingEnabled: true,
            width: 1280,
            height: 720,
            fps: 15,
            codec: 'h264',
            protocol: '0',
            priority: '5',
            record: true,
            segmentDuration: 900,
            detectionEnabled: false,
            detectionModel: '',
            detectionThreshold: 50,
            detectionInterval: 10,
            preBuffer: 10,
            postBuffer: 30
        },
        
        open(stream = null) {
            // Reset form or populate with stream data
            if (stream) {
                this.stream = {
                    id: stream.id,
                    name: stream.name,
                    url: stream.url,
                    enabled: stream.enabled !== false,
                    streamingEnabled: stream.streaming_enabled !== false,
                    width: stream.width || 1280,
                    height: stream.height || 720,
                    fps: stream.fps || 15,
                    codec: stream.codec || 'h264',
                    protocol: stream.protocol !== undefined ? stream.protocol.toString() : '0',
                    priority: stream.priority || '5',
                    record: stream.record !== false,
                    segmentDuration: stream.segment_duration || 900,
                    detectionEnabled: stream.detection_based_recording === true,
                    detectionModel: stream.detection_model || '',
                    detectionThreshold: stream.detection_threshold || 50,
                    detectionInterval: stream.detection_interval || 10,
                    preBuffer: stream.pre_detection_buffer || 10,
                    postBuffer: stream.post_detection_buffer || 30
                };
            } else {
                // Reset to defaults for new stream
                this.stream = {
                    id: null,
                    name: '',
                    url: '',
                    enabled: true,
                    streamingEnabled: true,
                    width: 1280,
                    height: 720,
                    fps: 15,
                    codec: 'h264',
                    protocol: '0',
                    priority: '5',
                    record: true,
                    segmentDuration: 900,
                    detectionEnabled: false,
                    detectionModel: '',
                    detectionThreshold: 50,
                    detectionInterval: 10,
                    preBuffer: 10,
                    postBuffer: 30
                };
            }
            
            // Open the modal
            const modal = document.getElementById('stream-modal');
            if (modal && modal.__x) {
                modal.__x.Alpine.data.open();
            }
        }
    });
    
    // Stream form component
    Alpine.data('streamForm', () => ({
        stream: {
            name: '',
            url: '',
            enabled: true,
            streamingEnabled: true,
            width: 1280,
            height: 720,
            fps: 15,
            codec: 'h264',
            protocol: '0',
            priority: '5',
            record: true,
            segmentDuration: 900,
            detectionEnabled: false,
            detectionModel: '',
            detectionThreshold: 50,
            detectionInterval: 10,
            preBuffer: 10,
            postBuffer: 30
        },
        availableModels: [],
        
        init() {
            // Get stream data from store
            this.stream = Alpine.store('streamModal').stream;
            
            // Load detection models
            this.loadDetectionModels();
        },
        
        async loadDetectionModels() {
            try {
                const response = await fetch('/api/detection/models');
                if (!response.ok) {
                    throw new Error('Failed to load detection models');
                }
                
                const data = await response.json();
                if (data.models && data.models.length > 0) {
                    this.availableModels = data.models
                        .filter(model => model.supported)
                        .map(model => ({
                            id: model.name,
                            name: `${model.name} (${model.type})`
                        }));
                }
            } catch (error) {
                console.error('Error loading detection models:', error);
                Alpine.store('statusMessage').show('Error loading detection models: ' + error.message);
            }
        },
        
        async testStream() {
            if (!this.stream.url) {
                Alpine.store('statusMessage').show('Please enter a stream URL');
                return;
            }
            
            try {
                const response = await fetch('/api/streams/test', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ url: this.stream.url })
                });
                
                if (!response.ok) {
                    throw new Error('Failed to test stream connection');
                }
                
                const data = await response.json();
                if (data.success) {
                    Alpine.store('statusMessage').show(`Stream connection test successful for URL: ${this.stream.url}`);
                    
                    // Auto-fill some form fields based on detected stream info
                    if (data.details) {
                        if (data.details.codec) {
                            this.stream.codec = data.details.codec;
                        }
                        if (data.details.width && data.details.height) {
                            this.stream.width = data.details.width;
                            this.stream.height = data.details.height;
                        }
                        if (data.details.fps) {
                            this.stream.fps = data.details.fps;
                        }
                    }
                } else {
                    Alpine.store('statusMessage').show(`Stream connection test failed: ${data.error || 'Unknown error'}`);
                }
            } catch (error) {
                console.error('Error testing stream connection:', error);
                Alpine.store('statusMessage').show('Error testing stream connection: ' + error.message);
            }
        },
        
        async saveStream() {
            // Validate required fields
            if (!this.stream.name || !this.stream.url) {
                Alpine.store('statusMessage').show('Name and URL are required');
                return;
            }
            
            // Validate detection options
            if (this.stream.detectionEnabled && !this.stream.detectionModel) {
                Alpine.store('statusMessage').show('Please select a detection model');
                return;
            }
            
            // Prepare data for API
            const streamData = {
                name: this.stream.name,
                url: this.stream.url,
                enabled: this.stream.enabled,
                streaming_enabled: this.stream.streamingEnabled,
                width: parseInt(this.stream.width, 10),
                height: parseInt(this.stream.height, 10),
                fps: parseInt(this.stream.fps, 10),
                codec: this.stream.codec,
                protocol: parseInt(this.stream.protocol, 10),
                priority: parseInt(this.stream.priority, 10),
                record: this.stream.record,
                segment_duration: parseInt(this.stream.segmentDuration, 10),
                detection_based_recording: this.stream.detectionEnabled,
                detection_model: this.stream.detectionModel,
                detection_threshold: parseInt(this.stream.detectionThreshold, 10),
                detection_interval: parseInt(this.stream.detectionInterval, 10),
                pre_detection_buffer: parseInt(this.stream.preBuffer, 10),
                post_detection_buffer: parseInt(this.stream.postBuffer, 10)
            };
            
            // Determine if this is a new stream or an update
            const isUpdate = this.stream.id !== null;
            const method = isUpdate ? 'PUT' : 'POST';
            const url = isUpdate ? `/api/streams/${encodeURIComponent(this.stream.id)}` : '/api/streams';
            
            try {
                const response = await fetch(url, {
                    method: method,
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(streamData)
                });
                
                if (!response.ok) {
                    throw new Error('Failed to save stream');
                }
                
                // Show success message
                Alpine.store('statusMessage').show('Stream saved successfully');
                
                // Close the modal
                const modal = document.getElementById('stream-modal');
                if (modal && modal.__x) {
                    modal.__x.Alpine.data.close();
                }
                
                // Reload streams
                if (typeof window.loadStreams === 'function') {
                    window.loadStreams();
                } else {
                    // Fallback: reload the page
                    window.location.reload();
                }
            } catch (error) {
                console.error('Error saving stream:', error);
                Alpine.store('statusMessage').show('Error saving stream: ' + error.message);
            }
        }
    }));
    
    // Video modal store
    Alpine.store('videoModal', {
        videoUrl: '',
        title: '',
        downloadUrl: '',
        
        open(videoUrl, title = 'Video Playback') {
            this.videoUrl = videoUrl;
            this.title = title;
            this.downloadUrl = videoUrl.replace('/play/', '/download/');
            
            // Update modal title
            const modalTitle = document.getElementById('video-modal-title');
            if (modalTitle) {
                modalTitle.textContent = title;
            }
            
            // Set video source
            const video = document.querySelector('#video-player video');
            if (video) {
                if (videoUrl.endsWith('.m3u8')) {
                    // HLS video
                    if (Hls.isSupported()) {
                        const hls = new Hls();
                        hls.loadSource(videoUrl);
                        hls.attachMedia(video);
                    } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
                        // Native HLS support
                        video.src = videoUrl;
                    }
                } else {
                    // Regular video
                    video.src = videoUrl;
                }
                
                // Auto-play
                video.play().catch(e => console.error('Error auto-playing video:', e));
            }
            
            // Open the modal
            const modal = document.getElementById('video-modal');
            if (modal && modal.__x) {
                modal.__x.Alpine.data.open();
            }
        },
        
        download() {
            if (!this.downloadUrl) return;
            
            // Create download link
            const link = document.createElement('a');
            link.href = this.downloadUrl;
            link.download = `recording_${new Date().toISOString().replace(/[:.]/g, '-')}.mp4`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            
            Alpine.store('statusMessage').show('Download started');
        }
    });
    
    // Snapshot modal store
    Alpine.store('snapshotModal', {
        imageUrl: '',
        title: '',
        
        open(imageUrl, title = 'Snapshot Preview') {
            this.imageUrl = imageUrl;
            this.title = title;
            
            // Update modal title
            const modalTitle = document.getElementById('snapshot-preview-title');
            if (modalTitle) {
                modalTitle.textContent = title;
            }
            
            // Set image source
            const image = document.getElementById('snapshot-preview-image');
            if (image) {
                image.src = imageUrl;
            }
            
            // Open the modal
            const modal = document.getElementById('snapshot-preview-modal');
            if (modal && modal.__x) {
                modal.__x.Alpine.data.open();
            }
        },
        
        download() {
            if (!this.imageUrl) return;
            
            // Create download link
            const link = document.createElement('a');
            link.href = this.imageUrl;
            link.download = `snapshot_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            
            Alpine.store('statusMessage').show('Download started');
        }
    });
});

/**
 * Set up modals
 */
function setupModals() {
    // Nothing to do here, Alpine.js handles the modals
}

/**
 * Set up snapshot modal
 */
function setupSnapshotModal() {
    // Nothing to do here, Alpine.js handles the snapshot modal
}
