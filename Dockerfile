FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    gdb \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy our files
COPY . .

# Build the Rust ptrace demo
RUN cargo build --release

# Default command
CMD ["./target/release/ptrace-demo"]
