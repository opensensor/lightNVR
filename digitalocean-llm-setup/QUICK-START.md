# 🚀 H200 LLM Server - Quick Start

## ✅ Server is ONLINE and READY!

**API Endpoint:** `http://162.243.44.244:8000`  
**Model:** Qwen 2.5 32B Instruct  
**Context:** 128K tokens (131,072) - MAXIMUM!  
**Streaming:** ✅ Enabled  
**Status:** 🟢 Running

---

## 🧪 Test It Now

### 1. Check if online
```bash
curl http://162.243.44.244:8000/health
```

### 2. Simple test
```bash
curl http://162.243.44.244:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-32B-Instruct",
    "messages": [{"role": "user", "content": "Hello!"}],
    "max_tokens": 100
  }'
```

### 3. Test streaming
```bash
curl http://162.243.44.244:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/Qwen2.5-32B-Instruct",
    "messages": [{"role": "user", "content": "Count to 10"}],
    "stream": true
  }'
```

---

## 🐍 Python Example

```python
from openai import OpenAI

client = OpenAI(
    api_key="not-needed",
    base_url="http://162.243.44.244:8000/v1"
)

# Simple request
response = client.chat.completions.create(
    model="Qwen/Qwen2.5-32B-Instruct",
    messages=[{"role": "user", "content": "Explain AI in one sentence"}],
    max_tokens=100
)

print(response.choices[0].message.content)

# Streaming request
stream = client.chat.completions.create(
    model="Qwen/Qwen2.5-32B-Instruct",
    messages=[{"role": "user", "content": "Write a haiku"}],
    stream=True
)

for chunk in stream:
    if chunk.choices[0].delta.content:
        print(chunk.choices[0].delta.content, end="", flush=True)
```

---

## 📊 Key Specs

- **Model Size:** 32B parameters
- **Context Window:** 128K tokens (96,000 words / 300+ pages)
- **Quality:** Excellent (beats many 70B models on benchmarks)
- **Speed:** Fast inference with Flash Attention
- **Memory:** 61GB model + 61GB KV cache = perfect fit for H200
- **Concurrency:** 1.92x for full 128K requests, 256 concurrent sequences

---

## 🔧 Service Management

```bash
# Check status
ssh root@162.243.44.244 'systemctl status vllm'

# View logs
ssh root@162.243.44.244 'journalctl -u vllm -f'

# Restart
ssh root@162.243.44.244 'systemctl restart vllm'

# Stop
ssh root@162.243.44.244 'systemctl stop vllm'
```

---

## 💰 Cost

**$3.44/hour** (~$2,475/month if running 24/7)

**Tip:** Stop the droplet when not in use to save money!

---

## 📚 Full Documentation

See `H200-CONFIG.md` for complete details, advanced examples, and troubleshooting.

---

## 🎯 Why Qwen 2.5 32B?

1. **Perfect fit:** 61GB model + 61GB KV cache = uses H200 optimally
2. **Full 128K context:** Unlike 72B models that OOM on long context
3. **High quality:** Beats many larger models on benchmarks
4. **Fast:** Smaller = faster inference
5. **No gating:** Works immediately without HuggingFace auth

**Bottom line:** Best balance of quality, speed, and context for single H200!

