# GPU/CPU Monitor

A Rust-based tool for monitoring GPU and CPU utilization on local and remote systems.

## Features

- **Local Monitoring**: Displays CPU load, memory usage, and GPU information
- **Remote SSH Monitoring**: Connects to remote hosts via SSH for monitoring
- **Color-coded Output**: Green (low), Yellow (medium), Red (high) usage indicators
- **Real-time Updates**: Configurable refresh intervals
- **Terminal-friendly**: Proper screen clearing and cursor positioning

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
- `--ssh-user <user>`: SSH user (default: root)
- `--ssh-port <port>`: SSH port (default: 22)

## Output Format

- CPU Load: Percentage of CPU utilization
- CPU Memory: Percentage of memory usage  
- GPU1:GPU Load Memory Load (for each GPU)
- Colors: Green (<40%), Yellow (40-80%), Red (>80%)

## Requirements

- Rust toolchain (cargo)
- nvidia-smi (for GPU monitoring)
- SSH access to remote hosts (for remote monitoring)

## Build Requirements

- musl libc (for static linking)