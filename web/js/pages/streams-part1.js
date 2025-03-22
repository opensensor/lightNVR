/**
 * LightNVR Web Interface - Streams Page (Part 1)
 * Vanilla JS functions for the streams management page
 */

// Vanilla JS modal and status message handlers
let currentStreamId = null;
let availableModels = [];

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
        showErrorToast('Error loading detection models: ' + error.message);
    }
}

async function testStreamConnection() {
    const urlInput = document.getElementById('stream-url');
    if (!urlInput || !urlInput.value) {
        showErrorToast('Please enter a stream URL');
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
            showSuccessToast(`Stream connection test successful for URL: ${urlInput.value}`);
            
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
            showErrorToast(`Stream connection test failed: ${data.error || 'Unknown error'}`);
        }
    } catch (error) {
        console.error('Error testing stream connection:', error);
        showErrorToast('Error testing stream connection: ' + error.message);
    }
}

async function saveStream() {
    // Get form values
    const nameInput = document.getElementById('stream-name');
    const urlInput = document.getElementById('stream-url');
    
    // Validate required fields
    if (!nameInput.value || !urlInput.value) {
        showErrorToast('Name and URL are required');
        return;
    }
    
    // Validate detection options
    const detectionEnabled = document.getElementById('stream-detection-enabled').checked;
    const detectionModel = document.getElementById('stream-detection-model').value;
    
    if (detectionEnabled && !detectionModel) {
        showErrorToast('Please select a detection model');
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
        
        // Close the modal
        closeStreamModal();
        
        // Store success message for after refresh
        const successMessage = 'Stream saved successfully';
        
        // Refresh data first
        const streamsManager = document.getElementById('streams-page')?.__x;
        if (streamsManager && streamsManager.$data && typeof streamsManager.$data.loadStreams === 'function') {
            await streamsManager.$data.loadStreams();
            // Show toast after data is refreshed
            showSuccessToast(successMessage, 3000);
        } else {
            // For page reload, we need to store the message in sessionStorage
            // so it can be shown after the page reloads
            sessionStorage.setItem('streamToast', JSON.stringify({
                type: 'success',
                message: successMessage
            }));
            window.location.reload();
        }
    } catch (error) {
        console.error('Error saving stream:', error);
        showErrorToast('Error saving stream: ' + error.message);
    }
}
