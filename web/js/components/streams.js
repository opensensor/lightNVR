/**
 * LightNVR Web Interface Stream Management
 * Contains functionality for managing streams (add, edit, delete, test)
 */

/**
 * Load streams - for Streams page or Live View page
 */
function loadStreams(forLiveView = false) {
    if (forLiveView) {
        // For Live View page, load streams for video grid
        const videoGrid = document.getElementById('video-grid');
        if (!videoGrid) return;

        showLoading(videoGrid);

        // Fetch streams from API
        fetch('/api/streams')
            .then(response => {
                if (!response.ok) {
                    throw new Error('Failed to load streams');
                }
                return response.json();
            })
            .then(streams => {
                // For live view, we need to fetch full details for each stream
                // to get detection settings
                const streamPromises = streams.map(stream => {
                    return fetch(`/api/streams/${encodeURIComponent(stream.id || stream.name)}`)
                        .then(response => {
                            if (!response.ok) {
                                throw new Error(`Failed to load details for stream ${stream.name}`);
                            }
                            return response.json();
                        })
                        .catch(error => {
                            console.error(`Error loading details for stream ${stream.name}:`, error);
                            // Return the basic stream info if we can't get details
                            return stream;
                        });
                });
                
                return Promise.all(streamPromises);
            })
            .then(detailedStreams => {
                hideLoading(videoGrid);
                console.log('Loaded detailed streams for live view:', detailedStreams);
                updateVideoGrid(detailedStreams);
            })
            .catch(error => {
                console.error('Error loading streams for live view:', error);
                hideLoading(videoGrid);

                const placeholder = document.createElement('div');
                placeholder.className = 'placeholder';
                placeholder.innerHTML = `
                    <p>Error loading streams</p>
                    <a href="/streams" class="btn">Configure Streams</a>
                `;
                videoGrid.innerHTML = '';
                videoGrid.appendChild(placeholder);
            });
    } else {
        // For Streams page, load streams for table
        const streamsTable = document.getElementById('streams-table');
        if (!streamsTable) return;

        const tbody = streamsTable.querySelector('tbody');

        showLoading(streamsTable);

        // Clear existing rows
        tbody.innerHTML = '<tr><td colspan="7" class="empty-message">Loading streams...</td></tr>';

        // Fetch streams from API
        fetch('/api/streams')
            .then(response => {
                if (!response.ok) {
                    throw new Error('Failed to load streams');
                }
                return response.json();
            })
            .then(streams => {
                tbody.innerHTML = '';

                if (!streams || streams.length === 0) {
                    tbody.innerHTML = '<tr><td colspan="7" class="empty-message">No streams configured</td></tr>';
                    return;
                }

                streams.forEach(stream => {
                    const tr = document.createElement('tr');

                    // Ensure we have an ID for the stream (use name as fallback if needed)
                    const streamId = stream.id || stream.name;

                    tr.innerHTML = `
                        <td>${stream.name}</td>
                        <td>${stream.url}</td>
                        <td><span class="status-indicator ${stream.status}">${stream.status}</span></td>
                        <td>${stream.width}x${stream.height}</td>
                        <td>${stream.fps}</td>
                        <td>${stream.record ? 'Yes' : 'No'}</td>
                        <td>
                            <span class="status-indicator ${stream.streaming_enabled !== false ? 'active' : 'inactive'}">
                                ${stream.streaming_enabled !== false ? 'Enabled' : 'Disabled'}
                            </span>
                        </td>
                        <td>
                            <button class="btn-icon edit-btn" data-id="${streamId}" title="Edit"><span class="icon">✎</span></button>
                            <button class="btn-icon delete-btn" data-id="${streamId}" title="Delete"><span class="icon">×</span></button>
                        </td>
                    `;

                    tbody.appendChild(tr);
                });

                // Add event listeners for edit, snapshot, and delete buttons
                document.querySelectorAll('.edit-btn').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const streamId = this.getAttribute('data-id');
                        console.log('Editing stream with ID:', streamId);
                        editStream(streamId);
                    });
                });

                document.querySelectorAll('.delete-btn').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const streamId = this.getAttribute('data-id');
                        console.log('Deleting stream with ID:', streamId);
                        if (confirm(`Are you sure you want to delete this stream?`)) {
                            deleteStream(streamId);
                        }
                    });
                });

                // Update stream filter in recordings page
                updateStreamFilter(streams);
            })
            .catch(error => {
                console.error('Error loading streams:', error);
                tbody.innerHTML = '<tr><td colspan="7" class="empty-message">Error loading streams</td></tr>';
            })
            .finally(() => {
                hideLoading(streamsTable);
            });
    }
}

/**
 * Update stream filter dropdown
 */
function updateStreamFilter(streams) {
    const streamFilter = document.getElementById('stream-filter');
    if (!streamFilter) return;

    console.log('Updating stream filter with streams:', streams);

    // Clear existing options except "All Streams"
    while (streamFilter.options.length > 1) {
        streamFilter.remove(1);
    }

    // Add stream options
    if (streams && streams.length > 0) {
        streams.forEach(stream => {
            const streamId = stream.id || stream.name;
            const option = document.createElement('option');
            option.value = stream.name; // Use stream name for filtering
            option.textContent = stream.name;
            streamFilter.appendChild(option);
            console.log(`Added stream option: ${stream.name}`);
        });
    }
}

/**
 * Load detection models for stream configuration
 */
function loadDetectionModels() {
    const modelSelect = document.getElementById('stream-detection-model');
    if (!modelSelect) return;
    
    console.log('Loading detection models...');
    
    // Clear existing options
    modelSelect.innerHTML = '<option value="">Loading models...</option>';
    
    // Fetch models from API
    fetch('/api/detection/models')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load detection models');
            }
            return response.json();
        })
        .then(data => {
            // Clear loading option
            modelSelect.innerHTML = '<option value="">Select a model</option>';
            
            if (data.models && data.models.length > 0) {
                // Add model options
                data.models.forEach(model => {
                    if (model.supported) {
                        const option = document.createElement('option');
                        option.value = model.name;
                        option.textContent = `${model.name} (${model.type})`;
                        modelSelect.appendChild(option);
                    }
                });
                
                console.log(`Loaded ${data.models.length} detection models`);
                
                // Make sure detection options are visible if checkbox is checked
                const detectionEnabled = document.getElementById('stream-detection-enabled');
                if (detectionEnabled && detectionEnabled.checked) {
                    const detectionOptions = document.querySelectorAll('.detection-options');
                    detectionOptions.forEach(el => {
                        el.style.display = 'block';
                    });
                }
            } else {
                // No models found
                const option = document.createElement('option');
                option.value = "";
                option.textContent = "No models available";
                option.disabled = true;
                modelSelect.appendChild(option);
                
                console.warn('No detection models found');
            }
        })
        .catch(error => {
            console.error('Error loading detection models:', error);
            modelSelect.innerHTML = '<option value="">Error loading models</option>';
        });
}

/**
 * Edit stream - Updated implementation with detection options
 */
function editStream(streamId) {
    console.log('Editing stream:', streamId);
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;

    showLoading(streamModal);
    
    // Load detection models
    loadDetectionModels();

    // Fetch stream details from API
    fetch(`/api/streams/${encodeURIComponent(streamId)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load stream details');
            }
            return response.json();
        })
        .then(stream => {
            console.log('Loaded stream details:', stream);
            // Fill the form with stream data
            document.getElementById('stream-name').value = stream.name;
            document.getElementById('stream-url').value = stream.url;
            document.getElementById('stream-enabled').checked = stream.enabled !== false; // Default to true if not specified
            document.getElementById('stream-streaming-enabled').checked = stream.streaming_enabled !== false; // Default to true if not specified
            document.getElementById('stream-width').value = stream.width || 1280;
            document.getElementById('stream-height').value = stream.height || 720;
            document.getElementById('stream-fps').value = stream.fps || 15;
            document.getElementById('stream-codec').value = stream.codec || 'h264';
            document.getElementById('stream-protocol').value = stream.protocol !== undefined ? stream.protocol : 0; // Default to TCP (0) if not specified
            document.getElementById('stream-priority').value = stream.priority || 5;
            document.getElementById('stream-record').checked = stream.record !== false; // Default to true if not specified
            document.getElementById('stream-segment').value = stream.segment_duration || 900;
            
            // Fill detection-based recording options
            const detectionEnabled = document.getElementById('stream-detection-enabled');
            if (detectionEnabled) {
                detectionEnabled.checked = stream.detection_based_recording === true;
                
                // Show/hide detection options based on checkbox
                const detectionOptions = document.querySelectorAll('.detection-options');
                detectionOptions.forEach(el => {
                    el.style.display = detectionEnabled.checked ? 'block' : 'none';
                });
                
                // If detection is enabled, make sure models are loaded immediately
                if (detectionEnabled.checked) {
                    loadDetectionModels();
                    
                    // Force display of detection options
                    detectionOptions.forEach(el => {
                        el.style.display = 'block';
                    });
                }
                
                // Always include detection model in the form, even if detection is disabled
                // This ensures the model is preserved when toggling detection on/off
            }
            
            // Set detection model if available
            if (stream.detection_model) {
                // Load models immediately and then set the selected model
                loadDetectionModels();
                
                // Wait a bit for models to load, but use a longer timeout to ensure they're loaded
                setTimeout(() => {
                    const modelSelect = document.getElementById('stream-detection-model');
                    if (modelSelect) {
                        // Try to find the model by name
                        for (let i = 0; i < modelSelect.options.length; i++) {
                            if (modelSelect.options[i].value === stream.detection_model) {
                                modelSelect.selectedIndex = i;
                                break;
                            }
                        }
                    }
                }, 1000);
            }
            
            // Set detection threshold
            const thresholdSlider = document.getElementById('stream-detection-threshold');
            if (thresholdSlider && typeof stream.detection_threshold === 'number') {
                // The API returns the threshold as an integer percentage (0-100)
                let thresholdPercent = stream.detection_threshold;
                
                // Sanity check to prevent extreme values
                if (thresholdPercent > 100) {
                    console.warn(`Threshold value too high (${thresholdPercent}), capping at 100%`);
                    thresholdPercent = 100;
                }
                
                console.log(`Setting threshold slider to ${thresholdPercent}%`);
                
                // Set the slider value
                thresholdSlider.value = thresholdPercent;
                
                // Trigger the oninput event to update the display
                // Create and dispatch an input event
                const event = new Event('input', { bubbles: true });
                thresholdSlider.dispatchEvent(event);
                
                console.log('Triggered input event on slider to update display');
            }
            
            // Set detection interval
            const detectionInterval = document.getElementById('stream-detection-interval');
            if (detectionInterval && typeof stream.detection_interval === 'number') {
                detectionInterval.value = stream.detection_interval;
            }
            
            // Set pre/post detection buffers
            const preBuffer = document.getElementById('stream-pre-buffer');
            if (preBuffer && typeof stream.pre_detection_buffer === 'number') {
                preBuffer.value = stream.pre_detection_buffer;
            }
            
            const postBuffer = document.getElementById('stream-post-buffer');
            if (postBuffer && typeof stream.post_detection_buffer === 'number') {
                postBuffer.value = stream.post_detection_buffer;
            }

            // Store original stream ID for later comparison
            streamModal.dataset.streamId = streamId;

            // Show the modal
            streamModal.style.display = 'block';
        })
        .catch(error => {
            console.error('Error loading stream details:', error);
            showStatusMessage('Error loading stream details: ' + error.message, 5000);
        })
        .finally(() => {
            hideLoading(streamModal);
        });
}

/**
 * Save stream - Updated implementation with detection options
 */
function saveStream() {
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;

    showLoading(streamModal);

    // Collect stream data from form
    const streamData = {
        name: document.getElementById('stream-name').value,
        url: document.getElementById('stream-url').value,
        enabled: document.getElementById('stream-enabled').checked,
        streaming_enabled: document.getElementById('stream-streaming-enabled').checked,
        width: parseInt(document.getElementById('stream-width').value, 10),
        height: parseInt(document.getElementById('stream-height').value, 10),
        fps: parseInt(document.getElementById('stream-fps').value, 10),
        codec: document.getElementById('stream-codec').value,
        protocol: parseInt(document.getElementById('stream-protocol').value, 10),
        priority: parseInt(document.getElementById('stream-priority').value, 10),
        record: document.getElementById('stream-record').checked,
        segment_duration: parseInt(document.getElementById('stream-segment').value, 10)
    };
    
    // Log the enabled state for debugging
    console.log('Stream enabled state:', streamData.enabled);
    
    // Add detection-based recording options
    const detectionEnabled = document.getElementById('stream-detection-enabled');
    if (detectionEnabled) {
        streamData.detection_based_recording = detectionEnabled.checked;
        
        // Always include detection options, even if detection is disabled
        // This ensures settings are preserved when toggling detection on/off
        const modelSelect = document.getElementById('stream-detection-model');
        
        // Check if model select exists and has options
        if (modelSelect) {
            console.log('Model select found, options count:', modelSelect.options.length);
            
            // If there are no options other than the default "Select a model", try loading models again
            if (modelSelect.options.length <= 1) {
                console.log('No model options found, loading models again...');
                loadDetectionModels();
                
                // Only show alert and return if detection is enabled
                if (detectionEnabled.checked) {
                    showStatusMessage('No detection models available. Please make sure models are installed in the models directory.', 5000);
                    hideLoading(streamModal);
                    return;
                }
            }
            
            if (modelSelect.value) {
                streamData.detection_model = modelSelect.value;
                console.log('Selected model:', streamData.detection_model);
            }
        } else {
            console.error('Model select element not found');
        }
        
        const thresholdSlider = document.getElementById('stream-detection-threshold');
        if (thresholdSlider) {
            // Ensure the threshold is between 0 and 100
            let thresholdValue = parseInt(thresholdSlider.value, 10);
            if (thresholdValue < 0) thresholdValue = 0;
            if (thresholdValue > 100) thresholdValue = 100;
            streamData.detection_threshold = thresholdValue;
            console.log('Setting detection threshold to:', thresholdValue);
        }
        
        const preBuffer = document.getElementById('stream-pre-buffer');
        if (preBuffer) {
            streamData.pre_detection_buffer = parseInt(preBuffer.value, 10);
        }
        
        const postBuffer = document.getElementById('stream-post-buffer');
        if (postBuffer) {
            streamData.post_detection_buffer = parseInt(postBuffer.value, 10);
        }
        
        // Get detection interval from form
        const detectionInterval = document.getElementById('stream-detection-interval');
        if (detectionInterval) {
            streamData.detection_interval = parseInt(detectionInterval.value, 10);
        } else {
            streamData.detection_interval = 10; // Default to 10 if field not found
        }
    }

    // Validate required fields
    if (!streamData.name || !streamData.url) {
        showStatusMessage('Name and URL are required', 5000);
        hideLoading(streamModal);
        return;
    }
    
    // Validate detection options
    if (streamData.detection_based_recording) {
        if (!streamData.detection_model) {
            // Check if models are available
            const modelSelect = document.getElementById('stream-detection-model');
            if (modelSelect && modelSelect.options.length <= 1) {
                showStatusMessage('No detection models available. Please make sure models are installed in the models directory.', 5000);
            } else {
                showStatusMessage('Please select a detection model', 5000);
            }
            
            // Make sure detection options are visible
            const detectionOptions = document.querySelectorAll('.detection-options');
            detectionOptions.forEach(el => {
                el.style.display = 'block';
            });
            
            hideLoading(streamModal);
            return;
        }
    }

    // Check if this is a new stream or an update
    const streamId = streamModal.dataset.streamId;
    let method = 'POST';
    let url = '/api/streams';

    if (streamId) {
        console.log('Updating existing stream:', streamId);
        method = 'PUT';
        url = `/api/streams/${encodeURIComponent(streamId)}`;
    } else {
        console.log('Creating new stream');
    }

    // Send to API
    fetch(url, {
        method: method,
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(streamData)
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to save stream');
            }
            return response.json();
        })
        .then(data => {
            console.log('Stream saved successfully:', data);
            // Close the modal
            streamModal.style.display = 'none';

            // Clear stream ID
            delete streamModal.dataset.streamId;

            // Reload streams
            if (document.getElementById('streams-table')) {
                loadStreams(); // For streams page
            } else if (document.getElementById('video-grid')) {
                loadStreams(true); // For live view page
            }

            showStatusMessage('Stream saved successfully', 5000);
        })
        .catch(error => {
            console.error('Error saving stream:', error);
            showStatusMessage('Error saving stream: ' + error.message, 5000);
        })
        .finally(() => {
            hideLoading(streamModal);
        });
}

/**
 * Delete stream - Fixed implementation
 */
function deleteStream(streamId) {
    console.log('Deleting stream:', streamId);
    
    const streamsTable = document.getElementById('streams-table');
    if (streamsTable) {
        showLoading(streamsTable);
    }

    // Send delete request to API
    fetch(`/api/streams/${encodeURIComponent(streamId)}`, {
        method: 'DELETE'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to delete stream');
            }
            return response.json();
        })
        .then(data => {
            console.log('Stream deleted successfully:', data);
            // Reload streams
            if (document.getElementById('streams-table')) {
                loadStreams(); // For streams page
            } else if (document.getElementById('video-grid')) {
                loadStreams(true); // For live view page
            }

            showStatusMessage('Stream deleted successfully', 5000);
        })
        .catch(error => {
            console.error('Error deleting stream:', error);
            showStatusMessage('Error deleting stream: ' + error.message, 5000);

            if (streamsTable) {
                hideLoading(streamsTable);
            }
        });
}

/**
 * Test stream connection - Fixed implementation
 */
function testStream() {
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;

    // Get URL from form
    const url = document.getElementById('stream-url').value;
    if (!url) {
        showStatusMessage('Please enter a stream URL', 5000);
        return;
    }

    showLoading(streamModal);

    // Send test request to API
    fetch('/api/streams/test', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ url: url })
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to test stream connection');
            }
            return response.json();
        })
        .then(data => {
            if (data.success) {
                showStatusMessage(`Stream connection test successful for URL: ${url}`, 5000);

                // Optionally auto-fill some form fields based on detected stream info
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
                showStatusMessage(`Stream connection test failed: ${data.error || 'Unknown error'}`, 5000);
            }
        })
        .catch(error => {
            console.error('Error testing stream connection:', error);
            showStatusMessage('Error testing stream connection: ' + error.message, 5000);
        })
        .finally(() => {
            hideLoading(streamModal);
        });
}

/**
 * Setup streams page event handlers
 */
function setupStreamsHandlers() {
    // Load detection models when page loads
    loadDetectionModels();
    
    // Add stream button click handler
    const addButton = document.getElementById('add-stream-btn');
    if (addButton) {
        addButton.addEventListener('click', function() {
            // Reset form
            const streamForm = document.getElementById('stream-form');
            if (streamForm) {
                streamForm.reset();
            }
            
            // Clear stream ID
            const streamModal = document.getElementById('stream-modal');
            if (streamModal) {
                delete streamModal.dataset.streamId;
            }
            
            // Load detection models
            loadDetectionModels();
            
            // Update threshold slider value display to match the reset slider value
            const thresholdSlider = document.getElementById('stream-detection-threshold');
            if (thresholdSlider) {
                // Trigger the oninput event to update the display
                const event = new Event('input', { bubbles: true });
                thresholdSlider.dispatchEvent(event);
                console.log('Reset threshold display via input event');
            }
            
            // Show modal
            if (streamModal) {
                streamModal.style.display = 'block';
            }
        });
    }
    
    // Save button click handler
    const saveButton = document.getElementById('stream-save-btn');
    if (saveButton) {
        saveButton.addEventListener('click', saveStream);
    }
    
    // Cancel button click handler
    const cancelButton = document.getElementById('stream-cancel-btn');
    if (cancelButton) {
        cancelButton.addEventListener('click', function() {
            const streamModal = document.getElementById('stream-modal');
            if (streamModal) {
                streamModal.style.display = 'none';
            }
        });
    }
    
    // Test button click handler
    const testButton = document.getElementById('stream-test-btn');
    if (testButton) {
        testButton.addEventListener('click', testStream);
    }
    
    // Close button click handler
    const closeButton = document.querySelector('#stream-modal .close');
    if (closeButton) {
        closeButton.addEventListener('click', function() {
            const streamModal = document.getElementById('stream-modal');
            if (streamModal) {
                streamModal.style.display = 'none';
            }
        });
    }
    
    // Detection checkbox change handler
    const detectionEnabled = document.getElementById('stream-detection-enabled');
    if (detectionEnabled) {
        detectionEnabled.addEventListener('change', function() {
            const detectionOptions = document.querySelectorAll('.detection-options');
            detectionOptions.forEach(el => {
                el.style.display = this.checked ? 'block' : 'none';
            });
            
            // Always load detection models when checkbox is toggled
            loadDetectionModels();
            
            // If checked, make sure the model dropdown is visible and populated
            if (this.checked) {
                console.log('Detection enabled, ensuring models are loaded');
                
                // Force display of detection options
                detectionOptions.forEach(el => {
                    el.style.display = 'block';
                });
                
                // Update threshold slider value display to match the current slider value
                const thresholdSlider = document.getElementById('stream-detection-threshold');
                if (thresholdSlider) {
                    // Trigger the oninput event to update the display
                    const event = new Event('input', { bubbles: true });
                    thresholdSlider.dispatchEvent(event);
                    console.log('Updated threshold display via input event on checkbox toggle');
                }
            }
        });
    }
    
    // Refresh models button click handler
    const refreshButton = document.getElementById('refresh-models-btn');
    if (refreshButton) {
        refreshButton.addEventListener('click', function(e) {
            e.preventDefault();
            loadDetectionModels();
        });
    }
}
