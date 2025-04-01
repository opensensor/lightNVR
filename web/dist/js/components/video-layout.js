/**
 * LightNVR Web Interface Video Layout Management
 * Contains functionality for managing video grid layouts
 */

/**
 * Update video layout
 */
function updateVideoLayout(layout) {
    const videoGrid = document.getElementById('video-grid');
    const streamSelector = document.getElementById('stream-selector');
    if (!videoGrid) return;

    // Remove all layout classes
    videoGrid.classList.remove('layout-1', 'layout-4', 'layout-9', 'layout-16');
    
    // Add selected layout class
    videoGrid.classList.add(`layout-${layout}`);
    
    // Show/hide stream selector based on layout
    if (layout === '1' && streamSelector) {
        streamSelector.style.display = 'inline-block';
        
        // If we're switching to single view, show only the selected stream
        // or the first stream if none is selected
        const selectedStreamName = streamSelector.value;
        const videoCells = videoGrid.querySelectorAll('.video-cell');
        
        if (videoCells.length > 0) {
            videoCells.forEach(cell => {
                const streamName = cell.querySelector('.stream-info span').textContent;
                if (selectedStreamName && streamName !== selectedStreamName) {
                    cell.style.display = 'none';
                } else {
                    cell.style.display = 'flex';
                }
            });
            
            // If no stream is selected or the selected stream is not found,
            // show the first stream and update the selector
            if (!selectedStreamName || !Array.from(videoCells).some(cell => 
                cell.querySelector('.stream-info span').textContent === selectedStreamName && 
                cell.style.display !== 'none')) {
                
                const firstCell = videoCells[0];
                if (firstCell) {
                    firstCell.style.display = 'flex';
                    const firstStreamName = firstCell.querySelector('.stream-info span').textContent;
                    if (streamSelector.querySelector(`option[value="${firstStreamName}"]`)) {
                        streamSelector.value = firstStreamName;
                    }
                }
            }
        }
    } else if (streamSelector) {
        streamSelector.style.display = 'none';
        
        // Show all streams in grid view
        const videoCells = videoGrid.querySelectorAll('.video-cell');
        videoCells.forEach(cell => {
            cell.style.display = 'flex';
        });
    }
    
    // Adjust video cells if needed
    const videoCells = videoGrid.querySelectorAll('.video-cell');
    if (videoCells.length > 0) {
        // Force video elements to redraw to adjust to new layout
        videoCells.forEach(cell => {
            const video = cell.querySelector('video');
            if (video) {
                // Trigger a reflow
                video.style.display = 'none';
                setTimeout(() => {
                    video.style.display = 'block';
                }, 10);
            }
        });
    }
}

/**
 * Toggle fullscreen for a specific stream
 */
function toggleStreamFullscreen(streamName) {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoCell) {
        console.error('Stream not found:', streamName);
        return;
    }

    if (!document.fullscreenElement) {
        videoCell.requestFullscreen().catch(err => {
            console.error(`Error attempting to enable fullscreen: ${err.message}`);
            showStatusMessage(`Could not enable fullscreen mode: ${err.message}`, 5000);
        });
    } else {
        document.exitFullscreen();
    }
}

/**
 * Toggle fullscreen mode for the entire video grid
 */
function toggleFullscreen() {
    const videoGrid = document.getElementById('video-grid');
    if (!videoGrid) return;

    if (!document.fullscreenElement) {
        videoGrid.requestFullscreen().catch(err => {
            console.error(`Error attempting to enable fullscreen: ${err.message}`);
        });
    } else {
        document.exitFullscreen();
    }
}

/**
 * Update video grid with streams
 */
function updateVideoGrid(streams) {
    const videoGrid = document.getElementById('video-grid');
    const streamSelector = document.getElementById('stream-selector');
    if (!videoGrid) return;

    // Clear existing content
    videoGrid.innerHTML = '';

    if (!streams || streams.length === 0) {
        const placeholder = document.createElement('div');
        placeholder.className = 'placeholder';
        placeholder.innerHTML = `
            <p>No streams configured</p>
            <a href="streams.html" class="btn">Configure Streams</a>
        `;
        videoGrid.appendChild(placeholder);
        return;
    }

    // Get layout
    const layout = document.getElementById('layout-selector').value;

    // Update stream selector dropdown
    if (streamSelector) {
        // Clear existing options
        streamSelector.innerHTML = '';
        
        // Add options for each stream
        streams.forEach(stream => {
            const option = document.createElement('option');
            option.value = stream.name;
            option.textContent = stream.name;
            streamSelector.appendChild(option);
        });
        
        // Remove existing event listeners by cloning and replacing the element
        const newStreamSelector = streamSelector.cloneNode(true);
        streamSelector.parentNode.replaceChild(newStreamSelector, streamSelector);
        
        // Get the new reference to the stream selector
        const updatedStreamSelector = document.getElementById('stream-selector');
        
        // Add change event listener
        updatedStreamSelector.addEventListener('change', function() {
            // Get the current layout
            const currentLayout = document.getElementById('layout-selector').value;
            
            if (currentLayout === '1') {
                // Show only the selected stream in single view mode
                const selectedStreamName = this.value;
                const videoCells = videoGrid.querySelectorAll('.video-cell');
                
                videoCells.forEach(cell => {
                    const streamName = cell.querySelector('.stream-info span').textContent;
                    cell.style.display = (streamName === selectedStreamName) ? 'flex' : 'none';
                });
            }
        });
    }

    // Add video elements for each stream
    streams.forEach(stream => {
        // Ensure we have an ID for the stream (use name as fallback if needed)
        const streamId = stream.id || stream.name;

        const videoCell = document.createElement('div');
        videoCell.className = 'video-cell';

        videoCell.innerHTML = `
            <video id="video-${stream.name.replace(/\s+/g, '-')}" autoplay muted></video>
            <div class="stream-info">
                <span>${stream.name}</span>
                <span>${stream.width}x${stream.height} · ${stream.fps}fps</span>
                <div class="stream-controls">
                    <button class="snapshot-btn" data-id="${streamId}" data-name="${stream.name}">
                        <span>📷</span> Snapshot
                    </button>
                    <button class="fullscreen-btn" data-id="${streamId}" data-name="${stream.name}">
                        <span>⛶</span> Fullscreen
                    </button>
                </div>
            </div>
            <div class="loading-indicator">
                <div class="loading-spinner"></div>
                <span>Connecting...</span>
            </div>
        `;

        videoGrid.appendChild(videoCell);
    });

    // Initialize video players and add event listeners
    streams.forEach(stream => {
        initializeVideoPlayer(stream);

        // Ensure we have an ID for the stream (use name as fallback if needed)
        const streamId = stream.id || stream.name;

        // Add event listener for snapshot button
        const snapshotBtn = videoGrid.querySelector(`.snapshot-btn[data-id="${streamId}"]`);
        if (snapshotBtn) {
            snapshotBtn.addEventListener('click', () => {
                console.log('Taking snapshot of stream with ID:', streamId);
                
                // Exit fullscreen if active before taking snapshot
                if (document.fullscreenElement) {
                    document.exitFullscreen().then(() => {
                        setTimeout(() => takeSnapshot(streamId), 100);
                    }).catch(err => {
                        console.error(`Error exiting fullscreen: ${err.message}`);
                        takeSnapshot(streamId);
                    });
                } else {
                    takeSnapshot(streamId);
                }
            });
        }

        // Add event listener for fullscreen button
        const fullscreenBtn = videoGrid.querySelector(`.fullscreen-btn[data-id="${streamId}"]`);
        if (fullscreenBtn) {
            fullscreenBtn.addEventListener('click', () => {
                console.log('Toggling fullscreen for stream with ID:', streamId);
                toggleStreamFullscreen(stream.name);
            });
        }
    });
    
    // Update video layout
    updateVideoLayout(layout);
}

// Export functions
window.updateVideoLayout = updateVideoLayout;
window.toggleStreamFullscreen = toggleStreamFullscreen;
window.toggleFullscreen = toggleFullscreen;
window.updateVideoGrid = updateVideoGrid;
