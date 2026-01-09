# Traits & Generics (inference)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None defined directly; crate consumes `vision_core::Detector` and provides concrete implementations.

## Generics and bounds
- Backend selection via type alias `InferenceBackend` (NdArray by default, WGPU when feature enabled).
- Model selection via aliases: `InferenceModel<B>` / `InferenceModelConfig` pick `models::TinyDet` or `models::BigDet` based on the `bigdet` feature. Both are generic over Burn `Backend`.
- `InferenceFactory` returns `Box<dyn vision_core::Detector + Send + Sync>` (trait object) so callers don’t need to name the concrete detector.

## Design notes
- Feature-gated model/backend keeps the public API stable while allowing different compile-time choices.
- Factory hides concrete detector types behind a trait object to allow heuristic fallback vs. Burn-backed detector without leaking types.
- If additional detectors are added, extend the factory (or add a new trait) but keep the trait-object boundary for runtime swapping.

## Links
- Source: `inference/src/factory.rs`
