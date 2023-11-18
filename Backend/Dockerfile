# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./

# Build the dependencies first to cache them
# RUN mkdir src && echo "fn main() {println!(\"dummy\")}" > src/main.rs
# RUN cargo build --release
# RUN rm -rf src

# Copy the entire source code to the container
COPY . .

# Build your Rust application
RUN cargo build --release
# Create a new image using a smaller base image
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/HackaTUM-Check24-Server /app/HackaTUM-Check24-Server

# Copy the Docker Compose file to the container
# COPY docker-compose.yml /app/docker-compose.yml

# Install necessary dependencies (if any) for your Rust app

# Expose the port your Rust app is listening on
EXPOSE 8080

# Start your Rust app when the container starts
CMD ["./HackaTUM-Check24-Server"]
