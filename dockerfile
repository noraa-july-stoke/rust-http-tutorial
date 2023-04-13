# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory to /usr/src/server
WORKDIR /usr/src/server

# Copy files into docker image
COPY . .

# Build the Rust application
RUN cargo build --release

# Copy the compiled source code from the run cargo build command
# to the container
COPY ./target/release ./

# Expose port 8080 to the outside world
EXPOSE 8080

# Run the server when the container launches
CMD ["./target/release/main"]

#usr/src/server/target/release/main
