# Use the official Node.js image (Debian-based) as the base image
FROM node:22-bullseye

# Update the system and install required packages
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install the Rust toolchain using rustup in non-interactive mode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's binary directory to the PATH environment variable
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm-pack for building the WebAssembly module from Rust
RUN cargo install wasm-pack

# Set the working directory to /app (this will be the root of our repository)
WORKDIR /psychroid

# Copy the entire repository into the container
COPY . .

# Move to the frontend directory and install Node.js dependencies
WORKDIR /psychroid/web
RUN npm install

# Expose the port used by the frontend development server (webpack-dev-server)
EXPOSE 8080

# Set the working directory back to the repository root (optional)
WORKDIR /psychroid

# Default command: start the frontend development server.
# This assumes that the "start" script in web/package.json runs webpack-dev-server.
CMD ["sh", "-c", "cd web && npm run start"]