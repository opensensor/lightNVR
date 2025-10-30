#!/bin/bash
# Script to bump version across all project files
# Usage: ./scripts/bump-version.sh <new_version>
# Example: ./scripts/bump-version.sh 0.13.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Function to print colored messages
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to validate version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format: $version"
        print_error "Version must be in format: MAJOR.MINOR.PATCH (e.g., 0.13.0)"
        exit 1
    fi
}

# Function to get current version from CMakeLists.txt
get_current_version() {
    grep -E "project\s*\(\s*LightNVR\s+VERSION\s+[0-9]+\.[0-9]+\.[0-9]+" "$PROJECT_ROOT/CMakeLists.txt" | \
        sed -E 's/.*VERSION\s+([0-9]+\.[0-9]+\.[0-9]+).*/\1/'
}

# Function to update CMakeLists.txt
update_cmake() {
    local new_version=$1
    local cmake_file="$PROJECT_ROOT/CMakeLists.txt"
    
    print_info "Updating CMakeLists.txt..."
    
    # Use sed to replace the version
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' -E "s/(project\s*\(\s*LightNVR\s+VERSION\s+)[0-9]+\.[0-9]+\.[0-9]+/\1$new_version/" "$cmake_file"
    else
        # Linux
        sed -i -E "s/(project\s*\(\s*LightNVR\s+VERSION\s+)[0-9]+\.[0-9]+\.[0-9]+/\1$new_version/" "$cmake_file"
    fi
    
    print_info "✓ Updated CMakeLists.txt to version $new_version"
}

# Function to update package.json
update_package_json() {
    local new_version=$1
    local package_file="$PROJECT_ROOT/web/package.json"
    
    if [ ! -f "$package_file" ]; then
        print_warning "package.json not found, skipping"
        return
    fi
    
    print_info "Updating web/package.json..."
    
    # Use sed to replace the version
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' -E "s/\"version\":\s*\"[0-9]+\.[0-9]+\.[0-9]+\"/\"version\": \"$new_version\"/" "$package_file"
    else
        # Linux
        sed -i -E "s/\"version\":\s*\"[0-9]+\.[0-9]+\.[0-9]+\"/\"version\": \"$new_version\"/" "$package_file"
    fi
    
    print_info "✓ Updated package.json to version $new_version"
}

# Function to regenerate version.h
regenerate_version_h() {
    print_info "Regenerating version.h..."
    
    # Extract version components
    local version=$1
    local major=$(echo "$version" | cut -d. -f1)
    local minor=$(echo "$version" | cut -d. -f2)
    local patch=$(echo "$version" | cut -d. -f3)
    
    # Get build date and git commit
    local build_date=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
    local git_commit=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    
    # Generate version.h from template
    local version_h="$PROJECT_ROOT/include/core/version.h"
    local version_h_in="$PROJECT_ROOT/include/core/version.h.in"
    
    if [ -f "$version_h_in" ]; then
        cat "$version_h_in" | \
            sed "s/@LightNVR_VERSION_MAJOR@/$major/g" | \
            sed "s/@LightNVR_VERSION_MINOR@/$minor/g" | \
            sed "s/@LightNVR_VERSION_PATCH@/$patch/g" | \
            sed "s/@LightNVR_VERSION@/$version/g" | \
            sed "s/@BUILD_DATE@/$build_date/g" | \
            sed "s/@GIT_COMMIT@/$git_commit/g" > "$version_h"
        
        print_info "✓ Regenerated version.h"
    else
        print_warning "version.h.in not found, skipping version.h generation"
    fi
}

# Function to regenerate version.js
regenerate_version_js() {
    print_info "Regenerating version.js..."
    
    local version=$1
    local version_js="$PROJECT_ROOT/web/js/version.js"
    
    # Create directory if it doesn't exist
    mkdir -p "$(dirname "$version_js")"
    
    # Generate version.js
    cat > "$version_js" << EOF
/**
 * LightNVR version information
 * This file is auto-generated during version bumping
 * DO NOT EDIT MANUALLY
 */

export const VERSION = '$version';
EOF
    
    print_info "✓ Regenerated version.js"
}

# Main script
main() {
    # Check for help flag first
    if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "LightNVR Version Bumper"
        echo "======================="
        echo ""
        echo "Usage: $0 <new_version>"
        echo ""
        echo "Arguments:"
        echo "  new_version    Version number in format MAJOR.MINOR.PATCH (e.g., 0.13.0)"
        echo ""
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo ""
        echo "Example:"
        echo "  $0 0.13.0"
        echo ""
        echo "This will update:"
        echo "  - CMakeLists.txt"
        echo "  - web/package.json"
        echo "  - include/core/version.h"
        echo "  - web/js/version.js"
        exit 0
    fi

    print_info "LightNVR Version Bumper"
    print_info "======================="
    echo

    # Check if version argument is provided
    if [ $# -eq 0 ]; then
        print_error "No version specified"
        echo "Usage: $0 <new_version>"
        echo "Example: $0 0.13.0"
        echo "Run '$0 --help' for more information"
        exit 1
    fi

    local new_version=$1
    
    # Validate version format
    validate_version "$new_version"
    
    # Get current version
    local current_version=$(get_current_version)
    
    if [ -z "$current_version" ]; then
        print_error "Could not determine current version from CMakeLists.txt"
        exit 1
    fi
    
    print_info "Current version: $current_version"
    print_info "New version: $new_version"
    echo
    
    # Confirm with user
    read -p "Do you want to proceed with version bump? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Version bump cancelled"
        exit 0
    fi
    
    # Update all files
    update_cmake "$new_version"
    update_package_json "$new_version"
    regenerate_version_h "$new_version"
    regenerate_version_js "$new_version"
    
    echo
    print_info "Version bump complete!"
    print_info "Files updated:"
    print_info "  - CMakeLists.txt"
    print_info "  - web/package.json"
    print_info "  - include/core/version.h"
    print_info "  - web/js/version.js"
    echo
    print_info "Next steps:"
    print_info "  1. Review the changes: git diff"
    print_info "  2. Commit the changes: git add -A && git commit -m 'Bump version to $new_version'"
    print_info "  3. Create a tag: git tag -a v$new_version -m 'Release v$new_version'"
    print_info "  4. Push changes: git push && git push --tags"
    echo
    print_info "Or use the release script: ./scripts/release.sh $new_version"
}

main "$@"

