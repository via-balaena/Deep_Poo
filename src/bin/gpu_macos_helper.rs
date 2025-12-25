use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Serialize)]
struct GpuSample {
    vendor: &'static str,
    available: bool,
    utilization: Option<f32>,
    mem_used_mb: Option<u64>,
    timestamp: f64,
    note: Option<String>,
}

/// Helper for macOS GPU probing.
///
/// Uses powermetrics for utilization and Metal for current allocated VRAM.
/// Falls back to an "unavailable" payload on failure.
fn main() {
    let sample = sample_gpu();
    println!(
        "{}",
        serde_json::to_string(&sample).unwrap_or_else(|_| "{}".into())
    );
}

fn sample_gpu() -> GpuSample {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    #[cfg(target_os = "macos")]
    {
        let util = sample_powermetrics();
        let mem = sample_metal_mem();
        if util.is_some() || mem.is_some() {
            return GpuSample {
                vendor: "apple",
                available: true,
                utilization: util,
                mem_used_mb: mem,
                timestamp: now,
                note: Some("powermetrics+metal sample".into()),
            };
        }
    }

    GpuSample {
        vendor: "apple",
        available: false,
        utilization: None,
        mem_used_mb: None,
        timestamp: now,
        note: Some("GPU stats unavailable".into()),
    }
}

#[cfg(target_os = "macos")]
fn sample_powermetrics() -> Option<f32> {
    // Attempt a single powermetrics sample; may require permissions on some systems.
    let output = Command::new("powermetrics")
        .args(["-n", "1", "-i", "1", "--samplers", "gpu_power"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    // Look for a line like "GPU Active residency: 12%"
    for line in text.lines() {
        if let Some(pos) = line.find("GPU Active") {
            // crude parse: grab trailing number before '%'
            let tail = &line[pos..];
            if let Some(percent_pos) = tail.find('%') {
                let num = tail[..percent_pos]
                    .rsplit_once(' ')
                    .map(|(_, n)| n.trim())
                    .or_else(|| tail[..percent_pos].split_whitespace().last())
                    .and_then(|n| n.parse::<f32>().ok());
                if let Some(p) = num {
                    return Some(p);
                }
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn sample_metal_mem() -> Option<u64> {
    let device = metal::Device::system_default()?;
    Some(device.current_allocated_size() / (1024 * 1024))
}
