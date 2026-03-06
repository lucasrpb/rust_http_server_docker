# Use an official Rust image as the builder stage
FROM rust:1.83-slim AS builder

# Set the working directory
WORKDIR /app

# Copy the source code and build the application
COPY . .
RUN cargo build --release

# Use a minimal base image for the runtime stage
FROM debian:bookworm-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust_http_server_docker .

# Expose the port the server runs on
EXPOSE 3000

# Run the executable
CMD ["./rust_http_server_docker"]