use clap::{Parser, ValueEnum};
use ssh2::Session;
use std::io::{self, Read, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

/// GPU and CPU monitoring tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// SSH host to connect to for remote monitoring
    #[clap(long)]
    ssh_host: Option<String>,

    /// SSH user for remote connection
    #[clap(long, default_value = "root")]
    ssh_user: String,

    /// SSH port (default is 22)
    #[clap(long, default_value = "22")]
    ssh_port: u16,

    /// Disable color output
    #[clap(long)]
    no_color: bool,

    /// Refresh interval in seconds
    #[clap(long, default_value = "2")]
    refresh: u64,
}

/// Color scheme for terminal output
#[derive(Debug, Clone, Copy, ValueEnum)]
enum ColorScheme {
    Light,
    Dark,
}

/// Structure to hold CPU information
#[derive(Debug, Clone)]
struct CpuInfo {
    load: f64,
    memory_percentage: f64,
}

/// Structure to hold GPU information
#[derive(Debug, Clone)]
struct GpuInfo {
    gpu_load: i32,
    memory_load: i32,
}

/// Structure to hold system information
#[derive(Debug, Clone)]
struct SystemInfo {
    cpu: CpuInfo,
    gpus: Vec<GpuInfo>,
}

/// Get memory information from /proc/meminfo
fn get_memory_info() -> (u64, u64) {
    let output = Command::new("cat").arg("/proc/meminfo").output();
    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut total_mem = 0u64;
            let mut free_mem = 0u64;

            for line in output_str.lines() {
                if line.starts_with("MemTotal:") {
                    total_mem = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                } else if line.starts_with("MemAvailable:") {
                    free_mem = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                }
            }
            (total_mem, free_mem)
        }
        _ => (0, 0),
    }
}

/// Get CPU info from /proc/cpuinfo
fn get_cpu_info() -> CpuInfo {
    // Get memory information
    let (total_mem, free_mem) = get_memory_info();
    let used_mem = total_mem.max(free_mem) - free_mem; // Ensure we don't have negative values
    let memory_percentage = if total_mem > 0 {
        (used_mem as f64 / total_mem as f64) * 100.0
    } else {
        0.0
    };

    // Calculate CPU load using /proc/stat
    let stat_output = Command::new("cat").arg("/proc/stat").output();
    let load_percentage = match stat_output {
        Ok(output) => {
            let stat_str = String::from_utf8_lossy(&output.stdout);
            let cpu_line = stat_str.lines().next().unwrap_or("");
            let cpu_parts: Vec<&str> = cpu_line.split_whitespace().collect();

            if cpu_parts.len() >= 5 {
                let user = cpu_parts[1].parse::<u64>().unwrap_or(0);
                let nice = cpu_parts[2].parse::<u64>().unwrap_or(0);
                let system = cpu_parts[3].parse::<u64>().unwrap_or(0);
                let idle = cpu_parts[4].parse::<u64>().unwrap_or(0);

                let total = user + nice + system + idle;
                let idle_total = idle;

                if total > 0 {
                    ((total - idle_total) as f64 / total as f64) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }
        _ => 0.0,
    };

    CpuInfo {
        load: load_percentage,
        memory_percentage,
    }
}

/// Get GPU information using utilization.gpu, memory.total,memory.used for accurate reporting
fn get_gpu_info() -> Vec<GpuInfo> {
    // Try to get GPU info using nvidia-smi if available
    let output = Command::new("nvidia-smi")
        .args(&[
            "--query-gpu=utilization.gpu,memory.total,memory.used",
            "--format=csv,noheader,nounits",
        ])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut gpus = Vec::new();

            for line in output_str.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 {
                    // Parse GPU utilization, memory total, and memory used
                    let gpu_utilization = parts[0].trim().parse::<i32>().unwrap_or(0);
                    let memory_total = parts[1].trim().parse::<u64>().unwrap_or(0);
                    let memory_used = parts[2].trim().parse::<u64>().unwrap_or(0);

                    // Calculate memory percentage
                    let memory_load = if memory_total > 0 {
                        ((memory_used as f64 / memory_total as f64) * 100.0) as i32
                    } else {
                        0
                    };

                    // Store both values - we'll need both for proper display
                    gpus.push(GpuInfo {
                        gpu_load: gpu_utilization,
                        memory_load,
                    });
                }
            }
            gpus
        }
        Ok(output) => {
            // nvidia-smi command exists but failed
            eprintln!("nvidia-smi command failed with status: {}", output.status);
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            vec![]
        }
        _ => {
            // nvidia-smi command not available
            vec![]
        }
    }
}

/// Get system info from remote host via SSH
fn get_remote_system_info(
    host: &str,
    user: &str,
    port: u16,
) -> Result<SystemInfo, Box<dyn std::error::Error>> {
    // Establish SSH connection
    let tcp = std::net::TcpStream::connect(format!("{}:{}", host, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // Authenticate with username only for now (password or key based authentication)
    // This is a simple implementation - in a real app you might want to support
    // multiple authentication methods or better credential handling
    sess.userauth_agent(user)?;

    // Create a session to execute commands
    let mut channel = sess.channel_session()?;

    // Execute command to get CPU load
    channel.exec("cat /proc/stat | head -n 1")?;
    let mut cpu_output = Vec::new();
    channel.read_to_end(&mut cpu_output)?;
    channel.close()?;

    // Parse CPU information
    let mut cpu_load = 0.0;

    // Convert bytes to string for parsing
    let cpu_output_str = String::from_utf8_lossy(&cpu_output);

    // Parse CPU data
    if !cpu_output_str.is_empty() {
        let lines: Vec<&str> = cpu_output_str.lines().collect();
        if let Some(line) = lines.first() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let user = parts[1].parse::<u64>().unwrap_or(0);
                let nice = parts[2].parse::<u64>().unwrap_or(0);
                let system = parts[3].parse::<u64>().unwrap_or(0);
                let idle = parts[4].parse::<u64>().unwrap_or(0);

                let total = user + nice + system + idle;
                let idle_total = idle;

                if total > 0 {
                    cpu_load = ((total - idle_total) as f64 / total as f64) * 100.0;
                }
            }
        }
    }

    // Get memory information
    let mut channel2 = sess.channel_session()?;
    channel2.exec("cat /proc/meminfo")?;
    let mut mem_output = Vec::new();
    channel2.read_to_end(&mut mem_output)?;
    channel2.close()?;

    // Parse memory data
    let mem_output_str = String::from_utf8_lossy(&mem_output);

    let mut total_mem = 0u64;
    let mut free_mem = 0u64;

    for line in mem_output_str.lines() {
        if line.starts_with("MemTotal:") {
            total_mem = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            free_mem = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0);
        }
    }

    let used_mem = total_mem.max(free_mem) - free_mem; // Ensure we don't have negative values
    let memory_percentage = if total_mem > 0 {
        (used_mem as f64 / total_mem as f64) * 100.0
    } else {
        0.0
    };

    // Try to get GPU information using nvidia-smi if available
    let mut gpus = Vec::new();
    let mut channel3 = sess.channel_session()?;
    channel3.exec("nvidia-smi --query-gpu=utilization.gpu,memory.total,memory.used --format=csv,noheader,nounits")?;
    let mut gpu_output = Vec::new();
    channel3.read_to_end(&mut gpu_output)?;
    channel3.close()?;

    // Parse GPU output
    let gpu_output_str = String::from_utf8_lossy(&gpu_output);

    if !gpu_output_str.is_empty() {
        for line in gpu_output_str.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3 {
                // Parse GPU utilization, memory total, and memory used
                let gpu_utilization = parts[0].trim().parse::<i32>().unwrap_or(0);
                let memory_total = parts[1].trim().parse::<u64>().unwrap_or(0);
                let memory_used = parts[2].trim().parse::<u64>().unwrap_or(0);

                // Calculate memory percentage
                let memory_load = if memory_total > 0 {
                    ((memory_used as f64 / memory_total as f64) * 100.0) as i32
                } else {
                    0
                };

                // Store both values - we'll need both for proper display
                gpus.push(GpuInfo {
                    gpu_load: gpu_utilization,
                    memory_load,
                });
            }
        }
    }

    Ok(SystemInfo {
        cpu: CpuInfo {
            load: cpu_load,
            memory_percentage,
        },
        gpus,
    })
}

/// Color functions for different levels
fn get_color(level: i32, no_color: bool) -> String {
    if no_color {
        return String::new();
    }

    match level {
        0..=40 => "\x1b[32m".to_string(),  // Green
        41..=80 => "\x1b[33m".to_string(), // Orange/Yellow
        _ => "\x1b[31m".to_string(),       // Red
    }
}

fn reset_color(no_color: bool) -> String {
    if no_color {
        String::new()
    } else {
        "\x1b[0m".to_string()
    }
}

/// Format value with appropriate color based on percentage
fn format_value(value: f64, no_color: bool) -> String {
    let level = value as i32;
    let color = get_color(level, no_color);
    let reset = reset_color(no_color);
    format!("{}{:.0}{}", color, value, reset)
}

/// Display information in minimal format
fn display_format(sys_info: &SystemInfo, args: &Args) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    // Simple format: CPU Load CPU Mem
    println!(
        "CPU {}  {}",
        format_value(sys_info.cpu.load, args.no_color),
        format_value(sys_info.cpu.memory_percentage, args.no_color)
    );

    // Simple format: GPU Load GPU Mem (showing both load and memory percentage)
    if !sys_info.gpus.is_empty() {
        let mut total_gpu_load = 0i32;
        let mut total_memory_load = 0i32;
        let mut gpu_count = 0i32;

        for gpu in &sys_info.gpus {
            total_gpu_load += gpu.gpu_load;
            total_memory_load += gpu.memory_load;
            gpu_count += 1;
        }

        let avg_gpu_load = total_gpu_load as f64 / gpu_count as f64;
        let avg_memory_load = total_memory_load as f64 / gpu_count as f64;

        // Show both GPU load and memory percentage like CPU
        println!(
            "GPU {} {}",
            format_value(avg_gpu_load, args.no_color),
            format_value(avg_memory_load, args.no_color)
        );
    } else {
        println!("GPU 0  0");
    }
}

fn main() {
    let args = Args::parse();

    if let Some(ssh_host) = &args.ssh_host {
        println!(
            "Connecting to remote host {} as user {}...",
            ssh_host, args.ssh_user
        );
        println!("Starting GPU/CPU Monitor ({}s refresh)...", args.refresh);
        println!("Use Ctrl+C to exit");

        loop {
            // For now, just return empty system info for remote connection
            // In a real implementation we would establish an SSH connection here
            let sys_info = get_remote_system_info(ssh_host, &args.ssh_user, args.ssh_port)
                .unwrap_or(SystemInfo {
                    cpu: CpuInfo {
                        load: 0.0,
                        memory_percentage: 0.0,
                    },
                    gpus: vec![],
                });

            // Display information
            display_format(&sys_info, &args);

            // Wait for the specified interval
            thread::sleep(Duration::from_secs(args.refresh));
        }
    } else {
        println!("Starting GPU/CPU Monitor ({}s refresh)...", args.refresh);
        println!("Use Ctrl+C to exit");

        loop {
            // Get system information
            let cpu_info = get_cpu_info();
            let gpus = get_gpu_info();
            let sys_info = SystemInfo {
                cpu: cpu_info,
                gpus,
            };

            // Display information in a table format
            display_format(&sys_info, &args);

            // Wait for the specified interval
            thread::sleep(Duration::from_secs(args.refresh));
        }
    }
}
