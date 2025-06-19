#!/bin/bash

echo "=== Chapter8 Rust Docker Runner ==="
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start Docker first."
    exit 1
fi

# Build image
echo "Step 1: Building Docker image..."
docker build -t chapter8-rust .

if [ $? -ne 0 ]; then
    echo "Error: Docker build failed!"
    exit 1
fi

echo "Step 2: Running the application in Docker container..."
echo ""

# Run container
docker run --rm chapter8-rust

echo ""
echo "=== Execution completed! ===" 