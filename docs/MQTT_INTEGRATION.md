# MQTT Integration for Detection Event Streaming

LightNVR can publish detection events to an MQTT broker in real-time, enabling integration with home automation systems, custom alerting, and external processing pipelines.

## What is MQTT?

MQTT (Message Queuing Telemetry Transport) is a lightweight publish/subscribe messaging protocol commonly used in IoT and home automation. When LightNVR detects an object (person, car, etc.), it publishes a message to an MQTT broker, and any subscribed clients receive that message instantly.

**Common use cases:**
- Home Assistant automations (turn on lights when person detected)
- Custom alerting systems (send notifications via Telegram, email, etc.)
- Data logging and analytics
- Integration with Node-RED for complex workflows

## Quick Start

### 1. Set Up an MQTT Broker

The easiest way to get started is with the Mosquitto broker.

**On Ubuntu/Debian:**
```bash
# Install Mosquitto broker and client tools
sudo apt-get update
sudo apt-get install mosquitto mosquitto-clients

# Start the broker
sudo systemctl start mosquitto
sudo systemctl enable mosquitto

# Verify it's running
sudo systemctl status mosquitto
```

**Using Docker:**
```bash
docker run -d --name mosquitto -p 1883:1883 eclipse-mosquitto:2
```

**On macOS:**
```bash
brew install mosquitto
brew services start mosquitto
```

### 2. Configure LightNVR

Edit your `lightnvr.ini` configuration file and add/modify the `[mqtt]` section:

```ini
[mqtt]
; Enable MQTT publishing
enabled = true

; Broker address (use your broker's IP or hostname)
broker_host = localhost

; Broker port (1883 for plain, 8883 for TLS)
broker_port = 1883

; Optional authentication
; username = lightnvr
; password = your_password

; Client ID (must be unique per client)
client_id = lightnvr

; Topic prefix - events published to: {topic_prefix}/detections/{stream_name}
topic_prefix = lightnvr

; TLS encryption (requires broker TLS support)
tls_enabled = false

; Connection settings
keepalive = 60
qos = 1
retain = false
```

### 3. Restart LightNVR

After changing the configuration, restart LightNVR:

```bash
# If running as a service
sudo systemctl restart lightnvr

# If running in Docker
docker restart lightnvr

# If running directly
pkill lightnvr && ./lightnvr -c /path/to/lightnvr.ini
```

### 4. Test the Connection

Open a terminal and subscribe to all LightNVR detection events:

```bash
# Subscribe to all detection events
mosquitto_sub -h localhost -t "lightnvr/detections/#" -v
```

The `-v` flag shows the topic name along with the message. You should see output when detections occur on any stream.

To subscribe to a specific stream:
```bash
mosquitto_sub -h localhost -t "lightnvr/detections/front_door" -v
```

## Message Format

Detection events are published as JSON with the following structure:

**Topic:** `{topic_prefix}/detections/{stream_name}`

**Payload Example:**
```json
{
  "stream": "front_door",
  "timestamp": 1706745600,
  "timestamp_iso": "2024-02-01T12:00:00Z",
  "count": 2,
  "detections": [
    {
      "label": "person",
      "confidence": 0.92,
      "x": 0.25,
      "y": 0.30,
      "width": 0.15,
      "height": 0.45,
      "track_id": "abc123",
      "zone_id": "entrance"
    },
    {
      "label": "car",
      "confidence": 0.87,
      "x": 0.60,
      "y": 0.50,
      "width": 0.30,
      "height": 0.25,
      "track_id": "",
      "zone_id": ""
    }
  ]
}
```

**Field descriptions:**
| Field | Description |
|-------|-------------|
| `stream` | Name of the camera/stream |
| `timestamp` | Unix timestamp (seconds since epoch) |
| `timestamp_iso` | ISO 8601 formatted timestamp |
| `count` | Number of detections in this event |
| `detections` | Array of detection objects |
| `label` | Object class (person, car, dog, etc.) |
| `confidence` | Detection confidence (0.0 to 1.0) |
| `x`, `y` | Normalized top-left corner (0.0 to 1.0) |
| `width`, `height` | Normalized dimensions (0.0 to 1.0) |
| `track_id` | Object tracking ID (if tracking enabled) |
| `zone_id` | Detection zone name (if zones configured) |

## Integration Examples

### Home Assistant

Add the following to your `configuration.yaml`:

```yaml
mqtt:
  sensor:
    - name: "Front Door Detections"
      state_topic: "lightnvr/detections/front_door"
      value_template: "{{ value_json.count }}"
      json_attributes_topic: "lightnvr/detections/front_door"
      json_attributes_template: "{{ value_json | tojson }}"

# Create a binary sensor for person detection
binary_sensor:
  - platform: mqtt
    name: "Person at Front Door"
    state_topic: "lightnvr/detections/front_door"
    value_template: >
      {% if value_json.detections | selectattr('label', 'eq', 'person') | list | count > 0 %}
        ON
      {% else %}
        OFF
      {% endif %}
    off_delay: 30  # Turn off after 30 seconds of no detection
```

**Example automation:**
```yaml
automation:
  - alias: "Notify on Person Detection"
    trigger:
      platform: mqtt
      topic: "lightnvr/detections/front_door"
    condition:
      - condition: template
        value_template: >
          {{ trigger.payload_json.detections | selectattr('label', 'eq', 'person') | list | count > 0 }}
    action:
      - service: notify.mobile_app
        data:
          title: "Person Detected"
          message: "Someone is at the front door"
```

### Python Script

Simple Python script to receive and process detection events:

```python
#!/usr/bin/env python3
import json
import paho.mqtt.client as mqtt

BROKER_HOST = "localhost"
BROKER_PORT = 1883
TOPIC = "lightnvr/detections/#"

def on_connect(client, userdata, flags, rc):
    print(f"Connected to MQTT broker (rc={rc})")
    client.subscribe(TOPIC)

def on_message(client, userdata, msg):
    data = json.loads(msg.payload)
    stream = data['stream']
    timestamp = data['timestamp_iso']

    for det in data['detections']:
        print(f"[{timestamp}] {stream}: {det['label']} ({det['confidence']:.0%})")

client = mqtt.Client()
client.on_connect = on_connect
client.on_message = on_message

client.connect(BROKER_HOST, BROKER_PORT, 60)
client.loop_forever()
```

Install dependencies: `pip install paho-mqtt`

### Shell Script (Simple Alerting)

```bash
#!/bin/bash
# Simple alert script - sends desktop notification on person detection

mosquitto_sub -h localhost -t "lightnvr/detections/#" | while read -r line; do
    # Extract stream name and check for person
    if echo "$line" | grep -q '"label":"person"'; then
        stream=$(echo "$line" | grep -oP '"stream":"\K[^"]+')
        notify-send "Person Detected" "Motion on $stream"
    fi
done
```

## Troubleshooting

### Connection Issues

**Check if broker is running:**
```bash
# Verify Mosquitto is listening
sudo netstat -tlnp | grep 1883

# Check broker logs
sudo journalctl -u mosquitto -f
```

**Test broker connectivity:**
```bash
# Try to connect and publish a test message
mosquitto_pub -h localhost -t "test" -m "hello"

# In another terminal, subscribe to test
mosquitto_sub -h localhost -t "test"
```

**Check LightNVR logs:**
```bash
# Look for MQTT-related messages
tail -f /var/log/lightnvr.log | grep -i mqtt
```

### Authentication Issues

If your broker requires authentication:

1. **Configure broker credentials** (in `/etc/mosquitto/mosquitto.conf`):
   ```
   allow_anonymous false
   password_file /etc/mosquitto/passwd
   ```

2. **Create password file:**
   ```bash
   sudo mosquitto_passwd -c /etc/mosquitto/passwd lightnvr
   ```

3. **Configure LightNVR:**
   ```ini
   [mqtt]
   username = lightnvr
   password = your_password
   ```

### No Messages Appearing

1. **Verify MQTT is enabled** in your config: `enabled = true`
2. **Check topic subscription** matches your config's `topic_prefix`
3. **Ensure detections are occurring** - check recordings for detection badges
4. **Verify detection is configured** - API detection must be enabled and working

### QoS Levels Explained

| QoS | Name | Description |
|-----|------|-------------|
| 0 | At most once | Fire and forget. Fastest, but messages may be lost |
| 1 | At least once | Message delivered at least once. May receive duplicates |
| 2 | Exactly once | Message delivered exactly once. Slowest, highest overhead |

**Recommendation:** Use QoS 1 for most cases. QoS 0 if you have many detections and can tolerate occasional loss.

## Advanced Configuration

### TLS/SSL Encryption

For secure connections over the internet:

1. **Configure broker with TLS** (in `/etc/mosquitto/mosquitto.conf`):
   ```
   listener 8883
   cafile /etc/mosquitto/certs/ca.crt
   certfile /etc/mosquitto/certs/server.crt
   keyfile /etc/mosquitto/certs/server.key
   ```

2. **Configure LightNVR:**
   ```ini
   [mqtt]
   broker_port = 8883
   tls_enabled = true
   ```

### Retained Messages

When `retain = true`, the broker stores the last message on each topic. New subscribers immediately receive the most recent detection. This is useful if you want to know the last detection state when a client connects.

```ini
[mqtt]
retain = true
```

### Multiple LightNVR Instances

If running multiple LightNVR instances, ensure each has a unique `client_id`:

```ini
# Instance 1
[mqtt]
client_id = lightnvr-garage

# Instance 2
[mqtt]
client_id = lightnvr-frontyard
```


