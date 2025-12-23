use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DatasetSample {
    pub frame_id: u64,
    /// Image in CHW layout, normalized to [0, 1].
    pub image_chw: Vec<f32>,
    pub width: u32,
    pub height: u32,
    /// Normalized bounding boxes: [x_min, y_min, x_max, y_max] in 0..1.
    pub boxes: Vec<[f32; 4]>,
}

#[derive(Deserialize)]
struct LabelEntry {
    frame_id: u64,
    image: String,
    image_present: bool,
    polyp_labels: Vec<PolypLabel>,
}

#[derive(Deserialize)]
struct PolypLabel {
    bbox_px: Option<[f32; 4]>,
    bbox_norm: Option<[f32; 4]>,
}

/// Load a capture run into a format ready to convert into Burn tensors.
///
/// Images are loaded as RGB, normalized to [0, 1], and flattened to CHW.
/// Bounding boxes are returned in normalized coordinates; if the label only
/// includes pixel coordinates, they are normalized against the image size.
pub fn load_run_dataset(run_dir: &Path) -> Result<Vec<DatasetSample>, Box<dyn Error + Send + Sync>> {
    let labels_dir = run_dir.join("labels");
    if !labels_dir.exists() {
        return Err(format!("labels directory not found at {}", labels_dir.display()).into());
    }

    let mut label_paths: Vec<PathBuf> = fs::read_dir(&labels_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    label_paths.sort();

    let mut samples = Vec::new();
    for label_path in label_paths {
        let raw = fs::read(&label_path)?;
        let meta: LabelEntry = serde_json::from_slice(&raw)?;
        if !meta.image_present {
            continue;
        }

        let img_path = run_dir.join(&meta.image);
        if !img_path.exists() {
            return Err(format!("image file missing: {}", img_path.display()).into());
        }
        let img = image::open(&img_path)?.to_rgb8();
        let (width, height) = img.dimensions();

        let mut image_chw = vec![0.0f32; (width * height * 3) as usize];
        for (y, x, pixel) in img.enumerate_pixels() {
            let base = (y * width + x) as usize;
            image_chw[base] = pixel[0] as f32 / 255.0;
            image_chw[(width * height) as usize + base] = pixel[1] as f32 / 255.0;
            image_chw[2 * (width * height) as usize + base] = pixel[2] as f32 / 255.0;
        }

        let boxes = meta
            .polyp_labels
            .iter()
            .filter_map(|l| {
                if let Some(norm) = l.bbox_norm {
                    Some(norm)
                } else {
                    l.bbox_px.map(|px| {
                        [
                            px[0] / width as f32,
                            px[1] / height as f32,
                            px[2] / width as f32,
                            px[3] / height as f32,
                        ]
                    })
                }
            })
            .map(|mut b| {
                for v in b.iter_mut() {
                    *v = v.clamp(0.0, 1.0);
                }
                b
            })
            .collect();

        samples.push(DatasetSample {
            frame_id: meta.frame_id,
            image_chw,
            width,
            height,
            boxes,
        });
    }

    Ok(samples)
}
