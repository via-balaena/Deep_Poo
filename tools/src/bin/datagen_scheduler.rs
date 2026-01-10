use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

use clap::Parser;
use cortenforge_tools::services::{self, DatagenOptions};
use cortenforge_tools::ToolConfig;
#[cfg(target_os = "macos")]
use serde::Deserialize;
use sysinfo::System;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Lightweight datagen runner with resource guards",
    long_about = None
)]
struct Args {
    /// Number of datagen runs to launch.
    #[arg(long, default_value_t = 1)]
    count: usize,
    /// Max concurrent runs.
    #[arg(long, default_value_t = 1)]
    concurrency: usize,
    /// Max global CPU usage percent before starting a new run.
    #[arg(long, default_value_t = 85.0)]
    max_cpu: f32,
    /// Minimum free memory (MB) required to start a new run.
    #[arg(long, default_value_t = 2048)]
    min_free_mem_mb: u64,
    /// Seconds between resource checks.
    #[arg(long, default_value_t = 5)]
    poll_secs: u64,
    /// Optional max GPU utilization percent (requires GPU probe).
    #[arg(long)]
    max_gpu: Option<f32>,
    /// Optional max GPU memory (MB) (requires GPU probe).
    #[arg(long)]
    max_gpu_mem_mb: Option<u64>,
    /// Output root for captures.
    #[arg(long)]
    output_root: Option<PathBuf>,
    /// Optional output root for pruned runs (defaults to <output_root>_filtered).
    #[arg(long)]
    prune_output_root: Option<PathBuf>,
    /// Optional seed; if set, each run adds the run index to avoid collisions.
    #[arg(long)]
    seed: Option<u64>,
    /// Optional frame cap per run.
    #[arg(long)]
    max_frames: Option<u32>,
    /// Hide window / run headless.
    #[arg(long, default_value_t = true)]
    headless: bool,
}

struct Running {
    pid: u32,
}

#[derive(Debug, Clone, Copy)]
enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    #[cfg(target_os = "macos")]
    Apple,
}

#[derive(Debug, Clone, Copy)]
struct GpuStats {
    utilization: f32,
    mem_used_mb: Option<u64>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut sys = System::new_all();
    let cfg = ToolConfig::load();
    let mut launched = 0usize;
    let mut running: Vec<Running> = Vec::new();
    let output_root = args
        .output_root
        .clone()
        .unwrap_or_else(|| cfg.captures_root.clone());
    let prune_root = args
        .prune_output_root
        .clone()
        .or_else(|| Some(cfg.captures_filtered_root.clone()))
        .or_else(|| default_prune_root(&output_root));
    let poll = Duration::from_secs(args.poll_secs);
    let gpu_vendor = detect_gpu_vendor();
    log_gpu_probe(gpu_vendor);
    if gpu_vendor.is_none() && (args.max_gpu.is_some() || args.max_gpu_mem_mb.is_some()) {
        println!(
            "GPU probe unavailable; max_gpu/max_gpu_mem_mb will be ignored unless a probe is found."
        );
    }

    while launched < args.count || !running.is_empty() {
        // Drop finished
        running.retain(|r| services::is_process_running(r.pid));

        // Try to launch new runs if slots and resources allow
        while launched < args.count
            && running.len() < args.concurrency
            && resources_ok(
                &mut sys,
                args.max_cpu,
                args.min_free_mem_mb,
                gpu_vendor,
                args.max_gpu,
                args.max_gpu_mem_mb,
            )
        {
            let run_idx = launched as u64;
            let opts = DatagenOptions {
                output_root: output_root.clone(),
                seed: args.seed.map(|s| s + run_idx),
                max_frames: args.max_frames,
                headless: args.headless,
                prune_empty: true,
                prune_output_root: prune_root.clone(),
            };
            match services::datagen_command(&opts).and_then(|cmd| services::spawn(&cmd)) {
                Ok(child) => {
                    println!(
                        "[{:>2}/{:>2}] launched pid={} output_root={} prune_root={}",
                        launched + 1,
                        args.count,
                        child.id(),
                        output_root.display(),
                        prune_root
                            .as_ref()
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|| "<auto>".to_string())
                    );
                    running.push(Running { pid: child.id() });
                    launched += 1;
                }
                Err(err) => {
                    eprintln!("spawn failed: {err}");
                    break;
                }
            }
        }

        if launched >= args.count && running.is_empty() {
            break;
        }
        thread::sleep(poll);
    }

    println!(
        "All runs complete (launched {}, active {}).",
        launched,
        running.len()
    );
    Ok(())
}

fn resources_ok(
    sys: &mut System,
    max_cpu: f32,
    min_free_mem_mb: u64,
    gpu_vendor: Option<GpuVendor>,
    max_gpu: Option<f32>,
    max_gpu_mem_mb: Option<u64>,
) -> bool {
    sys.refresh_cpu_all();
    sys.refresh_memory();
    let cpu = sys.global_cpu_usage();
    let free_mb = sys.available_memory() / 1024 / 1024;
    if cpu > max_cpu {
        return false;
    }
    if free_mb < min_free_mem_mb {
        return false;
    }
    if let (Some(vendor), Some(max_gpu_thresh)) = (gpu_vendor, max_gpu) {
        if let Some(stats) = sample_gpu(vendor) {
            if stats.utilization > max_gpu_thresh {
                return false;
            }
            if let (Some(mem_used), Some(max_mem)) = (stats.mem_used_mb, max_gpu_mem_mb) {
                if mem_used > max_mem {
                    return false;
                }
            }
        }
    }
    true
}

fn default_prune_root(output_root: &Path) -> Option<PathBuf> {
    let mut base = output_root.to_path_buf();
    let suffix = base
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| format!("{s}_filtered"))
        .unwrap_or_else(|| "captures_filtered".to_string());
    base.set_file_name(suffix);
    Some(base)
}

fn detect_gpu_vendor() -> Option<GpuVendor> {
    // Optional macOS helper probe; returns Apple if the probe yields stats.
    #[cfg(target_os = "macos")]
    if sample_apple_helper().is_some() {
        return Some(GpuVendor::Apple);
    }
    // Prefer NVIDIA: use presence of `nvidia-smi` as a quick probe.
    let probe = Command::new("nvidia-smi")
        .arg("--query-gpu=name")
        .arg("--format=csv,noheader")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .ok()?;
    if probe.success() {
        return Some(GpuVendor::Nvidia);
    }
    // Else AMD: try rocm-smi or radeontop.
    let amd_rocm = Command::new("rocm-smi")
        .arg("--showuse")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    if amd_rocm.as_ref().is_ok_and(|s| s.success()) {
        return Some(GpuVendor::Amd);
    }
    let amd_radeontop = Command::new("radeontop")
        .arg("--help")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    if amd_radeontop.as_ref().is_ok_and(|s| s.success()) {
        return Some(GpuVendor::Amd);
    }
    // Else Intel: linux-only probe via intel_gpu_top.
    let intel = Command::new("intel_gpu_top")
        .arg("--help")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    if intel.as_ref().is_ok_and(|s| s.success()) {
        return Some(GpuVendor::Intel);
    }
    None
}

fn log_gpu_probe(v: Option<GpuVendor>) {
    match v {
        Some(GpuVendor::Nvidia) => println!("GPU probe: detected NVIDIA via nvidia-smi"),
        Some(GpuVendor::Amd) => println!("GPU probe: detected AMD via rocm-smi/radeontop"),
        Some(GpuVendor::Intel) => println!("GPU probe: detected Intel via intel_gpu_top"),
        #[cfg(target_os = "macos")]
        Some(GpuVendor::Apple) => println!("GPU probe: detected Apple via helper"),
        None => println!("GPU probe: none detected; GPU gating disabled"),
    }
}

fn sample_gpu(vendor: GpuVendor) -> Option<GpuStats> {
    match vendor {
        GpuVendor::Nvidia => sample_nvidia(),
        GpuVendor::Amd => sample_amd(),
        GpuVendor::Intel => sample_intel(),
        #[cfg(target_os = "macos")]
        GpuVendor::Apple => sample_apple_helper(),
    }
}

fn sample_nvidia() -> Option<GpuStats> {
    // Prefer NVML if available under the gpu_nvidia feature; otherwise fall back to nvidia-smi.
    #[cfg(feature = "gpu_nvidia")]
    {
        if let Some(stats) = sample_nvidia_nvml() {
            return Some(stats);
        }
    }
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=utilization.gpu,memory.used",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&output.stdout);
    let mut parts = line.trim().split(',');
    let util = parts.next()?.trim().parse::<f32>().ok()?;
    let mem = parts.next().and_then(|m| m.trim().parse::<u64>().ok());
    Some(GpuStats {
        utilization: util,
        mem_used_mb: mem,
    })
}

#[cfg(feature = "gpu_nvidia")]
fn sample_nvidia_nvml() -> Option<GpuStats> {
    use nvml_wrapper::Nvml;
    let nvml = Nvml::init().ok()?;
    let device = nvml.device_by_index(0).ok()?;
    let util = device.utilization_rates().ok()?;
    let mem = device.memory_info().ok()?;
    Some(GpuStats {
        utilization: util.gpu as f32,
        mem_used_mb: Some(mem.used / (1024 * 1024)),
    })
}

fn sample_amd() -> Option<GpuStats> {
    // Try rocm-smi with JSON first for a cleaner parse.
    if let Ok(output) = Command::new("rocm-smi")
        .args(["--showuse", "--json"])
        .output()
    {
        if output.status.success() {
            if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                if let Some(pct) = val
                    .get("card")
                    .and_then(|c| c.get(0))
                    .and_then(|c| c.get("GPU use (%)"))
                    .and_then(|v| v.as_f64())
                {
                    return Some(GpuStats {
                        utilization: pct as f32,
                        mem_used_mb: sample_amd_mem(),
                    });
                }
            }
        }
    }
    // Fallback to text parse.
    let output = Command::new("rocm-smi")
        .arg("--showuse")
        .output()
        .or_else(|_| Command::new("radeontop").arg("--help").output())
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if line.to_ascii_lowercase().contains("gpu use") {
            if let Some(pct) = line
                .split('%')
                .next()
                .and_then(|s| s.split_whitespace().last())
                .and_then(|n| n.parse::<f32>().ok())
            {
                return Some(GpuStats {
                    utilization: pct,
                    mem_used_mb: sample_amd_mem(),
                });
            }
        }
    }
    None
}

fn sample_amd_mem() -> Option<u64> {
    // Try rocm-smi --showmeminfo for VRAM usage.
    if let Ok(output) = Command::new("rocm-smi")
        .args(["--showmeminfo", "vram", "--json"])
        .output()
    {
        if output.status.success() {
            if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                // Example path may be card[0].vram["used (B)"]
                if let Some(used) = val
                    .get("card")
                    .and_then(|c| c.get(0))
                    .and_then(|c| c.get("vram"))
                    .and_then(|v| v.get("used (B)"))
                    .and_then(|v| v.as_u64())
                {
                    return Some(used / (1024 * 1024));
                }
            }
        }
    }
    None
}

fn sample_intel() -> Option<GpuStats> {
    let output = Command::new("intel_gpu_top").arg("--json").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mem = sample_intel_mem_json(&output.stdout).or_else(|| sample_intel_mem_text(&text));
    for line in text.lines() {
        if line.to_ascii_lowercase().contains("render/3d") {
            if let Some(pct) = line
                .split('%')
                .next()
                .and_then(|s| s.split_whitespace().last())
                .and_then(|n| n.parse::<f32>().ok())
            {
                return Some(GpuStats {
                    utilization: pct,
                    mem_used_mb: mem,
                });
            }
        }
    }
    None
}

fn sample_intel_mem_json(data: &[u8]) -> Option<u64> {
    let val: serde_json::Value = serde_json::from_slice(data).ok()?;
    find_mem_value(&val)
}

fn find_mem_value(val: &serde_json::Value) -> Option<u64> {
    match val {
        serde_json::Value::Number(n) => n.as_u64(),
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let key = k.to_ascii_lowercase();
                if key.contains("mem") {
                    if let Some(n) = v.as_u64() {
                        // Heuristic: assume bytes if large.
                        if n > 10_000 {
                            return Some(n / (1024 * 1024));
                        }
                        return Some(n);
                    }
                    if let Some(f) = v.as_f64() {
                        if f > 10_000.0 {
                            return Some((f / 1024.0 / 1024.0) as u64);
                        }
                        return Some(f as u64);
                    }
                }
                if let Some(found) = find_mem_value(v) {
                    return Some(found);
                }
            }
            None
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                if let Some(found) = find_mem_value(v) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}

fn sample_intel_mem_text(text: &str) -> Option<u64> {
    for line in text.lines() {
        let lower = line.to_ascii_lowercase();
        if lower.contains("mem") {
            if let Some(num) = line
                .split_whitespace()
                .map(|w| w.trim_end_matches(['%', 'm', 'M', 'b', 'B']))
                .filter_map(|w| w.parse::<f64>().ok())
                .next_back()
            {
                if num > 10_000.0 {
                    return Some((num / 1024.0 / 1024.0) as u64);
                }
                return Some(num as u64);
            }
        }
    }
    None
}

#[allow(dead_code)] // only used when wiring macOS GPU sampling paths
fn sample_apple_helper() -> Option<GpuStats> {
    #[cfg(target_os = "macos")]
    {
        let mut output = None;
        for cmd in ["gpu_probe", "gpu_macos_helper"] {
            if let Ok(attempt) = Command::new(cmd).output() {
                if attempt.status.success() {
                    output = Some(attempt);
                    break;
                }
            }
        }
        let output = output?;
        #[derive(Deserialize)]
        struct HelperPayload {
            available: bool,
            utilization: Option<f64>,
            mem_used_mb: Option<u64>,
        }
        let payload: HelperPayload = serde_json::from_slice(&output.stdout).ok()?;
        if payload.available {
            if let Some(u) = payload.utilization {
                return Some(GpuStats {
                    utilization: u as f32,
                    mem_used_mb: payload.mem_used_mb,
                });
            }
        }
    }
    None
}
