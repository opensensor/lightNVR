#!/usr/bin/env node

/**
 * Script to extract version from CMakeLists.txt and generate a version.js file
 * This allows the web interface to use a static version number instead of fetching it from the API
 *
 * Usage:
 *   node extract_version.js [options]
 *
 * Options:
 *   --cmake-path <path>    Path to CMakeLists.txt (default: auto-detect)
 *   --output-path <path>   Path to output version.js file (default: web/js/version.js)
 *   --project-root <path>  Path to project root (default: auto-detect)
 *   --help                 Show this help message
 */

const fs = require('fs');
const path = require('path');

// Parse command line arguments
function parseArgs() {
  const args = {
    cmakePath: null,
    outputPath: null,
    projectRoot: null,
    help: false
  };

  for (let i = 2; i < process.argv.length; i++) {
    const arg = process.argv[i];

    if (arg === '--help') {
      args.help = true;
    } else if (arg === '--cmake-path' && i + 1 < process.argv.length) {
      args.cmakePath = process.argv[++i];
    } else if (arg === '--output-path' && i + 1 < process.argv.length) {
      args.outputPath = process.argv[++i];
    } else if (arg === '--project-root' && i + 1 < process.argv.length) {
      args.projectRoot = process.argv[++i];
    }
  }

  return args;
}

// Display help message
function showHelp() {
  console.log(`
Usage: node extract_version.js [options]

Options:
  --cmake-path <path>    Path to CMakeLists.txt (default: auto-detect)
  --output-path <path>   Path to output version.js file (default: web/js/version.js)
  --project-root <path>  Path to project root (default: auto-detect)
  --help                 Show this help message
`);
  process.exit(0);
}

// Find project root by looking for CMakeLists.txt
function findProjectRoot(startDir) {
  // Start with the script directory
  let currentDir = startDir;

  // Try to find CMakeLists.txt in current or parent directories
  while (currentDir !== path.parse(currentDir).root) {
    if (fs.existsSync(path.join(currentDir, 'CMakeLists.txt'))) {
      return currentDir;
    }
    currentDir = path.dirname(currentDir);
  }

  // If we couldn't find it, default to the starting directory
  return startDir;
}

// Main function
function main() {
  const args = parseArgs();

  if (args.help) {
    showHelp();
  }

  // Determine project root
  // 1. Use command line argument if provided
  // 2. Otherwise, try to auto-detect by finding CMakeLists.txt
  const scriptDir = path.dirname(fs.realpathSync(__filename));
  const projectRoot = args.projectRoot
    ? path.resolve(process.cwd(), args.projectRoot)
    : findProjectRoot(scriptDir);

  console.log(`Using project root: ${projectRoot}`);

  // Determine CMakeLists.txt path
  const cmakeListsPath = args.cmakePath
    ? path.resolve(process.cwd(), args.cmakePath)
    : path.join(projectRoot, 'CMakeLists.txt');

  // Determine output path
  const outputPath = args.outputPath
    ? path.resolve(process.cwd(), args.outputPath)
    : path.join(projectRoot, 'web', 'js', 'version.js');

  console.log(`Reading from: ${cmakeListsPath}`);
  console.log(`Writing to: ${outputPath}`);

  try {
    // Read CMakeLists.txt
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
}

// Run the main function
main();
