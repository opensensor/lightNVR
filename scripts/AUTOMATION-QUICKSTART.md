# Documentation Media Automation - Quick Start

## TL;DR

```bash
# One command to update all documentation media
./scripts/update-documentation-media.sh --docker
```

## What Gets Captured

### Screenshots (PNG)
- `live-streams.png` - Multi-stream live view
- `stream-management.png` - Stream configuration page
- `recording-management.png` - Recordings browser
- `settings-management.png` - Settings interface
- `System.png` - System information
- `zone-editor.png` - Detection zone editor
- `theme-selector.png` - Theme customization

### Videos (WebM/MP4/GIF)
- Zone creation demonstration
- Theme switching demonstration
- Live detection demonstration
- Stream configuration walkthrough

## Common Use Cases

### First Time Setup with Demo Camera

```bash
# 1. Configure your camera in scripts/demo-cameras.json
# 2. Start LightNVR
docker-compose up -d

# 3. Setup demo streams
node scripts/setup-demo-streams.js

# 4. Capture screenshots
./scripts/update-documentation-media.sh --screenshots-only
```

### Update Screenshots for README

```bash
# 1. Start LightNVR with Docker (auto-configures demo streams)
./scripts/update-documentation-media.sh --docker --screenshots-only

# 2. Review screenshots
ls -lh docs/images/

# 3. Commit if satisfied
git add docs/images/
git commit -m "docs: update screenshots"
```

### Capture All Theme Variations

```bash
# Captures 14 screenshots (7 themes × 2 modes)
./scripts/update-documentation-media.sh --docker --all-themes --screenshots-only
```

### Create Feature Demo Videos

```bash
# Capture all demo videos
./scripts/update-documentation-media.sh --docker --videos-only

# Videos are saved as WebM, MP4, and GIF
ls -lh docs/videos/
```

### Update Against Running Instance

```bash
# If LightNVR is already running
./scripts/update-documentation-media.sh --url http://localhost:8080
```

### GitHub Actions (No Local Setup)

1. Go to **Actions** tab
2. Select **Update Documentation Screenshots**
3. Click **Run workflow**
4. Review the PR created automatically

## File Locations

```
lightNVR/
├── docs/
│   ├── images/          # Screenshots (PNG)
│   └── videos/          # Demo videos (WebM/MP4/GIF)
├── scripts/
│   ├── capture-screenshots.js      # Screenshot automation
│   ├── capture-demos.js            # Video automation
│   ├── update-documentation-media.sh  # Orchestration script
│   └── README-screenshots.md       # Full documentation
└── .github/
    └── workflows/
        └── update-screenshots.yml  # GitHub Actions workflow
```

## Requirements

### Local Development
```bash
npm install --save-dev playwright
npx playwright install chromium
```

### Optional (for optimization)
```bash
sudo apt-get install optipng ffmpeg
```

### GitHub Actions
No setup required - everything is automated!

## Customization

### Change Default Theme
Edit `scripts/capture-screenshots.js`:
```javascript
const config = {
  theme: 'purple',  // Change from 'blue' to any theme
  darkMode: true,   // Change to true for dark mode
};
```

### Add New Screenshot
Edit `scripts/capture-screenshots.js`:
```javascript
async function captureMyFeature(page) {
  await page.goto(`${config.url}/my-page.html`);
  await sleep(1500);
  await captureScreenshot(page, 'my-feature', { delay: 1500 });
}

// Add to main()
await captureMyFeature(page);
```

### Add New Demo Video
Edit `scripts/capture-demos.js`:
```javascript
async function demoMyFeature(page) {
  await page.goto(`${config.url}/my-page.html`);
  // Perform actions...
  await sleep(2000);
}

// Add to main()
if (config.demo === 'all' || config.demo === 'my-feature') {
  await demoMyFeature(page);
}
```

## Troubleshooting

### "Cannot access LightNVR"
```bash
# Check if LightNVR is running
curl http://localhost:8080/login.html

# Start with Docker
docker-compose up -d
```

### "Playwright not found"
```bash
npm install --save-dev playwright
npx playwright install chromium
```

### Screenshots are blank
- Increase wait times in the scripts
- Check browser console for errors
- Run with `headless: false` to see what's happening

### Videos are too large
```bash
# Reduce quality in ffmpeg conversion
ffmpeg -i input.webm -crf 28 output.mp4  # Higher CRF = smaller file
```

## Best Practices

1. **Run on clean instance** - Fresh install with sample data
2. **Use realistic data** - Meaningful stream names, not "test123"
3. **Check file sizes** - Keep PNGs under 500KB, videos under 10MB
4. **Review before commit** - Always check screenshots look good
5. **Use automation** - Don't manually capture screenshots

## Next Steps

After capturing media:

1. **Review** - Check all screenshots/videos look professional
2. **Update README** - Reference new media in documentation
3. **Commit** - `git add docs/ && git commit -m "docs: update media"`
4. **Push** - `git push origin main`

## Support

- Full documentation: [scripts/README-screenshots.md](README-screenshots.md)
- Issues: Open a GitHub issue with `documentation` label
- Questions: Check existing issues or create new one

