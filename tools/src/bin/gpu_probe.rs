//! Cross-platform GPU probe helper.
//!
//! Prints a JSON payload consumed by datagen_scheduler.

fn main() {
    use cortenforge_tools::gpu_probe::{platform_probe, write_status_json};

    let args: Vec<String> = std::env::args().collect();
    let mut format = "json".to_string();
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if let Some(value) = arg.strip_prefix("--format=") {
            format = value.to_string();
        } else if arg == "--format" {
            if let Some(next) = iter.next() {
                format = next.clone();
            }
        }
    }

    if format != "json" {
        eprintln!("unsupported format: {format} (supported: json)");
        std::process::exit(2);
    }

    let probe = platform_probe();
    let status = probe.status();
    let _ = write_status_json(&status);
}
