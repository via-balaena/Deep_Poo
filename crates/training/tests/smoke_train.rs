use training::{collate, DatasetPathConfig};

#[test]
fn collate_runs_with_empty_images() {
    let _cfg = DatasetPathConfig {
        root: "assets/datasets/captures_filtered".into(),
        labels_subdir: "labels".into(),
        images_subdir: ".".into(),
    };
    let samples = Vec::new();
    assert!(collate::<burn_ndarray::NdArray<f32>>(&samples, 4).is_err());
}
