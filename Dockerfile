# Use the official Rust image as the build environment
FROM rust:latest as build

# Set the working directory in the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application with release optimizations
RUN cargo build --release

# Create a new stage with a minimal runtime image
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /app

# Copy the binary from the build stage to the runtime stage
COPY --from=build /app/target/release/backend /app/backend

# Expose port 8000
EXPOSE 8000

# Command to run the backend application
CMD ["./backend"]
