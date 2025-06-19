#!/bin/bash

# Build Docker image
echo "Building Docker image for chapter8..."
docker build -t chapter8-rust .

echo "Docker image built successfully!"
echo "To run the container, use: ./docker-run.sh" 