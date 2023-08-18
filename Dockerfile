# Use the official Rust image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Rust project files into the container
COPY . .

# Compile the Rust project
RUN cargo build --release

# Run the compiled executable
CMD ["target/release/backend"]
