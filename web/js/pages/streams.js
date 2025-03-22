/**
 * LightNVR Web Interface - Streams Page
 * Main entry point that loads the split stream management files
 */

// Register Alpine.js components
document.addEventListener('alpine:init', () => {
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
                showErrorToast('Error loading streams: ' + error.message);
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
                showErrorToast('Error loading stream details: ' + error.message);
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
                
                // First reload streams immediately
                await this.loadStreams();
                
                // Show success message
                showSuccessToast('Stream deleted successfully', 5000);
            } catch (error) {
                console.error('Error deleting stream:', error);
                showErrorToast('Error deleting stream: ' + error.message);
            }
        }
    }));
});

// Load the required scripts in the correct order
document.addEventListener('DOMContentLoaded', function() {
    // First ensure toast.js is loaded
    const toastScript = document.querySelector('script[src*="toast.js"]');
    if (!toastScript) {
        // If toast.js is not already loaded, load it
        const script = document.createElement('script');
        script.src = '/js/components/toast.js';
        script.onload = loadStreamScripts;
        document.head.appendChild(script);
    } else {
        // If toast.js is already loaded, proceed to load stream scripts
        loadStreamScripts();
    }
    
    // Load header and footer
    if (typeof loadHeader === 'function') {
        loadHeader('nav-streams');
    }
    if (typeof loadFooter === 'function') {
        loadFooter();
    }
    
    // Setup modal event listeners
    setTimeout(setupModalEventListeners, 500); // Give time for scripts to load
});

// Function to load stream-specific scripts
function loadStreamScripts() {
    // Load streams-part1.js (vanilla JS functions)
    const part1Script = document.createElement('script');
    part1Script.src = '/js/pages/streams-part1.js';
    document.head.appendChild(part1Script);
}

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
