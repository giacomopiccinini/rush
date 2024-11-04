# Rust base image
FROM rust:1.81.0

# Switch working directory
WORKDIR /app

# Install dependencies (mostly due to ffmpeg)
RUN apt update && apt install -y ffmpeg libavformat-dev libavutil-dev libavcodec-dev libavfilter-dev libavdevice-dev
RUN apt update && apt install -y libclang-dev

# Copy files from working environment to Docker image
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .


# Build the application
RUN cargo build --release

# # Run the application
# ENTRYPOINT ["./target/release/rush"]