# H200 LLM Server - Complete Configuration

## 🚀 Server Details

**Droplet:** ml-ai-ubuntu-gpu-h200x1-141gb-nyc2
**IP Address:** `162.243.44.244`
**GPU:** NVIDIA H200 (141GB VRAM)
**Region:** NYC2
**Cost:** $3.44/hour (~$2,475/month if running 24/7)
**Status:** ✅ **ONLINE AND WORKING!**

---

## 📊 Current Configuration

### Model: Qwen 2.5 32B Instruct

```yaml
Model: Qwen/Qwen2.5-32B-Instruct
Context Window: 128K tokens (131,072) - MAXIMUM!
Precision: FP16 (full quality, no quantization)
GPU Memory: 90% utilization (~126GB)
Model Weights: 61GB
KV Cache: 61GB (allows full 128K context!)
Batch Size: 16,384 tokens
Max Sequences: 256 concurrent
Concurrency: 1.92x for 128K token requests
Features:
  - Flash Attention (optimized)
  - Chunked Prefill (enabled)
  - CUDA Graphs (7 sec capture)
  - Streaming support (built-in)
```

### API Endpoint

```
http://162.243.44.244:8000
```

OpenAI-compatible API (drop-in replacement)

---

## 🧪 Test Commands

### 1. Check if server is ready

```bash
curl http://162.243.44.244:8000/v1/models
```

**Expected output:**
```json
{
  "object": "list",
  "data": [
    {
      "id": "Qwen/Qwen2.5-72B-Instruct",
      "object": "model",
      "created": 1234567890,
      "owned_by": "vllm"
    }
  ]
}
```

### 2. Simple test request

```bash
curl http://162.243.44.244:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-32B-Instruct",
    "messages": [
      {"role": "user", "content": "Hello! What model are you?"}
    ],
    "max_tokens": 100,
    "temperature": 0.7
  }'
```

**Example response:**
```json
{
  "id": "chatcmpl-...",
  "object": "chat.completion",
  "created": 1764489798,
  "model": "Qwen/Qwen2.5-32B-Instruct",
  "choices": [{
    "index": 0,
    "message": {
      "role": "assistant",
      "content": "Hello! I am Qwen, a large language model..."
    },
    "finish_reason": "stop"
  }],
  "usage": {
    "prompt_tokens": 42,
    "total_tokens": 123,
    "completion_tokens": 81
  }
}
```

### 3. Test with streaming

```bash
curl http://162.243.44.244:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-32B-Instruct",
    "messages": [
      {"role": "user", "content": "Write a haiku about AI"}
    ],
    "stream": true,
    "max_tokens": 50
  }'
```

### 4. Test long context (128K tokens!)

```bash
curl http://162.243.44.244:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-32B-Instruct",
    "messages": [
      {"role": "user", "content": "Summarize this: [paste up to 100K tokens of text here]"}
    ],
    "max_tokens": 500
  }'
```

---

## 🐍 Python Client Example

```python
from openai import OpenAI

# Point to your H200 server
client = OpenAI(
    api_key="not-needed",  # vLLM doesn't require auth by default
    base_url="http://162.243.44.244:8000/v1",
)

# Simple chat
response = client.chat.completions.create(
    model="Qwen/Qwen2.5-32B-Instruct",
    messages=[
        {"role": "user", "content": "Explain quantum computing in simple terms"}
    ],
    max_tokens=500,
    temperature=0.7,
)

print(response.choices[0].message.content)

# Streaming example (ENABLED!)
stream = client.chat.completions.create(
    model="Qwen/Qwen2.5-32B-Instruct",
    messages=[
        {"role": "user", "content": "Write a Python function to calculate fibonacci"}
    ],
    stream=True,
    max_tokens=500,
)

for chunk in stream:
    if chunk.choices[0].delta.content:
        print(chunk.choices[0].delta.content, end="", flush=True)
```

---

## 🔧 Service Management

### Check status
```bash
ssh root@162.243.44.244 'systemctl status vllm'
```

### View logs (live)
```bash
ssh root@162.243.44.244 'journalctl -u vllm -f'
```

### View recent logs
```bash
ssh root@162.243.44.244 'journalctl -u vllm -n 100 --no-pager'
```

### Restart service
```bash
ssh root@162.243.44.244 'systemctl restart vllm'
```

### Stop service
```bash
ssh root@162.243.44.244 'systemctl stop vllm'
```

---

## 📈 Performance Expectations

- **Speed:** ~40-60 tokens/second
- **First token latency:** ~100-200ms
- **Context:** Up to 128K tokens (~96,000 words)
- **Quality:** Near GPT-4 level for coding, reasoning, general tasks
- **Concurrent users:** 128 simultaneous requests

---

## 💰 Cost Management

**Current cost:** $3.44/hour

### To stop paying:
```bash
# Stop the service (keeps droplet running)
ssh root@162.243.44.244 'systemctl stop vllm'

# Destroy the droplet entirely
doctl compute droplet delete ml-ai-ubuntu-gpu-h200x1-141gb-nyc2 --force
```

### To pause and resume:
```bash
# Create snapshot (takes ~10 min, costs $0.05/GB/month)
doctl compute droplet-action snapshot <droplet-id> --snapshot-name h200-qwen-snapshot

# Delete droplet
doctl compute droplet delete <droplet-id>

# Restore later from snapshot
doctl compute droplet create --image <snapshot-id> --size gpu-h200x1-141gb ...
```


