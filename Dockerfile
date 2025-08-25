# Stage 1: Building the application
# Use the official Rust image to build the Rust application
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rustical
WORKDIR /rustical

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This trick will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/rustical*
RUN cargo build --release

# Stage 2: Setup the runtime environment
FROM rust:latest
WORKDIR /rustical

COPY ./static ./static

# Install required packages
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the build stage
COPY --from=builder /rustical/target/release/rustical /usr/local/bin/rustical

# Set the startup command to run your binary
CMD ["rustical"]

