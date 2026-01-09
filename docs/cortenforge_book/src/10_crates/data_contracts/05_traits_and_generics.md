# Traits & Generics (data_contracts)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None; crate is pure data models plus validation helpers.

## Glue types
- Plain structs/enums for schemas: `PolypLabel`, `CaptureMetadata`, `RunManifest`, `RunManifestSchemaVersion`.
- Validation helpers on structs (`validate`) return `Result<(), ValidationError>` or `Result<(), String>`.

## Generics and bounds
- No generics; all types are concrete, serde-serializable structures. Validation errors use `thiserror` enum (`ValidationError`) or `String`.
- Uses `PathBuf` for paths; no lifetime generics involved.

## Design notes
- Keeps contracts simple and portable (serde everywhere). If additional schema versions appear, extend `RunManifestSchemaVersion` and gate validation per version.
- Validation is minimal; downstream loaders should still handle IO/shape errors.

## Links
- Source: `data_contracts/src/capture.rs`
- Source: `data_contracts/src/manifest.rs`
