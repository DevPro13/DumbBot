# syntax=docker/dockerfile:1
FROM alpine:3.17
#working dir
WORKDIR /usr/src/SmartBot_Bhoos
#installing packages
RUN apk update && apk upgrade --available
RUN apk add --no-cache rust cargo
# Copy the source code into the container
COPY . .

# Build the project in release mode
RUN cargo build --release
EXPOSE 8001
# Set the default command to run the compiled binary
CMD ["./target/release/smartbot_bhoos"]


