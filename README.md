# GPU/CPU Monitor

A lightweight, fast Rust-based tool for real-time monitoring of GPU and CPU utilization on local and remote systems. Perfect for keeping an eye on system resources during intensive workloads.

## Features

- Local monitoring: Real-time display of CPU load, memory usage, and GPU information
- Remote SSH monitoring: Connect to remote hosts via SSH for distributed monitoring
- Color-coded output: Visual indicators - Green (<40%), Yellow (40-80%), Red (>80%)
- Real-time updates: Configurable refresh intervals for live monitoring
- Terminal-friendly: Clean screen clearing and proper cursor positioning
- Secure: Uses SSH key-based authentication for remote connections

## Quick Start

### Download Pre-built Binary

```bash
# Download the latest release
wget https://github.com/srikantpatnaik/gpu-cpu-monitor/releases/latest/download/gpu-cpu-monitor-x86_64-latest.tar.gz

# Extract and run
tar -xzf gpu-cpu-monitor-x86_64-latest.tar.gz
chmod +x gpu-cpu-monitor
./gpu-cpu-monitor
```

### Build from Source

```bash
git clone https://github.com/srikantpatnaik/gpu-cpu-monitor.git
cd gpu-cpu-monitor
make
```

## Usage

### Local Monitoring
```bash
gpu-cpu-monitor
```

### Remote Monitoring
```bash
gpu-cpu-monitor --ssh-host <host> --ssh-user <user>
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

### System Requirements
- Linux (tested on Ubuntu/Debian)
- nvidia-smi (for GPU monitoring - comes with NVIDIA drivers)
- SSH access to remote hosts (for remote monitoring)

### Build Requirements
- Rust toolchain (cargo)

## Installation

### Manual Download
```bash
# Download the latest release
wget https://github.com/srikantpatnaik/gpu-cpu-monitor/releases/latest/download/gpu-cpu-monitor-x86_64-latest.tar.gz

# Extract and install
tar -xzf gpu-cpu-monitor-x86_64-latest.tar.gz
sudo mv gpu-cpu-monitor /usr/local/bin/
```

## 💡 Usage Examples

### Local Monitoring
Monitor your local system with default settings:
```bash
gpu-cpu-monitor
```

### Remote Monitoring
Monitor remote server (will prompt for password or use SSH agent):
```bash
gpu-cpu-monitor --ssh-host server.example.com
```

Monitor with specific user and port:
```bash
gpu-cpu-monitor --ssh-host server.example.com --ssh-user admin --ssh-port 2222
```

### Customization
Monitor with custom refresh interval (5 seconds):
```bash
gpu-cpu-monitor --refresh 5
```

Monitor without colors (useful for scripts):
```bash
gpu-cpu-monitor --no-color
```

### SSH Key Authentication
For passwordless remote monitoring, set up SSH keys:
```bash
# Generate SSH key (if you don't have one)
ssh-keygen -t ed25519

# Copy to remote host
ssh-copy-id user@remote-host

# Now monitor without password prompts
gpu-cpu-monitor --ssh-host remote-host --ssh-user user
```

## 🔧 Troubleshooting

### GPU Not Detected
- Ensure NVIDIA drivers are properly installed
- Check if `nvidia-smi` command works in your terminal
- Verify that you have CUDA-compatible GPU

### SSH Connection Issues
- Check SSH connectivity: `ssh user@host`
- Verify SSH key permissions: `chmod 600 ~/.ssh/id_rsa`
- Ensure SSH agent is running for key-based auth

### High CPU Usage
- Increase refresh interval: `--refresh 5`
- The tool uses minimal resources, but high frequency updates may impact performance

### Permission Denied
- Ensure binary is executable: `chmod +x gpu-cpu-monitor`
- For system-wide installation: `sudo mv gpu-cpu-monitor /usr/local/bin/`

## Output Format

The tool displays metrics with color-coded indicators:
- CPU Load: Percentage of CPU utilization 
- CPU Memory: Percentage of system memory usage
- GPU Load: Average GPU load percentage
- GPU Memory: Average GPU memory usage percentage
- Colors: Green (<40%), Yellow (40-80%), Red (>80%)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [clap](https://clap.rs/) for command-line argument parsing
- GPU monitoring powered by NVIDIA's nvidia-smi tool