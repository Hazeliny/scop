# Single-stage: build and run in the same image
# Eliminates all dynamic library issues between builder and runtime
FROM rust:1.78-slim-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    cmake \
    build-essential \
    libsdl2-dev \
    libgl1-mesa-dev \
    libgl1-mesa-dri \
    libegl1-mesa \
    libx11-6 \
    libxext6 \
    && rm -rf /var/lib/apt/lists/*

ENV LIBGL_ALWAYS_SOFTWARE=1
ENV SDL_VIDEODRIVER=x11
ENV SDL_OPENGL_ES_DRIVER=0
ENV SDL_AUDIODRIVER=dummy

WORKDIR /app

COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY src ./src
COPY shaders ./shaders
COPY assets ./assets
RUN cargo build --release

CMD ["./target/release/scop"]