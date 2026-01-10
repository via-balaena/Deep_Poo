# models: Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
- Construct model from config and backend:
  ```rust,ignore
  type B = burn::backend::ndarray::NdArrayBackend<f32>;
  let device = B::Device::default();
  let model = TinyDet::<B>::new(TinyDetConfig::default(), &device);
  ```
- Swap to BigDet or a different backend type if your dependency graph enables it.

## Execution flow
- Consumer (training/inference) selects config (TinyDet/BigDet) and backend type.
- Build model instance; training/inference crates handle forward/backward/checkpointing.

## Notes
- Stateless definitions; lifecycle controlled by training/inference code.

## Links
- Source: `crates/models/src/lib.rs`
