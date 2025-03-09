/**
 * LightNVR Web Interface Recordings Management
 * Contains functionality for managing recordings (load, play, download, delete)
 */

/**
 * Load recordings with pagination
 */
function loadRecordings(page = 1) {
    const recordingsTable = document.getElementById('recordings-table');
    if (!recordingsTable) return;

    const tbody = recordingsTable.querySelector('tbody');

    showLoading(recordingsTable);

    // Clear existing rows
    tbody.innerHTML = '<tr><td colspan="5" class="empty-message">Loading recordings...</td></tr>';

    // Get filter values
    const dateFilter = document.getElementById('date-picker').value;
    const streamFilter = document.getElementById('stream-filter').value;
    const pageSizeSelect = document.getElementById('page-size');
    const pageSize = pageSizeSelect ? parseInt(pageSizeSelect.value, 10) : 20;

    // Build query string
    let queryParams = new URLSearchParams();
    
    if (dateFilter) {
        queryParams.append('date', dateFilter);
    }
    
    if (streamFilter && streamFilter !== 'all') {
        queryParams.append('stream', streamFilter);
    }
    
    // Add pagination parameters
    queryParams.append('page', page);
    queryParams.append('limit', pageSize);
    
    const queryString = queryParams.toString() ? `?${queryParams.toString()}` : '';

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
            alert('Recording deleted successfully');

            // Reload recordings
            loadRecordings();
        })
        .catch(error => {
            console.error('Error deleting recording:', error);
            alert('Error deleting recording: ' + error.message);

            if (recordingsTable) {
                hideLoading(recordingsTable);
            }
        });
}
