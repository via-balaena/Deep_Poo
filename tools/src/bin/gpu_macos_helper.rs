//! Minimal macOS GPU probe helper.
//!
//! Prints a JSON payload consumed by datagen_scheduler.
//! On non-macOS, returns available:false.
// TODO(v0.3+): remove this alias bin after a full release cycle on gpu_probe.

fn main() {
    use cortenforge_tools::gpu_probe::GpuProbe;

    let probe = cortenforge_tools::gpu_probe::MacGpuProbe;
    let status = probe.status();
    let _ = cortenforge_tools::gpu_probe::write_status_json(&status);
}
