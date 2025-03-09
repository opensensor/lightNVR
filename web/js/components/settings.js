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

    // Fetch settings from the server
    fetch('/api/settings')
        .then(response => {
            if (!response.ok) {
                throw new Error('Failed to load settings');
            }
            return response.json();
        })
        .then(settings => {
            // Update form fields with settings values
            document.getElementById('setting-log-level').value = settings.log_level;
            document.getElementById('setting-storage-path').value = settings.storage_path;
            document.getElementById('setting-max-storage').value = settings.max_storage;
            document.getElementById('setting-retention').value = settings.retention;
            document.getElementById('setting-auto-delete').checked = settings.auto_delete;
            document.getElementById('setting-web-port').value = settings.web_port;
            document.getElementById('setting-auth-enabled').checked = settings.auth_enabled;
            document.getElementById('setting-username').value = settings.username;
            document.getElementById('setting-password').value = settings.password;
            document.getElementById('setting-buffer-size').value = settings.buffer_size;
            document.getElementById('setting-use-swap').checked = settings.use_swap;
            document.getElementById('setting-swap-size').value = settings.swap_size;
        })
        .catch(error => {
            console.error('Error loading settings:', error);
            alert('Error loading settings. Please try again.');
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

    // Collect all settings from the form
    const settings = {
        log_level: parseInt(document.getElementById('setting-log-level').value, 10),
        storage_path: document.getElementById('setting-storage-path').value,
        max_storage: parseInt(document.getElementById('setting-max-storage').value, 10),
        retention: parseInt(document.getElementById('setting-retention').value, 10),
        auto_delete: document.getElementById('setting-auto-delete').checked,
        web_port: parseInt(document.getElementById('setting-web-port').value, 10),
        auth_enabled: document.getElementById('setting-auth-enabled').checked,
        username: document.getElementById('setting-username').value,
        password: document.getElementById('setting-password').value,
        buffer_size: parseInt(document.getElementById('setting-buffer-size').value, 10),
        use_swap: document.getElementById('setting-use-swap').checked,
        swap_size: parseInt(document.getElementById('setting-swap-size').value, 10)
    };

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
                throw new Error('Failed to save settings');
            }
            return response.json();
        })
        .then(data => {
            alert('Settings saved successfully');
        })
        .catch(error => {
            console.error('Error saving settings:', error);
            alert('Error saving settings: ' + error.message);
        })
        .finally(() => {
            hideLoading(settingsContainer);
        });
}
