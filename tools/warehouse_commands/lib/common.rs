#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum WarehouseStore {
    Memory,
    Mmap,
    Stream,
}

impl WarehouseStore {
    pub fn as_str(&self) -> &'static str {
        match self {
            WarehouseStore::Memory => "memory",
            WarehouseStore::Mmap => "mmap",
            WarehouseStore::Stream => "stream",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum ModelKind {
    Tiny,
    Big,
}

impl ModelKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelKind::Tiny => "tiny",
            ModelKind::Big => "big",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CmdConfig<'a> {
    pub manifest: &'a str,
    pub store: WarehouseStore,
    pub prefetch: Option<usize>, // used only when store == Stream
    pub model: ModelKind,
    pub batch_size: usize,
    pub log_every: usize,
    pub wgpu_backend: &'a str,
    pub wgpu_adapter: Option<&'a str>,
    pub extra_args: &'a str,
}

#[allow(dead_code)]
pub const DEFAULT_CONFIG: CmdConfig<'static> = CmdConfig {
    manifest: "artifacts/tensor_warehouse/v<version>/manifest.json",
    store: WarehouseStore::Stream,
    prefetch: Some(8),
    model: ModelKind::Big,
    batch_size: 32,
    log_every: 1,
    wgpu_backend: "vulkan",
    wgpu_adapter: None,
    extra_args: "",
};
