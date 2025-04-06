#!/bin/bash

# Script to extract version from CMakeLists.txt and generate a version.js file
# This allows the web interface to use a static version number instead of fetching it from the API

# Default paths
CMAKE_PATH="CMakeLists.txt"
OUTPUT_PATH="web/js/version.js"
PROJECT_ROOT="$(dirname "$(dirname "$0")")"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --cmake-path)
            CMAKE_PATH="$2"
            shift 2
            ;;
        --output-path)
            OUTPUT_PATH="$2"
            shift 2
            ;;
        --project-root)
            PROJECT_ROOT="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --cmake-path <path>    Path to CMakeLists.txt (default: auto-detect)"
            echo "  --output-path <path>   Path to output version.js file (default: web/js/version.js)"
            echo "  --project-root <path>  Path to project root (default: auto-detect)"
            echo "  --help                 Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $key"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Resolve paths
if [[ ! "$CMAKE_PATH" = /* ]]; then
    CMAKE_PATH="$PROJECT_ROOT/$CMAKE_PATH"
fi

if [[ ! "$OUTPUT_PATH" = /* ]]; then
    OUTPUT_PATH="$PROJECT_ROOT/$OUTPUT_PATH"
fi

echo "Using project root: $PROJECT_ROOT"
echo "Reading from: $CMAKE_PATH"
echo "Writing to: $OUTPUT_PATH"

# Extract version using grep and sed
VERSION=$(grep -E "project\s*\(\s*LightNVR\s+VERSION\s+[0-9]+\.[0-9]+\.[0-9]+" "$CMAKE_PATH" | sed -E 's/.*VERSION\s+([0-9]+\.[0-9]+\.[0-9]+).*/\1/')

if [ -z "$VERSION" ]; then
    echo "Failed to extract version from CMakeLists.txt"
    exit 1
fi

echo "Extracted version: $VERSION"

# Create output directory if it doesn't exist
mkdir -p "$(dirname "$OUTPUT_PATH")"

# Generate version.js file
cat > "$OUTPUT_PATH" << EOF
/**
 * LightNVR version information
 * This file is auto-generated from CMakeLists.txt during the build process
 * DO NOT EDIT MANUALLY
 */

export const VERSION = '$VERSION';
EOF

echo "Generated version.js with version $VERSION"
