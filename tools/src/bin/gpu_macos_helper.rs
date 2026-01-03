//! Minimal macOS GPU probe helper.
//!
//! Prints a JSON payload consumed by datagen_scheduler:
//! {
//!   "available": true|false,
//!   "utilization": <f64|null>,
//!   "mem_used_mb": <u64|null>
//! }
//! On non-macOS, returns available:false.

use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    available: bool,
    utilization: Option<f64>,
    mem_used_mb: Option<u64>,
}

fn main() {
    #[cfg(target_os = "macos")]
    {
        // TODO: replace with real Metal/GPU stats. For now report availability with zero utilization.
        let payload = Payload {
            available: true,
            utilization: Some(0.0),
            mem_used_mb: None,
        };
        let _ = serde_json::to_writer(std::io::stdout(), &payload);
        return;
    }

    #[cfg(not(target_os = "macos"))]
    {
        let payload = Payload {
            available: false,
            utilization: None,
            mem_used_mb: None,
        };
        let _ = serde_json::to_writer(std::io::stdout(), &payload);
    }
}
