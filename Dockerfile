# Use a Rust image as the base
FROM rust:latest

# Create a new directory for your application
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build project to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY . .

# Build application
RUN cargo install --path .

# The command to run application
CMD ["cargo","run"]
