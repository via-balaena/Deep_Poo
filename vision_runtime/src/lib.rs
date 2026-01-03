use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::gpu_readback::{Readback, ReadbackComplete};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_camera::{ImageRenderTarget, RenderTarget};
use futures_lite::future::{block_on, poll_once};
use image::RgbaImage;
use sim_core::{ModeSet, SimRunMode};
use vision_core::capture::{
    FrontCamera, FrontCaptureCamera, FrontCaptureReadback, FrontCaptureTarget,
};
use vision_core::interfaces::{self, Frame};
use vision_core::overlay::draw_rect;

type InferenceJobResult = (
    Box<dyn interfaces::Detector + Send + Sync>,
    DetectorKind,
    interfaces::DetectionResult,
    f32,
    (u32, u32),
);

#[derive(Clone)]
pub struct FrontCameraFrame {
    pub id: u64,
    pub transform: GlobalTransform,
    pub captured_at: f64,
}

#[derive(Resource, Default)]
pub struct FrontCameraState {
    pub active: bool,
    pub last_transform: Option<GlobalTransform>,
    pub frame_counter: u64,
}

#[derive(Resource, Default)]
pub struct FrontCameraFrameBuffer {
    pub latest: Option<FrontCameraFrame>,
}

#[derive(Resource, Default)]
pub struct BurnDetector {
    pub model_loaded: bool,
}

#[derive(Resource, Default, Clone)]
pub struct DetectionOverlayState {
    pub boxes: Vec<[f32; 4]>,
    pub scores: Vec<f32>,
    pub size: (u32, u32),
    pub fallback: Option<String>,
    pub inference_ms: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Resource)]
pub enum DetectorKind {
    Burn,
    Heuristic,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct InferenceThresholds {
    pub obj_thresh: f32,
    pub iou_thresh: f32,
}

#[derive(Clone)]
pub struct BurnDetectionResult {
    pub frame_id: u64,
    pub positive: bool,
    pub confidence: f32,
    pub boxes: Vec<[f32; 4]>,
    pub scores: Vec<f32>,
}

#[derive(Resource)]
pub struct BurnInferenceState {
    pub pending: Option<Task<InferenceJobResult>>,
    pub last_result: Option<BurnDetectionResult>,
    pub debounce: Timer,
}

impl Default for BurnInferenceState {
    fn default() -> Self {
        Self {
            pending: None,
            last_result: None,
            debounce: Timer::from_seconds(0.18, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct DetectorHandle {
    pub detector: Box<dyn interfaces::Detector + Send + Sync>,
    pub kind: DetectorKind,
}

struct HeuristicDetector;

impl interfaces::Detector for HeuristicDetector {
    fn detect(&mut self, frame: &Frame) -> interfaces::DetectionResult {
        interfaces::DetectionResult {
            frame_id: frame.id,
            positive: true,
            confidence: 0.8,
            boxes: Vec::new(),
            scores: Vec::new(),
        }
    }
}

// Capture setup/readback -----------------------------------------------------

pub fn setup_front_capture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<FrontCameraState>,
    mut target: ResMut<FrontCaptureTarget>,
) {
    // Only set up once.
    if target.size != UVec2::ZERO {
        return;
    }

    let size = UVec2::new(1280, 720);
    let mut image = Image::new_fill(
        Extent3d {
            width: size.x,
            height: size.y,
            ..default()
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_SRC | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    let handle = images.add(image);

    let cam_entity = commands
        .spawn((
            Camera3d::default(),
            Camera {
                order: -10,
                is_active: true,
                target: RenderTarget::Image(ImageRenderTarget::from(handle.clone())),
                ..default()
            },
            Projection::from(PerspectiveProjection {
                fov: 20.0f32.to_radians(),
                ..default()
            }),
            Transform::from_translation(Vec3::ZERO),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            FrontCamera,
            FrontCaptureCamera,
            Name::new("FrontCaptureCamera"),
        ))
        .id();

    target.size = size;
    target.handle = handle;
    target.entity = cam_entity;
    state.active = true;
}

pub fn track_front_camera_state(
    target: Res<FrontCaptureTarget>,
    mut state: ResMut<FrontCameraState>,
    mut buffer: ResMut<FrontCameraFrameBuffer>,
    cameras: Query<&GlobalTransform, With<FrontCaptureCamera>>,
    time: Res<Time>,
) {
    let Ok(transform) = cameras.get(target.entity) else {
        return;
    };
    state.last_transform = Some(*transform);
    state.frame_counter = state.frame_counter.wrapping_add(1);
    buffer.latest = Some(FrontCameraFrame {
        id: state.frame_counter,
        transform: *transform,
        captured_at: time.elapsed_secs_f64(),
    });
}

pub fn capture_front_camera_frame(
    mode: Res<SimRunMode>,
    mut commands: Commands,
    target: Res<FrontCaptureTarget>,
) {
    if !matches!(*mode, SimRunMode::Datagen | SimRunMode::Inference) {
        return;
    }
    commands
        .entity(target.entity)
        .insert(Readback::texture(target.handle.clone()));
}

pub fn on_front_capture_readback(
    ev: On<ReadbackComplete>,
    target: Res<FrontCaptureTarget>,
    mut readback: ResMut<FrontCaptureReadback>,
) {
    let expected_len = (target.size.x * target.size.y * 4) as usize;
    let ev = ev.event();
    if ev.entity != target.entity {
        return;
    }
    if ev.data.len() == expected_len {
        readback.latest = Some(ev.data.clone());
    }
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FrontCaptureTarget {
            handle: Handle::default(),
            size: UVec2::ZERO,
            entity: Entity::PLACEHOLDER,
        })
        .init_resource::<FrontCaptureReadback>()
        .init_resource::<FrontCameraState>()
        .init_resource::<FrontCameraFrameBuffer>()
        .add_systems(Startup, setup_front_capture)
        .add_systems(Update, track_front_camera_state.in_set(ModeSet::Common))
        .add_systems(Update, capture_front_camera_frame.in_set(ModeSet::Common))
        .add_observer(on_front_capture_readback);
    }
}

// Inference ---------------------------------------------------------------

pub fn schedule_burn_inference(
    mode: Res<SimRunMode>,
    time: Res<Time>,
    mut jobs: ResMut<BurnInferenceState>,
    mut buffer: ResMut<FrontCameraFrameBuffer>,
    handle: Option<ResMut<DetectorHandle>>,
    target: Res<FrontCaptureTarget>,
    mut readback: ResMut<FrontCaptureReadback>,
) {
    if !matches!(*mode, SimRunMode::Inference) {
        return;
    }
    let Some(mut handle) = handle else {
        return;
    };

    jobs.debounce.tick(time.delta());
    if jobs.pending.is_some() || !jobs.debounce.is_finished() {
        return;
    }
    let Some(frame) = buffer.latest.take() else {
        return;
    };

    let rgba = readback.latest.take();
    let start = std::time::Instant::now();
    let f = Frame {
        id: frame.id,
        timestamp: frame.captured_at,
        rgba,
        size: (target.size.x, target.size.y),
        path: None,
    };
    let mut detector = std::mem::replace(&mut handle.detector, Box::new(HeuristicDetector));
    let kind = handle.kind;
    let size = (target.size.x, target.size.y);
    let task = AsyncComputeTaskPool::get().spawn(async move {
        let result = detector.detect(&f);
        let infer_ms = start.elapsed().as_secs_f32() * 1000.0;
        (detector, kind, result, infer_ms, size)
    });
    jobs.pending = Some(task);
}

pub fn threshold_hotkeys(
    mode: Res<SimRunMode>,
    keys: Res<ButtonInput<KeyCode>>,
    thresh: Option<ResMut<InferenceThresholds>>,
    handle: Option<ResMut<DetectorHandle>>,
    burn_loaded: Option<ResMut<BurnDetector>>,
) {
    if !matches!(*mode, SimRunMode::Inference) {
        return;
    }
    let (Some(mut thresh), Some(mut handle)) = (thresh, handle) else {
        return;
    };
    let Some(mut burn_loaded) = burn_loaded else {
        return;
    };

    let mut changed = false;
    if keys.just_pressed(KeyCode::Minus) {
        thresh.obj_thresh = (thresh.obj_thresh - 0.05).clamp(0.0, 1.0);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Equal) {
        thresh.obj_thresh = (thresh.obj_thresh + 0.05).clamp(0.0, 1.0);
        changed = true;
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        thresh.iou_thresh = (thresh.iou_thresh - 0.05).clamp(0.1, 0.95);
        changed = true;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        thresh.iou_thresh = (thresh.iou_thresh + 0.05).clamp(0.1, 0.95);
        changed = true;
    }

    if keys.just_pressed(KeyCode::Digit0) {
        handle.detector = Box::new(HeuristicDetector);
        handle.kind = DetectorKind::Heuristic;
        burn_loaded.model_loaded = false;
        changed = true;
    }

    if changed {
        info!(
            "Updated inference thresholds: obj {:.2}, iou {:.2}",
            thresh.obj_thresh, thresh.iou_thresh
        );
    }
}

pub struct InferencePlugin;

impl Plugin for InferencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BurnInferenceState>()
            .init_resource::<BurnDetector>()
            .init_resource::<DetectionOverlayState>()
            .add_systems(
                Update,
                (
                    schedule_burn_inference,
                    poll_inference_task,
                    threshold_hotkeys,
                )
                    .in_set(ModeSet::Inference),
            );
    }
}

// Overlay helpers (draw run overlays)

pub fn recorder_draw_rect(
    img: &mut RgbaImage,
    bbox_px: [u32; 4],
    color: image::Rgba<u8>,
    thickness: u32,
) {
    draw_rect(img, bbox_px, color, thickness);
}

pub mod prelude {
    pub use super::{
        BurnDetectionResult, BurnDetector, BurnInferenceState, CapturePlugin,
        DetectionOverlayState, DetectorHandle, DetectorKind, FrontCameraFrame,
        FrontCameraFrameBuffer, FrontCameraState, InferencePlugin, InferenceThresholds,
    };
}
pub fn poll_inference_task(
    mut jobs: ResMut<BurnInferenceState>,
    mut overlay: ResMut<DetectionOverlayState>,
    handle: Option<ResMut<DetectorHandle>>,
    mut burn_detector: ResMut<BurnDetector>,
) {
    let Some(mut task) = jobs.pending.take() else {
        return;
    };
    if let Some((detector, kind, result, infer_ms, size)) = block_on(poll_once(&mut task)) {
        if let Some(mut handle) = handle {
            handle.detector = detector;
            handle.kind = kind;
        }
        burn_detector.model_loaded = matches!(kind, DetectorKind::Burn);
        if matches!(kind, DetectorKind::Heuristic) {
            overlay.fallback = Some("Heuristic detector active (Burn unavailable)".into());
        } else {
            overlay.fallback = None;
        }
        overlay.inference_ms = Some(infer_ms);
        overlay.boxes = result.boxes.clone();
        overlay.scores = result.scores.clone();
        overlay.size = size;
        jobs.last_result = Some(BurnDetectionResult {
            frame_id: result.frame_id,
            positive: result.positive,
            confidence: result.confidence,
            boxes: result.boxes,
            scores: result.scores,
        });
    } else {
        // Task not finished; put it back.
        jobs.pending = Some(task);
    }
}
