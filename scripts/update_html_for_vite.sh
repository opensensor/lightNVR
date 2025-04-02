#!/bin/bash
# Script to update HTML files to use Vite's output structure

# Set directories
WEB_DIR="$(dirname "$(dirname "$0")")/web"
DIST_DIR="$WEB_DIR/dist"
JS_DIR="$WEB_DIR/js"

# Check if the dist directory exists
if [ ! -d "$DIST_DIR" ]; then
    echo "Dist directory not found: $DIST_DIR"
    exit 1
fi

# Create directories for non-module scripts
mkdir -p "$DIST_DIR/js"
mkdir -p "$DIST_DIR/js/components"
mkdir -p "$DIST_DIR/img"
mkdir -p "$DIST_DIR/fonts"

# Copy non-module scripts
echo "Copying non-module scripts..."
cp "$JS_DIR/websocket-client.js" "$DIST_DIR/js/"
cp "$JS_DIR/hls.min.js" "$DIST_DIR/js/" 2>/dev/null || true
cp "$JS_DIR/components/batch-delete-modal.js" "$DIST_DIR/js/components/" 2>/dev/null || true

# Copy images and fonts if they exist
if [ -d "$WEB_DIR/img" ]; then
    cp -r "$WEB_DIR/img/"* "$DIST_DIR/img/" 2>/dev/null || true
fi

if [ -d "$WEB_DIR/fonts" ]; then
    cp -r "$WEB_DIR/fonts/"* "$DIST_DIR/fonts/" 2>/dev/null || true
fi

# Update HTML files to use the correct paths
echo "Updating HTML files to use Vite's output structure..."

# Find all HTML files in the dist directory
HTML_FILES=$(find "$DIST_DIR" -name "*.html")

# Loop through each HTML file
for file in $HTML_FILES; do
    echo "Updating $file..."
    
    # Update script tags to use the correct paths
    # Replace dist/js/ references with ./js/
    sed -i 's|src="dist/js/|src="./js/|g' "$file"
    
    # Replace dist/css/ references with ./assets/
    sed -i 's|href="dist/css/|href="./assets/|g' "$file"
    
    # Replace dist/img/ references with ./img/
    sed -i 's|src="dist/img/|src="./img/|g' "$file"
    sed -i 's|href="dist/img/|href="./img/|g' "$file"
    
    # Replace dist/fonts/ references with ./fonts/
    sed -i 's|src="dist/fonts/|src="./fonts/|g' "$file"
    sed -i 's|href="dist/fonts/|href="./fonts/|g' "$file"
    
    echo "Updated $file"
done

echo "HTML files updated successfully."
