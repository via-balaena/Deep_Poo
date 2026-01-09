# Traits & Generics (cortenforge-tools)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None defined; crate is thin wrappers/re-exports and concrete helpers.

## Generics and bounds
- Concrete structs/helpers only; no generic parameters.
- `warehouse_commands` builders use plain enums (`WarehouseStore`, `ModelKind`) and string-building helpers—no trait bounds.
- Services/overlay/recorder modules rely on external traits (`vision_core::Detector`, `capture_utils::Recorder`) but do not introduce new ones.

## Design notes
- Intentional lack of traits keeps CLI/bin usage simple. Extensibility is expected via adding new modules/bins rather than generic abstractions.
- As the crate is slated for split/retirement, avoid adding new trait surfaces here; shared functionality can move into other crates with clearer ownership.
