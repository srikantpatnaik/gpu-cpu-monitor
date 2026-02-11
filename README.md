# GPU/CPU Monitor

[![Build Status](https://github.com/sri/gpu-cpu-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/sri/gpu-cpu-monitor/actions)

A Rust-based tool for monitoring GPU and CPU utilization on local and remote systems.

## Features

- **Local Monitoring**: Displays CPU load, memory usage, and GPU information
- **Remote SSH Monitoring**: Connects to remote hosts via SSH for monitoring
- **Color-coded Output**: Green (low), Yellow (medium), Red (high) usage indicators
- **Real-time Updates**: Configurable refresh intervals
- **Terminal-friendly**: Proper screen clearing and cursor positioning
- **Static Binaries**: Pre-built release binaries for x86_64 and ARM64 architectures

## Build

```bash
make
```

This will build the release version and create a symbolic link to the executable.

## Usage

### Local Monitoring
```bash
./gpu-cpu-monitor
```

### Remote Monitoring
```bash
./gpu-cpu-monitor --ssh-host <host> --ssh-user <user>
```

### Options
- `--refresh <seconds>`: Refresh interval (default: 2)
- `--no-color`: Disable color output
- `--ssh-host <host>`: Remote host to connect to
- `--ssh-user <user>`: SSH user (default: current user)
- `--ssh-port <port>`: SSH port (default: 22)

## Output Format

- CPU Load: Percentage of CPU utilization
- CPU Memory: Percentage of memory usage  
- GPU Load: Average GPU load percentage
- GPU Memory: Average GPU memory usage percentage
- Colors: Green (<40%), Yellow (40-80%), Red (>80%)

## Requirements

- Rust toolchain (cargo)
- nvidia-smi (for GPU monitoring)
- SSH access to remote hosts (for remote monitoring)

## Build Requirements

- musl libc (for static linking)

## Pre-built Releases

Pre-built static binaries are available for download from the [releases page](https://github.com/srikantpatnaik/gpu-cpu-monitor/releases).

### Download and Extract
```bash
# Download for x86_64
wget https://github.com/srikantpatnaik/gpu-cpu-monitor/releases/latest/download/gpu-cpu-monitor-x86_64-latest.tar.gz

# Download for ARM64
wget https://github.com/srikantpatnaik/gpu-cpu-monitor/releases/latest/download/gpu-cpu-monitor-arm64-latest.tar.gz

# Extract
tar -xzf gpu-cpu-monitor-x86_64-latest.tar.gz
chmod +x gpu-cpu-monitor
```

### Examples

Monitor local system:
```bash
./gpu-cpu-monitor
```

Monitor remote system (user will be prompted for password or use SSH agent):
```bash
./gpu-cpu-monitor --ssh-host example.com
```

Monitor remote system with specific user:
```bash
./gpu-cpu-monitor --ssh-host example.com --ssh-user myuser
```

Monitor with custom refresh interval:
```bash
./gpu-cpu-monitor --refresh 5
```

Monitor without colors:
```bash
./gpu-cpu-monitor --no-color
```