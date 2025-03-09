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
                hideLoading(videoGrid);
                updateVideoGrid(streams);
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
                            <button class="btn-icon edit-btn" data-id="${streamId}" title="Edit"><span class="icon">✎</span></button>
                            <button class="btn-icon snapshot-btn" data-id="${streamId}" title="Snapshot"><span class="icon">📷</span></button>
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

                document.querySelectorAll('.snapshot-btn').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const streamId = this.getAttribute('data-id');
                        console.log('Taking snapshot of stream with ID:', streamId);
                        takeSnapshot(streamId);
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
 * Take a snapshot of a stream
 */
function takeSnapshot(streamId) {
    console.log('Taking snapshot for stream ID:', streamId);

    // First get the stream name from ID
    fetch(`/api/streams/${encodeURIComponent(streamId)}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load stream details');
            }
            return response.json();
        })
        .then(stream => {
            // Now take the snapshot
            const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
            const videoElement = document.getElementById(videoElementId);
            const containerId = `player-${stream.name.replace(/\s+/g, '-')}`;
            const container = document.getElementById(containerId);

            if (!videoElement || !container) {
                // If we're on the streams page, not the live view page
                // Send a direct request to the API to take a snapshot
                showStatusMessage('Taking snapshot...');

                fetch(`/api/streams/${encodeURIComponent(streamId)}/snapshot`, {
                    method: 'POST'
                })
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Failed to take snapshot');
                        }
                        return response.blob();
                    })
                    .then(blob => {
                        const imageUrl = URL.createObjectURL(blob);
                        showSnapshotPreview(imageUrl, stream.name);
                    })
                    .catch(error => {
                        console.error('Error taking snapshot:', error);
                        showStatusMessage('Error taking snapshot: ' + error.message, 3000);
                    });

                return;
            }

            // Show status message
            showStatusMessage('Taking snapshot...');

            // Create a canvas to capture the image
            const canvas = document.createElement('canvas');

            // For real video, use the video dimensions
            if (videoElement.videoWidth && videoElement.videoHeight) {
                canvas.width = videoElement.videoWidth;
                canvas.height = videoElement.videoHeight;

                const ctx = canvas.getContext('2d');
                ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

                // Add timestamp
                const now = new Date();
                const timestamp = now.toLocaleString();

                ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
                ctx.fillRect(0, canvas.height - 30, canvas.width, 30);

                ctx.fillStyle = 'white';
                ctx.font = '14px Arial';
                ctx.textAlign = 'left';
                ctx.fillText(`${stream.name} - ${timestamp}`, 10, canvas.height - 10);

                // Show preview instead of direct download
                const imageData = canvas.toDataURL('image/jpeg', 0.9);
                showSnapshotPreview(imageData, stream.name);
            } else {
                showStatusMessage('Cannot take snapshot: video not loaded', 3000);
            }
        })
        .catch(error => {
            console.error('Error getting stream details:', error);
            showStatusMessage('Error taking snapshot: ' + error.message, 3000);
        });
}

/**
 * Edit stream - Fixed implementation
 */
function editStream(streamId) {
    console.log('Editing stream:', streamId);
    const streamModal = document.getElementById('stream-modal');
    if (!streamModal) return;

    showLoading(streamModal);

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
            document.getElementById('stream-width').value = stream.width || 1280;
            document.getElementById('stream-height').value = stream.height || 720;
            document.getElementById('stream-fps').value = stream.fps || 15;
            document.getElementById('stream-codec').value = stream.codec || 'h264';
            document.getElementById('stream-priority').value = stream.priority || 5;
            document.getElementById('stream-record').checked = stream.record !== false; // Default to true if not specified
            document.getElementById('stream-segment').value = stream.segment_duration || 900;

            // Store original stream ID for later comparison
            streamModal.dataset.streamId = streamId;

            // Show the modal
            streamModal.style.display = 'block';
        })
        .catch(error => {
            console.error('Error loading stream details:', error);
            alert('Error loading stream details: ' + error.message);
        })
        .finally(() => {
            hideLoading(streamModal);
        });
}

/**
 * Save stream - Fixed implementation
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
        width: parseInt(document.getElementById('stream-width').value, 10),
        height: parseInt(document.getElementById('stream-height').value, 10),
        fps: parseInt(document.getElementById('stream-fps').value, 10),
        codec: document.getElementById('stream-codec').value,
        priority: parseInt(document.getElementById('stream-priority').value, 10),
        record: document.getElementById('stream-record').checked,
        segment_duration: parseInt(document.getElementById('stream-segment').value, 10)
    };

    // Validate required fields
    if (!streamData.name || !streamData.url) {
        alert('Name and URL are required');
        hideLoading(streamModal);
        return;
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

            alert('Stream saved successfully');
        })
        .catch(error => {
            console.error('Error saving stream:', error);
            alert('Error saving stream: ' + error.message);
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
    if (!confirm('Are you sure you want to delete this stream?')) {
        return;
    }

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

            alert('Stream deleted successfully');
        })
        .catch(error => {
            console.error('Error deleting stream:', error);
            alert('Error deleting stream: ' + error.message);

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
        alert('Please enter a stream URL');
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
                alert(`Stream connection test successful for URL: ${url}`);

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
                alert(`Stream connection test failed: ${data.error || 'Unknown error'}`);
            }
        })
        .catch(error => {
            console.error('Error testing stream connection:', error);
            alert('Error testing stream connection: ' + error.message);
        })
        .finally(() => {
            hideLoading(streamModal);
        });
}
