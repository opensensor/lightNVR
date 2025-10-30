#!/bin/bash
# Script to remove web/dist from git tracking
# This is a one-time migration script for the new build process

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Function to check if git working directory is clean
check_git_clean() {
    if [ -n "$(git status --porcelain | grep -v '^?? web/dist/')" ]; then
        print_error "Git working directory is not clean (excluding web/dist/)"
        print_error "Please commit or stash your changes before running this migration"
        git status --short | grep -v '^?? web/dist/'
        exit 1
    fi
}

# Function to check if web/dist is tracked
check_web_dist_tracked() {
    if git ls-files web/dist/ | grep -q .; then
        return 0  # web/dist is tracked
    else
        return 1  # web/dist is not tracked
    fi
}

# Main script
main() {
    print_info "LightNVR Web Assets Migration"
    print_info "=============================="
    echo
    
    print_info "This script will remove web/dist/ from git tracking."
    print_info "Web assets will now be built during CI/CD instead of being checked in."
    echo
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Check if web/dist is tracked
    if ! check_web_dist_tracked; then
        print_info "✓ web/dist/ is already not tracked in git"
        print_info "No migration needed!"
        exit 0
    fi
    
    # Count files to be removed
    local file_count=$(git ls-files web/dist/ | wc -l)
    print_warning "Found $file_count files in web/dist/ tracked by git"
    echo
    
    # Show some examples
    print_info "Examples of files to be removed from git:"
    git ls-files web/dist/ | head -10
    if [ $file_count -gt 10 ]; then
        echo "... and $(($file_count - 10)) more files"
    fi
    echo
    
    # Pre-flight checks
    print_step "Running pre-flight checks..."
    check_git_clean
    print_info "✓ Git working directory is clean"
    echo
    
    # Confirm with user
    print_warning "This will:"
    print_warning "  1. Remove web/dist/ from git tracking (but keep files on disk)"
    print_warning "  2. Create a commit with the removal"
    print_warning "  3. The .gitignore already excludes web/dist/"
    echo
    read -p "Do you want to proceed? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Migration cancelled"
        exit 0
    fi
    echo
    
    # Step 1: Remove from git tracking
    print_step "Step 1/2: Removing web/dist/ from git tracking..."
    git rm -r --cached web/dist/
    print_info "✓ Removed from git tracking (files still on disk)"
    echo
    
    # Step 2: Commit the change
    print_step "Step 2/2: Committing the change..."
    git commit -m "Remove web/dist from git tracking

Web assets are now built during CI/CD instead of being checked in.
This reduces repository size and prevents inconsistencies.

See docs/RELEASE_PROCESS.md for the new build process."
    print_info "✓ Changes committed"
    echo
    
    print_info "Migration complete!"
    echo
    print_info "Summary:"
    print_info "  ✓ Removed $file_count files from git tracking"
    print_info "  ✓ Files still exist on disk in web/dist/"
    print_info "  ✓ Changes committed locally"
    echo
    print_info "Next steps:"
    print_info "  1. Review the commit: git show"
    print_info "  2. Push the changes: git push"
    print_info "  3. Other developers should pull and may see web/dist/ as untracked"
    print_info "     (This is normal - they can delete it or leave it for local builds)"
    echo
    print_info "For more information, see:"
    print_info "  - docs/RELEASE_PROCESS.md"
    print_info "  - RELEASE_CHECKLIST.md"
}

main "$@"

