#!/bin/bash
# Digital Ocean H100 GPU Droplet Setup for Maximum Quality LLM
# Option A: Single H100 (80GB) - Best Quality Setup
# Date: 2025-01-30

set -e

echo "=========================================="
echo "Digital Ocean H100 LLM Setup - Option A"
echo "=========================================="

# Update system
echo "[1/8] Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install NVIDIA drivers and CUDA (if not already installed)
echo "[2/8] Checking NVIDIA drivers..."
if ! command -v nvidia-smi &> /dev/null; then
    echo "Installing NVIDIA drivers..."
    sudo apt-get install -y nvidia-driver-535 nvidia-utils-535
else
    echo "NVIDIA drivers already installed"
    nvidia-smi
fi

# Install Docker and NVIDIA Container Toolkit
echo "[3/8] Installing Docker and NVIDIA Container Toolkit..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
fi

# Install NVIDIA Container Toolkit
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

# Install vLLM (optimized for H100)
echo "[6/8] Installing vLLM..."
pip install --upgrade pip
pip install vllm==0.6.5 # Latest stable version
pip install huggingface-hub

# Download model (choose one)
echo "[7/8] Preparing to download model..."
echo ""
echo "Select model to deploy:"
echo "1) Qwen 2.5 72B Instruct (Recommended - Best all-around)"
echo "2) DeepSeek-R1-Distill-Llama-70B (Best reasoning)"
echo "3) DeepSeek-R1-Distill-Qwen-32B (Faster, still excellent)"
echo ""
read -p "Enter choice [1-3]: " model_choice

case $model_choice in
    1)
        MODEL_NAME="Qwen/Qwen2.5-72B-Instruct"
        MAX_MODEL_LEN=131072  # 128K context
        ;;
    2)
        MODEL_NAME="deepseek-ai/DeepSeek-R1-Distill-Llama-70B"
        MAX_MODEL_LEN=131072  # 128K context
        ;;
    3)
        MODEL_NAME="deepseek-ai/DeepSeek-R1-Distill-Qwen-32B"
        MAX_MODEL_LEN=131072  # 128K context
        ;;
    *)
        echo "Invalid choice, defaulting to Qwen 2.5 72B"
        MODEL_NAME="Qwen/Qwen2.5-72B-Instruct"
        MAX_MODEL_LEN=131072
        ;;
esac

echo "Selected model: $MODEL_NAME"

# Create systemd service for vLLM
echo "[8/8] Creating systemd service..."
sudo tee /etc/systemd/system/vllm.service > /dev/null <<EOF
[Unit]
Description=vLLM OpenAI-Compatible Server
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/llm-deployment
Environment="PATH=$HOME/llm-deployment/venv/bin:/usr/local/bin:/usr/bin:/bin"
ExecStart=$HOME/llm-deployment/venv/bin/vllm serve $MODEL_NAME \\
    --host 0.0.0.0 \\
    --port 8000 \\
    --max-model-len $MAX_MODEL_LEN \\
    --gpu-memory-utilization 0.95 \\
    --tensor-parallel-size 1 \\
    --dtype auto \\
    --enable-chunked-prefill \\
    --max-num-batched-tokens 8192 \\
    --max-num-seqs 256 \\
    --disable-log-requests
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
echo "Model: $MODEL_NAME"
echo "Max Context: $MAX_MODEL_LEN tokens (128K)"
echo "API Port: 8000"
echo ""
echo "To start the service:"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl enable vllm"
echo "  sudo systemctl start vllm"
echo ""
echo "To check status:"
echo "  sudo systemctl status vllm"
echo "  sudo journalctl -u vllm -f"
echo ""
echo "API will be available at: http://YOUR_DROPLET_IP:8000"
echo "OpenAI-compatible endpoint: http://YOUR_DROPLET_IP:8000/v1"
echo ""
echo "Test with:"
echo '  curl http://localhost:8000/v1/chat/completions \'
echo '    -H "Content-Type: application/json" \'
echo '    -d '"'"'{"model": "'"$MODEL_NAME"'", "messages": [{"role": "user", "content": "Hello!"}]}'"'"
echo ""

