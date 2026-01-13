//! Warehouse manifest and shard storage backends.

#[cfg(feature = "burn-runtime")]
use crate::batch::BurnBatch;
#[cfg(feature = "burn-runtime")]
use crate::types::{BurnDatasetError, DatasetResult};
use crate::types::{
    CacheableTransformConfig, DatasetSummary, ResizeMode, ShardDType, ShardMetadata,
    ValidationThresholds, WarehouseStoreMode,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "burn-runtime")]
use crossbeam_channel::{bounded, Receiver};
#[cfg(feature = "burn-runtime")]
use memmap2::MmapOptions;
#[cfg(feature = "burn-runtime")]
use rand::prelude::SliceRandom;
#[cfg(feature = "burn-runtime")]
use rand::SeedableRng;
#[cfg(feature = "burn-runtime")]
use std::fs::File;
#[cfg(feature = "burn-runtime")]
use std::io::{BufReader, Read, Seek, SeekFrom};
#[cfg(feature = "burn-runtime")]
use std::thread;
#[cfg(feature = "burn-runtime")]
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseManifest {
    /// Source dataset root as a UTF-8 string.
    pub dataset_root: String,
    pub transform: CacheableTransformConfig,
    /// Warehouse version key (hex-encoded SHA256 of source + config tuple).
    pub version: String,
    /// Version recipe: sha256(dataset_root + cacheable_transform + max_boxes + skip_empty + code_version).
    pub version_recipe: String,
    /// Code version used in the key (crate version or VCS hash).
    pub code_version: String,
    /// Default shard dtype for this manifest.
    pub default_dtype: ShardDType,
    /// Default shard format version.
    pub default_shard_version: u32,
    pub created_at_ms: u64,
    pub shards: Vec<ShardMetadata>,
    pub summary: DatasetSummary,
    pub thresholds: ValidationThresholds,
}

impl WarehouseManifest {
    /// Default code version string (crate version).
    pub fn default_code_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Resolve code version with optional override (e.g., git hash).
    pub fn resolve_code_version() -> String {
        if let Ok(val) = std::env::var("CODE_VERSION") {
            if !val.trim().is_empty() {
                return val;
            }
        }
        Self::default_code_version()
    }

    /// Compute a canonical warehouse version (SHA256 hex) from inputs.
    pub fn compute_version(
        dataset_root: &Path,
        transform: &CacheableTransformConfig,
        skip_empty: bool,
        code_version: &str,
    ) -> String {
        #[derive(Serialize)]
        struct VersionTuple<'a> {
            dataset_root: &'a str,
            target_size: Option<(u32, u32)>,
            resize_mode: &'a ResizeMode,
            max_boxes: usize,
            skip_empty: bool,
            code_version: &'a str,
        }
        let tuple = VersionTuple {
            dataset_root: &dataset_root.display().to_string(),
            target_size: transform.target_size,
            resize_mode: &transform.resize_mode,
            max_boxes: transform.max_boxes,
            skip_empty,
            code_version,
        };
        let bytes = serde_json::to_vec(&tuple).unwrap_or_default();
        use sha2::Digest;
        let hash = sha2::Sha256::digest(bytes);
        format!("{:x}", hash)
    }

    pub fn save(&self, path: &Path) -> DatasetResult<()> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| BurnDatasetError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let data =
            serde_json::to_vec_pretty(self).map_err(|e| BurnDatasetError::Other(e.to_string()))?;
        fs::write(path, data).map_err(|e| BurnDatasetError::Io {
            path: path.to_path_buf(),
            source: e,
        })
    }

    pub fn load(path: &Path) -> DatasetResult<Self> {
        let raw = fs::read(path).map_err(|e| BurnDatasetError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        serde_json::from_slice(&raw).map_err(|e| BurnDatasetError::Json {
            path: path.to_path_buf(),
            source: e,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dataset_root: PathBuf,
        transform: CacheableTransformConfig,
        version: String,
        version_recipe: String,
        code_version: String,
        shards: Vec<ShardMetadata>,
        summary: DatasetSummary,
        thresholds: ValidationThresholds,
    ) -> Self {
        let created_at_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or_default();
        Self {
            dataset_root: dataset_root.display().to_string(),
            transform,
            version,
            version_recipe,
            code_version,
            default_dtype: ShardDType::F32,
            default_shard_version: 1,
            created_at_ms,
            shards,
            summary,
            thresholds,
        }
    }
}

struct ShardBuffer {
    samples: usize,
    width: u32,
    height: u32,
    max_boxes: usize,
    backing: ShardBacking,
}

#[cfg(feature = "burn-runtime")]
enum ShardBacking {
    Owned {
        images: Vec<f32>,
        boxes: Vec<f32>,
        masks: Vec<f32>,
    },
    Mmap {
        mmap: std::sync::Arc<memmap2::Mmap>,
        image_offset: usize,
        boxes_offset: usize,
        mask_offset: usize,
    },
    #[allow(dead_code)]
    Streamed {
        path: PathBuf,
        image_offset: usize,
        boxes_offset: usize,
        mask_offset: usize,
        samples: usize,
    }, // placeholder for future extensions
}

#[cfg(feature = "burn-runtime")]
impl ShardBuffer {
    fn copy_sample(
        &self,
        sample_idx: usize,
        out_images: &mut Vec<f32>,
        out_boxes: &mut Vec<f32>,
        out_masks: &mut Vec<f32>,
    ) -> DatasetResult<()> {
        let w = self.width as usize;
        let h = self.height as usize;
        let img_elems = 3 * w * h;
        let box_elems = self.max_boxes * 4;
        let mask_elems = self.max_boxes;
        match &self.backing {
            ShardBacking::Owned {
                images,
                boxes,
                masks,
            } => {
                let img_offset = sample_idx
                    .checked_mul(img_elems)
                    .ok_or_else(|| BurnDatasetError::Other("image offset overflow".into()))?;
                let box_offset = sample_idx
                    .checked_mul(box_elems)
                    .ok_or_else(|| BurnDatasetError::Other("box offset overflow".into()))?;
                let mask_offset = sample_idx
                    .checked_mul(mask_elems)
                    .ok_or_else(|| BurnDatasetError::Other("mask offset overflow".into()))?;
                out_images.extend_from_slice(&images[img_offset..img_offset + img_elems]);
                out_boxes.extend_from_slice(&boxes[box_offset..box_offset + box_elems]);
                out_masks.extend_from_slice(&masks[mask_offset..mask_offset + mask_elems]);
                Ok(())
            }
            ShardBacking::Mmap {
                mmap,
                image_offset,
                boxes_offset,
                mask_offset,
            } => {
                let img_bytes = img_elems
                    .checked_mul(std::mem::size_of::<f32>())
                    .ok_or_else(|| BurnDatasetError::Other("image byte size overflow".into()))?;
                let box_bytes = box_elems
                    .checked_mul(std::mem::size_of::<f32>())
                    .ok_or_else(|| BurnDatasetError::Other("box byte size overflow".into()))?;
                let mask_bytes = mask_elems
                    .checked_mul(std::mem::size_of::<f32>())
                    .ok_or_else(|| BurnDatasetError::Other("mask byte size overflow".into()))?;

                let img_start = image_offset
                    .checked_add(sample_idx * img_bytes)
                    .ok_or_else(|| BurnDatasetError::Other("image offset overflow".into()))?;
                let box_start = boxes_offset
                    .checked_add(sample_idx * box_bytes)
                    .ok_or_else(|| BurnDatasetError::Other("box offset overflow".into()))?;
                let mask_start = mask_offset
                    .checked_add(sample_idx * mask_bytes)
                    .ok_or_else(|| BurnDatasetError::Other("mask offset overflow".into()))?;

                if img_start + img_bytes > mmap.len()
                    || box_start + box_bytes > mmap.len()
                    || mask_start + mask_bytes > mmap.len()
                {
                    return Err(BurnDatasetError::Other(
                        "shard mmap truncated for requested sample".into(),
                    ));
                }

                for chunk in mmap[img_start..img_start + img_bytes].chunks_exact(4) {
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(chunk);
                    out_images.push(f32::from_le_bytes(arr));
                }
                for chunk in mmap[box_start..box_start + box_bytes].chunks_exact(4) {
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(chunk);
                    out_boxes.push(f32::from_le_bytes(arr));
                }
                for chunk in mmap[mask_start..mask_start + mask_bytes].chunks_exact(4) {
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(chunk);
                    out_masks.push(f32::from_le_bytes(arr));
                }
                Ok(())
            }
            ShardBacking::Streamed {
                path,
                image_offset,
                boxes_offset,
                mask_offset,
                samples,
            } => {
                if sample_idx >= *samples {
                    return Err(BurnDatasetError::Other(format!(
                        "sample {} out of range for {}",
                        sample_idx,
                        path.display()
                    )));
                }
                let img_elems = 3 * w * h;
                let box_elems = self.max_boxes * 4;
                let mask_elems = self.max_boxes;
                let img_bytes = img_elems * std::mem::size_of::<f32>();
                let box_bytes = box_elems * std::mem::size_of::<f32>();
                let mask_bytes = mask_elems * std::mem::size_of::<f32>();

                let img_start = image_offset
                    .checked_add(sample_idx * img_bytes)
                    .ok_or_else(|| BurnDatasetError::Other("image offset overflow".into()))?;
                let boxes_start = boxes_offset
                    .checked_add(sample_idx * box_bytes)
                    .ok_or_else(|| BurnDatasetError::Other("box offset overflow".into()))?;
                let mask_start = mask_offset
                    .checked_add(sample_idx * mask_bytes)
                    .ok_or_else(|| BurnDatasetError::Other("mask offset overflow".into()))?;

                let mut file =
                    BufReader::new(File::open(path).map_err(|e| BurnDatasetError::Io {
                        path: path.clone(),
                        source: e,
                    })?);

                fn read_f32s<R: Read + Seek>(
                    file: &mut R,
                    offset: usize,
                    bytes: usize,
                    out: &mut Vec<f32>,
                    path: &Path,
                ) -> DatasetResult<()> {
                    file.seek(SeekFrom::Start(offset as u64)).map_err(|e| {
                        BurnDatasetError::Io {
                            path: path.to_path_buf(),
                            source: e,
                        }
                    })?;
                    let mut buf = vec![0u8; bytes];
                    file.read_exact(&mut buf)
                        .map_err(|e| BurnDatasetError::Io {
                            path: path.to_path_buf(),
                            source: e,
                        })?;
                    for chunk in buf.chunks_exact(4) {
                        let mut arr = [0u8; 4];
                        arr.copy_from_slice(chunk);
                        out.push(f32::from_le_bytes(arr));
                    }
                    Ok(())
                }

                read_f32s(&mut file, img_start, img_bytes, out_images, path)?;
                read_f32s(&mut file, boxes_start, box_bytes, out_boxes, path)?;
                read_f32s(&mut file, mask_start, mask_bytes, out_masks, path)?;
                Ok(())
            }
        }
    }
}

pub struct WarehouseBatchIter {
    inner: WarehouseBatchIterKind,
    width: u32,
    height: u32,
    max_boxes: usize,
}

#[cfg(feature = "burn-runtime")]
enum WarehouseBatchIterKind {
    Direct {
        order: Vec<(usize, usize)>,
        shards: std::sync::Arc<Vec<ShardBuffer>>,
        cursor: usize,
        drop_last: bool,
    },
    Stream {
        rx: Receiver<Option<StreamedSample>>,
        remaining: usize,
        drop_last: bool,
        ended: bool,
    },
}

#[cfg(feature = "burn-runtime")]
struct StreamedSample {
    images: Vec<f32>,
    boxes: Vec<f32>,
    masks: Vec<f32>,
}

#[cfg(feature = "burn-runtime")]
struct StreamingStore {
    shards: std::sync::Arc<Vec<ShardBuffer>>,
    train_order: Vec<(usize, usize)>,
    val_order: Vec<(usize, usize)>,
    drop_last: bool,
    width: u32,
    height: u32,
    max_boxes: usize,
    prefetch: usize,
}

#[cfg(feature = "burn-runtime")]
impl StreamingStore {
    pub fn from_manifest_path(
        manifest_path: &Path,
        val_ratio: f32,
        seed: Option<u64>,
        drop_last: bool,
        prefetch: usize,
    ) -> DatasetResult<Self> {
        let manifest = WarehouseManifest::load(manifest_path)?;
        let root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
        let shards_vec = manifest
            .shards
            .iter()
            .enumerate()
            .map(|(i, meta)| {
                let t0 = Instant::now();
                let shard = load_shard_streamed(root, meta)?;
                let ms = t0.elapsed().as_millis();
                println!(
                    "[warehouse] stream shard {} (id={}, samples={}, size={}x{}, max_boxes={}) in {} ms",
                    i,
                    meta.id,
                    shard.samples,
                    shard.width,
                    shard.height,
                    shard.max_boxes,
                    ms
                );
                Ok(shard)
            })
            .collect::<DatasetResult<Vec<_>>>()?;
        let shards = std::sync::Arc::new(shards_vec);
        let total_samples: usize = shards.iter().map(|s| s.samples).sum();
        let mut order: Vec<(usize, usize)> = Vec::with_capacity(total_samples);
        for (si, shard) in shards.iter().enumerate() {
            for i in 0..shard.samples {
                order.push((si, i));
            }
        }
        if let Some(s) = seed {
            let mut rng = rand::rngs::StdRng::seed_from_u64(s);
            order.shuffle(&mut rng);
        }
        let val_count =
            ((val_ratio.clamp(0.0, 1.0) * order.len() as f32).round() as usize).min(order.len());
        let (val_order, train_order) = order.split_at(val_count);
        let width = shards.first().map(|s| s.width).unwrap_or(0);
        let height = shards.first().map(|s| s.height).unwrap_or(0);
        let max_boxes = shards.first().map(|s| s.max_boxes).unwrap_or(0);
        Ok(StreamingStore {
            shards,
            train_order: train_order.to_vec(),
            val_order: val_order.to_vec(),
            drop_last,
            width,
            height,
            max_boxes,
            prefetch: prefetch.max(1),
        })
    }

    fn spawn_iter(&self, order: &[(usize, usize)], drop_last: bool) -> WarehouseBatchIter {
        let (tx, rx) = bounded(self.prefetch);
        let shards = self.shards.clone();
        let order_vec: Vec<(usize, usize)> = order.to_vec();
        let width = self.width;
        let height = self.height;
        let max_boxes = self.max_boxes;
        thread::spawn(move || {
            for (shard_idx, sample_idx) in order_vec.into_iter() {
                let shard = match shards.get(shard_idx) {
                    Some(s) => s,
                    None => break,
                };
                let mut images = Vec::new();
                let mut boxes = Vec::new();
                let mut masks = Vec::new();
                if let Err(e) = shard.copy_sample(sample_idx, &mut images, &mut boxes, &mut masks) {
                    eprintln!("[warehouse] streaming copy error: {:?}", e);
                    break;
                }
                if tx
                    .send(Some(StreamedSample {
                        images,
                        boxes,
                        masks,
                    }))
                    .is_err()
                {
                    break;
                }
            }
            let _ = tx.send(None);
        });

        WarehouseBatchIter {
            inner: WarehouseBatchIterKind::Stream {
                rx,
                remaining: order.len(),
                drop_last,
                ended: false,
            },
            width,
            height,
            max_boxes,
        }
    }
}

#[cfg(feature = "burn-runtime")]
impl WarehouseShardStore for StreamingStore {
    fn train_iter(&self) -> WarehouseBatchIter {
        self.spawn_iter(&self.train_order, self.drop_last)
    }

    fn val_iter(&self) -> WarehouseBatchIter {
        self.spawn_iter(&self.val_order, false)
    }

    fn train_len(&self) -> usize {
        self.train_order.len()
    }

    fn val_len(&self) -> usize {
        self.val_order.len()
    }

    fn total_shards(&self) -> usize {
        self.shards.len()
    }

    fn mode(&self) -> WarehouseStoreMode {
        WarehouseStoreMode::default_streaming()
    }
}

#[cfg(feature = "burn-runtime")]
pub trait WarehouseShardStore: Send + Sync {
    fn train_iter(&self) -> WarehouseBatchIter;
    fn val_iter(&self) -> WarehouseBatchIter;
    fn train_len(&self) -> usize;
    fn val_len(&self) -> usize;
    fn total_shards(&self) -> usize;
    #[allow(dead_code)]
    fn mode(&self) -> WarehouseStoreMode {
        WarehouseStoreMode::InMemory
    }
}

#[cfg(feature = "burn-runtime")]
pub struct WarehouseLoaders {
    store: Box<dyn WarehouseShardStore>,
}

#[cfg(feature = "burn-runtime")]
struct InMemoryStore {
    shards: std::sync::Arc<Vec<ShardBuffer>>,
    train_order: Vec<(usize, usize)>,
    val_order: Vec<(usize, usize)>,
    drop_last: bool,
    width: u32,
    height: u32,
    max_boxes: usize,
}

#[cfg(feature = "burn-runtime")]
impl InMemoryStore {
    pub fn from_manifest_path(
        manifest_path: &Path,
        val_ratio: f32,
        seed: Option<u64>,
        drop_last: bool,
    ) -> DatasetResult<Self> {
        let manifest = WarehouseManifest::load(manifest_path)?;
        let root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
        let shards_vec = manifest
            .shards
            .iter()
            .enumerate()
            .map(|(i, meta)| {
                let t0 = Instant::now();
                let shard = load_shard_owned(root, meta)?;
                let ms = t0.elapsed().as_millis();
                println!(
                    "[warehouse] loaded shard {} (id={}, samples={}, size={}x{}, max_boxes={}) in {} ms",
                    i,
                    meta.id,
                    shard.samples,
                    shard.width,
                    shard.height,
                    shard.max_boxes,
                    ms
                );
                Ok(shard)
            })
            .collect::<DatasetResult<Vec<_>>>()?;
        let shards = std::sync::Arc::new(shards_vec);
        let total_samples: usize = shards.iter().map(|s| s.samples).sum();
        let mut order: Vec<(usize, usize)> = Vec::with_capacity(total_samples);
        for (si, shard) in shards.iter().enumerate() {
            for i in 0..shard.samples {
                order.push((si, i));
            }
        }
        if let Some(s) = seed {
            let mut rng = rand::rngs::StdRng::seed_from_u64(s);
            order.shuffle(&mut rng);
        }
        let val_count =
            ((val_ratio.clamp(0.0, 1.0) * order.len() as f32).round() as usize).min(order.len());
        let (val_order, train_order) = order.split_at(val_count);
        let width = shards.first().map(|s| s.width).unwrap_or(0);
        let height = shards.first().map(|s| s.height).unwrap_or(0);
        let max_boxes = shards.first().map(|s| s.max_boxes).unwrap_or(0);
        Ok(InMemoryStore {
            shards,
            train_order: train_order.to_vec(),
            val_order: val_order.to_vec(),
            drop_last,
            width,
            height,
            max_boxes,
        })
    }
}

#[cfg(feature = "burn-runtime")]
impl WarehouseShardStore for InMemoryStore {
    fn train_iter(&self) -> WarehouseBatchIter {
        WarehouseBatchIter {
            inner: WarehouseBatchIterKind::Direct {
                order: self.train_order.clone(),
                shards: self.shards.clone(),
                cursor: 0,
                drop_last: self.drop_last,
            },
            width: self.width,
            height: self.height,
            max_boxes: self.max_boxes,
        }
    }

    fn val_iter(&self) -> WarehouseBatchIter {
        WarehouseBatchIter {
            inner: WarehouseBatchIterKind::Direct {
                order: self.val_order.clone(),
                shards: self.shards.clone(),
                cursor: 0,
                drop_last: false,
            },
            width: self.width,
            height: self.height,
            max_boxes: self.max_boxes,
        }
    }

    fn train_len(&self) -> usize {
        self.train_order.len()
    }

    fn val_len(&self) -> usize {
        self.val_order.len()
    }

    fn total_shards(&self) -> usize {
        self.shards.len()
    }
}

#[cfg(feature = "burn-runtime")]
struct MmapStore {
    shards: std::sync::Arc<Vec<ShardBuffer>>,
    train_order: Vec<(usize, usize)>,
    val_order: Vec<(usize, usize)>,
    drop_last: bool,
    width: u32,
    height: u32,
    max_boxes: usize,
}

#[cfg(feature = "burn-runtime")]
impl MmapStore {
    pub fn from_manifest_path(
        manifest_path: &Path,
        val_ratio: f32,
        seed: Option<u64>,
        drop_last: bool,
    ) -> DatasetResult<Self> {
        let manifest = WarehouseManifest::load(manifest_path)?;
        let root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
        let shards_vec = manifest
            .shards
            .iter()
            .enumerate()
            .map(|(i, meta)| {
                let t0 = Instant::now();
                let shard = load_shard_mmap(root, meta)?;
                let ms = t0.elapsed().as_millis();
                println!(
                    "[warehouse] mmap shard {} (id={}, samples={}, size={}x{}, max_boxes={}) in {} ms",
                    i,
                    meta.id,
                    shard.samples,
                    shard.width,
                    shard.height,
                    shard.max_boxes,
                    ms
                );
                Ok(shard)
            })
            .collect::<DatasetResult<Vec<_>>>()?;
        let shards = std::sync::Arc::new(shards_vec);
        let total_samples: usize = shards.iter().map(|s| s.samples).sum();
        let mut order: Vec<(usize, usize)> = Vec::with_capacity(total_samples);
        for (si, shard) in shards.iter().enumerate() {
            for i in 0..shard.samples {
                order.push((si, i));
            }
        }
        if let Some(s) = seed {
            let mut rng = rand::rngs::StdRng::seed_from_u64(s);
            order.shuffle(&mut rng);
        }
        let val_count =
            ((val_ratio.clamp(0.0, 1.0) * order.len() as f32).round() as usize).min(order.len());
        let (val_order, train_order) = order.split_at(val_count);
        let width = shards.first().map(|s| s.width).unwrap_or(0);
        let height = shards.first().map(|s| s.height).unwrap_or(0);
        let max_boxes = shards.first().map(|s| s.max_boxes).unwrap_or(0);
        Ok(MmapStore {
            shards,
            train_order: train_order.to_vec(),
            val_order: val_order.to_vec(),
            drop_last,
            width,
            height,
            max_boxes,
        })
    }
}

#[cfg(feature = "burn-runtime")]
impl WarehouseShardStore for MmapStore {
    fn train_iter(&self) -> WarehouseBatchIter {
        WarehouseBatchIter {
            inner: WarehouseBatchIterKind::Direct {
                order: self.train_order.clone(),
                shards: self.shards.clone(),
                cursor: 0,
                drop_last: self.drop_last,
            },
            width: self.width,
            height: self.height,
            max_boxes: self.max_boxes,
        }
    }

    fn val_iter(&self) -> WarehouseBatchIter {
        WarehouseBatchIter {
            inner: WarehouseBatchIterKind::Direct {
                order: self.val_order.clone(),
                shards: self.shards.clone(),
                cursor: 0,
                drop_last: false,
            },
            width: self.width,
            height: self.height,
            max_boxes: self.max_boxes,
        }
    }

    fn train_len(&self) -> usize {
        self.train_order.len()
    }

    fn val_len(&self) -> usize {
        self.val_order.len()
    }

    fn total_shards(&self) -> usize {
        self.shards.len()
    }

    fn mode(&self) -> WarehouseStoreMode {
        WarehouseStoreMode::Mmap
    }
}
#[cfg(feature = "burn-runtime")]
impl WarehouseBatchIter {
    pub fn len(&self) -> usize {
        match &self.inner {
            WarehouseBatchIterKind::Direct { order, .. } => order.len(),
            WarehouseBatchIterKind::Stream { remaining, .. } => *remaining,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn next_batch<B: burn::tensor::backend::Backend>(
        &mut self,
        batch_size: usize,
        device: &B::Device,
    ) -> DatasetResult<Option<BurnBatch<B>>> {
        match &mut self.inner {
            WarehouseBatchIterKind::Direct {
                order,
                shards,
                cursor,
                drop_last,
            } => {
                if *cursor >= order.len() {
                    return Ok(None);
                }
                let end = (*cursor + batch_size).min(order.len());
                let slice = &order[*cursor..end];
                *cursor = end;
                if *drop_last && slice.len() < batch_size {
                    return Ok(None);
                }
                let mut images = Vec::new();
                let mut boxes = Vec::new();
                let mut masks = Vec::new();
                let mut frame_ids = Vec::new();
                for (global_idx, (shard_idx, sample_idx)) in slice.iter().enumerate() {
                    let shard = &shards[*shard_idx];
                    shard.copy_sample(*sample_idx, &mut images, &mut boxes, &mut masks)?;
                    frame_ids.push(global_idx as f32);
                }
                let image_shape = [slice.len(), 3, self.height as usize, self.width as usize];
                let boxes_shape = [slice.len(), self.max_boxes, 4];
                let mask_shape = [slice.len(), self.max_boxes];

                let images = burn::tensor::Tensor::<B, 1>::from_floats(images.as_slice(), device)
                    .reshape(image_shape);
                let boxes = burn::tensor::Tensor::<B, 1>::from_floats(boxes.as_slice(), device)
                    .reshape(boxes_shape);
                let box_mask = burn::tensor::Tensor::<B, 1>::from_floats(masks.as_slice(), device)
                    .reshape(mask_shape);
                let frame_ids =
                    burn::tensor::Tensor::<B, 1>::from_floats(frame_ids.as_slice(), device)
                        .reshape([slice.len()]);

                Ok(Some(BurnBatch {
                    images,
                    boxes,
                    box_mask,
                    frame_ids,
                }))
            }
            WarehouseBatchIterKind::Stream {
                rx,
                remaining,
                drop_last,
                ended,
            } => {
                if *ended || *remaining == 0 {
                    return Ok(None);
                }
                let mut images = Vec::new();
                let mut boxes = Vec::new();
                let mut masks = Vec::new();
                let mut frame_ids = Vec::new();
                let mut pulled = 0usize;
                while pulled < batch_size {
                    match rx.recv() {
                        Ok(Some(sample)) => {
                            images.extend_from_slice(&sample.images);
                            boxes.extend_from_slice(&sample.boxes);
                            masks.extend_from_slice(&sample.masks);
                            frame_ids.push(pulled as f32);
                            pulled += 1;
                        }
                        Ok(None) => {
                            *ended = true;
                            break;
                        }
                        Err(_) => {
                            *ended = true;
                            break;
                        }
                    }
                }
                if pulled == 0 || (*drop_last && pulled < batch_size) {
                    return Ok(None);
                }
                *remaining = remaining.saturating_sub(pulled);
                let image_shape = [pulled, 3, self.height as usize, self.width as usize];
                let boxes_shape = [pulled, self.max_boxes, 4];
                let mask_shape = [pulled, self.max_boxes];
                let images = burn::tensor::Tensor::<B, 1>::from_floats(images.as_slice(), device)
                    .reshape(image_shape);
                let boxes = burn::tensor::Tensor::<B, 1>::from_floats(boxes.as_slice(), device)
                    .reshape(boxes_shape);
                let box_mask = burn::tensor::Tensor::<B, 1>::from_floats(masks.as_slice(), device)
                    .reshape(mask_shape);
                let frame_ids =
                    burn::tensor::Tensor::<B, 1>::from_floats(frame_ids.as_slice(), device)
                        .reshape([pulled]);
                Ok(Some(BurnBatch {
                    images,
                    boxes,
                    box_mask,
                    frame_ids,
                }))
            }
        }
    }
}

#[cfg(feature = "burn-runtime")]
impl WarehouseLoaders {
    pub fn store_len(&self) -> usize {
        self.store.total_shards()
    }
    pub fn from_manifest_path(
        manifest_path: &Path,
        val_ratio: f32,
        seed: Option<u64>,
        drop_last: bool,
    ) -> DatasetResult<Self> {
        let mode = WarehouseStoreMode::from_env();
        println!("[warehouse] store mode: {:?}", mode);
        match mode {
            WarehouseStoreMode::InMemory => {
                let store =
                    InMemoryStore::from_manifest_path(manifest_path, val_ratio, seed, drop_last)?;
                Ok(WarehouseLoaders {
                    store: Box::new(store),
                })
            }
            WarehouseStoreMode::Mmap => {
                let store =
                    MmapStore::from_manifest_path(manifest_path, val_ratio, seed, drop_last)?;
                Ok(WarehouseLoaders {
                    store: Box::new(store),
                })
            }
            WarehouseStoreMode::Streaming { prefetch } => {
                println!("[warehouse] streaming prefetch depth: {}", prefetch);
                let store = StreamingStore::from_manifest_path(
                    manifest_path,
                    val_ratio,
                    seed,
                    drop_last,
                    prefetch,
                )?;
                Ok(WarehouseLoaders {
                    store: Box::new(store),
                })
            }
        }
    }

    pub fn train_iter(&self) -> WarehouseBatchIter {
        self.store.train_iter()
    }

    pub fn val_iter(&self) -> WarehouseBatchIter {
        self.store.val_iter()
    }

    pub fn train_len(&self) -> usize {
        self.store.train_len()
    }

    pub fn val_len(&self) -> usize {
        self.store.val_len()
    }
}

#[cfg(feature = "burn-runtime")]
fn read_u32_le(data: &[u8]) -> u32 {
    let mut arr = [0u8; 4];
    arr.copy_from_slice(data);
    u32::from_le_bytes(arr)
}

#[cfg(feature = "burn-runtime")]
fn read_u64_le(data: &[u8]) -> u64 {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(data);
    u64::from_le_bytes(arr)
}

#[cfg(feature = "burn-runtime")]
fn load_shard_owned(root: &Path, meta: &ShardMetadata) -> DatasetResult<ShardBuffer> {
    let path = root.join(&meta.relative_path);
    let data = fs::read(&path).map_err(|e| BurnDatasetError::Io {
        path: path.clone(),
        source: e,
    })?;
    if data.len() < 4 {
        return Err(BurnDatasetError::Other(format!(
            "shard {} too small",
            path.display()
        )));
    }
    if &data[0..4] != b"TWH1" {
        return Err(BurnDatasetError::Other(format!(
            "bad magic in shard {}",
            path.display()
        )));
    }
    let shard_version = read_u32_le(&data[4..8]);
    if shard_version != meta.shard_version {
        return Err(BurnDatasetError::Other(format!(
            "shard version mismatch {} vs {}",
            shard_version, meta.shard_version
        )));
    }
    let dtype = read_u32_le(&data[8..12]);
    if dtype != 0 {
        return Err(BurnDatasetError::Other(format!(
            "unsupported dtype {} in {}",
            dtype,
            path.display()
        )));
    }
    let width = read_u32_le(&data[16..20]);
    let height = read_u32_le(&data[20..24]);
    let channels = read_u32_le(&data[24..28]);
    if channels != 3 {
        return Err(BurnDatasetError::Other(format!(
            "unsupported channels {} in {}",
            channels,
            path.display()
        )));
    }
    let max_boxes = read_u32_le(&data[28..32]) as usize;
    let samples = read_u64_le(&data[32..40]) as usize;
    let image_offset = read_u64_le(&data[40..48]) as usize;
    let boxes_offset = read_u64_le(&data[48..56]) as usize;
    let mask_offset = read_u64_le(&data[56..64]) as usize;

    let image_elems = samples
        .checked_mul(3)
        .and_then(|v| v.checked_mul(width as usize))
        .and_then(|v| v.checked_mul(height as usize))
        .ok_or_else(|| BurnDatasetError::Other("overflow computing image elems".into()))?;
    let box_elems = samples
        .checked_mul(max_boxes)
        .and_then(|v| v.checked_mul(4))
        .ok_or_else(|| BurnDatasetError::Other("overflow computing box elems".into()))?;
    let mask_elems = samples
        .checked_mul(max_boxes)
        .ok_or_else(|| BurnDatasetError::Other("overflow computing mask elems".into()))?;

    let image_bytes = image_elems * std::mem::size_of::<f32>();
    let box_bytes = box_elems * std::mem::size_of::<f32>();
    let mask_bytes = mask_elems * std::mem::size_of::<f32>();

    if image_offset + image_bytes > data.len()
        || boxes_offset + box_bytes > data.len()
        || mask_offset + mask_bytes > data.len()
    {
        return Err(BurnDatasetError::Other(format!(
            "shard {} truncated",
            path.display()
        )));
    }

    let images = data[image_offset..image_offset + image_bytes]
        .chunks_exact(4)
        .map(|c| {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(c);
            f32::from_le_bytes(arr)
        })
        .collect();
    let boxes = data[boxes_offset..boxes_offset + box_bytes]
        .chunks_exact(4)
        .map(|c| {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(c);
            f32::from_le_bytes(arr)
        })
        .collect();
    let masks = data[mask_offset..mask_offset + mask_bytes]
        .chunks_exact(4)
        .map(|c| {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(c);
            f32::from_le_bytes(arr)
        })
        .collect();

    Ok(ShardBuffer {
        samples,
        width,
        height,
        max_boxes,
        backing: ShardBacking::Owned {
            images,
            boxes,
            masks,
        },
    })
}

#[cfg(feature = "burn-runtime")]
fn load_shard_mmap(root: &Path, meta: &ShardMetadata) -> DatasetResult<ShardBuffer> {
    let path = root.join(&meta.relative_path);
    let file = File::open(&path).map_err(|e| BurnDatasetError::Io {
        path: path.clone(),
        source: e,
    })?;
    let mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .map_err(|e| BurnDatasetError::Io {
                path: path.clone(),
                source: std::io::Error::other(e.to_string()),
            })?
    };
    let data = &mmap[..];
    if data.len() < 4 {
        return Err(BurnDatasetError::Other(format!(
            "shard {} too small",
            path.display()
        )));
    }
    if &data[0..4] != b"TWH1" {
        return Err(BurnDatasetError::Other(format!(
            "bad magic in shard {}",
            path.display()
        )));
    }
    let shard_version = read_u32_le(&data[4..8]);
    if shard_version != meta.shard_version {
        return Err(BurnDatasetError::Other(format!(
            "shard version mismatch {} vs {}",
            shard_version, meta.shard_version
        )));
    }
    let dtype = read_u32_le(&data[8..12]);
    if dtype != 0 {
        return Err(BurnDatasetError::Other(format!(
            "unsupported dtype {} in {}",
            dtype,
            path.display()
        )));
    }
    let width = read_u32_le(&data[16..20]);
    let height = read_u32_le(&data[20..24]);
    let channels = read_u32_le(&data[24..28]);
    if channels != 3 {
        return Err(BurnDatasetError::Other(format!(
            "unsupported channels {} in {}",
            channels,
            path.display()
        )));
    }
    let max_boxes = read_u32_le(&data[28..32]) as usize;
    let samples = read_u64_le(&data[32..40]) as usize;
    let image_offset = read_u64_le(&data[40..48]) as usize;
    let boxes_offset = read_u64_le(&data[48..56]) as usize;
    let mask_offset = read_u64_le(&data[56..64]) as usize;

    let image_elems = samples
        .checked_mul(3)
        .and_then(|v| v.checked_mul(width as usize))
        .and_then(|v| v.checked_mul(height as usize))
        .ok_or_else(|| BurnDatasetError::Other("overflow computing image elems".into()))?;
    let box_elems = samples
        .checked_mul(max_boxes)
        .and_then(|v| v.checked_mul(4))
        .ok_or_else(|| BurnDatasetError::Other("overflow computing box elems".into()))?;
    let mask_elems = samples
        .checked_mul(max_boxes)
        .ok_or_else(|| BurnDatasetError::Other("overflow computing mask elems".into()))?;

    let image_bytes = image_elems * std::mem::size_of::<f32>();
    let box_bytes = box_elems * std::mem::size_of::<f32>();
    let mask_bytes = mask_elems * std::mem::size_of::<f32>();

    if image_offset + image_bytes > data.len()
        || boxes_offset + box_bytes > data.len()
        || mask_offset + mask_bytes > data.len()
    {
        return Err(BurnDatasetError::Other(format!(
            "shard {} truncated",
            path.display()
        )));
    }

    Ok(ShardBuffer {
        samples,
        width,
        height,
        max_boxes,
        backing: ShardBacking::Mmap {
            mmap: std::sync::Arc::new(mmap),
            image_offset,
            boxes_offset,
            mask_offset,
        },
    })
}

#[cfg(feature = "burn-runtime")]
fn load_shard_streamed(root: &Path, meta: &ShardMetadata) -> DatasetResult<ShardBuffer> {
    let path = root.join(&meta.relative_path);
    let mut file = File::open(&path).map_err(|e| BurnDatasetError::Io {
        path: path.clone(),
        source: e,
    })?;
    let mut header = vec![0u8; 64];
    let read = file.read(&mut header).map_err(|e| BurnDatasetError::Io {
        path: path.clone(),
        source: e,
    })?;
    if read < 64 {
        return Err(BurnDatasetError::Other(format!(
            "shard {} too small",
            path.display()
        )));
    }
    if &header[0..4] != b"TWH1" {
        return Err(BurnDatasetError::Other(format!(
            "bad magic in shard {}",
            path.display()
        )));
    }
    let shard_version = read_u32_le(&header[4..8]);
    if shard_version != meta.shard_version {
        return Err(BurnDatasetError::Other(format!(
            "shard version mismatch {} vs {}",
            shard_version, meta.shard_version
        )));
    }
    let dtype = read_u32_le(&header[8..12]);
    if dtype != 0 {
        return Err(BurnDatasetError::Other(format!(
            "unsupported dtype {} in {}",
            dtype,
            path.display()
        )));
    }
    let width = read_u32_le(&header[16..20]);
    let height = read_u32_le(&header[20..24]);
    let channels = read_u32_le(&header[24..28]);
    if channels != 3 {
        return Err(BurnDatasetError::Other(format!(
            "unsupported channels {} in {}",
            channels,
            path.display()
        )));
    }
    let max_boxes = read_u32_le(&header[28..32]) as usize;
    let samples = read_u64_le(&header[32..40]) as usize;
    let image_offset = read_u64_le(&header[40..48]) as usize;
    let boxes_offset = read_u64_le(&header[48..56]) as usize;
    let mask_offset = read_u64_le(&header[56..64]) as usize;

    let img_elems = samples
        .checked_mul(3)
        .and_then(|v| v.checked_mul(width as usize))
        .and_then(|v| v.checked_mul(height as usize))
        .ok_or_else(|| BurnDatasetError::Other("overflow computing image elems".into()))?;
    let box_elems = samples
        .checked_mul(max_boxes)
        .and_then(|v| v.checked_mul(4))
        .ok_or_else(|| BurnDatasetError::Other("overflow computing box elems".into()))?;
    let mask_elems = samples
        .checked_mul(max_boxes)
        .ok_or_else(|| BurnDatasetError::Other("overflow computing mask elems".into()))?;

    let image_bytes = img_elems * std::mem::size_of::<f32>();
    let box_bytes = box_elems * std::mem::size_of::<f32>();
    let mask_bytes = mask_elems * std::mem::size_of::<f32>();

    let file_len = file
        .metadata()
        .map_err(|e| BurnDatasetError::Io {
            path: path.clone(),
            source: e,
        })?
        .len() as usize;

    if image_offset
        .checked_add(image_bytes)
        .map(|v| v > file_len)
        .unwrap_or(true)
        || boxes_offset
            .checked_add(box_bytes)
            .map(|v| v > file_len)
            .unwrap_or(true)
        || mask_offset
            .checked_add(mask_bytes)
            .map(|v| v > file_len)
            .unwrap_or(true)
    {
        return Err(BurnDatasetError::Other(format!(
            "shard {} truncated",
            path.display()
        )));
    }

    Ok(ShardBuffer {
        samples,
        width,
        height,
        max_boxes,
        backing: ShardBacking::Streamed {
            path,
            image_offset,
            boxes_offset,
            mask_offset,
            samples,
        },
    })
}
