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

# Check if Chrome is installed
if ! command -v google-chrome &> /dev/null && ! command -v google-chrome-stable &> /dev/null; then
    echo "Warning: Chrome does not appear to be installed. Tests may fail."
else
    # Check Chrome version
    CHROME_VERSION=$(google-chrome --version | grep -oP '(?<=Chrome )[0-9]+')
    echo "Detected Chrome version: $CHROME_VERSION"
    
    # Check package.json for ChromeDriver version
    CHROMEDRIVER_VERSION=$(grep -oP '(?<="chromedriver": "\^)[0-9]+' package.json 2>/dev/null)
    echo "Configured ChromeDriver version: $CHROMEDRIVER_VERSION"
    
    # Compare versions
    if [ "$CHROME_VERSION" != "$CHROMEDRIVER_VERSION" ]; then
        echo "Warning: Chrome version ($CHROME_VERSION) does not match ChromeDriver version ($CHROMEDRIVER_VERSION)"
        echo "This may cause tests to fail. Consider updating the ChromeDriver version in package.json."
    else
        echo "Chrome and ChromeDriver versions match. Good to go!"
    fi
fi

# Create screenshots directory if it doesn't exist
mkdir -p tests/screenshots

# Check if the application server is running
echo "Checking if the application server is running..."
if curl -s http://localhost:8080 > /dev/null; then
    echo "Application server is running. Proceeding with tests."
else
    echo "Warning: Application server does not appear to be running at http://localhost:8080"
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
