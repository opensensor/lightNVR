#!/bin/bash

##
# LightNVR Documentation Media Update Script
#
# This script automates the process of capturing screenshots and videos
# for the LightNVR documentation. It can be run locally or in CI/CD.
#
# Usage:
#   ./scripts/update-documentation-media.sh [options]
#
# Options:
#   --local              Run against local LightNVR instance
#   --docker             Start LightNVR in Docker first
#   --url <url>          Custom LightNVR URL
#   --screenshots-only   Only capture screenshots
#   --videos-only        Only capture videos
#   --all-themes         Capture all theme variations
#   --skip-install       Skip npm install step
##

set -e

# Default configuration
LIGHTNVR_URL="${LIGHTNVR_URL:-http://localhost:8080}"
USERNAME="${LIGHTNVR_USERNAME:-admin}"
PASSWORD="${LIGHTNVR_PASSWORD:-admin}"
SCREENSHOTS_ONLY=false
VIDEOS_ONLY=false
ALL_THEMES=false
SKIP_INSTALL=false
USE_DOCKER=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --local)
      LIGHTNVR_URL="http://localhost:8080"
      shift
      ;;
    --docker)
      USE_DOCKER=true
      shift
      ;;
    --url)
      LIGHTNVR_URL="$2"
      shift 2
      ;;
    --screenshots-only)
      SCREENSHOTS_ONLY=true
      shift
      ;;
    --videos-only)
      VIDEOS_ONLY=true
      shift
      ;;
    --all-themes)
      ALL_THEMES=true
      shift
      ;;
    --skip-install)
      SKIP_INSTALL=true
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}LightNVR Documentation Media Update${NC}"
echo "===================================="
echo ""
echo "Configuration:"
echo "  URL: $LIGHTNVR_URL"
echo "  Username: $USERNAME"
echo "  Screenshots: $([ "$VIDEOS_ONLY" = true ] && echo "No" || echo "Yes")"
echo "  Videos: $([ "$SCREENSHOTS_ONLY" = true ] && echo "No" || echo "Yes")"
echo "  All Themes: $ALL_THEMES"
echo ""

# Check if we're in the project root
if [ ! -f "CMakeLists.txt" ]; then
  echo -e "${RED}Error: Must be run from project root${NC}"
  exit 1
fi

# Install dependencies if needed
if [ "$SKIP_INSTALL" = false ]; then
  echo -e "${YELLOW}Installing dependencies...${NC}"
  if [ ! -d "node_modules/playwright" ]; then
    npm install --save-dev playwright
  fi

  # Always ensure Playwright browsers are installed (handles sudo case)
  echo -e "${YELLOW}Ensuring Playwright browsers are installed...${NC}"
  npx playwright install chromium

  echo -e "${GREEN}✓ Dependencies installed${NC}"
  echo ""
fi

# Start Docker if requested
if [ "$USE_DOCKER" = true ]; then
  echo -e "${YELLOW}Starting LightNVR in Docker...${NC}"
  docker-compose up -d
  
  echo "Waiting for LightNVR to start..."
  sleep 15
  
  # Wait for LightNVR to be ready
  MAX_RETRIES=30
  RETRY_COUNT=0
  while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s -o /dev/null -w "%{http_code}" "$LIGHTNVR_URL/login.html" | grep -q "200"; then
      echo -e "${GREEN}✓ LightNVR is ready${NC}"
      break
    fi
    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo "Waiting... ($RETRY_COUNT/$MAX_RETRIES)"
    sleep 2
  done
  
  if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo -e "${RED}Error: LightNVR did not start in time${NC}"
    exit 1
  fi
  echo ""
fi

# Check if LightNVR is accessible
echo -e "${YELLOW}Checking LightNVR accessibility...${NC}"
if ! curl -s -o /dev/null -w "%{http_code}" "$LIGHTNVR_URL/login.html" | grep -q "200"; then
  echo -e "${RED}Error: Cannot access LightNVR at $LIGHTNVR_URL${NC}"
  echo "Make sure LightNVR is running and accessible."
  exit 1
fi
echo -e "${GREEN}✓ LightNVR is accessible${NC}"
echo ""

# Setup demo streams
echo -e "${YELLOW}Setting up demo streams...${NC}"
if [ -f "scripts/setup-demo-streams.js" ]; then
  node scripts/setup-demo-streams.js --url "$LIGHTNVR_URL" --username "$USERNAME" --password "$PASSWORD" || {
    echo -e "${YELLOW}⚠ Could not setup demo streams (may not be accessible)${NC}"
    echo "  Continuing with existing streams..."
  }
else
  echo -e "${YELLOW}⚠ Demo stream setup script not found, skipping${NC}"
fi
echo ""

# Capture screenshots
if [ "$VIDEOS_ONLY" = false ]; then
  echo -e "${YELLOW}Capturing screenshots...${NC}"
  
  SCREENSHOT_ARGS="--url $LIGHTNVR_URL --username $USERNAME --password $PASSWORD"
  
  if [ "$ALL_THEMES" = true ]; then
    SCREENSHOT_ARGS="$SCREENSHOT_ARGS --all-themes"
  fi
  
  node scripts/capture-screenshots.js $SCREENSHOT_ARGS
  
  echo -e "${GREEN}✓ Screenshots captured${NC}"
  echo ""
fi

# Capture videos
if [ "$SCREENSHOTS_ONLY" = false ]; then
  echo -e "${YELLOW}Capturing demo videos...${NC}"
  
  VIDEO_ARGS="--url $LIGHTNVR_URL --username $USERNAME --password $PASSWORD"
  
  node scripts/capture-demos.js $VIDEO_ARGS
  
  echo -e "${GREEN}✓ Demo videos captured${NC}"
  echo ""
fi

# Optimize images
if [ "$VIDEOS_ONLY" = false ]; then
  echo -e "${YELLOW}Optimizing images...${NC}"
  
  if command -v optipng &> /dev/null; then
    find docs/images -name "*.png" -type f -exec optipng -o2 {} \;
    echo -e "${GREEN}✓ Images optimized with optipng${NC}"
  else
    echo -e "${YELLOW}⚠ optipng not found, skipping image optimization${NC}"
    echo "  Install with: sudo apt-get install optipng"
  fi
  echo ""
fi

# Convert videos to MP4 and GIF
if [ "$SCREENSHOTS_ONLY" = false ]; then
  echo -e "${YELLOW}Converting videos...${NC}"
  
  if command -v ffmpeg &> /dev/null; then
    for webm in docs/videos/*.webm; do
      if [ -f "$webm" ]; then
        base=$(basename "$webm" .webm)
        
        # Convert to MP4
        if [ ! -f "docs/videos/${base}.mp4" ]; then
          echo "Converting $base to MP4..."
          ffmpeg -i "$webm" -c:v libx264 -crf 23 -preset medium -c:a aac -b:a 128k "docs/videos/${base}.mp4" -y -loglevel error
        fi
        
        # Convert to GIF (smaller, for quick demos)
        if [ ! -f "docs/videos/${base}.gif" ]; then
          echo "Converting $base to GIF..."
          ffmpeg -i "$webm" -vf "fps=15,scale=1280:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" "docs/videos/${base}.gif" -y -loglevel error
        fi
      fi
    done
    echo -e "${GREEN}✓ Videos converted${NC}"
  else
    echo -e "${YELLOW}⚠ ffmpeg not found, skipping video conversion${NC}"
    echo "  Install with: sudo apt-get install ffmpeg"
  fi
  echo ""
fi

# Generate summary
echo -e "${GREEN}Summary${NC}"
echo "======="
echo ""

if [ "$VIDEOS_ONLY" = false ]; then
  SCREENSHOT_COUNT=$(find docs/images -name "*.png" -type f | wc -l)
  echo "Screenshots: $SCREENSHOT_COUNT files"
  du -sh docs/images
fi

if [ "$SCREENSHOTS_ONLY" = false ]; then
  VIDEO_COUNT=$(find docs/videos -name "*.mp4" -o -name "*.webm" -o -name "*.gif" 2>/dev/null | wc -l)
  echo "Videos: $VIDEO_COUNT files"
  if [ -d "docs/videos" ]; then
    du -sh docs/videos
  fi
fi

echo ""
echo -e "${GREEN}✓ Documentation media update complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Review the captured media in docs/images/ and docs/videos/"
echo "  2. Update README.md to reference new screenshots/videos"
echo "  3. Commit changes: git add docs/ && git commit -m 'docs: update screenshots and videos'"
echo ""

# Cleanup Docker if we started it
if [ "$USE_DOCKER" = true ]; then
  echo -e "${YELLOW}Stopping Docker containers...${NC}"
  docker-compose down
  echo -e "${GREEN}✓ Docker containers stopped${NC}"
fi

