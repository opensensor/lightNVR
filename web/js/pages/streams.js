/**
 * LightNVR Web Interface - Streams Page (Tailwind CSS version)
 * Alpine.js components for the streams management page
 */

// Vanilla JS modal and status message handlers
let currentStreamId = null;
let availableModels = [];

// Show status message
function showStatusMessage(message, duration = 3000) {
    const statusMessage = document.getElementById('status-message');
    if (!statusMessage) return;
    
    statusMessage.textContent = message;
    statusMessage.classList.remove('opacity-0');
    statusMessage.classList.add('opacity-100');
    
    // Clear any existing timeout
    if (statusMessage._timeout) {
        clearTimeout(statusMessage._timeout);
    }
    
    // Set timeout to hide the message
    statusMessage._timeout = setTimeout(() => {
        statusMessage.classList.remove('opacity-100');
        statusMessage.classList.add('opacity-0');
    }, duration);
}

// Modal functions
function openStreamModal(streamData = null) {
    const modal = document.getElementById('stream-modal');
    if (!modal) return;
    
    // Reset form
    resetStreamForm();
    
    // If editing existing stream, populate form
    if (streamData) {
        populateStreamForm(streamData);
    }
    
    // Show modal
    modal.classList.remove('hidden');
    
    // Load detection models if needed
    loadDetectionModels();
    
    // Setup detection checkbox toggle behavior
    setupDetectionToggle();
}

function closeStreamModal() {
    const modal = document.getElementById('stream-modal');
    if (!modal) return;
    
    modal.classList.add('hidden');
    currentStreamId = null;
}

function resetStreamForm() {
    const form = document.getElementById('stream-form');
    if (!form) return;
    
    // Reset form fields to defaults
    const nameInput = document.getElementById('stream-name');
    nameInput.value = '';
    nameInput.disabled = false; // Enable name field for new streams
    nameInput.classList.remove('bg-gray-100', 'dark:bg-gray-700'); // Remove gray styling
    
    document.getElementById('stream-url').value = '';
    document.getElementById('stream-enabled').checked = true;
    document.getElementById('stream-streaming-enabled').checked = true;
    document.getElementById('stream-width').value = '1280';
    document.getElementById('stream-height').value = '720';
    document.getElementById('stream-fps').value = '15';
    document.getElementById('stream-codec').value = 'h264';
    document.getElementById('stream-protocol').value = '0';
    document.getElementById('stream-priority').value = '5';
    document.getElementById('stream-segment').value = '900';
    document.getElementById('stream-record').checked = true;
    document.getElementById('stream-detection-enabled').checked = false;
    
    // Reset detection options
    document.querySelectorAll('.detection-options').forEach(el => {
        el.classList.add('hidden');
    });
    
    document.getElementById('stream-detection-threshold').value = '50';
    document.getElementById('stream-threshold-value').textContent = '50%';
    document.getElementById('stream-detection-interval').value = '10';
    document.getElementById('stream-pre-buffer').value = '10';
    document.getElementById('stream-post-buffer').value = '30';
    
    currentStreamId = null;
}

function populateStreamForm(streamData) {
    if (!streamData) return;
    
    // Store the stream ID for later use
    currentStreamId = streamData.id || streamData.name;
    
    // Populate form fields
    const nameInput = document.getElementById('stream-name');
    nameInput.value = streamData.name || '';
    nameInput.disabled = true; // Disable name field when editing
    nameInput.classList.add('bg-gray-100', 'dark:bg-gray-700'); // Gray out the field
    document.getElementById('stream-url').value = streamData.url || '';
    document.getElementById('stream-enabled').checked = streamData.enabled !== false;
    document.getElementById('stream-streaming-enabled').checked = streamData.streaming_enabled !== false;
    document.getElementById('stream-width').value = streamData.width || '1280';
    document.getElementById('stream-height').value = streamData.height || '720';
    document.getElementById('stream-fps').value = streamData.fps || '15';
    document.getElementById('stream-codec').value = streamData.codec || 'h264';
    document.getElementById('stream-protocol').value = streamData.protocol !== undefined ? streamData.protocol.toString() : '0';
    document.getElementById('stream-priority').value = streamData.priority ? streamData.priority.toString() : '5';
    document.getElementById('stream-segment').value = streamData.segment_duration || '900';
    document.getElementById('stream-record').checked = streamData.record !== false;
    
    // Detection options
    const detectionEnabled = streamData.detection_based_recording === true;
    document.getElementById('stream-detection-enabled').checked = detectionEnabled;
    
    if (detectionEnabled) {
        document.querySelectorAll('.detection-options').forEach(el => {
            el.classList.remove('hidden');
        });
    }
    
    document.getElementById('stream-detection-threshold').value = streamData.detection_threshold || '50';
    document.getElementById('stream-threshold-value').textContent = (streamData.detection_threshold || '50') + '%';
    document.getElementById('stream-detection-interval').value = streamData.detection_interval || '10';
    document.getElementById('stream-pre-buffer').value = streamData.pre_detection_buffer || '10';
    document.getElementById('stream-post-buffer').value = streamData.post_detection_buffer || '30';
    
    // If we have a detection model, select it
    if (streamData.detection_model) {
        const modelSelect = document.getElementById('stream-detection-model');
        // We'll need to wait for models to load, then select the right one
        const checkModelInterval = setInterval(() => {
            if (modelSelect.options.length > 1) {
                clearInterval(checkModelInterval);
                
                // Find and select the model
                for (let i = 0; i < modelSelect.options.length; i++) {
                    if (modelSelect.options[i].value === streamData.detection_model) {
                        modelSelect.selectedIndex = i;
                        break;
                    }
                }
            }
        }, 100);
    }
}

function setupDetectionToggle() {
    const detectionCheckbox = document.getElementById('stream-detection-enabled');
    if (!detectionCheckbox) return;
    
    // Initial state
    const detectionOptions = document.querySelectorAll('.detection-options');
    if (detectionCheckbox.checked) {
        detectionOptions.forEach(el => el.classList.remove('hidden'));
    } else {
        detectionOptions.forEach(el => el.classList.add('hidden'));
    }
    
    // Toggle detection options visibility when checkbox changes
    detectionCheckbox.addEventListener('change', function() {
        if (this.checked) {
            detectionOptions.forEach(el => el.classList.remove('hidden'));
        } else {
            detectionOptions.forEach(el => el.classList.add('hidden'));
        }
    });
    
    // Update threshold value display when slider changes
    const thresholdSlider = document.getElementById('stream-detection-threshold');
    const thresholdValue = document.getElementById('stream-threshold-value');
    if (thresholdSlider && thresholdValue) {
        thresholdSlider.addEventListener('input', function() {
            thresholdValue.textContent = this.value + '%';
        });
    }
}

async function loadDetectionModels() {
    try {
        const modelSelect = document.getElementById('stream-detection-model');
        if (!modelSelect) return;
        
        // Clear existing options except the first one
        while (modelSelect.options.length > 1) {
            modelSelect.remove(1);
        }
        
        // Fetch models from API
        const response = await fetch('/api/detection/models');
        if (!response.ok) {
            throw new Error('Failed to load detection models');
        }
        
        const data = await response.json();
        if (data.models && data.models.length > 0) {
            availableModels = data.models
                .filter(model => model.supported)
                .map(model => ({
                    id: model.name,
                    name: `${model.name} (${model.type})`
                }));
                
            // Add models to select
            availableModels.forEach(model => {
                const option = document.createElement('option');
                option.value = model.id;
                option.textContent = model.name;
                modelSelect.appendChild(option);
            });
        } else {
            console.warn('No detection models found');
            
            // Add a placeholder option
            const option = document.createElement('option');
            option.value = '';
            option.textContent = 'No models available';
            modelSelect.appendChild(option);
        }
    } catch (error) {
        console.error('Error loading detection models:', error);
        showStatusMessage('Error loading detection models: ' + error.message);
    }
}

async function testStreamConnection() {
    const urlInput = document.getElementById('stream-url');
    if (!urlInput || !urlInput.value) {
        showStatusMessage('Please enter a stream URL');
        return;
    }
    
    try {
        const response = await fetch('/api/streams/test', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ url: urlInput.value })
        });
        
        if (!response.ok) {
            throw new Error('Failed to test stream connection');
        }
        
        const data = await response.json();
        
        if (data.success) {
            showStatusMessage(`Stream connection test successful for URL: ${urlInput.value}`);
            
            // Auto-fill some form fields based on detected stream info
            if (data.details) {
                if (data.details.codec) {
                    document.getElementById('stream-codec').value = data.details.codec;
                }
                if (data.details.width && data.details.height) {
                    document.getElementById('stream-width').value = data.details.width;
                    document.getElementById('stream-height').value = data.details.height;
                }
                if (data.details.fps) {
                    document.getElementById('stream-fps').value = data.details.fps;
                }
            }
        } else {
            showStatusMessage(`Stream connection test failed: ${data.error || 'Unknown error'}`);
        }
    } catch (error) {
        console.error('Error testing stream connection:', error);
        showStatusMessage('Error testing stream connection: ' + error.message);
    }
}

async function saveStream() {
    // Get form values
    const nameInput = document.getElementById('stream-name');
    const urlInput = document.getElementById('stream-url');
    
    // Validate required fields
    if (!nameInput.value || !urlInput.value) {
        showStatusMessage('Name and URL are required');
        return;
    }
    
    // Validate detection options
    const detectionEnabled = document.getElementById('stream-detection-enabled').checked;
    const detectionModel = document.getElementById('stream-detection-model').value;
    
    if (detectionEnabled && !detectionModel) {
        showStatusMessage('Please select a detection model');
        return;
    }
    
    // Prepare data for API
    const streamData = {
        name: nameInput.value,
        url: urlInput.value,
        enabled: document.getElementById('stream-enabled').checked,
        streaming_enabled: document.getElementById('stream-streaming-enabled').checked,
        width: parseInt(document.getElementById('stream-width').value, 10),
        height: parseInt(document.getElementById('stream-height').value, 10),
        fps: parseInt(document.getElementById('stream-fps').value, 10),
        codec: document.getElementById('stream-codec').value,
        protocol: parseInt(document.getElementById('stream-protocol').value, 10),
        priority: parseInt(document.getElementById('stream-priority').value, 10),
        record: document.getElementById('stream-record').checked,
        segment_duration: parseInt(document.getElementById('stream-segment').value, 10),
        detection_based_recording: detectionEnabled,
        detection_model: detectionModel,
        detection_threshold: parseInt(document.getElementById('stream-detection-threshold').value, 10),
        detection_interval: parseInt(document.getElementById('stream-detection-interval').value, 10),
        pre_detection_buffer: parseInt(document.getElementById('stream-pre-buffer').value, 10),
        post_detection_buffer: parseInt(document.getElementById('stream-post-buffer').value, 10)
    };
    
    console.log('Saving stream data:', streamData);
    
    // Determine if this is a new stream or an update
    const isUpdate = currentStreamId !== null;
    const method = isUpdate ? 'PUT' : 'POST';
    const url = isUpdate ? `/api/streams/${encodeURIComponent(currentStreamId)}` : '/api/streams';
    
    console.log(`${isUpdate ? 'Updating' : 'Creating'} stream with method ${method} to URL ${url}`);
    
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
        showStatusMessage('Stream saved successfully');
        
        // Close the modal
        closeStreamModal();
        
        // Reload streams - force a direct call to the loadStreams function
        setTimeout(() => {
            const streamsManager = document.getElementById('streams-page')?.__x;
            if (streamsManager && streamsManager.$data && typeof streamsManager.$data.loadStreams === 'function') {
                streamsManager.$data.loadStreams();
            } else {
                // Fallback to window reload if we can't find the component
                window.location.reload();
            }
        }, 100); // Small delay to ensure modal is closed first
    } catch (error) {
        console.error('Error saving stream:', error);
        showStatusMessage('Error saving stream: ' + error.message);
    }
}

document.addEventListener('alpine:init', () => {
    // Register Alpine.js store for status message
    Alpine.store('statusMessage', {
        visible: false,
        message: '',
        timeout: null,

        show(msg, duration = 3000) {
            this.message = msg;
            this.visible = true;
            
            // Clear any existing timeout
            if (this.timeout) {
                clearTimeout(this.timeout);
            }
            
            // Set timeout to hide the message
            this.timeout = setTimeout(() => {
                this.visible = false;
            }, duration);
        }
    });

    // Status message component
    Alpine.data('statusMessage', () => ({
        visible: false,
        message: '',
        timeout: null,

        show(msg, duration = 3000) {
            this.message = msg;
            this.visible = true;
            
            // Clear any existing timeout
            if (this.timeout) {
                clearTimeout(this.timeout);
            }
            
            // Set timeout to hide the message
            this.timeout = setTimeout(() => {
                this.visible = false;
            }, duration);
        }
    }));

    // Stream modal component
    Alpine.data('streamModal', () => ({
        isOpen: false,
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
        availableModels: [],
        
        init() {
            // Load detection models
            this.loadDetectionModels();
        },
        
        open(streamData = null) {
            if (streamData) {
                // Edit existing stream
                this.stream = {
                    id: streamData.id || streamData.name,
                    name: streamData.name || '',
                    url: streamData.url || '',
                    enabled: streamData.enabled !== false,
                    streamingEnabled: streamData.streaming_enabled !== false,
                    width: streamData.width || 1280,
                    height: streamData.height || 720,
                    fps: streamData.fps || 15,
                    codec: streamData.codec || 'h264',
                    protocol: streamData.protocol !== undefined ? streamData.protocol.toString() : '0',
                    priority: streamData.priority ? streamData.priority.toString() : '5',
                    record: streamData.record !== false,
                    segmentDuration: streamData.segment_duration || 900,
                    detectionEnabled: streamData.detection_based_recording === true,
                    detectionModel: streamData.detection_model || '',
                    detectionThreshold: streamData.detection_threshold || 50,
                    detectionInterval: streamData.detection_interval || 10,
                    preBuffer: streamData.pre_detection_buffer || 10,
                    postBuffer: streamData.post_detection_buffer || 30
                };
            } else {
                // Add new stream - reset to defaults
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
            
            this.isOpen = true;
        },
        
        close() {
            this.isOpen = false;
        },
        
        async loadDetectionModels() {
            try {
                // Fetch models from API
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
                } else {
                    console.warn('No detection models found');
                }
            } catch (error) {
                console.error('Error loading detection models:', error);
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Error loading detection models: ' + error.message);
                }
            }
        },
        
        async testStream() {
            if (!this.stream.url) {
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Please enter a stream URL');
                }
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
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                
                if (data.success) {
                    if (statusMessage) {
                        statusMessage.show(`Stream connection test successful for URL: ${this.stream.url}`);
                    }
                    
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
                    if (statusMessage) {
                        statusMessage.show(`Stream connection test failed: ${data.error || 'Unknown error'}`);
                    }
                }
            } catch (error) {
                console.error('Error testing stream connection:', error);
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Error testing stream connection: ' + error.message);
                }
            }
        },
        
        async saveStream() {
            // Validate required fields
            if (!this.stream.name || !this.stream.url) {
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Name and URL are required');
                }
                return;
            }
            
            // Validate detection options
            if (this.stream.detectionEnabled && !this.stream.detectionModel) {
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Please select a detection model');
                }
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
            
            console.log('Saving stream data:', streamData);
            
            // Determine if this is a new stream or an update
            const isUpdate = this.stream.id !== null && this.stream.id !== undefined;
            const method = isUpdate ? 'PUT' : 'POST';
            const url = isUpdate ? `/api/streams/${encodeURIComponent(this.stream.id)}` : '/api/streams';
            
            console.log(`${isUpdate ? 'Updating' : 'Creating'} stream with method ${method} to URL ${url}`);
            
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
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Stream saved successfully');
                }
                
                // Close the modal
                this.close();
                
                // Reload streams - force a direct call to the loadStreams function
                setTimeout(() => {
                    const streamsManager = document.getElementById('streams-page')?.__x;
                    if (streamsManager && streamsManager.$data && typeof streamsManager.$data.loadStreams === 'function') {
                        streamsManager.$data.loadStreams();
                    } else {
                        // Fallback to window reload if we can't find the component
                        window.location.reload();
                    }
                }, 100); // Small delay to ensure modal is closed first
            } catch (error) {
                console.error('Error saving stream:', error);
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Error saving stream: ' + error.message);
                }
            }
        }
    }));

    // Streams manager component
    Alpine.data('streamsManager', () => ({
        streams: [],
        
        init() {
            // Load streams
            this.loadStreams();
        },
        
        async loadStreams() {
            try {
                const streamsTable = document.getElementById('streams-table');
                if (!streamsTable) return;
                
                const tbody = streamsTable.querySelector('tbody');
                
                // Clear existing rows
                tbody.innerHTML = '<tr><td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">Loading streams...</td></tr>';
                
                // Fetch streams from API
                const response = await fetch('/api/streams');
                if (!response.ok) {
                    throw new Error('Failed to load streams');
                }
                
                const streams = await response.json();
                this.streams = streams || [];
                
                // Update table
                tbody.innerHTML = '';
                
                if (!streams || streams.length === 0) {
                    tbody.innerHTML = '<tr><td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">No streams configured</td></tr>';
                    return;
                }
                
                streams.forEach(stream => {
                    const tr = document.createElement('tr');
                    tr.className = 'hover:bg-gray-50 dark:hover:bg-gray-700';
                    
                    // Ensure we have an ID for the stream (use name as fallback if needed)
                    const streamId = stream.id || stream.name;
                    
                    tr.innerHTML = `
                        <td class="px-6 py-4 whitespace-nowrap">${stream.name}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${stream.url}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${stream.width}x${stream.height}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${stream.fps}</td>
                        <td class="px-6 py-4 whitespace-nowrap">
                            <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${stream.record ? 'bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100' : 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-100'}">
                                ${stream.record ? 'Yes' : 'No'}
                            </span>
                        </td>
                        <td class="px-6 py-4 whitespace-nowrap">
                            <div class="flex space-x-2">
                                <button class="edit-btn p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                                        data-id="${streamId}"
                                        title="Edit">
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                        <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                                    </svg>
                                </button>
                                <button class="delete-btn p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                                        data-id="${streamId}"
                                        title="Delete">
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                        <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                                    </svg>
                                </button>
                            </div>
                        </td>
                    `;
                    
                    tbody.appendChild(tr);
                });
                
                // Add event listeners for edit and delete buttons
                document.querySelectorAll('.edit-btn').forEach(btn => {
                    btn.addEventListener('click', () => {
                        const streamId = btn.getAttribute('data-id');
                        this.editStream(streamId);
                    });
                });
                
                document.querySelectorAll('.delete-btn').forEach(btn => {
                    btn.addEventListener('click', () => {
                        const streamId = btn.getAttribute('data-id');
                        if (confirm(`Are you sure you want to delete this stream?`)) {
                            this.deleteStream(streamId);
                        }
                    });
                });
            } catch (error) {
                console.error('Error loading streams:', error);
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Error loading streams: ' + error.message);
                }
            }
        },
        
        openAddStreamModal() {
            console.log('Opening add stream modal');
            openStreamModal();
        },
        
        async editStream(streamId) {
            try {
                // Fetch stream details from API
                const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`);
                if (!response.ok) {
                    throw new Error('Failed to load stream details');
                }
                
                const stream = await response.json();
                console.log('Loaded stream details:', stream);
                
                // Open modal with stream data
                openStreamModal(stream);
            } catch (error) {
                console.error('Error loading stream details:', error);
                showStatusMessage('Error loading stream details: ' + error.message);
            }
        },
        
        async deleteStream(streamId) {
            try {
                // Send delete request to API
                const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`, {
                    method: 'DELETE'
                });
                
                if (!response.ok) {
                    throw new Error('Failed to delete stream');
                }
                
                // Reload streams
                this.loadStreams();
                
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Stream deleted successfully');
                }
            } catch (error) {
                console.error('Error deleting stream:', error);
                const statusMessage = document.getElementById('status-message')?.__x?.Alpine?.data;
                if (statusMessage) {
                    statusMessage.show('Error deleting stream: ' + error.message);
                }
            }
        }
    }));
});

// Load header and footer when the DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    // Load header and footer
    if (typeof loadHeader === 'function') {
        loadHeader('nav-streams');
    }
    if (typeof loadFooter === 'function') {
        loadFooter();
    }
    
    // Setup modal event listeners
    setupModalEventListeners();
});

// Setup event listeners for the modal buttons
function setupModalEventListeners() {
    // Close modal button
    const closeModalBtn = document.getElementById('close-modal-btn');
    if (closeModalBtn) {
        closeModalBtn.addEventListener('click', closeStreamModal);
    }
    
    // Cancel button
    const cancelBtn = document.getElementById('stream-cancel-btn');
    if (cancelBtn) {
        cancelBtn.addEventListener('click', closeStreamModal);
    }
    
    // Test connection button
    const testBtn = document.getElementById('stream-test-btn');
    if (testBtn) {
        testBtn.addEventListener('click', testStreamConnection);
    }
    
    // Save button
    const saveBtn = document.getElementById('stream-save-btn');
    if (saveBtn) {
        saveBtn.addEventListener('click', saveStream);
    }
    
    // Refresh models button
    const refreshModelsBtn = document.getElementById('refresh-models-btn');
    if (refreshModelsBtn) {
        refreshModelsBtn.addEventListener('click', loadDetectionModels);
    }
    
    // Setup detection checkbox toggle
    setupDetectionToggle();
}
