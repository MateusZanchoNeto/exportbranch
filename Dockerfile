# stage-1: builder
# Use Ubuntu 16.04 as the base image
FROM ubuntu:16.04 as builder

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using Rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's bin directory to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release

# copy the source and build the application
RUN rm -rf ./src
COPY ./src ./src

# touch to force cargo rebuild main.rs with the real project
# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs
RUN cargo build --release

# stage-2: image only with the binary
FROM debian:buster-slim

WORKDIR /app

# copy binary from builder
COPY --from=builder /app/target/release/exportbranch .

CMD ["./exportbranch"]
