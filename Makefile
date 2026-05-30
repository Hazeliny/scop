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
all: deps build run

# ── Dependency check & install hint ──────────────────────────────────────────
.PHONY: deps
deps:
ifeq ($(OS), Darwin)
	@echo "==> [macOS] Checking dependencies..."
	@which xquartz > /dev/null 2>&1 || \
		(echo "  XQuartz not found. Installing via Homebrew..." && \
		 brew install --cask xquartz && \
		 echo "  !! Please LOG OUT and LOG BACK IN after XQuartz install, then re-run make.")
	@which docker > /dev/null 2>&1 || \
		(echo "  Docker not found. Please install Docker Desktop from https://www.docker.com" && exit 1)
	@echo "  OK: XQuartz and Docker found."
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
	@echo "==> [macOS] Launching with XQuartz X11 forwarding..."
	@# Ensure XQuartz is running
	@open -a XQuartz 2>/dev/null || true
	@sleep 1
	@# Allow connections from localhost
	@xhost +localhost 2>/dev/null || true
	docker run --rm \
		-e DISPLAY=host.docker.internal:0 \
		-v /tmp/.X11-unix:/tmp/.X11-unix \
		$(IMAGE_NAME)
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
dev:
ifeq ($(OS), Darwin)
	@which rustc > /dev/null 2>&1 || \
		(echo "Rust not found. Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" && exit 1)
	cargo run --release
else
	@which rustc > /dev/null 2>&1 || \
		(echo "Rust not found. Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" && exit 1)
	@sudo apt-get install -y libgl1-mesa-dev libx11-dev libxrandr-dev \
		libxi-dev libxxf86vm-dev 2>/dev/null || true
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