# Examples (data_contracts)

## 1) Validate a label entry
```rust,ignore
use data_contracts::capture::PolypLabel;

fn main() {
    let lbl = PolypLabel { center_world: [0.0, 0.0, 0.0], bbox_px: Some([10.0, 20.0, 50.0, 60.0]), bbox_norm: None };
    lbl.validate().expect("valid bbox");
}
```

## 2) Build and validate capture metadata
```rust,ignore
use data_contracts::capture::{CaptureMetadata, PolypLabel};

fn main() {
    let meta = CaptureMetadata {
        frame_id: 1,
        sim_time: 0.1,
        unix_time: 1_700_000_000.0,
        image: "images/frame_00001.png".into(),
        image_present: true,
        camera_active: true,
        polyp_seed: 42,
        polyp_labels: vec![PolypLabel { center_world: [0.0, 0.0, 0.0], bbox_px: None, bbox_norm: Some([0.1, 0.1, 0.2, 0.2]) }],
    };
    meta.validate().expect("metadata valid");
}
```

## 3) Create and check a run manifest
```rust,ignore
use data_contracts::manifest::{RunManifest, RunManifestSchemaVersion};
use std::path::PathBuf;

fn main() {
    let manifest = RunManifest {
        schema_version: RunManifestSchemaVersion::V1,
        seed: Some(1234),
        output_root: PathBuf::from("assets/datasets/captures"),
        run_dir: PathBuf::from("assets/datasets/captures/run_00001"),
        started_at_unix: 1_700_000_000.0,
        max_frames: Some(10),
    };
    manifest.validate().expect("manifest valid");
}
```
