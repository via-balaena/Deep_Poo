use cortenforge_tools::warehouse_commands::{
    builder::{build_command, Shell},
    common::{CmdConfig, WarehouseStore},
};

#[test]
fn powershell_amd_uses_default_template() {
    let cfg = CmdConfig::default()
        .with_backend("dx12")
        .with_adapter("AMD");
    let cmd = build_command(&cfg, Shell::PowerShell);
    let expected = "$env:TENSOR_WAREHOUSE_MANIFEST=\"artifacts/tensor_warehouse/v<version>/manifest.json\"; $env:WAREHOUSE_STORE=\"stream\"; $env:WAREHOUSE_PREFETCH=\"8\"; $env:WGPU_BACKEND=\"dx12\"; $env:WGPU_ADAPTER_NAME=\"AMD\"; $env:WGPU_POWER_PREF=\"high-performance\"; $env:RUST_LOG=\"trace,wgpu_core=trace,wgpu_hal=trace\"; cargo train_hp --model big --batch-size 32 --log-every 1";
    assert_eq!(cmd, expected);
}

#[test]
fn bash_amd_uses_default_template() {
    let cfg = CmdConfig::default().with_adapter("AMD");
    let cmd = build_command(&cfg, Shell::Bash);
    let expected = "TENSOR_WAREHOUSE_MANIFEST=\"artifacts/tensor_warehouse/v<version>/manifest.json\" WAREHOUSE_STORE=\"stream\" WAREHOUSE_PREFETCH=\"8\" WGPU_BACKEND=\"vulkan\" WGPU_ADAPTER_NAME=\"AMD\" WGPU_POWER_PREF=\"high-performance\" RUST_LOG=\"trace,wgpu_core=trace,wgpu_hal=trace\" cargo train_hp --model big --batch-size 32 --log-every 1";
    assert_eq!(cmd, expected);
}

#[test]
fn powershell_nvidia_uses_default_template() {
    let cfg = CmdConfig::default()
        .with_backend("dx12")
        .with_adapter("NVIDIA");
    let cmd = build_command(&cfg, Shell::PowerShell);
    let expected = "$env:TENSOR_WAREHOUSE_MANIFEST=\"artifacts/tensor_warehouse/v<version>/manifest.json\"; $env:WAREHOUSE_STORE=\"stream\"; $env:WAREHOUSE_PREFETCH=\"8\"; $env:WGPU_BACKEND=\"dx12\"; $env:WGPU_ADAPTER_NAME=\"NVIDIA\"; $env:WGPU_POWER_PREF=\"high-performance\"; $env:RUST_LOG=\"trace,wgpu_core=trace,wgpu_hal=trace\"; cargo train_hp --model big --batch-size 32 --log-every 1";
    assert_eq!(cmd, expected);
}

#[test]
fn bash_nvidia_uses_default_template() {
    let cfg = CmdConfig::default().with_adapter("NVIDIA");
    let cmd = build_command(&cfg, Shell::Bash);
    let expected = "TENSOR_WAREHOUSE_MANIFEST=\"artifacts/tensor_warehouse/v<version>/manifest.json\" WAREHOUSE_STORE=\"stream\" WAREHOUSE_PREFETCH=\"8\" WGPU_BACKEND=\"vulkan\" WGPU_ADAPTER_NAME=\"NVIDIA\" WGPU_POWER_PREF=\"high-performance\" RUST_LOG=\"trace,wgpu_core=trace,wgpu_hal=trace\" cargo train_hp --model big --batch-size 32 --log-every 1";
    assert_eq!(cmd, expected);
}

#[test]
fn non_stream_omits_prefetch_and_allows_extra_args() {
    let cfg = CmdConfig::default()
        .with_store(WarehouseStore::Memory)
        .with_prefetch(None)
        .with_extra_args("--dry-run");
    let cmd = build_command(&cfg, Shell::Bash);
    let expected = "TENSOR_WAREHOUSE_MANIFEST=\"artifacts/tensor_warehouse/v<version>/manifest.json\" WAREHOUSE_STORE=\"memory\" WGPU_BACKEND=\"vulkan\" WGPU_POWER_PREF=\"high-performance\" RUST_LOG=\"trace,wgpu_core=trace,wgpu_hal=trace\" cargo train_hp --model big --batch-size 32 --log-every 1 --dry-run";
    assert_eq!(cmd, expected);
}
