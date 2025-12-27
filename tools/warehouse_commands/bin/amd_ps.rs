//! PowerShell one-liner generator for AMD / Windows (DX12).
//! Run: `cargo run --bin warehouse_amd_ps_command`.

#[path = "../lib/common.rs"]
mod common;
#[path = "../lib/ps_builder.rs"]
mod ps_builder;

use common::{CmdConfig, ModelKind, WarehouseStore};
use ps_builder::build_ps_command;

const CONFIG: CmdConfig = CmdConfig {
    manifest: "artifacts/tensor_warehouse/v<version>/manifest.json",
    store: WarehouseStore::Stream,
    prefetch: Some(8),
    model: ModelKind::Big,
    batch_size: 32,
    log_every: 1,
    wgpu_backend: "dx12",
    wgpu_adapter: Some("AMD"),
    extra_args: "",
};

fn main() {
    let cmd = build_ps_command(&CONFIG);
    println!("{cmd}");
}
