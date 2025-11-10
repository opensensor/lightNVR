# Automated Screenshot & Video Capture

This directory contains scripts for automatically capturing screenshots and videos of the LightNVR web interface for documentation purposes using Playwright automation.

## Overview

The automation system consists of:

- **`capture-screenshots.js`** - Captures static screenshots of all major UI pages
- **`capture-demos.js`** - Records video demonstrations of key features
- **`update-documentation-media.sh`** - Orchestrates the entire capture process
- **GitHub Actions workflow** - Automatically updates screenshots on demand

## Prerequisites

```bash
# Install Playwright (from project root)
npm install --save-dev playwright

# Install browser binaries
npx playwright install chromium

# Optional: Install image optimization tools
sudo apt-get install optipng ffmpeg
```

## Quick Start

### Easiest Method: Use the Orchestration Script

```bash
# Capture everything (screenshots + videos) using Docker
./scripts/update-documentation-media.sh --docker

# Screenshots only
./scripts/update-documentation-media.sh --docker --screenshots-only

# All theme variations
./scripts/update-documentation-media.sh --docker --all-themes

# Against running instance
./scripts/update-documentation-media.sh --url http://localhost:8080
```

### Manual Screenshot Capture

```bash
# From project root
node scripts/capture-screenshots.js

# With custom URL
node scripts/capture-screenshots.js --url http://192.168.1.100:8080

# With custom credentials
node scripts/capture-screenshots.js --username myuser --password mypass

# Dark mode
node scripts/capture-screenshots.js --dark

# Specific theme
node scripts/capture-screenshots.js --theme purple --dark
```

### Capture All Themes

```bash
# Capture screenshots for all 7 themes (light and dark)
node scripts/capture-screenshots.js --all-themes
```

### Capture Demo Videos

```bash
# Capture all demos
node scripts/capture-demos.js

# Specific demo only
node scripts/capture-demos.js --demo zone-creation
node scripts/capture-demos.js --demo theme-switch
node scripts/capture-demos.js --demo detection
```

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `--url <url>` | LightNVR URL | `http://localhost:8080` |
| `--username <user>` | Login username | `admin` |
| `--password <pass>` | Login password | `admin` |
| `--output <dir>` | Output directory | `docs/images` |
| `--theme <theme>` | Theme to use | `blue` |
| `--dark` | Use dark mode | `false` |
| `--all-themes` | Capture all themes | `false` |

## Available Themes

- `default` - Neutral Gray
- `blue` - Ocean Blue
- `emerald` - Forest Green
- `purple` - Royal Purple
- `rose` - Sunset Rose
- `amber` - Golden Amber
- `slate` - Cool Slate

## Screenshots Captured

The script automatically captures:

1. **live-streams.png** - Live view with camera streams
2. **stream-management.png** - Stream configuration page
3. **recording-management.png** - Recordings page
4. **settings-management.png** - Settings page
5. **System.png** - System information page
6. **zone-editor.png** - Detection zone editor (if available)
7. **theme-selector.png** - Theme customization interface

## Advanced Usage

### Capture for Specific Documentation

```bash
# Capture with Ocean Blue theme in dark mode for README
node scripts/capture-screenshots.js --theme blue --dark

# Capture all themes for theme showcase
node scripts/capture-screenshots.js --all-themes --output docs/images/themes
```

### GitHub Actions Integration

The repository includes a GitHub Actions workflow for automated screenshot updates:

1. Go to **Actions** tab in GitHub
2. Select **Update Documentation Screenshots**
3. Click **Run workflow**
4. Choose options:
   - Capture all themes: Yes/No
   - Create PR: Yes/No
5. Click **Run workflow**

The workflow will:
- Build and start LightNVR in Docker
- Capture screenshots
- Optimize images with optipng
- Create a PR with the changes (if enabled)

### Manual CI/CD Integration

```bash
# In GitHub Actions or other CI
docker-compose up -d
sleep 20  # Wait for LightNVR to start
./scripts/update-documentation-media.sh --screenshots-only --skip-install
```

## Customizing the Script

The script is designed to be easily customizable. Edit `capture-screenshots.js` to:

- Add new screenshot captures
- Modify viewport size
- Adjust wait times
- Capture specific UI elements
- Add video recording capabilities

### Example: Add New Screenshot

```javascript
async function captureMyFeature(page) {
  console.log('\n=== Capturing My Feature ===');
  await page.goto(`${config.url}/my-page.html`);
  await sleep(1500);
  
  await captureScreenshot(page, 'my-feature', { delay: 1500 });
}

// Add to main() function
await captureMyFeature(page);
```

## Video Recording (Future Enhancement)

To add video recording capabilities:

```javascript
// Start recording
await page.video();

// Perform actions
await page.click('button');
await sleep(2000);

// Video is automatically saved when browser closes
```

## Troubleshooting

### Screenshots are blank
- Ensure LightNVR is running and accessible
- Check that credentials are correct
- Increase wait times in the script

### Login fails
- Verify URL is correct
- Check username/password
- Ensure web interface is accessible

### Missing elements
- Some features may not be available in your setup
- The script will skip unavailable features
- Check console output for details

### Playwright installation issues
```bash
# Install system dependencies (Linux)
npx playwright install-deps chromium

# Or use Docker
docker run -v $(pwd):/work -w /work mcr.microsoft.com/playwright:latest node scripts/capture-screenshots.js
```

## Best Practices

1. **Run on a clean instance** - Use a fresh LightNVR installation with sample data
2. **Use realistic data** - Configure streams with meaningful names
3. **Consistent timing** - Adjust sleep times if your system is slower
4. **Check output** - Review screenshots before committing
5. **Version control** - Commit screenshots with descriptive messages

## Example Workflow

```bash
# 1. Start LightNVR with sample configuration
docker-compose up -d

# 2. Wait for startup
sleep 15

# 3. Configure sample streams (manual or via API)
# ...

# 4. Capture screenshots
node scripts/capture-screenshots.js --theme blue

# 5. Review output
ls -lh docs/images/

# 6. Commit if satisfied
git add docs/images/
git commit -m "docs: update screenshots with latest UI"
```

## Contributing

When updating the screenshot script:

1. Test with multiple themes
2. Ensure screenshots are high quality (1920x1080)
3. Keep file sizes reasonable (<500KB per PNG)
4. Document any new options or features
5. Update this README

## Future Enhancements

Planned improvements:

- [ ] Video recording for feature demos
- [ ] GIF generation for animations
- [ ] Comparison screenshots (before/after)
- [ ] Mobile viewport screenshots
- [ ] Automated upload to documentation site
- [ ] Screenshot diffing for visual regression testing
- [ ] Integration with visual testing tools (Percy, Chromatic)

