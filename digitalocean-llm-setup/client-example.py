#!/usr/bin/env python3
"""
Example client for Digital Ocean H100 LLM deployment
Demonstrates long-context usage and streaming responses
"""

import os
from openai import OpenAI

# Configure client to point to your Digital Ocean droplet
# Replace with your actual droplet IP
DROPLET_IP = os.getenv("DROPLET_IP", "localhost")
API_BASE = f"http://{DROPLET_IP}:8000/v1"

client = OpenAI(
    api_key="not-needed",  # vLLM doesn't require API key
    base_url=API_BASE,
)

def simple_chat():
    """Simple chat completion example"""
    print("=== Simple Chat Example ===\n")
    
    response = client.chat.completions.create(
        model="auto",  # vLLM auto-detects the loaded model
        messages=[
            {"role": "system", "content": "You are a helpful AI assistant."},
            {"role": "user", "content": "Explain quantum computing in simple terms."}
        ],
        max_tokens=500,
        temperature=0.7,
    )
    
    print(response.choices[0].message.content)
    print(f"\nTokens used: {response.usage.total_tokens}")


def streaming_chat():
    """Streaming response example"""
    print("\n=== Streaming Chat Example ===\n")
    
    stream = client.chat.completions.create(
        model="auto",
        messages=[
            {"role": "user", "content": "Write a short poem about AI."}
        ],
        max_tokens=200,
        temperature=0.8,
        stream=True,
    )
    
    for chunk in stream:
        if chunk.choices[0].delta.content:
            print(chunk.choices[0].delta.content, end="", flush=True)
    print("\n")


def long_context_example():
    """Example using long context (128K tokens)"""
    print("\n=== Long Context Example ===\n")
    
    # Simulate a long document (you can load actual files here)
    long_document = """
    [This would be your long document - up to 128K tokens]
    
    For demonstration, imagine this contains:
    - Multiple research papers
    - Code repositories
    - Documentation
    - Conversation history
    
    The model can process all of this in a single request!
    """
    
    response = client.chat.completions.create(
        model="auto",
        messages=[
            {"role": "system", "content": "You are analyzing a large document."},
            {"role": "user", "content": f"Document:\n{long_document}\n\nQuestion: Summarize the key points."}
        ],
        max_tokens=1000,
        temperature=0.3,
    )
    
    print(response.choices[0].message.content)
    print(f"\nInput tokens: {response.usage.prompt_tokens}")
    print(f"Output tokens: {response.usage.completion_tokens}")


def reasoning_example():
    """Example for DeepSeek-R1 reasoning capabilities"""
    print("\n=== Reasoning Example (DeepSeek-R1) ===\n")
    
    response = client.chat.completions.create(
        model="auto",
        messages=[
            {"role": "user", "content": """
Solve this problem step by step:

A farmer has 17 sheep. All but 9 die. How many sheep are left?

Think through this carefully and show your reasoning.
"""}
        ],
        max_tokens=1000,
        temperature=0.1,  # Lower temperature for reasoning
    )
    
    print(response.choices[0].message.content)


def code_generation_example():
    """Example for code generation (Qwen 2.5 excels at this)"""
    print("\n=== Code Generation Example ===\n")
    
    response = client.chat.completions.create(
        model="auto",
        messages=[
            {"role": "user", "content": """
Write a Python function that:
1. Takes a list of numbers
2. Removes duplicates
3. Sorts in descending order
4. Returns only even numbers

Include type hints and docstring.
"""}
        ],
        max_tokens=500,
        temperature=0.2,
    )
    
    print(response.choices[0].message.content)


def check_server_health():
    """Check if the server is running and responsive"""
    print("=== Checking Server Health ===\n")
    
    try:
        models = client.models.list()
        print(f"✓ Server is running")
        print(f"✓ Available models: {[m.id for m in models.data]}")
        return True
    except Exception as e:
        print(f"✗ Server error: {e}")
        return False


if __name__ == "__main__":
    print(f"Connecting to: {API_BASE}\n")
    
    if not check_server_health():
        print("\nMake sure your vLLM server is running!")
        print("Check with: sudo systemctl status vllm")
        exit(1)
    
    print("\n" + "="*50 + "\n")
    
    # Run examples
    simple_chat()
    streaming_chat()
    long_context_example()
    
    # Uncomment based on your model:
    # reasoning_example()  # For DeepSeek-R1
    # code_generation_example()  # For Qwen 2.5
    
    print("\n" + "="*50)
    print("All examples completed!")
    print("\nTo use in your own code:")
    print(f"  export DROPLET_IP={DROPLET_IP}")
    print("  python your_script.py")

