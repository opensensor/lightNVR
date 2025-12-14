#!/usr/bin/env node

/**
 * LightNVR Demo Stream Setup
 * 
 * This script configures demo camera streams in LightNVR for documentation
 * screenshot and video capture.
 * 
 * Usage:
 *   node scripts/setup-demo-streams.js [options]
 * 
 * Options:
 *   --url <url>          LightNVR URL (default: http://localhost:8080)
 *   --username <user>    Username (default: admin)
 *   --password <pass>    Password (default: admin)
 */

const http = require('http');
const https = require('https');
const fs = require('fs');
const path = require('path');

// Parse command line arguments
const args = process.argv.slice(2);
const config = {
  url: 'http://localhost:8080',
  username: 'admin',
  password: 'admin',
  configFile: path.join(__dirname, 'demo-cameras.json'),
};

for (let i = 0; i < args.length; i++) {
  switch (args[i]) {
    case '--url':
      config.url = args[++i];
      break;
    case '--username':
      config.username = args[++i];
      break;
    case '--password':
      config.password = args[++i];
      break;
    case '--config':
      config.configFile = args[++i];
      break;
  }
}

// Load demo streams from configuration file
let DEMO_STREAMS = [];
try {
  const configData = fs.readFileSync(config.configFile, 'utf8');
  const parsed = JSON.parse(configData);
  DEMO_STREAMS = parsed.cameras || [];
  console.log(`Loaded ${DEMO_STREAMS.length} camera(s) from ${config.configFile}`);
} catch (error) {
  console.error(`Warning: Could not load config file ${config.configFile}: ${error.message}`);
  console.log('Using default configuration...');

  // Fallback to default configuration
  DEMO_STREAMS = [
    {
      name: 'Front Yard',
      url: 'rtsp://thingino:thingino@192.168.50.136:554/ch0',
      description: 'Thingino camera - front yard view',
      enabled: true,
      detection_enabled: true,
      detection_url: 'http://localhost:9001/api/v1/detect',
      detection_backend: 'onnx',
      detection_confidence: 0.5,
      detection_classes: 'person,car,dog,cat',
      recording_mode: 'detection',
    },
  ];
}

async function makeRequest(method, path, data = null, token = null) {
  return new Promise((resolve, reject) => {
    const url = new URL(path, config.url);
    const isHttps = url.protocol === 'https:';
    const lib = isHttps ? https : http;

    const options = {
      hostname: url.hostname,
      port: url.port || (isHttps ? 443 : 80),
      path: url.pathname + url.search,
      method: method,
      headers: {
        'Content-Type': 'application/json',
      },
      timeout: 30000, // 30 second timeout
    };

    // LightNVR uses session cookies, not Bearer tokens
    if (token) {
      options.headers['Cookie'] = `session=${token}`;
    }

    if (data) {
      const body = JSON.stringify(data);
      options.headers['Content-Length'] = Buffer.byteLength(body);
    }

    const req = lib.request(options, (res) => {
      let responseData = '';

      res.on('data', (chunk) => {
        responseData += chunk;
      });

      res.on('end', () => {
        try {
          const parsed = responseData ? JSON.parse(responseData) : {};
          resolve({ status: res.statusCode, data: parsed, headers: res.headers });
        } catch (e) {
          resolve({ status: res.statusCode, data: responseData, headers: res.headers });
        }
      });
    });

    req.on('timeout', () => {
      req.destroy();
      reject(new Error('Request timeout'));
    });

    req.on('error', (e) => {
      reject(e);
    });

    if (data) {
      req.write(JSON.stringify(data));
    }

    req.end();
  });
}

async function login(username, password) {
  console.log('Logging in...');
  
  try {
    const response = await makeRequest('POST', '/api/auth/login', {
      username: username,
      password: password,
    });
    
    if (response.status === 200 && response.data.token) {
      console.log('✓ Login successful');
      return response.data.token;
    } else {
      throw new Error(`Login failed: ${response.status} - ${JSON.stringify(response.data)}`);
    }
  } catch (error) {
    throw new Error(`Login error: ${error.message}`);
  }
}

async function getStreams(token) {
  try {
    const response = await makeRequest('GET', '/api/streams', null, token);
    
    if (response.status === 200) {
      return response.data.streams || response.data || [];
    } else {
      throw new Error(`Failed to get streams: ${response.status}`);
    }
  } catch (error) {
    throw new Error(`Get streams error: ${error.message}`);
  }
}

async function createStream(token, streamConfig) {
  console.log(`Creating stream: ${streamConfig.name}`);
  
  try {
    const response = await makeRequest('POST', '/api/streams', streamConfig, token);
    
    if (response.status === 200 || response.status === 201) {
      console.log(`✓ Created stream: ${streamConfig.name}`);
      return response.data;
    } else {
      throw new Error(`Failed to create stream: ${response.status} - ${JSON.stringify(response.data)}`);
    }
  } catch (error) {
    console.error(`✗ Error creating stream ${streamConfig.name}: ${error.message}`);
    return null;
  }
}

async function updateStream(token, streamName, streamConfig) {
  console.log(`Updating stream: ${streamName}`);
  
  try {
    const response = await makeRequest('PUT', `/api/streams/${encodeURIComponent(streamName)}`, streamConfig, token);
    
    if (response.status === 200) {
      console.log(`✓ Updated stream: ${streamName}`);
      return response.data;
    } else {
      throw new Error(`Failed to update stream: ${response.status} - ${JSON.stringify(response.data)}`);
    }
  } catch (error) {
    console.error(`✗ Error updating stream ${streamName}: ${error.message}`);
    return null;
  }
}

async function setupDemoZones(token, streamName, zones) {
  if (!zones || zones.length === 0) {
    console.log(`  No zones configured for ${streamName}`);
    return;
  }

  console.log(`Setting up ${zones.length} zone(s) for: ${streamName}`);

  try {
    for (const zone of zones) {
      const response = await makeRequest(
        'POST',
        `/api/streams/${encodeURIComponent(streamName)}/zones`,
        zone,
        token
      );

      if (response.status === 200 || response.status === 201) {
        console.log(`  ✓ Created zone: ${zone.name}`);
      } else {
        console.log(`  ⚠ Could not create zone ${zone.name}: ${response.status}`);
      }
    }
  } catch (error) {
    console.error(`  ✗ Error creating zones: ${error.message}`);
  }
}

async function main() {
  console.log('LightNVR Demo Stream Setup');
  console.log('==========================\n');
  console.log('Configuration:');
  console.log(`  URL: ${config.url}`);
  console.log(`  Username: ${config.username}`);
  console.log(`  Streams to configure: ${DEMO_STREAMS.length}\n`);
  
  try {
    // Login
    const token = await login(config.username, config.password);
    
    // Get existing streams
    console.log('\nChecking existing streams...');
    const existingStreams = await getStreams(token);
    const existingNames = existingStreams.map(s => s.name);
    console.log(`Found ${existingStreams.length} existing streams\n`);
    
    // Create or update demo streams
    console.log('Setting up demo streams...');
    for (const streamConfig of DEMO_STREAMS) {
      // Extract zones from config (not part of stream API)
      const zones = streamConfig.zones || [];
      const streamData = { ...streamConfig };
      delete streamData.zones;

      if (existingNames.includes(streamConfig.name)) {
        console.log(`Stream "${streamConfig.name}" already exists, updating...`);
        await updateStream(token, streamConfig.name, streamData);
      } else {
        await createStream(token, streamData);
      }

      // Set up demo zones
      await setupDemoZones(token, streamConfig.name, zones);
    }
    
    console.log('\n✓ Demo stream setup complete!');
    console.log('\nNext steps:');
    console.log('  1. Verify streams are working in LightNVR UI');
    console.log('  2. Adjust detection zones if needed');
    console.log('  3. Run screenshot capture: ./scripts/update-documentation-media.sh');
    
  } catch (error) {
    console.error('\n✗ Error:', error.message);
    process.exit(1);
  }
}

main();

