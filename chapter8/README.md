# Chapter8 Rust Docker Runner Guide

This directory contains all necessary files to run the Rust project in a Linux Docker container.

## File Description

- `Dockerfile`: Docker image build file
- `docker-build.sh`: Script to build Docker image
- `docker-run.sh`: Script to run Docker container
- `run-all.sh`: One-click build and run script
- `docker-compose.yml`: Docker Compose configuration file

## Usage

### Method 1: One-click run (Recommended)
```bash
./run-all.sh
```

### Method 2: Step by step execution
```bash
# 1. Build image
./docker-build.sh

# 2. Run container
./docker-run.sh
```

### Method 3: Using Docker Compose
```bash
# Build and run
docker-compose up --build

# Run only (if image exists)
docker-compose up

# Run in background
docker-compose up -d
```

### Method 4: Direct Docker commands
```bash
# Build image
docker build -t chapter8-rust .

# Run container
docker run --rm chapter8-rust
```

## Notes

1. Make sure Docker Desktop is running
2. First run may take time to download Rust base image
3. Container will be automatically cleaned up (using `--rm` flag)
4. Project runs in Linux environment, suitable for testing Linux-specific features (like futex)

## Troubleshooting

If you encounter permission issues, make sure scripts have execute permissions:
```bash
chmod +x *.sh
```

If Docker build fails, check:
1. Is Docker running?
2. Is network connection normal?
3. Is there enough disk space? 