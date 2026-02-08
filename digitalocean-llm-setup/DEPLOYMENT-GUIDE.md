# Digital Ocean 8x H100 Deployment Guide

## Quick Start: Deprovision & Deploy

### Step 1: Deprovision Current Instance

**Via Digital Ocean Web Console:**
1. Go to https://cloud.digitalocean.com/droplets
2. Find your current droplet
3. Click on the droplet name
4. Click "Destroy" in the top right
5. Type the droplet name to confirm
6. Click "Destroy Droplet"

**Via CLI (if you have doctl installed):**
```bash
# List droplets
doctl compute droplet list

# Destroy by ID
doctl compute droplet delete <DROPLET_ID>
```

### Step 2: Create 8x H100 GPU Droplet

**Via Digital Ocean Web Console:**
1. Go to https://cloud.digitalocean.com/droplets/new
2. Choose Region: Select closest to you (e.g., NYC, SFO, AMS)
3. Choose Image: Ubuntu 22.04 LTS x64
4. Choose Size: 
   - Click "GPU Droplets"
   - Select **"NVIDIA HGX H100 (8x)"** - $23.92/hour
5. Choose Authentication: SSH Key (recommended) or Password
6. Finalize: 
   - Hostname: `llm-h100-8x` (or your choice)
   - Click "Create Droplet"

**Via CLI:**
```bash
# Create 8x H100 droplet
doctl compute droplet create llm-h100-8x \
  --region nyc3 \
  --size gpu-h100-8x \
  --image ubuntu-22-04-x64 \
  --ssh-keys <YOUR_SSH_KEY_ID>
```

### Step 3: Wait for Droplet to be Ready

This takes 1-2 minutes. Get the IP address:

**Web Console:**
- The IP will appear in the droplet list

**CLI:**
```bash
doctl compute droplet list
```

### Step 4: SSH into New Droplet

```bash
ssh root@<DROPLET_IP>
```

### Step 5: Run Setup Script

```bash
# Download the setup script
curl -O https://raw.githubusercontent.com/YOUR_REPO/setup-multi-h100.sh

# Or if you have it locally, copy it:
# scp setup-multi-h100.sh root@<DROPLET_IP>:~/

# Make executable
chmod +x setup-multi-h100.sh

# Run setup
./setup-multi-h100.sh
```

**Or manual setup:**
```bash
# Update system
sudo apt-get update && sudo apt-get upgrade -y

# Install git
sudo apt-get install -y git

# Clone your setup repo (if you've pushed it)
git clone https://github.com/YOUR_USERNAME/digitalocean-llm-setup.git
cd digitalocean-llm-setup

# Run setup
chmod +x setup-multi-h100.sh
./setup-multi-h100.sh
```

### Step 6: Start the Service

After setup completes:

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable vllm

# Start the service
sudo systemctl start vllm

# Monitor the startup (this will take 1-2 hours for first download)
sudo journalctl -u vllm -f
```

### Step 7: Monitor Progress

The first startup will download ~400GB of model files. Monitor with:

```bash
# Watch logs
sudo journalctl -u vllm -f

# Check GPU usage
watch -n 1 nvidia-smi

# Check disk space
df -h

# Check network usage
sudo apt-get install -y nethogs
sudo nethogs
```

### Step 8: Test the API

Once you see "Application startup complete" in the logs:

```bash
# Test from the droplet
curl http://localhost:8000/v1/models

# Test chat completion
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "deepseek-ai/DeepSeek-R1",
    "messages": [{"role": "user", "content": "Hello! Test message."}],
    "max_tokens": 100
  }'
```

### Step 9: Test from Your Local Machine

```bash
# Set environment variable
export DROPLET_IP=<YOUR_DROPLET_IP>

# Run the example client
python3 digitalocean-llm-setup/client-example.py
```

## Cost Monitoring

**8x H100 Pricing: $23.92/hour**

- 1 hour: $23.92
- 8 hours: $191.36
- 24 hours: $574.08
- 1 week: $4,018.56
- 1 month: $17,241.60

**Tips to Save Money:**
1. **Destroy when not in use** - You're charged by the hour
2. **Use snapshots** - Take a snapshot after setup, destroy droplet, recreate from snapshot when needed
3. **Set billing alerts** - In Digital Ocean settings
4. **Consider reserved instances** - If available, can save 30-50%

## Troubleshooting

### GPU Not Detected
```bash
nvidia-smi
# Should show 8x H100 GPUs
```

### Service Won't Start
```bash
sudo journalctl -u vllm -n 100 --no-pager
```

### Out of Memory
- Reduce `--gpu-memory-utilization` from 0.95 to 0.90
- Use 4-bit quantization instead of FP8

### Slow Download
- Digital Ocean has fast network, but HuggingFace can be slow
- Consider using a mirror or pre-downloading to object storage

### Port Not Accessible
```bash
# Check firewall
sudo ufw status

# Allow port 8000
sudo ufw allow 8000/tcp

# Or disable firewall (not recommended for production)
sudo ufw disable
```

## Next Steps

Once running:
1. Test with the example client
2. Integrate with your application
3. Set up monitoring (Prometheus/Grafana)
4. Configure SSL/TLS for production
5. Set up load balancing if needed

## Snapshot for Quick Restart

After successful setup:

```bash
# From your local machine
doctl compute droplet-action snapshot <DROPLET_ID> --snapshot-name "llm-h100-ready"

# Later, create new droplet from snapshot
doctl compute droplet create llm-h100-new \
  --region nyc3 \
  --size gpu-h100-8x \
  --image <SNAPSHOT_ID>
```

This saves the 1-2 hour download time!

