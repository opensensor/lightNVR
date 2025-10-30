#!/bin/bash
# Script to automate the release process
# Usage: ./scripts/release.sh <new_version> [--no-push]
# Example: ./scripts/release.sh 0.13.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Default options
PUSH_CHANGES=true

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
    if [ -n "$(git status --porcelain)" ]; then
        print_error "Git working directory is not clean"
        print_error "Please commit or stash your changes before creating a release"
        git status --short
        exit 1
    fi
}

# Function to check if on main branch
check_main_branch() {
    local current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$current_branch" != "main" ]; then
        print_warning "You are not on the main branch (current: $current_branch)"
        read -p "Do you want to continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_warning "Release cancelled"
            exit 0
        fi
    fi
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

# Function to check if tag already exists
check_tag_exists() {
    local version=$1
    if git rev-parse "v$version" >/dev/null 2>&1; then
        print_error "Tag v$version already exists"
        print_error "Please use a different version number or delete the existing tag"
        exit 1
    fi
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --no-push)
                PUSH_CHANGES=false
                shift
                ;;
            --help)
                echo "Usage: $0 <new_version> [--no-push]"
                echo ""
                echo "Arguments:"
                echo "  new_version    Version number in format MAJOR.MINOR.PATCH (e.g., 0.13.0)"
                echo ""
                echo "Options:"
                echo "  --no-push      Don't push changes to remote (useful for testing)"
                echo "  --help         Show this help message"
                exit 0
                ;;
            *)
                if [ -z "$NEW_VERSION" ]; then
                    NEW_VERSION=$1
                else
                    print_error "Unknown argument: $1"
                    exit 1
                fi
                shift
                ;;
        esac
    done
}

# Main script
main() {
    print_info "LightNVR Release Automation"
    print_info "==========================="
    echo
    
    # Parse arguments
    parse_args "$@"
    
    # Check if version argument is provided
    if [ -z "$NEW_VERSION" ]; then
        print_error "No version specified"
        echo "Usage: $0 <new_version> [--no-push]"
        echo "Example: $0 0.13.0"
        exit 1
    fi
    
    # Validate version format
    validate_version "$NEW_VERSION"
    
    # Pre-flight checks
    print_step "Running pre-flight checks..."
    check_git_clean
    check_main_branch
    check_tag_exists "$NEW_VERSION"
    print_info "✓ Pre-flight checks passed"
    echo
    
    # Show release plan
    print_info "Release Plan:"
    print_info "  Version: $NEW_VERSION"
    print_info "  Tag: v$NEW_VERSION"
    print_info "  Push to remote: $([ "$PUSH_CHANGES" = true ] && echo "Yes" || echo "No")"
    echo
    
    # Confirm with user
    read -p "Do you want to proceed with the release? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Release cancelled"
        exit 0
    fi
    echo
    
    # Step 1: Bump version
    print_step "Step 1/4: Bumping version to $NEW_VERSION..."
    "$PROJECT_ROOT/scripts/bump-version.sh" "$NEW_VERSION" <<< "y"
    echo
    
    # Step 2: Commit changes
    print_step "Step 2/4: Committing version changes..."
    git add -A
    git commit -m "Bump version to $NEW_VERSION"
    print_info "✓ Changes committed"
    echo
    
    # Step 3: Create tag
    print_step "Step 3/4: Creating tag v$NEW_VERSION..."
    git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
    print_info "✓ Tag created"
    echo
    
    # Step 4: Push changes (if enabled)
    if [ "$PUSH_CHANGES" = true ]; then
        print_step "Step 4/4: Pushing changes to remote..."
        
        # Push commits
        print_info "Pushing commits..."
        git push
        
        # Push tags
        print_info "Pushing tags..."
        git push --tags
        
        print_info "✓ Changes pushed to remote"
        echo
        
        print_info "Release v$NEW_VERSION completed successfully!"
        print_info "GitHub Actions will now build and publish the Docker images"
        echo
        print_info "Monitor the build at:"
        print_info "  https://github.com/opensensor/lightNVR/actions"
    else
        print_step "Step 4/4: Skipping push (--no-push flag set)"
        echo
        
        print_info "Release v$NEW_VERSION prepared locally!"
        print_info "To push the release, run:"
        print_info "  git push && git push --tags"
    fi
    
    echo
    print_info "Release Summary:"
    print_info "  ✓ Version bumped to $NEW_VERSION"
    print_info "  ✓ Changes committed"
    print_info "  ✓ Tag v$NEW_VERSION created"
    if [ "$PUSH_CHANGES" = true ]; then
        print_info "  ✓ Changes pushed to remote"
    else
        print_info "  - Changes NOT pushed (use --no-push flag)"
    fi
}

main "$@"

