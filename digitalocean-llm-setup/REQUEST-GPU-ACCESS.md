# How to Get Access to Digital Ocean H100/H200 GPUs

## Issue
The 8x H100 and other high-end GPU droplets are showing "Size is not available in this region" errors.

## Solutions

### Option 1: Contact Digital Ocean Support (Recommended)

**Submit a support ticket:**
1. Go to https://cloud.digitalocean.com/support/tickets/new
2. Subject: "Request Access to H100/H200 GPU Droplets"
3. Message template:

```
Hello,

I would like to request access to the following GPU droplet sizes for AI/ML workloads:

- gpu-h100x8-640gb (8x H100, $23.92/hour)
- gpu-h100x1-80gb (1x H100, $3.39/hour)
- gpu-h200x1-141gb (1x H200, $3.44/hour)

Use case: Running large language models (DeepSeek-R1 671B, Qwen 2.5 72B) for research and development.

Preferred regions: NYC3, SFO3, or any available region

Expected usage: [Brief testing / Ongoing development / Production workload]

Please let me know if there are any additional requirements or if I need to increase my account limits.

Thank you!
```

**Expected response time:** 24-48 hours

### Option 2: Use Paperspace (Digital Ocean's GPU Platform)

Paperspace has better GPU availability and is owned by Digital Ocean:

```bash
# Install Paperspace CLI
pip install paperspace

# Login
paperspace login

# List available machines
paperspace machines list

# Create H100 machine
paperspace machines create \
  --machineType H100 \
  --region East Coast (NY2) \
  --size 500 \
  --billingType hourly \
  --machineName llm-h100
```

**Paperspace Pricing (as of 2025):**
- H100 (80GB): ~$5.95/hour
- A100 (80GB): ~$3.09/hour
- Multi-GPU available

**Paperspace Web Console:**
https://console.paperspace.com/

### Option 3: Alternative Cloud Providers

If you need immediate access:

**1. RunPod**
- H100 80GB: $2.89-3.99/hour
- 8x H100: Available
- Best for: Quick deployment
- https://www.runpod.io/

```bash
# RunPod CLI
pip install runpod
runpod config
runpod create pod --name llm-h100 --gpu-type "NVIDIA H100" --gpu-count 8
```

**2. Lambda Labs**
- H100 80GB: $2.49/hour
- 8x H100: $19.92/hour
- Best for: Cost savings
- https://lambdalabs.com/

**3. Vast.ai**
- H100 80GB: $1.50-2.50/hour (spot pricing)
- 8x H100: Available
- Best for: Cheapest option
- https://vast.ai/

**4. CoreWeave**
- H100 80GB: $2.21/hour
- 8x H100: $17.68/hour
- Best for: Enterprise features
- https://www.coreweave.com/

### Option 4: Use Single GPU with Smaller Model

While waiting for access, deploy on available hardware:

**Best models for single GPU (48GB):**
- DeepSeek-R1-Distill-Qwen-32B (excellent quality)
- Qwen 2.5 32B (great for coding)
- Llama 3.3 70B (4-bit quantized)

**Try RTX 6000 Ada (48GB) - $1.57/hour:**
```bash
doctl compute droplet create llm-qwen-32b \
  --region nyc1 \
  --size gpu-6000adax1-48gb \
  --image ubuntu-22-04-x64 \
  --ssh-keys 52309285
```

## Immediate Action Plan

**Step 1:** Submit Digital Ocean support ticket (do this now)

**Step 2:** While waiting, try Paperspace:
```bash
# Quick Paperspace setup
pip install paperspace
paperspace login
paperspace machines create --machineType H100 --region "East Coast (NY2)"
```

**Step 3:** If urgent, use RunPod for immediate H100 access:
```bash
# RunPod has best availability
# Sign up at https://www.runpod.io/
# Deploy via web console (easiest)
```

## Cost Comparison (8x H100 equivalent)

| Provider | 8x H100 Price/hr | Availability | Setup Time |
|----------|------------------|--------------|------------|
| Digital Ocean | $23.92 | Limited | Requires approval |
| Paperspace | ~$47.60 | Good | Minutes |
| RunPod | ~$31.92 | Excellent | Minutes |
| Lambda Labs | $19.92 | Limited | Hours-Days |
| CoreWeave | $17.68 | Good | Hours |
| Vast.ai | $12-20 | Excellent | Minutes |

## Recommendation

**For immediate testing:**
1. Use RunPod or Vast.ai (available now, cheaper)
2. Deploy DeepSeek-R1 671B with 4-bit quantization
3. Test your workload

**For production:**
1. Wait for Digital Ocean approval (better integration with your existing infra)
2. Or use Paperspace (same company, better GPU availability)

**Budget option:**
1. Use single H100 on RunPod ($2.89/hr)
2. Deploy Qwen 2.5 72B (fits in 80GB, excellent quality)
3. Save $21/hour vs 8x H100

Would you like me to create setup scripts for any of these alternatives?

