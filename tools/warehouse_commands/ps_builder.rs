use super::common::{CmdConfig, WarehouseStore};

pub fn build_ps_command(cfg: &CmdConfig<'_>) -> String {
    let mut env_parts = Vec::new();
    env_parts.push(format!(
        "$env:TENSOR_WAREHOUSE_MANIFEST=\"{}\"",
        cfg.manifest
    ));
    env_parts.push(format!("$env:WAREHOUSE_STORE=\"{}\"", cfg.store.as_str()));
    if matches!(cfg.store, WarehouseStore::Stream) {
        let depth = cfg.prefetch.unwrap_or(2);
        env_parts.push(format!("$env:WAREHOUSE_PREFETCH=\"{}\"", depth));
    }
    env_parts.push("$env:WGPU_POWER_PREF=\"high-performance\"".into());
    env_parts.push("$env:RUST_LOG=\"info,wgpu_core=info\"".into());

    let mut cmd_parts = Vec::new();
    cmd_parts.push("cargo train_hp".to_string());
    cmd_parts.push(format!("--model {}", cfg.model.as_str()));
    cmd_parts.push(format!("--batch-size {}", cfg.batch_size));
    cmd_parts.push(format!("--log-every {}", cfg.log_every));
    if !cfg.extra_args.trim().is_empty() {
        cmd_parts.push(cfg.extra_args.trim().to_string());
    }

    format!("{}; {}", env_parts.join("; "), cmd_parts.join(" "))
}
