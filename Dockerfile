FROM rust:slim-bookworm

# Install wasm
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install --locked trunk

# Copy the source code into the container
COPY . .

# Build the application using Trunk
WORKDIR /applications/web

# Expose the port
EXPOSE 8080

# Start a server and serve static files
CMD ["trunk", "serve", "--release", "--address", "0.0.0.0", "--port", "8080"]

