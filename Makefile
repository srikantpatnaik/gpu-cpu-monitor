# Simple Makefile for building and linking the release binary
# This file ensures the release build is properly used

.PHONY: all build clean link

# Default target - build and link
all: build link

# Build release version
build:
	cargo build --release

# Create symbolic link to release binary
link:
	@rm -f gpu-cpu-monitor 2>/dev/null || true
	@ln -sf target/release/gpu-cpu-monitor gpu-cpu-monitor

# Clean build artifacts
clean:
	rm -rf target/
	rm -f gpu-cpu-monitor

# Show help
help:
	@echo "Usage:"
	@echo "  make all     - Build release binary and create link"
	@echo "  make build   - Build release binary"
	@echo "  make link    - Create link to release binary"
	@echo "  make clean   - Clean build artifacts"
	@echo ""
	@echo "The release build is now being used as requested."