FROM rust:latest as builder
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM scratch
COPY --from=builder /target/x86_64-unknown-linux-musl/release/backend .
CMD ["/backend"]












## Use the official Rust image as the build environment
#FROM rust:latest as build
#
## Set the working directory in the container
#WORKDIR /app
#
## Copy the source code into the container
#COPY . .
#
## Build the Rust application with release optimizations
#RUN cargo build --release
#
## Create a new stage with a minimal runtime image
#FROM debian:buster-slim
#
## Set the working directory in the container
#WORKDIR /app
#
## Copy the binary from the build stage to the runtime stage
#COPY --from=build /app/target/release/backend /app/backend
#
## Expose port 8000
#EXPOSE 8000
#
## Command to run the backend application
#CMD ["./backend"]
