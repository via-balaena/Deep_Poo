# Traits & Generics (models)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None defined; extensibility via generic backends and configuration structs.

## Generics and bounds
- `TinyDet<B: Backend>` and `BigDet<B: Backend>` are generic over Burn backends.
- Config structs (`TinyDetConfig`, `BigDetConfig`) are concrete; no trait bounds.
- Methods take `&B::Device` to construct layers; forward methods operate on `Tensor<B, 2>` (and `Tensor<B, 3>` outputs for multibox).
- Uses derive `Module` for Burn module composition; no custom traits introduced.

## Design notes
- Generic over backend keeps CPU/GPU swap flexible without additional traits.
- No trait objects; consumers choose backend type at compile time.
- `forward_multibox` enforces box ordering/clamping procedurally—no trait/generic constraints there.

## Links
- Source: `crates/models/src/lib.rs`
