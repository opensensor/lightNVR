#!/usr/bin/env node

/**
 * Script to extract version from CMakeLists.txt and generate a version.js file
 * This allows the web interface to use a static version number instead of fetching it from the API
 */

const fs = require('fs');
const path = require('path');

// Paths
const rootDir = path.resolve(__dirname, '..');
const cmakeListsPath = path.join(rootDir, 'CMakeLists.txt');
const outputPath = path.join(rootDir, 'web', 'js', 'version.js');

// Read CMakeLists.txt
try {
  const cmakeContent = fs.readFileSync(cmakeListsPath, 'utf8');
  
  // Extract version using regex
  const versionMatch = cmakeContent.match(/project\s*\(\s*LightNVR\s+VERSION\s+([0-9]+\.[0-9]+\.[0-9]+)/i);
  
  if (!versionMatch || !versionMatch[1]) {
    console.error('Failed to extract version from CMakeLists.txt');
    process.exit(1);
  }
  
  const version = versionMatch[1];
  console.log(`Extracted version: ${version}`);
  
  // Generate version.js file
  const versionJsContent = `/**
 * LightNVR version information
 * This file is auto-generated from CMakeLists.txt during the build process
 * DO NOT EDIT MANUALLY
 */

export const VERSION = '${version}';
`;
  
  // Create directory if it doesn't exist
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }
  
  // Write the file
  fs.writeFileSync(outputPath, versionJsContent);
  console.log(`Generated version.js with version ${version}`);
  
} catch (error) {
  console.error('Error:', error.message);
  process.exit(1);
}
