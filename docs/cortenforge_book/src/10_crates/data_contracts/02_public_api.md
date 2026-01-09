# data_contracts: Public API
Quick read: The public surface; use docs.rs for exact signatures.

| Item | Kind | Purpose |
| ---- | ---- | ------- |
| PolypLabel | struct | Label for a captured frame (center_world + bbox) |
| CaptureMetadata | struct | Capture-level metadata |
| ValidationError | enum | Errors from schema validation |
| RunManifestSchemaVersion | enum | Manifest schema version identifier |
| RunManifest | struct | Run manifest (id/seed/camera/resize/frame count/checksum) |
| Modules (pub mod) | module | capture, manifest |
| Re-exports | re-export | CaptureMetadata, PolypLabel, ValidationError, RunManifest, RunManifestSchemaVersion |

## Links
- Source: `data_contracts/src/lib.rs`
- Module: `data_contracts/src/capture.rs`
- Module: `data_contracts/src/manifest.rs`
- Docs.rs: https://docs.rs/cortenforge-data-contracts
