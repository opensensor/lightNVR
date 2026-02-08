#!/bin/bash
# Digital Ocean 8x H100 GPU Droplet Setup for DeepSeek-R1 671B
# Option A: Multi-GPU (8x H100 = 640GB) - Maximum Quality Setup
# Date: 2025-01-30

set -e

echo "=========================================="
echo "Digital Ocean 8x H100 LLM Setup"
echo "DeepSeek-R1 671B Full Model"
echo "=========================================="

# Update system
echo "[1/8] Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install NVIDIA drivers and CUDA
echo "[2/8] Checking NVIDIA drivers..."
if ! command -v nvidia-smi &> /dev/null; then
    echo "Installing NVIDIA drivers..."
    sudo apt-get install -y nvidia-driver-535 nvidia-utils-535
else
    echo "NVIDIA drivers already installed"
    nvidia-smi
fi

# Verify 8 GPUs are available
GPU_COUNT=$(nvidia-smi --list-gpus | wc -l)
echo "Detected $GPU_COUNT GPUs"
if [ "$GPU_COUNT" -ne 8 ]; then
    echo "WARNING: Expected 8 GPUs but found $GPU_COUNT"
    read -p "Continue anyway? [y/N]: " continue_choice
    if [[ ! $continue_choice =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Install Docker and NVIDIA Container Toolkit
echo "[3/8] Installing Docker and NVIDIA Container Toolkit..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
fi

distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
curl -s -L https://nvidia.github.io/libnvidia-container/gpgkey | sudo apt-key add -
curl -s -L https://nvidia.github.io/libnvidia-container/$distribution/libnvidia-container.list | \
    sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
sudo apt-get update
sudo apt-get install -y nvidia-container-toolkit
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker

# Install Python and dependencies
echo "[4/8] Installing Python and dependencies..."
sudo apt-get install -y python3.11 python3.11-venv python3-pip git

# Create working directory
echo "[5/8] Setting up working directory..."
mkdir -p ~/llm-deployment
cd ~/llm-deployment

# Create Python virtual environment
python3.11 -m venv venv
source venv/bin/activate

# Install vLLM with multi-GPU support
echo "[6/8] Installing vLLM with multi-GPU support..."
pip install --upgrade pip
pip install vllm==0.6.5
pip install huggingface-hub
pip install flash-attn --no-build-isolation

# Model selection
echo "[7/8] Preparing DeepSeek-R1 671B deployment..."
echo ""
echo "Select quantization level:"
echo "1) FP8 (Recommended - ~335GB, best quality/speed)"
echo "2) 4-bit AWQ (~200GB, faster but lower quality)"
echo ""
read -p "Enter choice [1-2]: " quant_choice

case $quant_choice in
    1)
        MODEL_NAME="deepseek-ai/DeepSeek-R1"
        QUANTIZATION="--quantization fp8"
        VRAM_EST="~335GB"
        ;;
    2)
        MODEL_NAME="deepseek-ai/DeepSeek-R1-AWQ"
        QUANTIZATION="--quantization awq"
        VRAM_EST="~200GB"
        ;;
    *)
        echo "Invalid choice, defaulting to FP8"
        MODEL_NAME="deepseek-ai/DeepSeek-R1"
        QUANTIZATION="--quantization fp8"
        VRAM_EST="~335GB"
        ;;
esac

echo "Selected: $MODEL_NAME with $VRAM_EST VRAM usage"

# Create systemd service for vLLM
echo "[8/8] Creating systemd service..."
sudo tee /etc/systemd/system/vllm.service > /dev/null <<EOF
[Unit]
Description=vLLM OpenAI-Compatible Server - DeepSeek-R1 671B
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/llm-deployment
Environment="PATH=$HOME/llm-deployment/venv/bin:/usr/local/bin:/usr/bin:/bin"
Environment="CUDA_VISIBLE_DEVICES=0,1,2,3,4,5,6,7"
ExecStart=$HOME/llm-deployment/venv/bin/vllm serve $MODEL_NAME \\
    --host 0.0.0.0 \\
    --port 8000 \\
    --max-model-len 131072 \\
    --gpu-memory-utilization 0.95 \\
    --tensor-parallel-size 8 \\
    --dtype auto \\
    $QUANTIZATION \\
    --enable-chunked-prefill \\
    --max-num-batched-tokens 16384 \\
    --max-num-seqs 512 \\
    --disable-log-requests \\
    --trust-remote-code
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

echo ""
echo "=========================================="
echo "Setup Complete!"
echo "=========================================="
echo ""
echo "Model: DeepSeek-R1 671B"
echo "Quantization: $VRAM_EST"
echo "GPUs: 8x H100 (Tensor Parallel)"
echo "Max Context: 131072 tokens (128K)"
echo "API Port: 8000"
echo ""
echo "IMPORTANT: First start will download ~400GB+ model files!"
echo "This may take 1-2 hours depending on network speed."
echo ""
echo "To start the service:"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl enable vllm"
echo "  sudo systemctl start vllm"
echo ""
echo "To monitor download/startup:"
echo "  sudo journalctl -u vllm -f"
echo ""
echo "API will be available at: http://YOUR_DROPLET_IP:8000"
echo "OpenAI-compatible endpoint: http://YOUR_DROPLET_IP:8000/v1"
echo ""

