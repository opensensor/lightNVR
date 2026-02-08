#!/bin/bash
# Digital Ocean H200 GPU Droplet Setup - Maximum Quality LLM
# H200: 141GB VRAM - Can run larger models than H100!
# Date: 2025-01-30

set -e

echo "=========================================="
echo "Digital Ocean H200 LLM Setup"
echo "141GB VRAM - Premium Configuration"
echo "=========================================="

# Update system
echo "[1/8] Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install NVIDIA drivers and CUDA
echo "[2/8] Checking NVIDIA drivers..."
if ! command -v nvidia-smi &> /dev/null; then
    echo "Installing NVIDIA drivers..."
    sudo apt-get install -y nvidia-driver-550 nvidia-utils-550
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
sudo apt-get install -y python3.11 python3.11-venv python3-pip git htop nvtop

# Create working directory
echo "[5/8] Setting up working directory..."
mkdir -p ~/llm-deployment
cd ~/llm-deployment

# Create Python virtual environment
python3.11 -m venv venv
source venv/bin/activate

# Install vLLM (optimized for H200)
echo "[6/8] Installing vLLM with H200 optimizations..."
pip install --upgrade pip
pip install vllm==0.6.5
pip install huggingface-hub
pip install flash-attn --no-build-isolation

# Download model
echo "[7/8] Select model to deploy..."
echo ""
echo "H200 (141GB) can run these models:"
echo "1) Qwen 2.5 72B - Full FP16 (Recommended - Best quality, no quantization)"
echo "2) DeepSeek-R1-Distill-Llama-70B - Full FP16 (Best reasoning)"
echo "3) DeepSeek-R1 671B - 8-bit quantized (Experimental - might fit!)"
echo "4) Qwen 2.5 72B + Qwen 2.5 32B - Run TWO models simultaneously"
echo ""
read -p "Enter choice [1-4]: " model_choice

case $model_choice in
    1)
        MODEL_NAME="Qwen/Qwen2.5-72B-Instruct"
        MAX_MODEL_LEN=131072
        DTYPE="float16"
        EXTRA_ARGS=""
        ;;
    2)
        MODEL_NAME="deepseek-ai/DeepSeek-R1-Distill-Llama-70B"
        MAX_MODEL_LEN=131072
        DTYPE="float16"
        EXTRA_ARGS=""
        ;;
    3)
        MODEL_NAME="deepseek-ai/DeepSeek-R1"
        MAX_MODEL_LEN=131072
        DTYPE="auto"
        EXTRA_ARGS="--quantization fp8 --trust-remote-code"
        echo "WARNING: DeepSeek-R1 671B is VERY large. This may not fit even with quantization."
        read -p "Continue? [y/N]: " continue_choice
        if [[ ! $continue_choice =~ ^[Yy]$ ]]; then
            echo "Defaulting to Qwen 2.5 72B"
            MODEL_NAME="Qwen/Qwen2.5-72B-Instruct"
            DTYPE="float16"
            EXTRA_ARGS=""
        fi
        ;;
    4)
        echo "Multi-model setup requires manual configuration."
        echo "Defaulting to Qwen 2.5 72B for now."
        MODEL_NAME="Qwen/Qwen2.5-72B-Instruct"
        MAX_MODEL_LEN=131072
        DTYPE="float16"
        EXTRA_ARGS=""
        ;;
    *)
        echo "Invalid choice, defaulting to Qwen 2.5 72B"
        MODEL_NAME="Qwen/Qwen2.5-72B-Instruct"
        MAX_MODEL_LEN=131072
        DTYPE="float16"
        EXTRA_ARGS=""
        ;;
esac

echo "Selected model: $MODEL_NAME"

# Create systemd service
echo "[8/8] Creating systemd service..."
sudo tee /etc/systemd/system/vllm.service > /dev/null <<EOF
[Unit]
Description=vLLM OpenAI-Compatible Server on H200
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
    --dtype $DTYPE \\
    --enable-chunked-prefill \\
    --max-num-batched-tokens 16384 \\
    --max-num-seqs 512 \\
    --disable-log-requests \\
    $EXTRA_ARGS
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

echo ""
echo "=========================================="
echo "H200 Setup Complete!"
echo "=========================================="
echo ""
echo "GPU: NVIDIA H200 (141GB VRAM)"
echo "Model: $MODEL_NAME"
echo "Max Context: $MAX_MODEL_LEN tokens (128K)"
echo "Precision: $DTYPE (Full precision - no quality loss!)"
echo "API Port: 8000"
echo ""
echo "To start the service:"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl enable vllm"
echo "  sudo systemctl start vllm"
echo ""
echo "To monitor startup (first run downloads model ~150GB):"
echo "  sudo journalctl -u vllm -f"
echo ""
echo "To monitor GPU usage:"
echo "  watch -n 1 nvidia-smi"
echo "  # or use nvtop for better visualization"
echo "  nvtop"
echo ""
echo "API will be available at: http://YOUR_DROPLET_IP:8000"
echo ""

