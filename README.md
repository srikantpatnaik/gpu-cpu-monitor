# GPU/CPU Monitor

A Rust-based tool for monitoring GPU and CPU utilization in real-time with color-coded terminal output.

## Features

- Monitors CPU load and memory usage
- Monitors GPU utilization using `nvidia-smi`
- Supports both local and remote monitoring via SSH
- Color-coded terminal output with real-time updates
- Configurable refresh interval
- Graceful shutdown with Ctrl+C

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- `nvidia-smi` command-line tool (for GPU monitoring)
- SSH access (for remote monitoring)

### Build from Source

```bash
git clone https://github.com/srikantpatnaik/gpu-cpu-monitor.git
cd gpu-cpu-monitor
cargo build --release
```

The binary will be located at `target/release/gpu-cpu-monitor`.

## Usage

### Local Monitoring
```bash
# Basic usage with default 2-second refresh
./gpu-cpu-monitor

# Custom refresh interval (in seconds)
./gpu-cpu-monitor --refresh 5

# Disable color output
./gpu-cpu-monitor --no-color

# Custom refresh interval with color disabled
./gpu-cpu-monitor --refresh 3 --no-color
```

### Remote Monitoring
```bash
# Monitor a remote host via SSH
./gpu-cpu-monitor --ssh-host <host> --ssh-user <user> --ssh-port <port>

# Example with default SSH port (22)
./gpu-cpu-monitor --ssh-host 192.168.1.100 --ssh-user user

# Example with custom SSH port
./gpu-cpu-monitor --ssh-host 192.168.1.100 --ssh-user user --ssh-port 2222
```

## Output Format

The tool displays system information in a minimal format:

```
CPU 45.2  34.7
GPU 23.5  67.8
```

- First line: CPU load percentage and memory usage percentage
- Second line: Average GPU load percentage and memory usage percentage (if GPUs are present)

The color scheme indicates load levels:
- Green (< 40%)
- Yellow (41-80%)
- Red (> 80%)

## Development

This project uses:
- `clap` for command-line argument parsing
- `ssh2` for SSH connections (for remote monitoring)
- `ctrlc` for graceful shutdown handling

### Running Tests

```bash
cargo test
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.