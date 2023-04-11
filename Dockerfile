# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/voltdb

# Copy the project's Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm src/main.rs

# Copy the rest of the source code
COPY . .

# Build the application
RUN cargo build --release

# Expose the server's port
EXPOSE 3344

# Run the server when the container starts
CMD ["cargo", "run", "--release"]
