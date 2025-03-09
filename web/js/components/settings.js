/**
 * LightNVR Web Interface Settings Management
 * Contains functionality for managing system settings
 */

/**
 * Load settings
 */
function loadSettings() {
    const settingsContainer = document.querySelector('.settings-container');
    if (!settingsContainer) return;

    showLoading(settingsContainer);

    // Fetch settings from the server with error handling
    fetch('/api/settings')
        .then(response => {
            if (!response.ok) {
                return response.text().then(text => {
                    console.error('Server returned error:', text);
                    throw new Error(`Server error: ${response.status} ${response.statusText}`);
                });
            }
            return response.text().then(text => {
                try {
                    return JSON.parse(text);
                } catch (e) {
                    console.error('Invalid JSON received:', text);
                    throw new Error('Invalid response format');
                }
            });
        })
        .then(settings => {
            console.log('Received settings:', settings); // Debug log

            if (!settings || typeof settings !== 'object') {
                console.error('Unexpected settings format:', settings);
                throw new Error('Unexpected settings format');
            }

            // Update form fields with settings values - with extensive error handling
            try {
                const logLevel = document.getElementById('setting-log-level');
                if (logLevel) logLevel.value = typeof settings.log_level === 'number' ? settings.log_level : 2;

                const storagePath = document.getElementById('setting-storage-path');
                if (storagePath) storagePath.value = typeof settings.storage_path === 'string' ?
                    settings.storage_path : '/var/lib/lightnvr/recordings';

                const maxStorage = document.getElementById('setting-max-storage');
                if (maxStorage) maxStorage.value = typeof settings.max_storage === 'number' ?
                    settings.max_storage : 0;

                const retention = document.getElementById('setting-retention');
                if (retention) retention.value = typeof settings.retention === 'number' ?
                    settings.retention : 30;

                const autoDelete = document.getElementById('setting-auto-delete');
                if (autoDelete) autoDelete.checked = settings.auto_delete === true ||
                    settings.auto_delete === 'true';

                const webPort = document.getElementById('setting-web-port');
                if (webPort) webPort.value = typeof settings.web_port === 'number' ?
                    settings.web_port : 8080;

                const authEnabled = document.getElementById('setting-auth-enabled');
                if (authEnabled) authEnabled.checked = settings.auth_enabled === true ||
                    settings.auth_enabled === 'true';

                const username = document.getElementById('setting-username');
                if (username) username.value = typeof settings.username === 'string' ?
                    settings.username : 'admin';

                const password = document.getElementById('setting-password');
                if (password) password.value = ''; // Always blank for security

                const bufferSize = document.getElementById('setting-buffer-size');
                if (bufferSize) bufferSize.value = typeof settings.buffer_size === 'number' ?
                    settings.buffer_size : 1024;

                const useSwap = document.getElementById('setting-use-swap');
                if (useSwap) useSwap.checked = settings.use_swap === true ||
                    settings.use_swap === 'true';

                const swapSize = document.getElementById('setting-swap-size');
                if (swapSize) swapSize.value = typeof settings.swap_size === 'number' ?
                    settings.swap_size : 128;

                console.log('Settings loaded successfully');
            } catch (error) {
                console.error('Error processing settings:', error);
                throw new Error('Failed to process settings: ' + error.message);
            }
        })
        .catch(error => {
            console.error('Error loading settings:', error);
            showStatusMessage('Error loading settings: ' + error.message, 'error');
        })
        .finally(() => {
            hideLoading(settingsContainer);
        });
}

/**
 * Save settings
 */
function saveSettings() {
    const settingsContainer = document.querySelector('.settings-container');
    if (!settingsContainer) return;

    showLoading(settingsContainer);

    try {
        // Collect all settings from the form with validation
        const settings = {
            log_level: parseInt(document.getElementById('setting-log-level')?.value || '2', 10),
            storage_path: document.getElementById('setting-storage-path')?.value || '/var/lib/lightnvr/recordings',
            max_storage: parseInt(document.getElementById('setting-max-storage')?.value || '0', 10),
            retention: parseInt(document.getElementById('setting-retention')?.value || '30', 10),
            auto_delete: document.getElementById('setting-auto-delete')?.checked || true,
            web_port: parseInt(document.getElementById('setting-web-port')?.value || '8080', 10),
            auth_enabled: document.getElementById('setting-auth-enabled')?.checked || true,
            username: document.getElementById('setting-username')?.value || 'admin',
            password: document.getElementById('setting-password')?.value || '********',
            buffer_size: parseInt(document.getElementById('setting-buffer-size')?.value || '1024', 10),
            use_swap: document.getElementById('setting-use-swap')?.checked || true,
            swap_size: parseInt(document.getElementById('setting-swap-size')?.value || '128', 10)
        };

        // Validate settings
        if (settings.web_port < 1 || settings.web_port > 65535) {
            throw new Error('Web port must be between 1 and 65535');
        }

        if (settings.buffer_size < 128) {
            throw new Error('Buffer size must be at least 128 KB');
        }

        if (settings.swap_size < 32) {
            throw new Error('Swap size must be at least 32 MB');
        }

        // Send settings to the server
        fetch('/api/settings', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(settings)
        })
            .then(response => {
                if (!response.ok) {
                    return response.json().then(data => {
                        throw new Error(data.error || 'Failed to save settings');
                    });
                }
                return response.json();
            })
            .then(data => {
                console.log('Settings saved successfully:', data);
                // Show success message in the status element
                const statusEl = document.getElementById('settings-status');
                if (statusEl) {
                    statusEl.textContent = 'Settings saved successfully';
                    statusEl.className = 'status-message success';
                    statusEl.style.display = 'block';
                    
                    // Hide after 3 seconds
                    setTimeout(() => {
                        statusEl.style.display = 'none';
                    }, 3000);
                } else {
                    // Fallback to the global status message
                    showStatusMessage('Settings saved successfully');
                }
            })
            .catch(error => {
                console.error('Error saving settings:', error);
                
                // Show error message in the status element
                const statusEl = document.getElementById('settings-status');
                if (statusEl) {
                    statusEl.textContent = 'Error saving settings: ' + error.message;
                    statusEl.className = 'status-message error';
                    statusEl.style.display = 'block';
                } else {
                    // Fallback to alert
                    alert('Error saving settings: ' + error.message);
                }
            })
            .finally(() => {
                hideLoading(settingsContainer);
            });
    } catch (error) {
        console.error('Error preparing settings:', error);
        alert('Error: ' + error.message);
        hideLoading(settingsContainer);
    }
}
