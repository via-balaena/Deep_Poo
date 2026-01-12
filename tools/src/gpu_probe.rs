use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GpuStatus {
    pub available: bool,
    pub utilization: Option<f64>,
    pub mem_used_mb: Option<u64>,
    pub vendor: Option<String>,
    pub device_name: Option<String>,
}

impl GpuStatus {
    pub fn unavailable() -> Self {
        Self {
            available: false,
            utilization: None,
            mem_used_mb: None,
            vendor: None,
            device_name: None,
        }
    }
}

pub trait GpuProbe {
    fn status(&self) -> GpuStatus;
}

pub struct MacGpuProbe;

#[cfg(feature = "gpu-nvidia")]
pub struct NvidiaGpuProbe;

pub struct FallbackGpuProbe;

#[cfg(target_os = "linux")]
pub struct LinuxGpuProbe;

#[cfg(target_os = "windows")]
pub struct WindowsGpuProbe;

#[cfg(all(target_os = "windows", feature = "gpu-windows"))]
pub struct AmdWindowsProbe;

impl GpuProbe for MacGpuProbe {
    fn status(&self) -> GpuStatus {
        #[cfg(target_os = "macos")]
        {
            // TODO: replace with real Metal/GPU stats. For now report availability with zero utilization.
            GpuStatus {
                available: true,
                utilization: Some(0.0),
                mem_used_mb: None,
                vendor: None,
                device_name: None,
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            GpuStatus::unavailable()
        }
    }
}

#[cfg(feature = "gpu-nvidia")]
impl GpuProbe for NvidiaGpuProbe {
    fn status(&self) -> GpuStatus {
        use nvml_wrapper::Nvml;

        let nvml = match Nvml::init() {
            Ok(nvml) => nvml,
            Err(_) => return GpuStatus::unavailable(),
        };
        let device = match nvml.device_by_index(0) {
            Ok(device) => device,
            Err(_) => return GpuStatus::unavailable(),
        };

        let utilization = device
            .utilization_rates()
            .ok()
            .map(|rates| rates.gpu as f64);
        let mem_used_mb = device
            .memory_info()
            .ok()
            .map(|mem| mem.used / (1024 * 1024));

        GpuStatus {
            available: true,
            utilization,
            mem_used_mb,
            vendor: Some("NVIDIA".to_string()),
            device_name: device.name().ok(),
        }
    }
}

impl GpuProbe for FallbackGpuProbe {
    fn status(&self) -> GpuStatus {
        GpuStatus::unavailable()
    }
}

#[cfg(target_os = "linux")]
impl GpuProbe for LinuxGpuProbe {
    fn status(&self) -> GpuStatus {
        #[cfg(feature = "gpu-nvidia")]
        {
            let status = NvidiaGpuProbe.status();
            if status.available {
                return status;
            }
        }

        if let Some(status) = nvidia_smi_status() {
            return status;
        }
        if let Some(status) = amd_status() {
            return status;
        }
        if let Some(status) = intel_status() {
            return status;
        }

        GpuStatus::unavailable()
    }
}

#[cfg(target_os = "windows")]
impl GpuProbe for WindowsGpuProbe {
    fn status(&self) -> GpuStatus {
        #[cfg(feature = "gpu-nvidia")]
        {
            return NvidiaGpuProbe.status();
        }

        #[cfg(all(not(feature = "gpu-nvidia"), feature = "gpu-windows"))]
        {
            return AmdWindowsProbe.status();
        }

        #[cfg(all(not(feature = "gpu-nvidia"), not(feature = "gpu-windows")))]
        {
            return GpuStatus::unavailable();
        }
    }
}

#[cfg(all(target_os = "windows", feature = "gpu-windows"))]
impl GpuProbe for AmdWindowsProbe {
    fn status(&self) -> GpuStatus {
        // TODO: replace with a Windows WMI/DirectX implementation.
        if let Some(status) = windows_wmi_status() {
            return status;
        }
        GpuStatus::unavailable()
    }
}

#[cfg(target_os = "windows")]
fn windows_wmi_status() -> Option<GpuStatus> {
    None
}

pub fn platform_probe() -> Box<dyn GpuProbe> {
    #[cfg(target_os = "windows")]
    {
        return Box::new(WindowsGpuProbe);
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxGpuProbe)
    }

    #[cfg(all(
        feature = "gpu-nvidia",
        not(target_os = "windows"),
        not(target_os = "linux"),
        not(target_os = "macos")
    ))]
    {
        return Box::new(NvidiaGpuProbe);
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(MacGpuProbe)
    }

    #[cfg(all(
        not(target_os = "macos"),
        not(target_os = "windows"),
        not(target_os = "linux"),
        not(feature = "gpu-nvidia")
    ))]
    {
        return Box::new(FallbackGpuProbe);
    }
}

#[cfg(target_os = "linux")]
fn nvidia_smi_status() -> Option<GpuStatus> {
    use std::process::Command;

    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,utilization.gpu,memory.used",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&output.stdout);
    let mut parts = line.lines().next()?.split(',');
    let name = parts.next()?.trim().to_string();
    let utilization = parts.next().and_then(|v| v.trim().parse::<f64>().ok());
    let mem_used_mb = parts.next().and_then(|v| v.trim().parse::<u64>().ok());

    Some(GpuStatus {
        available: true,
        utilization,
        mem_used_mb,
        vendor: Some("NVIDIA".to_string()),
        device_name: Some(name),
    })
}

#[cfg(target_os = "linux")]
fn amd_status() -> Option<GpuStatus> {
    use std::process::Command;

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
                    return Some(GpuStatus {
                        available: true,
                        utilization: Some(pct),
                        mem_used_mb: amd_mem_used_mb(),
                        vendor: Some("AMD".to_string()),
                        device_name: None,
                    });
                }
            }
        }
    }

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
                .and_then(|n| n.parse::<f64>().ok())
            {
                return Some(GpuStatus {
                    available: true,
                    utilization: Some(pct),
                    mem_used_mb: amd_mem_used_mb(),
                    vendor: Some("AMD".to_string()),
                    device_name: None,
                });
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn amd_mem_used_mb() -> Option<u64> {
    use std::process::Command;

    if let Ok(output) = Command::new("rocm-smi")
        .args(["--showmeminfo", "vram", "--json"])
        .output()
    {
        if output.status.success() {
            if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
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

#[cfg(target_os = "linux")]
fn intel_status() -> Option<GpuStatus> {
    use std::process::Command;

    let output = Command::new("intel_gpu_top").arg("--json").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mem_used_mb = intel_mem_json(&output.stdout).or_else(|| intel_mem_text(&text));
    for line in text.lines() {
        if line.to_ascii_lowercase().contains("render/3d") {
            if let Some(pct) = line
                .split('%')
                .next()
                .and_then(|s| s.split_whitespace().last())
                .and_then(|n| n.parse::<f64>().ok())
            {
                return Some(GpuStatus {
                    available: true,
                    utilization: Some(pct),
                    mem_used_mb,
                    vendor: Some("Intel".to_string()),
                    device_name: None,
                });
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn intel_mem_json(data: &[u8]) -> Option<u64> {
    let val: serde_json::Value = serde_json::from_slice(data).ok()?;
    find_mem_value(&val)
}

#[cfg(target_os = "linux")]
fn find_mem_value(val: &serde_json::Value) -> Option<u64> {
    match val {
        serde_json::Value::Number(n) => n.as_u64(),
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let key = k.to_ascii_lowercase();
                if key.contains("mem") {
                    if let Some(n) = v.as_u64() {
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

#[cfg(target_os = "linux")]
fn intel_mem_text(text: &str) -> Option<u64> {
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

pub fn to_json_value(status: &GpuStatus) -> serde_json::Value {
    serde_json::to_value(status).unwrap_or(serde_json::Value::Null)
}

pub fn write_status_json(status: &GpuStatus) -> Result<(), serde_json::Error> {
    serde_json::to_writer(std::io::stdout(), status)
}

#[cfg(test)]
mod tests {
    use super::{to_json_value, FallbackGpuProbe, GpuProbe, GpuStatus};

    struct MockProbe {
        status: GpuStatus,
    }

    impl GpuProbe for MockProbe {
        fn status(&self) -> GpuStatus {
            self.status.clone()
        }
    }

    #[test]
    fn status_serializes_with_expected_keys() {
        let status = GpuStatus::unavailable();
        let value = to_json_value(&status);
        let obj = value.as_object().expect("status must be a JSON object");
        for key in [
            "available",
            "utilization",
            "mem_used_mb",
            "vendor",
            "device_name",
        ] {
            assert!(obj.contains_key(key), "missing key: {key}");
        }
    }

    #[test]
    fn fallback_reports_unavailable() {
        let status = FallbackGpuProbe.status();
        assert!(!status.available);
        assert!(status.utilization.is_none());
        assert!(status.mem_used_mb.is_none());
    }

    #[test]
    fn mock_probe_serializes_full_status() {
        let probe = MockProbe {
            status: GpuStatus {
                available: true,
                utilization: Some(55.5),
                mem_used_mb: Some(2048),
                vendor: Some("MockGPU".to_string()),
                device_name: Some("MockDevice".to_string()),
            },
        };

        let value = to_json_value(&probe.status());
        let obj = value.as_object().expect("status must be a JSON object");
        assert_eq!(obj.get("available").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(obj.get("utilization").and_then(|v| v.as_f64()), Some(55.5));
        assert_eq!(obj.get("mem_used_mb").and_then(|v| v.as_u64()), Some(2048));
        assert_eq!(obj.get("vendor").and_then(|v| v.as_str()), Some("MockGPU"));
        assert_eq!(
            obj.get("device_name").and_then(|v| v.as_str()),
            Some("MockDevice")
        );
    }
}
