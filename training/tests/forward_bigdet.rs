use burn::backend::{ndarray::NdArray, Autodiff};
use burn::tensor::Tensor;
use training::{BigDet, BigDetConfig};

type ADBackend = Autodiff<NdArray<f32>>;

#[test]
fn forward_shapes_bigdet_quick() {
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let max_boxes = 3;
    let input_dim = 4 + 8; // box features + global features in collate

    let model = BigDet::<ADBackend>::new(
        BigDetConfig {
            max_boxes,
            input_dim: Some(input_dim),
            ..Default::default()
        },
        &device,
    );

    let batch = 2;
    let input = Tensor::<ADBackend, 2>::zeros([batch, input_dim], &device);
    let (boxes, scores) = model.forward_multibox(input);

    assert_eq!(boxes.dims(), [batch, max_boxes, 4]);
    assert_eq!(scores.dims(), [batch, max_boxes]);
}
