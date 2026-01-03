use super::common::{CmdConfig, WarehouseStore};

#[derive(Clone, Copy)]
pub enum Shell {
    PowerShell,
    Bash,
}

impl Shell {
    fn env_kv(&self, key: &str, val: &str) -> String {
        match self {
            Shell::PowerShell => format!("$env:{key}=\"{val}\""),
            Shell::Bash => format!("{key}=\"{val}\""),
        }
    }

    fn separator(&self) -> &'static str {
        match self {
            Shell::PowerShell => "; ",
            Shell::Bash => " ",
        }
    }
}

#[allow(dead_code)]
pub fn build_command(cfg: &CmdConfig<'_>, shell: Shell) -> String {
    let mut env_parts = Vec::new();
    env_parts.push(shell.env_kv("TENSOR_WAREHOUSE_MANIFEST", cfg.manifest.as_ref()));
    env_parts.push(shell.env_kv("WAREHOUSE_STORE", cfg.store.as_str()));
    if matches!(cfg.store, WarehouseStore::Stream) {
        let depth = cfg.prefetch.unwrap_or(2);
        env_parts.push(shell.env_kv("WAREHOUSE_PREFETCH", &depth.to_string()));
    }
    env_parts.push(shell.env_kv("WGPU_BACKEND", cfg.wgpu_backend.as_ref()));
    if let Some(adapter) = &cfg.wgpu_adapter {
        env_parts.push(shell.env_kv("WGPU_ADAPTER_NAME", adapter.as_ref()));
    }
    env_parts.push(shell.env_kv("WGPU_POWER_PREF", "high-performance"));
    env_parts.push(shell.env_kv("RUST_LOG", "trace,wgpu_core=trace,wgpu_hal=trace"));

    let mut cmd_parts = Vec::new();
    cmd_parts.push("cargo train_hp".to_string());
    cmd_parts.push(format!("--model {}", cfg.model.as_str()));
    cmd_parts.push(format!("--batch-size {}", cfg.batch_size));
    cmd_parts.push(format!("--log-every {}", cfg.log_every));
    if !cfg.extra_args.as_ref().trim().is_empty() {
        cmd_parts.push(cfg.extra_args.as_ref().trim().to_string());
    }

    let sep = shell.separator();
    match shell {
        Shell::PowerShell => format!("{}; {}", env_parts.join(sep), cmd_parts.join(" ")),
        Shell::Bash => format!("{} {}", env_parts.join(sep), cmd_parts.join(" ")),
    }
}
