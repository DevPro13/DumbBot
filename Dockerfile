# syntax=docker/dockerfile:1
FROM rust:latest

# Copy the source code into the container
COPY . /smartbot_botmen

# Build the project in release mode
RUN cargo build --release

# Set the default command to run the compiled binary
CMD ["/app/target/release/botmen"]


