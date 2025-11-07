#!/bin/bash
# Install web assets for LightNVR
# This script builds (if needed) and installs the web interface files

set -e

# Parse command line arguments
BUILD_SOURCEMAPS=false
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--with-maps)
            BUILD_SOURCEMAPS=true
            shift
            ;;
        *)
            # Unknown option, ignore for compatibility
            shift
            ;;
    esac
done

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== LightNVR Web Assets Installation ===${NC}\n"

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: This script must be run as root${NC}"
    echo "Please run: sudo $0"
    exit 1
fi

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${BLUE}Project root: $PROJECT_ROOT${NC}\n"

# Determine web root from config or use default
CONFIG_FILE="/etc/lightnvr/lightnvr.ini"
if [ -f "$CONFIG_FILE" ]; then
    WEB_ROOT=$(grep -E "^root\s*=" "$CONFIG_FILE" | sed 's/.*=\s*//' | tr -d ' ')
fi

if [ -z "$WEB_ROOT" ]; then
    WEB_ROOT="/var/lib/lightnvr/www"
    echo -e "${YELLOW}Using default web root: $WEB_ROOT${NC}"
else
    echo -e "${GREEN}Using web root from config: $WEB_ROOT${NC}"
fi
echo ""

# Check if web directory exists in source
if [ ! -d "$PROJECT_ROOT/web" ]; then
    echo -e "${RED}Error: Web directory not found at $PROJECT_ROOT/web${NC}"
    echo "Please run this script from the LightNVR source directory"
    exit 1
fi

# Check if dist directory exists (prebuilt assets)
if [ -d "$PROJECT_ROOT/web/dist" ] && [ -f "$PROJECT_ROOT/web/dist/index.html" ]; then
    echo -e "${GREEN}✓ Found prebuilt web assets in web/dist/${NC}"
    USE_DIST=1
else
    echo -e "${YELLOW}⚠ No prebuilt web assets found${NC}"
    USE_DIST=0
    
    # Check if we can build
    if ! command -v npm &> /dev/null; then
        echo -e "${RED}Error: npm is not installed and no prebuilt assets found${NC}"
        echo "Please install Node.js and npm, or provide prebuilt assets"
        exit 1
    fi
    
    echo -e "${BLUE}Building web assets...${NC}"
    cd "$PROJECT_ROOT/web"
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        echo -e "${BLUE}Installing npm dependencies...${NC}"
        npm install
    fi
    
    # Build assets
    if [ "$BUILD_SOURCEMAPS" = true ]; then
        echo -e "${BLUE}Running build (with source maps)...${NC}"
        BUILD_SOURCEMAPS=true npm run build
    else
        echo -e "${BLUE}Running build (without source maps)...${NC}"
        npm run build
    fi

    if [ ! -d "dist" ] || [ ! -f "dist/index.html" ]; then
        echo -e "${RED}Error: Build failed - dist directory not created${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Build completed successfully${NC}"
    USE_DIST=1
fi

# Create web root directory
echo -e "\n${BLUE}Creating web root directory...${NC}"
mkdir -p "$WEB_ROOT"

# Backup existing files if any
if [ -d "$WEB_ROOT" ] && [ "$(ls -A $WEB_ROOT)" ]; then
    BACKUP_DIR="$WEB_ROOT.backup.$(date +%Y%m%d_%H%M%S)"
    echo -e "${YELLOW}Backing up existing files to: $BACKUP_DIR${NC}"
    cp -r "$WEB_ROOT" "$BACKUP_DIR"
fi

# Install web assets
echo -e "${BLUE}Installing web assets...${NC}"
if [ $USE_DIST -eq 1 ]; then
    # Copy from dist directory
    echo -e "Copying from: $PROJECT_ROOT/web/dist/"
    echo -e "Copying to: $WEB_ROOT/"
    
    # Remove old files first
    rm -rf "$WEB_ROOT"/*
    
    # Copy new files
    cp -r "$PROJECT_ROOT/web/dist/"* "$WEB_ROOT/"
    
    echo -e "${GREEN}✓ Web assets installed from dist directory${NC}"
else
    # This shouldn't happen given our checks above, but just in case
    echo -e "${RED}Error: No valid source for web assets${NC}"
    exit 1
fi

# Set proper permissions
echo -e "\n${BLUE}Setting permissions...${NC}"
chown -R root:root "$WEB_ROOT"
chmod -R 755 "$WEB_ROOT"
find "$WEB_ROOT" -type f -exec chmod 644 {} \;

# Verify installation
echo -e "\n${BLUE}Verifying installation...${NC}"
CRITICAL_FILES=("index.html" "login.html" "streams.html" "recordings.html")
ALL_PRESENT=1

for file in "${CRITICAL_FILES[@]}"; do
    if [ -f "$WEB_ROOT/$file" ]; then
        SIZE=$(stat -c%s "$WEB_ROOT/$file")
        echo -e "${GREEN}✓ $file${NC} (${SIZE} bytes)"
    else
        echo -e "${RED}✗ $file - MISSING${NC}"
        ALL_PRESENT=0
    fi
done

# Check for assets directory
if [ -d "$WEB_ROOT/assets" ]; then
    ASSET_COUNT=$(find "$WEB_ROOT/assets" -type f | wc -l)
    echo -e "${GREEN}✓ Assets directory${NC} ($ASSET_COUNT files)"
else
    echo -e "${RED}✗ Assets directory - MISSING${NC}"
    ALL_PRESENT=0
fi

# Final status
echo ""
if [ $ALL_PRESENT -eq 1 ]; then
    echo -e "${GREEN}=== Installation Successful! ===${NC}\n"
    echo -e "Web assets have been installed to: $WEB_ROOT"
    echo ""
    echo -e "Next steps:"
    echo -e "  1. Restart LightNVR service:"
    echo -e "     ${BLUE}sudo systemctl restart lightnvr${NC}"
    echo -e "  2. Check service status:"
    echo -e "     ${BLUE}sudo systemctl status lightnvr${NC}"
    echo -e "  3. Access web interface:"
    echo -e "     ${BLUE}http://your-server-ip:8080${NC}"
    echo -e "  4. Default credentials:"
    echo -e "     Username: ${BLUE}admin${NC}"
    echo -e "     Password: ${BLUE}admin${NC}"
else
    echo -e "${RED}=== Installation Incomplete ===${NC}\n"
    echo -e "Some files are missing. Please check the errors above."
    exit 1
fi

