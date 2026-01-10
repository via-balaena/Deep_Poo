# Traits & Generics (training)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None defined; crate composes concrete functions and type aliases.

## Generics and bounds
- Heavy use of the Burn `Backend` trait:
  - `CollatedBatch<B: Backend>` and `collate<B: Backend>` produce tensors for an arbitrary backend.
  - `TrainBackend` type alias switches between `burn_ndarray::NdArray<f32>` (default) and `burn_wgpu::Wgpu<f32>` under the `backend-wgpu` feature.
- No custom traits; functions operate on generic tensors but keep APIs concrete (`RunSample`, `DatasetConfig`).

## Design notes
- Backend generic keeps training usable on CPU/GPU without changing call sites; callers pick backend via features.
- No trait objects; compile-time backend selection aligns with Burn patterns.
- If adding new data sources, prefer composing on `RunSample`/`collate` instead of introducing new traits unless multiple loaders must coexist.

## Links
- Source: `crates/training/src/dataset.rs`
