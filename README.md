# GPU/CPU Monitor Project

## Project Overview

This project is a Rust-based command-line tool for monitoring both CPU and GPU statistics with a colorful tabular display. It supports both local system monitoring and remote monitoring via SSH.

## Key Features

1. **Real-time Monitoring**: Updates every 2 seconds
2. **Colorful Table Output**: Different colors for different information types
3. **Cross-Platform Support**: Built as a static binary with musl libc
4. **GPU Support**: Shows GPU usage when NVIDIA GPU is detected
5. **System Agnostic**: Gracefully handles systems without GPU drivers

## Directory Structure

```
rust-gpu-cpu-monitor/
├── src/
│   └── main.rs          # Main implementation
├── Cargo.toml           # Build configuration
├── Makefile             # Build automation
├── build.sh             # Build script for static linking
└── README.md            # Documentation
```

## Technical Implementation Details

### Core Components

1. **System Information Parsing**:
   - CPU load from `/proc/stat`
   - Memory usage from `/proc/meminfo`
   - GPU info from `nvidia-smi` when available

2. **Color Coding**:
   - Green: CPU Load Percentage
   - Yellow: Memory Usage Percentage
   - Red: GPU Usage Percentage
   - Blue: GPU Memory Usage Percentage

3. **Display Format**:
   - Tabular structure with Unicode box characters
   - Clean layout with clear sectioning
   - Colorized output for quick information scanning

### Build Configuration

The project is configured for:
- Static linking with musl libc (`x86_64-unknown-linux-musl`)
- Release build with optimizations (`opt-level = "z"`)
- Stripped debug symbols (`strip = true`)
- LTO (Link Time Optimization) enabled

## Usage Instructions

1. **Build the static binary**:
   ```bash
   make build-static
   ```

2. **Run the monitor**:
   ```bash
   ./target/x86_64-unknown-linux-musl/release/gpu-cpu-monitor
   ```

3. **Exit**:
   Press Ctrl+C to exit the monitoring loop

## Requirements

- Rust toolchain (1.56 or higher)
- musl-tools for static linking
- On systems with NVIDIA GPUs, `nvidia-smi` utility must be installed

## Design Principles

1. **Minimal Footprint**: Only essential system information is displayed
2. **Performance**: Efficient parsing of system information files
3. **Reliability**: Graceful handling of missing components
4. **Usability**: Clear visual distinction between different metrics
5. **Portability**: Static binary ensures compatibility across systems

## Troubleshooting

If you encounter build issues, ensure:
1. You have Rust installed (`rustc --version`)
2. You have the musl target installed (`rustup target add x86_64-unknown-linux-musl`)
3. You have musl-tools installed (on Ubuntu/Debian: `sudo apt install musl-tools`)

## Note about Sandboxing

Due to sandbox restrictions in the current environment, the build may fail when trying to execute shell commands. If you're unable to build the project in this environment, please try:

1. Running the build outside of the sandboxed environment
2. Ensuring all required Rust components are installed:
   - Rust toolchain
   - musl-tools
   - x86_64-unknown-linux-musl target