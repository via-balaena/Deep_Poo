//! Bash one-liner generator for AMD / Linux/macOS (Vulkan/Metal).
//! Run: `cargo run --bin warehouse_amd_sh_command`.

#[path = "../lib/common.rs"]
mod common;
#[path = "../lib/sh_builder.rs"]
mod sh_builder;

use common::{CmdConfig, ModelKind, WarehouseStore};
use sh_builder::build_bash_command;

const CONFIG: CmdConfig = CmdConfig {
    manifest: "artifacts/tensor_warehouse/v<version>/manifest.json",
    store: WarehouseStore::Stream,
    prefetch: Some(8),
    model: ModelKind::Big,
    batch_size: 32,
    log_every: 1,
    wgpu_backend: "vulkan",
    wgpu_adapter: Some("AMD"),
    extra_args: "",
};

fn main() {
    let cmd = build_bash_command(&CONFIG);
    println!("{cmd}");
}
