# Examples (models)

## 1) Build and run TinyDet (NdArray backend)
```rust,ignore
use models::{TinyDet, TinyDetConfig};

fn main() {
    let device = <models::TrainBackend as burn::tensor::backend::Backend>::Device::default();
    let model = TinyDet::<models::TrainBackend>::new(TinyDetConfig::default(), &device);
    let input = burn::tensor::Tensor::<models::TrainBackend, 2>::zeros([1, 4], &device);
    let logits = model.forward(input);
    println!("logits shape = {:?}", logits.dims());
}
```

## 2) BigDet multibox forward
```rust,ignore
use models::{BigDet, BigDetConfig};

fn main() {
    let device = <models::TrainBackend as burn::tensor::backend::Backend>::Device::default();
    let cfg = BigDetConfig { max_boxes: 8, ..Default::default() };
    let model = BigDet::<models::TrainBackend>::new(cfg, &device);
    let input = burn::tensor::Tensor::<models::TrainBackend, 2>::zeros([1, model.input_dim], &device);
    let (boxes, scores) = model.forward_multibox(input);
    println!("boxes shape = {:?}, scores shape = {:?}", boxes.dims(), scores.dims());
}
```

## 3) Switch to WGPU backend (feature)
```rust,ignore
// In Cargo.toml enable: models = { features = ["backend-wgpu"] }
use models::{TinyDet, TinyDetConfig, InferenceBackend};

fn main() {
    let device = <InferenceBackend as burn::tensor::backend::Backend>::Device::default();
    let model = TinyDet::<InferenceBackend>::new(TinyDetConfig::default(), &device);
    let input = burn::tensor::Tensor::<InferenceBackend, 2>::zeros([1, 4], &device);
    let _ = model.forward(input);
}
```
