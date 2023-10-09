# Use an official Rust runtime as a parent image
FROM rust:1.57 AS rust-builder

# Copy local code to the container image
COPY . .

# Set the working directory
WORKDIR /bindings/web

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the Rust code
RUN wasm-pack build --target web

# Use an official node runtime as a parent image
FROM node:16 AS node-builder

# Set the working directory
WORKDIR /ionic

# Copy local code and wasm pkg from the rust build stage to the container image
COPY --from=rust-builder . .

# Install and build the JavaScript/TypeScript app
RUN yarn install
RUN yarn build

# Specify the runtime environment
FROM nginx:stable

# Set the working directory
WORKDIR /

# Copy built artifacts from the build stage
COPY --from=node-builder /ionic/dist .

# Expose the service on port 80
EXPOSE 80
