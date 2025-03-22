/**
 * LightNVR Web Interface Recordings Management
 * Contains functionality for managing recordings (load, play, download, delete)
 */

/**
 * Load recordings with pagination and advanced filtering
 * Supports date range filtering with timestamps and URL parameters
 */
function loadRecordings(page = 1, updateUrl = true) {
    const recordingsTable = document.getElementById('recordings-table');
    if (!recordingsTable) return;

    const tbody = recordingsTable.querySelector('tbody');

    showLoading(recordingsTable);

    // Clear existing rows
    tbody.innerHTML = '<tr><td colspan="5" class="empty-message">Loading recordings...</td></tr>';

    // Get filter values
    const dateRangeSelect = document.getElementById('date-range-select');
    const startDate = document.getElementById('start-date');
    const startTime = document.getElementById('start-time');
    const endDate = document.getElementById('end-date');
    const endTime = document.getElementById('end-time');
    const streamFilter = document.getElementById('stream-filter');
    const detectionFilter = document.getElementById('detection-filter');
    const pageSizeSelect = document.getElementById('page-size');
    
    // Get values with defaults
    const dateRangeType = dateRangeSelect ? dateRangeSelect.value : 'today';
    const streamValue = streamFilter ? streamFilter.value : 'all';
    const detectionValue = detectionFilter ? detectionFilter.value : 'all';
    const pageSize = pageSizeSelect ? parseInt(pageSizeSelect.value, 10) : 20;

    // Get current sort parameters
    const currentSort = document.querySelector('th.sort-asc, th.sort-desc');
    let sortBy = currentSort ? currentSort.getAttribute('data-sort') : 'start_time';
    let sortOrder = currentSort && currentSort.classList.contains('sort-asc') ? 'asc' : 'desc';
    
    // Debug log for sorting parameters
    console.log(`Sorting by: ${sortBy}, order: ${sortOrder}`);
    
    // Ensure sortBy matches the database column names
    // No need to map 'stream' to 'stream_name' as the HTML already uses 'stream_name'
    
    // Build query string
    let queryParams = new URLSearchParams();
    
    // Add sort parameters
    if (sortBy) {
        queryParams.append('sort', sortBy);
        queryParams.append('order', sortOrder);
    }
    
    // Add date range parameters based on selection
    if (dateRangeType === 'custom' && startDate && startDate.value) {
        // For custom range, use the start and end date/time inputs
        let startDateTime = startDate.value;
        if (startTime && startTime.value) {
            startDateTime += `T${startTime.value}:00`;
        } else {
            startDateTime += 'T00:00:00';
        }
        queryParams.append('start', startDateTime);
        
        // End date (default to start date if not provided)
        let endDateTime = endDate && endDate.value ? endDate.value : startDate.value;
        if (endTime && endTime.value) {
            endDateTime += `T${endTime.value}:59`;
        } else {
            endDateTime += 'T23:59:59';
        }
        queryParams.append('end', endDateTime);
    } else {
        // For preset ranges, calculate the appropriate dates
        const now = new Date();
        let startDateTime, endDateTime;
        
        switch (dateRangeType) {
            case 'today':
                startDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0);
                endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                break;
            case 'yesterday':
                // Create a new date object for yesterday and set to midnight-to-midnight
                const yesterday = new Date(now);
                yesterday.setDate(yesterday.getDate() - 1);
                // Set to beginning of day (00:00:00)
                startDateTime = new Date(yesterday.getFullYear(), yesterday.getMonth(), yesterday.getDate(), 0, 0, 0);
                // Set to end of day (23:59:59)
                endDateTime = new Date(yesterday.getFullYear(), yesterday.getMonth(), yesterday.getDate(), 23, 59, 59);
                break;
            case 'last7days':
                // Create a date for 7 days ago
                const sevenDaysAgo = new Date(now);
                sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 6);
                startDateTime = new Date(sevenDaysAgo.getFullYear(), sevenDaysAgo.getMonth(), sevenDaysAgo.getDate(), 0, 0, 0);
                endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                break;
            case 'last30days':
                // Create a date for 30 days ago
                const thirtyDaysAgo = new Date(now);
                thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 29);
                startDateTime = new Date(thirtyDaysAgo.getFullYear(), thirtyDaysAgo.getMonth(), thirtyDaysAgo.getDate(), 0, 0, 0);
                endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
                break;
            default:
                startDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0);
                endDateTime = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
        }
        
        // Format dates for API
        const formatDate = (date) => {
            return date.toISOString().split('.')[0]; // Remove milliseconds
        };
        
        queryParams.append('start', formatDate(startDateTime));
        queryParams.append('end', formatDate(endDateTime));
    }
    
    // Add stream filter if not "all"
    if (streamValue && streamValue !== 'all') {
        queryParams.append('stream', streamValue);
    }
    
    // Add detection filter if "detection"
    if (detectionValue === 'detection') {
        queryParams.append('detection', '1');
    }
    
    // Add pagination parameters
    queryParams.append('page', page);
    queryParams.append('limit', pageSize);
    
    // Create the query string
    const queryString = queryParams.toString() ? `?${queryParams.toString()}` : '';
    
    // Update URL if requested (for bookmarking and sharing)
    if (updateUrl) {
        const newUrl = `${window.location.pathname}${queryString}`;
        window.history.pushState({ path: newUrl }, '', newUrl);
        
        // Update active filters display
        updateActiveFilters(queryParams);
    }

    // Fetch recordings from API
    fetch(`/api/recordings${queryString}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load recordings');
            }
            return response.json();
        })
        .then(data => {
            tbody.innerHTML = '';

            // Check if we have recordings
            if (!data.recordings || data.recordings.length === 0) {
                tbody.innerHTML = '<tr><td colspan="5" class="empty-message">No recordings found</td></tr>';
                
                // Update pagination info
                updatePaginationInfo(0, 0, 0, 1, 1);
                
                hideLoading(recordingsTable);
                return;
            }

            // Render recordings
            data.recordings.forEach(recording => {
                const tr = document.createElement('tr');

                tr.innerHTML = `
                    <td>${recording.stream}</td>
                    <td>${recording.start_time}</td>
                    <td>${recording.duration}</td>
                    <td>${recording.size}</td>
                    <td>
                        <button class="btn-icon play-btn" data-id="${recording.id}" title="Play"><span class="icon">▶</span></button>
                        <button class="btn-icon download-btn" data-id="${recording.id}" title="Download"><span class="icon">↓</span></button>
                        <button class="btn-icon delete-btn" data-id="${recording.id}" title="Delete"><span class="icon">×</span></button>
                    </td>
                `;

                tbody.appendChild(tr);
            });

            // Add event listeners for play, download, and delete buttons
            document.querySelectorAll('.play-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const recordingId = this.getAttribute('data-id');
                    playRecording(recordingId);
                });
            });

            document.querySelectorAll('.download-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const recordingId = this.getAttribute('data-id');
                    downloadRecording(recordingId);
                });
            });

            document.querySelectorAll('.delete-btn').forEach(btn => {
                btn.addEventListener('click', function() {
                    const recordingId = this.getAttribute('data-id');
                    if (confirm('Are you sure you want to delete this recording?')) {
                        deleteRecording(recordingId);
                    }
                });
            });

            // Update pagination info
            const { pagination } = data;
            const currentPage = pagination.page;
            const totalPages = pagination.pages;
            const totalRecords = pagination.total;
            const limit = pagination.limit;
            
            // Calculate the range of records being displayed
            const startRecord = (currentPage - 1) * limit + 1;
            const endRecord = Math.min(startRecord + data.recordings.length - 1, totalRecords);
            
            updatePaginationInfo(startRecord, endRecord, totalRecords, currentPage, totalPages);
            
            // Setup pagination buttons
            setupPaginationButtons(currentPage, totalPages);
        })
        .catch(error => {
            console.error('Error loading recordings:', error);
            tbody.innerHTML = '<tr><td colspan="5" class="empty-message">Error loading recordings</td></tr>';
            
            // Reset pagination info on error
            updatePaginationInfo(0, 0, 0, 1, 1);
        })
        .finally(() => {
            hideLoading(recordingsTable);
        });
}

/**
 * Update the active filters display
 */
function updateActiveFilters(queryParams) {
    const activeFiltersContainer = document.getElementById('active-filters');
    if (!activeFiltersContainer) return;
    
    // Clear existing filters
    activeFiltersContainer.innerHTML = '';
    
    // If no filters, hide the container
    if (!queryParams || queryParams.toString() === '') {
        activeFiltersContainer.style.display = 'none';
        return;
    }
    
    // Show the container
    activeFiltersContainer.style.display = 'flex';
    
    // Add date range filter tag
    if (queryParams.has('start') || queryParams.has('end')) {
        let dateRangeText = '';
        
        if (queryParams.has('start') && queryParams.has('end')) {
            // Format dates for display
            const startDate = new Date(queryParams.get('start'));
            const endDate = new Date(queryParams.get('end'));
            
            const formatDateForDisplay = (date) => {
                return date.toLocaleString(undefined, {
                    year: 'numeric',
                    month: 'short',
                    day: 'numeric',
                    hour: '2-digit',
                    minute: '2-digit'
                });
            };
            
            dateRangeText = `${formatDateForDisplay(startDate)} - ${formatDateForDisplay(endDate)}`;
        } else if (queryParams.has('start')) {
            const startDate = new Date(queryParams.get('start'));
            dateRangeText = `From ${startDate.toLocaleString()}`;
        } else if (queryParams.has('end')) {
            const endDate = new Date(queryParams.get('end'));
            dateRangeText = `Until ${endDate.toLocaleString()}`;
        }
        
        const dateFilterTag = document.createElement('div');
        dateFilterTag.className = 'filter-tag';
        dateFilterTag.innerHTML = `
            <span>Date: ${dateRangeText}</span>
            <span class="remove-filter" data-filter="date">×</span>
        `;
        activeFiltersContainer.appendChild(dateFilterTag);
    }
    
    // Add stream filter tag
    if (queryParams.has('stream')) {
        const streamFilterTag = document.createElement('div');
        streamFilterTag.className = 'filter-tag';
        streamFilterTag.innerHTML = `
            <span>Stream: ${queryParams.get('stream')}</span>
            <span class="remove-filter" data-filter="stream">×</span>
        `;
        activeFiltersContainer.appendChild(streamFilterTag);
    }
    
    // Add detection filter tag
    if (queryParams.has('detection')) {
        const detectionFilterTag = document.createElement('div');
        detectionFilterTag.className = 'filter-tag';
        detectionFilterTag.innerHTML = `
            <span>Type: Detection Events</span>
            <span class="remove-filter" data-filter="detection">×</span>
        `;
        activeFiltersContainer.appendChild(detectionFilterTag);
    }
    
    // Add event listeners to remove filter buttons
    document.querySelectorAll('.remove-filter').forEach(btn => {
        btn.addEventListener('click', function() {
            const filterType = this.getAttribute('data-filter');
            
            // Get current URL parameters
            const currentParams = new URLSearchParams(window.location.search);
            
            // Remove the appropriate parameters
            if (filterType === 'date') {
                currentParams.delete('start');
                currentParams.delete('end');
                
                // Reset date inputs
                const dateRangeSelect = document.getElementById('date-range-select');
                if (dateRangeSelect) dateRangeSelect.value = 'today';
                
                const startDate = document.getElementById('start-date');
                if (startDate) startDate.value = '';
                
                const startTime = document.getElementById('start-time');
                if (startTime) startTime.value = '';
                
                const endDate = document.getElementById('end-date');
                if (endDate) endDate.value = '';
                
                const endTime = document.getElementById('end-time');
                if (endTime) endTime.value = '';
                
                // Hide custom date range if it was showing
                toggleCustomDateRange();
            } else if (filterType === 'stream') {
                currentParams.delete('stream');
                
                // Reset stream filter
                const streamFilter = document.getElementById('stream-filter');
                if (streamFilter) streamFilter.value = 'all';
            } else if (filterType === 'detection') {
                currentParams.delete('detection');
                
                // Reset detection filter
                const detectionFilter = document.getElementById('detection-filter');
                if (detectionFilter) detectionFilter.value = 'all';
            }
            
            // Reload with new parameters
            loadRecordings(1);
        });
    });
}

/**
 * Toggle the custom date range inputs based on the date range select value
 */
function toggleCustomDateRange() {
    const dateRangeSelect = document.getElementById('date-range-select');
    const customDateRange = document.getElementById('custom-date-range');
    
    if (!dateRangeSelect || !customDateRange) return;
    
    if (dateRangeSelect.value === 'custom') {
        customDateRange.style.display = 'block';
    } else {
        customDateRange.style.display = 'none';
    }
}

/**
 * Toggle the filters sidebar on mobile
 */
function toggleFiltersSidebar() {
    const sidebar = document.getElementById('filters-sidebar');
    if (!sidebar) return;
    
    sidebar.classList.toggle('collapsed');
}

/**
 * Reset all filters to default values
 */
function resetFilters() {
    // Reset date range
    const dateRangeSelect = document.getElementById('date-range-select');
    if (dateRangeSelect) dateRangeSelect.value = 'today';
    
    // Reset date inputs
    const startDate = document.getElementById('start-date');
    if (startDate) startDate.value = '';
    
    const startTime = document.getElementById('start-time');
    if (startTime) startTime.value = '';
    
    const endDate = document.getElementById('end-date');
    if (endDate) endDate.value = '';
    
    const endTime = document.getElementById('end-time');
    if (endTime) endTime.value = '';
    
    // Reset stream filter
    const streamFilter = document.getElementById('stream-filter');
    if (streamFilter) streamFilter.value = 'all';
    
    // Reset detection filter
    const detectionFilter = document.getElementById('detection-filter');
    if (detectionFilter) detectionFilter.value = 'all';
    
    // Reset page size
    const pageSizeSelect = document.getElementById('page-size');
    if (pageSizeSelect) pageSizeSelect.value = '20';
    
    // Hide custom date range
    toggleCustomDateRange();
    
    // Load recordings with default filters
    loadRecordings(1);
}

/**
 * Initialize the page with URL parameters if present
 */
function initializeFromUrl() {
    const urlParams = new URLSearchParams(window.location.search);
    
    // Set sort parameters if present
    if (urlParams.has('sort') && urlParams.has('order')) {
        const sortBy = urlParams.get('sort');
        const sortOrder = urlParams.get('order');
        
        // Find the header with the matching data-sort attribute
        const header = document.querySelector(`th[data-sort="${sortBy}"]`);
        if (header) {
            // Remove sort classes from all headers
            document.querySelectorAll('th.sortable').forEach(th => {
                th.classList.remove('sort-asc', 'sort-desc');
                th.querySelector('.sort-icon').textContent = '';
            });
            
            // Set the sort class and icon
            if (sortOrder === 'asc') {
                header.classList.add('sort-asc');
                header.querySelector('.sort-icon').textContent = '▲';
            } else {
                header.classList.add('sort-desc');
                header.querySelector('.sort-icon').textContent = '▼';
            }
        }
    }
    
    // Set date range
    if (urlParams.has('start') || urlParams.has('end')) {
        const dateRangeSelect = document.getElementById('date-range-select');
        if (dateRangeSelect) dateRangeSelect.value = 'custom';
        
        // Set start date and time if present
        if (urlParams.has('start')) {
            const startDateTime = new Date(urlParams.get('start'));
            
            const startDate = document.getElementById('start-date');
            if (startDate) {
                const year = startDateTime.getFullYear();
                const month = String(startDateTime.getMonth() + 1).padStart(2, '0');
                const day = String(startDateTime.getDate()).padStart(2, '0');
                startDate.value = `${year}-${month}-${day}`;
            }
            
            const startTime = document.getElementById('start-time');
            if (startTime) {
                const hours = String(startDateTime.getHours()).padStart(2, '0');
                const minutes = String(startDateTime.getMinutes()).padStart(2, '0');
                startTime.value = `${hours}:${minutes}`;
            }
        }
        
        // Set end date and time if present
        if (urlParams.has('end')) {
            const endDateTime = new Date(urlParams.get('end'));
            
            const endDate = document.getElementById('end-date');
            if (endDate) {
                const year = endDateTime.getFullYear();
                const month = String(endDateTime.getMonth() + 1).padStart(2, '0');
                const day = String(endDateTime.getDate()).padStart(2, '0');
                endDate.value = `${year}-${month}-${day}`;
            }
            
            const endTime = document.getElementById('end-time');
            if (endTime) {
                const hours = String(endDateTime.getHours()).padStart(2, '0');
                const minutes = String(endDateTime.getMinutes()).padStart(2, '0');
                endTime.value = `${hours}:${minutes}`;
            }
        }
        
        // Show custom date range
        toggleCustomDateRange();
    }
    
    // Set stream filter
    if (urlParams.has('stream')) {
        const streamFilter = document.getElementById('stream-filter');
        if (streamFilter) streamFilter.value = urlParams.get('stream');
    }
    
    // Set detection filter
    if (urlParams.has('detection')) {
        const detectionFilter = document.getElementById('detection-filter');
        if (detectionFilter) detectionFilter.value = 'detection';
    }
    
    // Set page size
    if (urlParams.has('limit')) {
        const pageSizeSelect = document.getElementById('page-size');
        if (pageSizeSelect) pageSizeSelect.value = urlParams.get('limit');
    }
    
    // Set page number
    const page = urlParams.has('page') ? parseInt(urlParams.get('page'), 10) : 1;
    
    // Load recordings with URL parameters
    loadRecordings(page, false);
}

/**
 * Update pagination information display
 */
function updatePaginationInfo(startRecord, endRecord, totalRecords, currentPage, totalPages) {
    const showingElement = document.getElementById('pagination-showing');
    const totalElement = document.getElementById('pagination-total');
    const currentElement = document.getElementById('pagination-current');
    
    if (showingElement) {
        showingElement.textContent = totalRecords > 0 ? `${startRecord}-${endRecord}` : '0-0';
    }
    
    if (totalElement) {
        totalElement.textContent = totalRecords;
    }
    
    if (currentElement) {
        currentElement.textContent = `Page ${currentPage} of ${totalPages}`;
    }
}

/**
 * Setup pagination button event handlers
 */
function setupPaginationButtons(currentPage, totalPages) {
    const firstBtn = document.getElementById('pagination-first');
    const prevBtn = document.getElementById('pagination-prev');
    const nextBtn = document.getElementById('pagination-next');
    const lastBtn = document.getElementById('pagination-last');
    
    // Disable first/prev buttons if on first page
    if (firstBtn) {
        firstBtn.disabled = currentPage <= 1;
        firstBtn.onclick = () => loadRecordings(1);
    }
    
    if (prevBtn) {
        prevBtn.disabled = currentPage <= 1;
        prevBtn.onclick = () => loadRecordings(currentPage - 1);
    }
    
    // Disable next/last buttons if on last page
    if (nextBtn) {
        nextBtn.disabled = currentPage >= totalPages;
        nextBtn.onclick = () => loadRecordings(currentPage + 1);
    }
    
    if (lastBtn) {
        lastBtn.disabled = currentPage >= totalPages;
        lastBtn.onclick = () => loadRecordings(totalPages);
    }
}

/**
 * Download recording - Reliable approach using link element
 */
function downloadRecording(recordingId) {
    // Create and click an anchor element for more reliable download
    const link = document.createElement('a');
    link.href = `/api/recordings/download/${recordingId}?download=1`;
    link.download = '';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}

/**
 * Delete recording
 */
function deleteRecording(recordingId) {
    if (!confirm('Are you sure you want to delete this recording?')) {
        return;
    }

    const recordingsTable = document.getElementById('recordings-table');
    if (recordingsTable) {
        showLoading(recordingsTable);
    }

    // Send delete request to API
    fetch(`/api/recordings/${recordingId}`, {
        method: 'DELETE'
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to delete recording');
            }
            return response.json();
        })
        .then(data => {
            // Show success message
            showStatusMessage('Recording deleted successfully', 5000);

            // Reload recordings
            loadRecordings();
        })
        .catch(error => {
            console.error('Error deleting recording:', error);
            showStatusMessage('Error deleting recording: ' + error.message, 5000);

            if (recordingsTable) {
                hideLoading(recordingsTable);
            }
        });
}

/**
 * Play recording in modal - Generic implementation without Alpine.js
 */
function playRecording(recordingId) {
    // Get the modal elements
    const modal = document.getElementById('video-modal');
    const videoPlayer = document.getElementById('video-player');
    const videoTitle = document.getElementById('video-modal-title');
    const videoCloseBtn = document.getElementById('video-close-btn');
    const modalCloseBtn = modal?.querySelector('.close');
    const downloadBtn = document.getElementById('video-download-btn');
    
    if (!modal || !videoPlayer || !videoTitle) {
        console.error('Video modal elements not found');
        return;
    }
    
    // Define close modal function
    function closeModal() {
        modal.style.display = 'none';
        
        // Stop video playback
        const videoElement = videoPlayer.querySelector('video');
        if (videoElement) {
            videoElement.pause();
            videoElement.src = '';
        }
    }

    // Set up close button event handlers
    if (videoCloseBtn) {
        videoCloseBtn.onclick = closeModal;
    }
    
    if (modalCloseBtn) {
        modalCloseBtn.onclick = closeModal;
    }

    // Close on click outside
    window.onclick = function(event) {
        if (event.target === modal) {
            closeModal();
        }
    };
    
    // Show loading state and make modal visible
    videoTitle.textContent = 'Loading Recording...';
    modal.classList.remove('hidden');
    modal.style.display = 'block';
    
    // Reset video player
    videoPlayer.innerHTML = '<video class="w-full h-full" controls></video>';
    
    // Fetch recording details
    fetch(`/api/recordings/${recordingId}`)
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load recording details');
            }
            return response.json();
        })
        .then(data => {
            // Update modal title
            videoTitle.textContent = `${data.stream} - ${data.start_time}`;
            
            // Get video element
            const videoElement = videoPlayer.querySelector('video');
            if (!videoElement) {
                throw new Error('Video element not found');
            }
            
            // Set video source
            const videoUrl = `/api/recordings/play/${recordingId}`;
            const downloadUrl = `/api/recordings/download/${recordingId}?download=1`;
            
            // Determine if HLS or MP4
            if (data.path && data.path.endsWith('.m3u8')) {
                if (Hls && Hls.isSupported()) {
                    let hls = new Hls();
                    hls.loadSource(videoUrl);
                    hls.attachMedia(videoElement);
                    hls.on(Hls.Events.MANIFEST_PARSED, function () {
                        videoElement.play();
                    });
                } else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
                    videoElement.src = videoUrl;
                    videoElement.play();
                } else {
                    console.error('HLS not supported');
                    videoPlayer.innerHTML = `
                        <div style="height:70vh;display:flex;flex-direction:column;justify-content:center;align-items:center;background:#000;color:#fff;padding:20px;text-align:center;">
                            <p style="font-size:18px;margin-bottom:10px;">HLS playback is not supported in this browser.</p>
                            <a href="${downloadUrl}" class="btn btn-primary" download>Download Video</a>
                        </div>
                    `;
                }
            } else {
                // Standard MP4 playback
                videoElement.src = videoUrl;
                videoElement.play().catch(e => console.error('Error playing video:', e));
            }
            
            // Setup download button
            if (downloadBtn) {
                downloadBtn.onclick = function(e) {
                    e.preventDefault();
                    downloadRecording(recordingId);
                    return false;
                };
            }
        })
        .catch(error => {
            console.error('Error loading recording details:', error);
            videoTitle.textContent = 'Error Loading Recording';
            videoPlayer.innerHTML = `
                <div style="height:70vh;display:flex;flex-direction:column;justify-content:center;align-items:center;background:#000;color:#fff;padding:20px;text-align:center;">
                    <p style="font-size:18px;margin-bottom:10px;">Error: ${error.message}</p>
                    <p style="margin-bottom:20px;">Cannot load the recording.</p>
                    <a href="/api/recordings/play/${recordingId}" class="btn btn-primary">Play Video</a>
                    <a href="/api/recordings/download/${recordingId}?download=1" class="btn btn-primary" download>Download Video</a>
                </div>
            `;
        });
}

/**
 * Update stream filter dropdown with available streams
 */
function updateStreamFilter(streams) {
    const streamFilter = document.getElementById('stream-filter');
    if (!streamFilter) return;
    
    // Keep the "All Streams" option
    streamFilter.innerHTML = '<option value="all">All Streams</option>';
    
    // Add each stream as an option
    if (streams && streams.length > 0) {
        streams.forEach(stream => {
            const option = document.createElement('option');
            option.value = stream.name;
            option.textContent = stream.name;
            streamFilter.appendChild(option);
        });
    }
    
    // If URL has a stream parameter, select it
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.has('stream')) {
        streamFilter.value = urlParams.get('stream');
    }
}

/**
 * Setup event handlers for recordings page
 */
function setupRecordingsHandlers() {
    // Initialize from URL parameters
    initializeFromUrl();
    
    // Setup date range select
    const dateRangeSelect = document.getElementById('date-range-select');
    if (dateRangeSelect) {
        dateRangeSelect.addEventListener('change', toggleCustomDateRange);
        toggleCustomDateRange(); // Initial toggle
    }
    
    // Setup apply filters button
    const applyFiltersBtn = document.getElementById('apply-filters-btn');
    if (applyFiltersBtn) {
        applyFiltersBtn.addEventListener('click', () => loadRecordings(1));
    }
    
    // Setup reset filters button
    const resetFiltersBtn = document.getElementById('reset-filters-btn');
    if (resetFiltersBtn) {
        resetFiltersBtn.addEventListener('click', resetFilters);
    }
    
    // Setup toggle filters button for mobile
    const toggleFiltersBtn = document.getElementById('toggle-filters-btn');
    if (toggleFiltersBtn) {
        toggleFiltersBtn.addEventListener('click', toggleFiltersSidebar);
    }
    
    // Setup sortable table headers
    document.querySelectorAll('th.sortable').forEach(th => {
        th.addEventListener('click', function() {
            // Get current sort state
            const isAsc = this.classList.contains('sort-asc');
            const isDesc = this.classList.contains('sort-desc');
            
            // Remove sort classes from all headers
            document.querySelectorAll('th.sortable').forEach(header => {
                header.classList.remove('sort-asc', 'sort-desc');
                header.querySelector('.sort-icon').textContent = '';
            });
            
            // Set new sort state
            if (!isAsc && !isDesc) {
                // Default to descending
                this.classList.add('sort-desc');
                this.querySelector('.sort-icon').textContent = '▼';
            } else if (isDesc) {
                // Toggle to ascending
                this.classList.add('sort-asc');
                this.querySelector('.sort-icon').textContent = '▲';
            } else {
                // Toggle to descending
                this.classList.add('sort-desc');
                this.querySelector('.sort-icon').textContent = '▼';
            }
            
            // Reload recordings with new sort
            loadRecordings(1);
        });
    });
    
    // Setup video modal close buttons
    const videoModal = document.getElementById('video-modal');
    const videoCloseBtn = document.getElementById('video-close-btn');
    const modalCloseBtn = document.querySelector('.modal .close');
    
    if (videoModal) {
        // Close button in footer
        if (videoCloseBtn) {
            videoCloseBtn.addEventListener('click', () => {
                videoModal.style.display = 'none';
                
                // Stop video playback
                const videoPlayer = document.getElementById('video-player');
                if (videoPlayer) {
                    videoPlayer.pause();
                    videoPlayer.src = '';
                }
            });
        }
        
        // X button in header
        if (modalCloseBtn) {
            modalCloseBtn.addEventListener('click', () => {
                videoModal.style.display = 'none';
                
                // Stop video playback
                const videoPlayer = document.getElementById('video-player');
                if (videoPlayer) {
                    videoPlayer.pause();
                    videoPlayer.src = '';
                }
            });
        }
        
        // Close on click outside
        window.addEventListener('click', (event) => {
            if (event.target === videoModal) {
                videoModal.style.display = 'none';
                
                // Stop video playback
                const videoPlayer = document.getElementById('video-player');
                if (videoPlayer) {
                    videoPlayer.pause();
                    videoPlayer.src = '';
                }
            }
        });
    }
}
