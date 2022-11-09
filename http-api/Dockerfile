FROM rust:1.65.0-slim as builder
WORKDIR /app

# Optimize Docker Image build time
# Create a blank rust project 
# Copy Cargo.toml and Cargo.lock
# Install necessary native libraries to compile the app 
# Build without actual source code to download all dependencies
RUN USER=root cargo init --name plain --vcs none .
COPY Cargo.toml Cargo.lock ./
RUN apt update && apt install --yes libpq-dev && apt-get clean && rm -rf /var/lib/apt/lists/*;
RUN cargo build -r

# Copy all the source code
COPY . .
## Touch the main.rs to force a release build
RUN touch ./src/main.rs
# Build the app with release flag
RUN cargo build -r

FROM debian:stable-slim AS runtime

# install native deps also on the runtime image 
RUN apt update && apt install --yes libpq-dev && apt-get clean && rm -rf /var/lib/apt/lists/*;
# Create a non-root user
RUN useradd -m -u 1000 -U -s /bin/sh -d /app ferris
WORKDIR /app
# Impersonate into that non-root user (ferris 🦀)
USER ferris

# Copy application binary from builder image and ensure ownership is set to the non-root user
COPY --from=builder --chown=ferris /app/target/release/http-api /app

# Expose the port for the application and set corresponding env var
# Keep in mind! non-root means we need to use a port > 1024
EXPOSE 8080
ENV PORT=8080

# Finally run the 🦀 application
CMD ["./http-api"]
