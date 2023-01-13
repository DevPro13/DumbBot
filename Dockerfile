#syntax=docker/dockerfile:1
# 1: Build the exe
FROM rust:latest as builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 1b:  compile Rust dependencies (and store as a separate Docker layer)
WORKDIR /usr/src/smartbot_bhoos
COPY Cargo.toml Cargo.lock ./
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"Hello, this is a build failure message. Believe me.! \")}" > src/main.rs
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 1c: Build the exe using the actual source code
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/smartbot_bhoos*
#copy src code from src dir
RUN rm ./src/*.rs
COPY src/* ./src/
RUN cargo install --target x86_64-unknown-linux-musl --path .
# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM scratch
COPY --from=builder /usr/src/smartbot_bhoos/target/x86_64-unknown-linux-musl/release/smartbot_bhoos .
USER 1000
CMD ["./smartbot_bhoos"]
EXPOSE 8001/tcp