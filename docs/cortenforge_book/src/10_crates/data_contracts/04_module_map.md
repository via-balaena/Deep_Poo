# data_contracts: Module Map
Quick read: What each module owns and why it exists.

- `capture`: Capture metadata/labels schema; validation errors. Types: PolypLabel, CaptureMetadata, ValidationError.
- `manifest`: Run manifest schema/versioning. Types: RunManifest, RunManifestSchemaVersion.
- `lib.rs`: Re-exports capture/manifest modules and types.

Cross-module dependencies:
- none beyond serde.
- consumers are capture/utils/tools/training.

## Links
- Source: `data_contracts/src/lib.rs`
