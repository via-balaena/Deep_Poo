# Examples (models)
Quick read: Minimal examples you can adapt safely.

## 1) Build and run TinyDet (NdArray backend)
```rust,ignore
use models::{TinyDet, TinyDetConfig};

fn main() {
    type B = burn::backend::ndarray::NdArrayBackend<f32>;
    let device = B::Device::default();
    let model = TinyDet::<B>::new(TinyDetConfig::default(), &device);
    let input = burn::tensor::Tensor::<B, 2>::zeros([1, 4], &device);
    let logits = model.forward(input);
    println!("logits shape = {:?}", logits.dims());
}
```

## 2) BigDet multibox forward
```rust,ignore
use models::{BigDet, BigDetConfig};

fn main() {
    type B = burn::backend::ndarray::NdArrayBackend<f32>;
    let device = B::Device::default();
    let cfg = BigDetConfig { max_boxes: 8, ..Default::default() };
    let model = BigDet::<B>::new(cfg, &device);
    let input = burn::tensor::Tensor::<B, 2>::zeros([1, 4], &device);
    let (boxes, scores) = model.forward_multibox(input);
    println!("boxes shape = {:?}, scores shape = {:?}", boxes.dims(), scores.dims());
}
```

## Links
- Source: `models/src/lib.rs`
