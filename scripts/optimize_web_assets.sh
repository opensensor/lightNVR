#!/bin/bash
# Script to optimize web assets (minify JS and CSS)

# Check if required tools are installed
if ! command -v npm &> /dev/null; then
    echo "npm is not installed. Please install Node.js and npm first."
    exit 1
fi

# Install required packages if not already installed
echo "Installing required packages..."
npm install -g terser clean-css-cli

# Set directories
WEB_DIR="$(dirname "$(dirname "$0")")/web"
JS_DIR="$WEB_DIR/js"
CSS_DIR="$WEB_DIR/css"
DIST_DIR="$WEB_DIR/dist"

# Create dist directory if it doesn't exist
mkdir -p "$DIST_DIR/js"
mkdir -p "$DIST_DIR/css"

# Function to minify JavaScript files
minify_js() {
    echo "Minifying JavaScript files..."
    
    # Process each JS file
    find "$JS_DIR" -name "*.js" -type f | while read -r file; do
        # Get relative path
        rel_path="${file#$JS_DIR/}"
        dir_path=$(dirname "$rel_path")
        
        # Create directory structure in dist if needed
        mkdir -p "$DIST_DIR/js/$dir_path"
        
        # Skip files that are already minified
        if [[ "$file" == *".min.js" ]]; then
            echo "Copying already minified file: $rel_path"
            cp "$file" "$DIST_DIR/js/$rel_path"
        else
            echo "Minifying: $rel_path"
            # Create minified version
            terser "$file" --compress --mangle --output "$DIST_DIR/js/$rel_path"
            
            # Create source map if needed
            # terser "$file" --compress --mangle --source-map "url='$rel_path.map'" --output "$DIST_DIR/js/$rel_path"
        fi
    done
}

# Function to minify CSS files
minify_css() {
    echo "Minifying CSS files..."
    
    # Process each CSS file
    find "$CSS_DIR" -name "*.css" -type f | while read -r file; do
        # Get relative path
        rel_path="${file#$CSS_DIR/}"
        dir_path=$(dirname "$rel_path")
        
        # Create directory structure in dist if needed
        mkdir -p "$DIST_DIR/css/$dir_path"
        
        echo "Minifying: $rel_path"
        # Create minified version
        cleancss -o "$DIST_DIR/css/$rel_path" "$file"
    done
}

# Function to copy other assets
copy_assets() {
    echo "Copying other assets..."
    
    # Copy images
    if [ -d "$WEB_DIR/img" ]; then
        mkdir -p "$DIST_DIR/img"
        cp -r "$WEB_DIR/img"/* "$DIST_DIR/img/"
    fi
    
    # Copy fonts
    if [ -d "$WEB_DIR/fonts" ]; then
        mkdir -p "$DIST_DIR/fonts"
        cp -r "$WEB_DIR/fonts"/* "$DIST_DIR/fonts/"
    fi
    
    # Copy HTML files
    find "$WEB_DIR" -name "*.html" -type f -exec cp {} "$DIST_DIR/" \;
}

# Function to update HTML files to use minified assets
update_html_files() {
    echo "Updating HTML files to use minified assets..."
    
    # Process each HTML file
    find "$DIST_DIR" -name "*.html" -type f | while read -r file; do
        # Update CSS references
        sed -i 's/href="css\//href="dist\/css\//g' "$file"
        
        # Update JS references
        sed -i 's/src="js\//src="dist\/js\//g' "$file"
    done
}

# Main execution
echo "Starting web asset optimization..."
minify_js
minify_css
copy_assets
# update_html_files  # Uncomment if you want to update HTML files

echo "Optimization complete. Minified files are in $DIST_DIR"
echo "To use minified assets, update your HTML files or server configuration."
