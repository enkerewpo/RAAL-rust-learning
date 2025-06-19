# Use official Rust image as base
FROM rust:1-slim-bullseye

# Set working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create src directory and copy source code
COPY src/ ./src/

# Build the project
RUN cargo build --release

# Run the project
CMD ["./target/release/chapter8"] 