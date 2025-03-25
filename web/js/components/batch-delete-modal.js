/**
 * Batch Delete Modal Component
 * Displays progress of batch delete operations with WebSocket updates
 */

/**
 * Show a status message to the user
 * @param {string} message - Message to display
 * @param {number} duration - Duration in milliseconds (default: 3000)
 */
function showStatusMessage(message, duration = 3000) {
    // Check if a status message container already exists
    let statusContainer = document.getElementById('status-message-container');
    
    // Create container if it doesn't exist
    if (!statusContainer) {
        statusContainer = document.createElement('div');
        statusContainer.id = 'status-message-container';
        statusContainer.className = 'fixed bottom-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col items-center';
        document.body.appendChild(statusContainer);
    }
    
    // Create message element
    const messageElement = document.createElement('div');
    messageElement.className = 'bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg mb-2 transition-all duration-300 opacity-0 transform translate-y-2';
    messageElement.textContent = message;
    
    // Add to container
    statusContainer.appendChild(messageElement);
    
    // Trigger animation to show message
    setTimeout(() => {
        messageElement.classList.remove('opacity-0', 'translate-y-2');
    }, 10);
    
    // Set timeout to remove message
    setTimeout(() => {
        // Trigger animation to hide message
        messageElement.classList.add('opacity-0', 'translate-y-2');
        
        // Remove element after animation completes
        setTimeout(() => {
            if (messageElement.parentNode === statusContainer) {
                statusContainer.removeChild(messageElement);
            }
            
            // Remove container if no more messages
            if (statusContainer.children.length === 0) {
                document.body.removeChild(statusContainer);
            }
        }, 300);
    }, duration);
}

/**
 * Show loading indicator on an element
 * @param {HTMLElement} element - Element to show loading on
 */
function showLoading(element) {
    if (!element) return;
    
    // Add loading class
    element.classList.add('loading');
    
    // Create loading overlay if it doesn't exist
    let loadingOverlay = element.querySelector('.loading-overlay');
    if (!loadingOverlay) {
        loadingOverlay = document.createElement('div');
        loadingOverlay.className = 'loading-overlay';
        loadingOverlay.innerHTML = '<div class="loading-spinner"></div>';
        element.appendChild(loadingOverlay);
    }
    
    // Show loading overlay
    loadingOverlay.style.display = 'flex';
}

/**
 * Hide loading indicator on an element
 * @param {HTMLElement} element - Element to hide loading from
 */
function hideLoading(element) {
    if (!element) return;
    
    // Remove loading class
    element.classList.remove('loading');
    
    // Hide loading overlay
    const loadingOverlay = element.querySelector('.loading-overlay');
    if (loadingOverlay) {
        loadingOverlay.style.display = 'none';
    }
}

/**
 * Initialize the batch delete modal
 */
function initBatchDeleteModal() {
    // Create modal container if it doesn't exist
    let modalContainer = document.getElementById('batch-delete-modal-container');
    if (!modalContainer) {
        modalContainer = document.createElement('div');
        modalContainer.id = 'batch-delete-modal-container';
        document.body.appendChild(modalContainer);
    }

    // Create modal HTML
    modalContainer.innerHTML = `
        <div id="batch-delete-modal" class="modal hidden fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center z-50">
            <div class="modal-content relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md mx-auto p-6 w-full">
                <div class="modal-header flex justify-between items-center mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">
                    <h3 id="batch-delete-modal-title" class="text-xl font-bold text-gray-900 dark:text-white">Batch Delete Progress</h3>
                    <button id="batch-delete-close-btn" class="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 text-2xl font-bold">&times;</button>
                </div>
                <div class="modal-body">
                    <div id="batch-delete-status" class="mb-4 text-gray-700 dark:text-gray-300">
                        Preparing to delete recordings...
                    </div>
                    <div class="progress-container bg-gray-200 dark:bg-gray-700 rounded-full h-4 mb-4">
                        <div id="batch-delete-progress-bar" class="bg-green-500 h-4 rounded-full text-center text-xs text-white" style="width: 0%"></div>
                    </div>
                    <div class="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-6">
                        <div id="batch-delete-count">0 / 0</div>
                        <div id="batch-delete-percentage">0%</div>
                    </div>
                    <div id="batch-delete-details" class="mb-4">
                        <div class="flex justify-between mb-2">
                            <span class="text-gray-700 dark:text-gray-300">Succeeded:</span>
                            <span id="batch-delete-succeeded" class="font-bold text-green-600 dark:text-green-400">0</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-gray-700 dark:text-gray-300">Failed:</span>
                            <span id="batch-delete-failed" class="font-bold text-red-600 dark:text-red-400">0</span>
                        </div>
                    </div>
                    <div id="batch-delete-message" class="text-sm italic text-gray-600 dark:text-gray-400 mb-4"></div>
                </div>
                <div class="modal-footer flex justify-end pt-2 border-t border-gray-200 dark:border-gray-700">
                    <button id="batch-delete-done-btn" class="btn btn-primary hidden">Done</button>
                    <button id="batch-delete-cancel-btn" class="btn btn-secondary">Cancel</button>
                </div>
            </div>
        </div>
    `;

    // Get modal elements
    const modal = document.getElementById('batch-delete-modal');
    const closeBtn = document.getElementById('batch-delete-close-btn');
    const doneBtn = document.getElementById('batch-delete-done-btn');
    const cancelBtn = document.getElementById('batch-delete-cancel-btn');

    // Setup close button event handlers
    if (closeBtn) {
        closeBtn.addEventListener('click', closeBatchDeleteModal);
    }

    if (doneBtn) {
        doneBtn.addEventListener('click', closeBatchDeleteModal);
    }

    if (cancelBtn) {
        cancelBtn.addEventListener('click', cancelBatchDelete);
    }

    // Close on click outside
    window.addEventListener('click', (event) => {
        if (event.target === modal) {
            closeBatchDeleteModal();
        }
    });
}

/**
 * Show the batch delete modal
 */
function showBatchDeleteModal() {
    const modal = document.getElementById('batch-delete-modal');
    if (!modal) {
        initBatchDeleteModal();
    }

    // Reset modal state
    resetBatchDeleteModal();

    // Show modal
    modal.classList.remove('hidden');
}

/**
 * Close the batch delete modal
 */
function closeBatchDeleteModal() {
    const modal = document.getElementById('batch-delete-modal');
    if (modal) {
        modal.classList.add('hidden');
    }
}

/**
 * Reset the batch delete modal state
 */
function resetBatchDeleteModal() {
    // Reset progress bar
    const progressBar = document.getElementById('batch-delete-progress-bar');
    if (progressBar) {
        progressBar.style.width = '0%';
    }

    // Reset status text
    const status = document.getElementById('batch-delete-status');
    if (status) {
        status.textContent = 'Preparing to delete recordings...';
    }

    // Reset count and percentage
    const count = document.getElementById('batch-delete-count');
    if (count) {
        count.textContent = '0 / 0';
    }

    const percentage = document.getElementById('batch-delete-percentage');
    if (percentage) {
        percentage.textContent = '0%';
    }

    // Reset success/fail counts
    const succeeded = document.getElementById('batch-delete-succeeded');
    if (succeeded) {
        succeeded.textContent = '0';
    }

    const failed = document.getElementById('batch-delete-failed');
    if (failed) {
        failed.textContent = '0';
    }

    // Reset message
    const message = document.getElementById('batch-delete-message');
    if (message) {
        message.textContent = '';
    }

    // Hide done button, show cancel button
    const doneBtn = document.getElementById('batch-delete-done-btn');
    if (doneBtn) {
        doneBtn.classList.add('hidden');
    }

    const cancelBtn = document.getElementById('batch-delete-cancel-btn');
    if (cancelBtn) {
        cancelBtn.classList.remove('hidden');
    }
}

/**
 * Update batch delete progress
 * 
 * @param {Object} progress Progress data from WebSocket
 */
function updateBatchDeleteProgress(progress) {
    console.log('Updating batch delete progress UI:', progress);
    
    // Update progress bar
    const progressBar = document.getElementById('batch-delete-progress-bar');
    if (progressBar) {
        console.log(`Updating progress bar: current=${progress.current}, total=${progress.total}`);
        
        if (progress.total > 0) {
            // We have a known total, show percentage
            const percent = Math.round((progress.current / progress.total) * 100);
            console.log(`Setting progress bar width to ${percent}%`);
            progressBar.style.width = `${percent}%`;
            progressBar.classList.remove('animate-pulse');
            
            // Force a reflow to ensure the browser updates the UI
            void progressBar.offsetWidth;
        } else if (progress.current > 0) {
            // We don't know the total but have processed some items
            // Show an indeterminate but growing progress bar
            const estimatedPercent = Math.min(90, progress.current / 10);
            console.log(`Setting progress bar width to ${estimatedPercent}% (estimated)`);
            progressBar.style.width = `${estimatedPercent}%`;
            progressBar.classList.add('animate-pulse');
            
            // Force a reflow to ensure the browser updates the UI
            void progressBar.offsetWidth;
        } else if (progress.complete) {
            // Operation is complete but we don't know the total
            console.log('Setting progress bar to 100% (complete)');
            progressBar.style.width = '100%';
            progressBar.classList.remove('animate-pulse');
            
            // Force a reflow to ensure the browser updates the UI
            void progressBar.offsetWidth;
        } else {
            // Indeterminate progress
            console.log('Setting progress bar to 50% (indeterminate)');
            progressBar.style.width = '50%';
            progressBar.classList.add('animate-pulse');
            
            // Force a reflow to ensure the browser updates the UI
            void progressBar.offsetWidth;
        }
    }

    // Update status text
    const status = document.getElementById('batch-delete-status');
    if (status && progress.status) {
        status.textContent = progress.status;
    }

    // Update count and percentage
    const count = document.getElementById('batch-delete-count');
    if (count) {
        if (progress.total > 0) {
            count.textContent = `${progress.current} / ${progress.total}`;
        } else {
            count.textContent = `${progress.current} / ?`;
        }
    }

    const percentage = document.getElementById('batch-delete-percentage');
    if (percentage) {
        if (progress.total > 0) {
            const percent = Math.round((progress.current / progress.total) * 100);
            percentage.textContent = `${percent}%`;
        } else if (progress.complete) {
            percentage.textContent = '100%';
        } else {
            percentage.textContent = 'In progress';
        }
    }

    // Update success/fail counts
    const succeeded = document.getElementById('batch-delete-succeeded');
    if (succeeded) {
        succeeded.textContent = progress.succeeded || '0';
    }

    const failed = document.getElementById('batch-delete-failed');
    if (failed) {
        failed.textContent = progress.failed || '0';
    }

    // If complete, show done button, hide cancel button
    if (progress.complete) {
        const doneBtn = document.getElementById('batch-delete-done-btn');
        if (doneBtn) {
            doneBtn.classList.remove('hidden');
        }

        const cancelBtn = document.getElementById('batch-delete-cancel-btn');
        if (cancelBtn) {
            cancelBtn.classList.add('hidden');
        }

        // Update status if not already set
        if (status && (!progress.status || progress.status === 'Preparing to delete recordings...')) {
            status.textContent = 'Batch delete operation complete';
        }
        
        // Ensure progress bar shows 100%
        if (progressBar) {
            progressBar.style.width = '100%';
            progressBar.classList.remove('animate-pulse');
        }
    }
}

// Make updateBatchDeleteProgress globally accessible
window.updateBatchDeleteProgress = updateBatchDeleteProgress;

/**
 * Cancel batch delete operation
 */
function cancelBatchDelete() {
    // Close modal
    closeBatchDeleteModal();

    // Show status message
    showStatusMessage('Batch delete operation cancelled', 5000);
}

/**
 * Initialize batch delete client
 * 
 * @returns {BatchDeleteRecordingsClient} Batch delete client
 */
function initBatchDeleteClient() {
    // Use the global WebSocket client that was initialized in preact-app.js
    // If it doesn't exist for some reason, create it
    if (!window.wsClient) {
        console.log('Creating WebSocket client in batch-delete-modal.js (fallback)');
        window.wsClient = new WebSocketClient();
    } else {
        console.log('Using existing WebSocket client from preact-app.js');
    }

    // Create batch delete client if it doesn't exist
    if (!window.batchDeleteClient) {
        console.log('Creating BatchDeleteRecordingsClient');
        window.batchDeleteClient = new BatchDeleteRecordingsClient(window.wsClient);

        // Set up event handlers
        window.batchDeleteClient.onProgress((payload) => {
            console.log('Batch delete progress:', payload);
            updateBatchDeleteProgress(payload);
        });

        window.batchDeleteClient.onResult((payload) => {
            console.log('Batch delete result:', payload);
            // Update final progress
            updateBatchDeleteProgress({
                current: payload.total,
                total: payload.total,
                succeeded: payload.succeeded,
                failed: payload.failed,
                complete: true
            });

            // Show status message
            const message = payload.success
                ? `Successfully deleted ${payload.succeeded} recordings`
                : `Deleted ${payload.succeeded} recordings with ${payload.failed} failures`;
            
            showStatusMessage(message, 5000);

            // Reload recordings after a short delay
            setTimeout(() => {
                loadRecordings();
            }, 1000);
        });

        window.batchDeleteClient.onError((payload) => {
            console.error('Batch delete error:', payload);
            // Show error message
            showStatusMessage(`Error: ${payload.error || 'Unknown error'}`, 5000);

            // Close modal
            closeBatchDeleteModal();
        });
    }

    return window.batchDeleteClient;
}

/**
 * Delete recordings by HTTP request (fallback when WebSocket is not available)
 * 
 * @param {Object} params Delete parameters (ids or filter)
 * @returns {Promise<Object>} Promise that resolves when the operation is complete
 */
function batchDeleteRecordingsByHttpRequest(params) {
    console.log('Using HTTP fallback for batch delete with params:', params);
    
    return new Promise((resolve, reject) => {
        // Show modal
        showBatchDeleteModal();
        
        // Update progress to show we're using HTTP
        updateBatchDeleteProgress({
            current: 0,
            total: params.ids ? params.ids.length : 0,
            status: 'Using HTTP fallback for batch delete operation',
            succeeded: 0,
            failed: 0
        });
        
        // Send HTTP request
        fetch('/api/recordings/batch-delete', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(params)
        })
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error ${response.status}: ${response.statusText}`);
            }
            return response.json();
        })
        .then(result => {
            console.log('HTTP batch delete result:', result);
            
            // Update final progress
            updateBatchDeleteProgress({
                current: result.total || 0,
                total: result.total || 0,
                succeeded: result.succeeded || 0,
                failed: result.failed || 0,
                complete: true
            });
            
            // Show status message
            const message = result.success
                ? `Successfully deleted ${result.succeeded} recordings`
                : `Deleted ${result.succeeded} recordings with ${result.failed} failures`;
            
            showStatusMessage(message, 5000);
            
            // Reload recordings after a short delay
            setTimeout(() => {
                if (typeof loadRecordings === 'function') {
                    loadRecordings();
                }
            }, 1000);
            
            resolve(result);
        })
        .catch(error => {
            console.error('HTTP batch delete error:', error);
            
            // Show error message
            showStatusMessage(`Error: ${error.message || 'Unknown error'}`, 5000);
            
            // Close modal
            closeBatchDeleteModal();
            
            reject(error);
        });
    });
}

/**
 * Delete multiple recordings by IDs
 * 
 * @param {Array<number>} ids Recording IDs to delete
 */
function batchDeleteRecordings(ids) {
    if (!ids || ids.length === 0) {
        showStatusMessage('No recordings selected for deletion', 5000);
        return;
    }

    // Confirm deletion
    if (!confirm(`Are you sure you want to delete ${ids.length} recordings?`)) {
        return;
    }

    // Initialize batch delete client
    const batchDeleteClient = initBatchDeleteClient();

    // Show modal
    showBatchDeleteModal();

    // Start batch delete operation
    batchDeleteClient.deleteWithProgress({ ids })
        .catch(error => {
            console.error('Error starting batch delete:', error);
            showStatusMessage(`Error: ${error.message || 'Failed to start batch delete operation'}`, 5000);
            closeBatchDeleteModal();
        });
}

/**
 * Delete recordings by filter
 * 
 * @param {Object} filter Filter to delete by
 */
function batchDeleteRecordingsByFilter(filter) {
    if (!filter) {
        showStatusMessage('No filter specified for deletion', 5000);
        return;
    }

    // Confirm deletion
    if (!confirm('Are you sure you want to delete all recordings matching the current filter?')) {
        return;
    }

    // Initialize batch delete client
    const batchDeleteClient = initBatchDeleteClient();

    // Show modal
    showBatchDeleteModal();

    // Start batch delete operation
    batchDeleteClient.deleteWithProgress({ filter })
        .catch(error => {
            console.error('Error starting batch delete:', error);
            showStatusMessage(`Error: ${error.message || 'Failed to start batch delete operation'}`, 5000);
            closeBatchDeleteModal();
        });
}

// Initialize batch delete modal when the page loads
document.addEventListener('DOMContentLoaded', initBatchDeleteModal);
