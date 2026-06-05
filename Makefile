# ─────────────────────────────────────────────────────────────────────────────
#  SCOP — Makefile
#  Detects macOS or Linux and launches with correct X11/display setup.
#  Usage:
#    make          → build + run
#    make build    → build Docker image only
#    make run      → run existing image
#    make dev      → build & run natively (no Docker, needs Rust installed)
#    make clean    → remove Docker image + Rust build artifacts
#    make re       → clean + build + run
# ─────────────────────────────────────────────────────────────────────────────

IMAGE_NAME  := scop
BINARY      := target/release/scop

# ── OS detection ─────────────────────────────────────────────────────────────
OS := $(shell uname -s)

# ── Default target ───────────────────────────────────────────────────────────
.PHONY: all
all:
ifeq ($(OS), Darwin)
	@$(MAKE) dev
else
	@$(MAKE) deps build run
endif

# ── Dependency check & install hint ──────────────────────────────────────────
.PHONY: deps
deps:
ifeq ($(OS), Darwin)
	@echo "==> [macOS] Checking dependencies..."
	@which rustc > /dev/null 2>&1 || \
		(echo "Rust not found. Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" && exit 1)
	@which brew > /dev/null 2>&1 || \
		(echo "Homebrew not found. See https://brew.sh" && exit 1)
	@brew list sdl2 > /dev/null 2>&1 || \
		(echo "Installing SDL2..." && brew install sdl2)
	@echo "  OK: Rust and SDL2 found."
else
	@echo "==> [Linux] Checking dependencies..."
	@which docker > /dev/null 2>&1 || \
		(echo "  Docker not found. Installing..." && \
		 sudo apt-get update && sudo apt-get install -y docker.io && \
		 sudo usermod -aG docker $$USER && \
		 echo "  !! Log out and back in for group changes, then re-run make.")
	@echo "  OK: Docker found."
endif

# ── Build Docker image ────────────────────────────────────────────────────────
.PHONY: build
build:
	@echo "==> Building Docker image [$(IMAGE_NAME)]..."
	docker build -t $(IMAGE_NAME) .
	@echo "==> Build complete."

# ── Run ───────────────────────────────────────────────────────────────────────
.PHONY: run
run:
ifeq ($(OS), Darwin)
	@echo "macOS detected — use 'make dev' instead of 'make run'"
else
	@echo "==> [Linux] Launching with X11 socket passthrough..."
	@xhost +local:docker 2>/dev/null || true
	docker run --rm \
		--network host \
		-e DISPLAY=$(DISPLAY) \
		-v /tmp/.X11-unix:/tmp/.X11-unix \
		$(IMAGE_NAME)
endif

# ── Native dev (no Docker) ────────────────────────────────────────────────────
# Useful for fast iteration. Requires Rust + system OpenGL libs installed.
.PHONY: dev
dev: deps
ifeq ($(OS), Darwin)
	@echo "==> [macOS] Building and running natively..."
	@# Help cargo find SDL2 installed by Homebrew
	LIBRARY_PATH="$$(brew --prefix sdl2)/lib" \
	CPATH="$$(brew --prefix sdl2)/include" \
	cargo run --release
else
	@echo "==> [Linux] Building and running natively..."
	@sudo apt-get install -y libsdl2-dev libgl1-mesa-dev 2>/dev/null || true
	cargo run --release
endif

# ── Clean ─────────────────────────────────────────────────────────────────────
.PHONY: clean
clean:
	@echo "==> Removing Docker image..."
	docker rmi $(IMAGE_NAME) 2>/dev/null || true
	@echo "==> Removing Rust build artifacts..."
	cargo clean 2>/dev/null || true
	@echo "==> Clean done."

# ── Full rebuild ──────────────────────────────────────────────────────────────
.PHONY: re
re: clean all
