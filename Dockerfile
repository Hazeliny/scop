# ── Stage 1: builder ──────────────────────────────────────────────────────────
FROM rust:1.78-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    cmake \
    build-essential \
    libsdl2-dev \
    libgl1-mesa-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build real source
COPY src ./src
COPY shaders ./shaders
COPY assets ./assets
RUN cargo build --release

# ── Stage 2: runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    # Core OpenGL + Mesa software renderer (no GPU needed)
    libgl1 \
    libgl1-mesa-dri \
    # EGL (needed by Mesa)
    libegl1 \
    libegl-mesa0 \
    # X11 display
    libx11-6 \
    libxext6 \
    # SDL2 runtime (dynamically linked against system SDL2)
    libsdl2-2.0-0 \
    && rm -rf /var/lib/apt/lists/*

# Force software OpenGL renderer — no GPU passthrough needed
ENV LIBGL_ALWAYS_SOFTWARE=1
# Force SDL2 to use X11 backend explicitly
ENV SDL_VIDEODRIVER=x11
# Force SDL2 to use Mesa OpenGL
ENV SDL_OPENGL_ES_DRIVER=0

WORKDIR /app
COPY --from=builder /app/target/release/scop ./scop
COPY --from=builder /app/shaders ./shaders
COPY --from=builder /app/assets ./assets

CMD ["./scop"]