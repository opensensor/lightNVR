#!/bin/bash

# Script to install dependencies and run tests for LightNVR

# Change to the web directory
cd "$(dirname "$0")/.."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Please install Node.js before running this script."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "npm is not installed. Please install npm before running this script."
    exit 1
fi

# Install dependencies
echo "Installing dependencies..."
npm install

# Check if Chrome is installed. The driver itself needs no version check:
# Selenium Manager downloads one matching whatever browser is installed.
if ! command -v google-chrome &> /dev/null && ! command -v google-chrome-stable &> /dev/null; then
    echo "Warning: Chrome does not appear to be installed. Tests may fail."
else
    CHROME_VERSION=$(google-chrome --version 2>/dev/null | grep -oP '(?<=Chrome )[0-9.]+')
    echo "Detected Chrome version: $CHROME_VERSION"
    echo "Selenium Manager will fetch a matching driver automatically."
fi

# Create screenshots directory if it doesn't exist.
# Specs pass paths like 'screenshots/foo.png', resolved from jest's cwd (web/),
# so this must be web/screenshots -- which is also what .gitignore covers.
mkdir -p screenshots

# Check if the application server is running
BASE_URL="${E2E_BASE_URL:-http://localhost:8080}"
echo "Checking if the application server is running at $BASE_URL..."
if curl -s "$BASE_URL" > /dev/null; then
    echo "Application server is running. Proceeding with tests."
else
    echo "Warning: Application server does not appear to be running at $BASE_URL"
    echo "Tests that require server connectivity may fail."
    echo "Please start the application server in another terminal with 'npm start' before running tests."
    
    # Ask user if they want to continue
    read -p "Do you want to continue with the tests anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Tests aborted. Please start the application server and try again."
        exit 1
    fi
fi

# Run tests
echo "Running tests..."
npm run test:e2e

echo "Tests completed."
