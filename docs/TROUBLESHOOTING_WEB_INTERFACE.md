# Troubleshooting Web Interface Issues

This guide helps you diagnose and fix common web interface issues with LightNVR.

## Blank Page Issue

### Symptoms
- Browser shows a blank white page
- Page title shows "WebRTC View - LightNVR" but no content
- No error messages visible on the page

### Most Common Cause
The web assets (HTML, CSS, JavaScript files) were not installed to the web root directory during installation.

### Quick Diagnosis

Run the diagnostic script:
```bash
sudo bash scripts/diagnose_web_issue.sh
```

This will check:
- Service status
- Configuration file
- Web root directory existence
- Critical web files
- Assets directory
- Recent logs

### Quick Fix

If the diagnostic confirms missing web assets, run:
```bash
sudo bash scripts/install_web_assets.sh
```

This script will:
1. Build web assets if needed (requires Node.js/npm)
2. Install them to the correct location
3. Set proper permissions
4. Verify the installation

Then restart the service:
```bash
sudo systemctl restart lightnvr
```

### Manual Fix

If you prefer to fix it manually:

#### Option 1: Install from prebuilt assets (if available)
```bash
# Navigate to LightNVR source directory
cd /path/to/lightnvr

# Copy prebuilt assets
sudo mkdir -p /var/lib/lightnvr/www
sudo cp -r web/dist/* /var/lib/lightnvr/www/

# Set permissions
sudo chown -R root:root /var/lib/lightnvr/www
sudo chmod -R 755 /var/lib/lightnvr/www

# Restart service
sudo systemctl restart lightnvr
```

#### Option 2: Build and install from source
```bash
# Navigate to LightNVR source directory
cd /path/to/lightnvr

# Install Node.js and npm if not already installed
# On Debian/Ubuntu:
sudo apt-get update
sudo apt-get install -y nodejs npm

# Build web assets
cd web
npm install
npm run build

# Install to web root
sudo mkdir -p /var/lib/lightnvr/www
sudo cp -r dist/* /var/lib/lightnvr/www/

# Set permissions
sudo chown -R root:root /var/lib/lightnvr/www
sudo chmod -R 755 /var/lib/lightnvr/www

# Restart service
cd ..
sudo systemctl restart lightnvr
```

### Verification

After applying the fix, verify the installation:

1. Check that files exist:
```bash
ls -la /var/lib/lightnvr/www/
```

You should see:
- `index.html`
- `login.html`
- `streams.html`
- `recordings.html`
- `assets/` directory with CSS and JS files

2. Check service status:
```bash
sudo systemctl status lightnvr
```

3. Check logs for errors:
```bash
sudo tail -f /var/log/lightnvr/lightnvr.log
```

4. Access the web interface:
```
http://your-server-ip:8080
```

Default credentials:
- Username: `admin`
- Password: `admin`

## Other Common Issues

### Issue: 404 Not Found

**Cause**: Web root path is incorrect in configuration

**Fix**:
1. Check configuration:
```bash
sudo cat /etc/lightnvr/lightnvr.ini | grep "root"
```

2. Verify the path exists and contains files:
```bash
ls -la /var/lib/lightnvr/www/
```

3. If path is different, either:
   - Update config to point to correct path, OR
   - Install web assets to the configured path

### Issue: Permission Denied

**Cause**: Incorrect file permissions

**Fix**:
```bash
sudo chown -R root:root /var/lib/lightnvr/www
sudo chmod -R 755 /var/lib/lightnvr/www
sudo find /var/lib/lightnvr/www -type f -exec chmod 644 {} \;
sudo systemctl restart lightnvr
```

### Issue: JavaScript Errors in Browser Console

**Symptoms**: Browser console (F12) shows errors like "Failed to load module" or "404 for assets"

**Cause**: Incomplete or corrupted web assets

**Fix**:
1. Clear browser cache
2. Reinstall web assets:
```bash
sudo bash scripts/install_web_assets.sh
sudo systemctl restart lightnvr
```

### Issue: Cannot Connect to Server

**Symptoms**: Browser shows "Connection refused" or "Unable to connect"

**Possible Causes**:
1. Service not running
2. Firewall blocking port
3. Wrong IP address or port

**Fix**:

1. Check service status:
```bash
sudo systemctl status lightnvr
```

2. Check if port is listening:
```bash
sudo netstat -tlnp | grep 8080
# or
sudo ss -tlnp | grep 8080
```

3. Check firewall (if using ufw):
```bash
sudo ufw status
sudo ufw allow 8080/tcp
```

4. Check firewall (if using firewalld):
```bash
sudo firewall-cmd --list-all
sudo firewall-cmd --permanent --add-port=8080/tcp
sudo firewall-cmd --reload
```

5. Verify configuration:
```bash
sudo cat /etc/lightnvr/lightnvr.ini | grep "port"
```

### Issue: Login Page Doesn't Work

**Symptoms**: Login page loads but credentials don't work

**Fix**:

1. Check default credentials in config:
```bash
sudo cat /etc/lightnvr/lightnvr.ini | grep -A 2 "\[web\]"
```

2. Try default credentials:
   - Username: `admin`
   - Password: `admin`

3. If you changed the password and forgot it, reset in config:
```bash
sudo nano /etc/lightnvr/lightnvr.ini
```
Change the password line under `[web]` section, then:
```bash
sudo systemctl restart lightnvr
```

## Getting More Help

If none of these solutions work:

1. Collect diagnostic information:
```bash
# Run diagnostic
sudo bash scripts/diagnose_web_issue.sh > diagnostic_output.txt

# Collect logs
sudo journalctl -u lightnvr -n 100 --no-pager > service_logs.txt
sudo tail -n 100 /var/log/lightnvr/lightnvr.log > app_logs.txt 2>/dev/null || true

# Check browser console
# Open browser, press F12, go to Console tab, copy any errors
```

2. Create an issue on GitHub with:
   - Output from diagnostic script
   - Service logs
   - Application logs
   - Browser console errors (if any)
   - Your OS and version
   - Installation method used

## Prevention

To avoid web interface issues in future installations:

1. **Always build web assets before installation**:
```bash
cd web
npm install
npm run build
cd ..
```

2. **Use the installation script**:
```bash
sudo bash scripts/install.sh
```
The install script will automatically detect and install prebuilt assets.

3. **Verify installation**:
```bash
sudo bash scripts/diagnose_web_issue.sh
```

4. **Keep backups**:
The install_web_assets.sh script automatically creates backups before overwriting files.

