use std::{borrow::Cow, fmt};

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

impl fmt::Display for WarehouseStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
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

impl fmt::Display for ModelKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CmdConfig<'a> {
    pub manifest: Cow<'a, str>,
    pub store: WarehouseStore,
    pub prefetch: Option<usize>, // used only when store == Stream
    pub model: ModelKind,
    pub batch_size: usize,
    pub log_every: usize,
    pub wgpu_backend: Cow<'a, str>,
    pub wgpu_adapter: Option<Cow<'a, str>>,
    pub extra_args: Cow<'a, str>,
}

impl<'a> CmdConfig<'a> {
    pub fn with_manifest<T: Into<Cow<'a, str>>>(mut self, manifest: T) -> Self {
        self.manifest = manifest.into();
        self
    }

    pub fn with_store(mut self, store: WarehouseStore) -> Self {
        self.store = store;
        self
    }

    pub fn with_prefetch(mut self, prefetch: Option<usize>) -> Self {
        self.prefetch = prefetch;
        self
    }

    pub fn with_model(mut self, model: ModelKind) -> Self {
        self.model = model;
        self
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn with_log_every(mut self, log_every: usize) -> Self {
        self.log_every = log_every;
        self
    }

    pub fn with_backend<T: Into<Cow<'a, str>>>(mut self, backend: T) -> Self {
        self.wgpu_backend = backend.into();
        self
    }

    pub fn with_adapter<T: Into<Cow<'a, str>>>(mut self, adapter: T) -> Self {
        self.wgpu_adapter = Some(adapter.into());
        self
    }

    pub fn with_extra_args<T: Into<Cow<'a, str>>>(mut self, extra_args: T) -> Self {
        self.extra_args = extra_args.into();
        self
    }
}

#[allow(dead_code)]
pub const DEFAULT_CONFIG: CmdConfig<'static> = CmdConfig {
    manifest: Cow::Borrowed("artifacts/tensor_warehouse/v<version>/manifest.json"),
    store: WarehouseStore::Stream,
    prefetch: Some(8),
    model: ModelKind::Big,
    batch_size: 32,
    log_every: 1,
    wgpu_backend: Cow::Borrowed("vulkan"),
    wgpu_adapter: None,
    extra_args: Cow::Borrowed(""),
};

impl Default for CmdConfig<'_> {
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}
