use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::gpu_readback::{Readback, ReadbackComplete};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_camera::{ImageRenderTarget, RenderTarget};
use futures_lite::future::{block_on, poll_once};
use image::RgbaImage;
use inference::InferenceThresholds;
use sim_core::{ModeSet, SimRunMode};
use vision_core::capture::{PrimaryCaptureCamera, PrimaryCaptureReadback, PrimaryCaptureTarget};
use vision_core::interfaces::{self, Frame};
use vision_core::overlay::draw_rect;

/// Bevy resource wrapper for inference thresholds.
///
/// This bridges the framework-agnostic `inference` crate with Bevy ECS.
/// The inner `InferenceThresholds` type can be used in non-Bevy contexts
/// (CLI tools, web services, etc.) without pulling in Bevy dependencies.
#[derive(Resource, Debug, Clone, Copy)]
pub struct InferenceThresholdsResource(pub InferenceThresholds);

impl Default for InferenceThresholdsResource {
    fn default() -> Self {
        Self(InferenceThresholds::default())
    }
}

impl std::ops::Deref for InferenceThresholdsResource {
    type Target = InferenceThresholds;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for InferenceThresholdsResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type InferenceJobResult = (
    Box<dyn interfaces::Detector + Send + Sync>,
    DetectorKind,
    interfaces::DetectionResult,
    f32,
    (u32, u32),
);

#[derive(Clone)]
pub struct PrimaryCameraFrame {
    pub id: u64,
    pub transform: GlobalTransform,
    pub captured_at: f64,
}

#[derive(Resource, Default)]
pub struct PrimaryCameraState {
    pub active: bool,
    pub last_transform: Option<GlobalTransform>,
    pub frame_counter: u64,
}

#[derive(Resource, Default)]
pub struct PrimaryCameraFrameBuffer {
    pub latest: Option<PrimaryCameraFrame>,
}

/// Resource tracking whether a model detector is loaded (vs. heuristic fallback).
#[derive(Resource, Default)]
pub struct ModelLoadState {
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

/// Detection result from vision_runtime async inference.
///
/// This is a runtime-specific result type that aggregates detection output.
/// Not to be confused with `vision_core::interfaces::DetectionResult`.
#[derive(Clone)]
pub struct RuntimeDetectionResult {
    pub frame_id: u64,
    pub positive: bool,
    pub confidence: f32,
    pub boxes: Vec<[f32; 4]>,
    pub scores: Vec<f32>,
}

/// Resource managing async inference task state.
///
/// Tracks pending async inference jobs, debouncing, and the most recent result.
#[derive(Resource)]
pub struct AsyncInferenceState {
    pub pending: Option<Task<InferenceJobResult>>,
    pub last_result: Option<RuntimeDetectionResult>,
    pub debounce: Timer,
}

impl Default for AsyncInferenceState {
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

pub fn setup_primary_capture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<PrimaryCameraState>,
    mut target: ResMut<PrimaryCaptureTarget>,
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
            PrimaryCaptureCamera,
            Name::new("PrimaryCaptureCamera"),
        ))
        .id();

    target.size = size;
    target.handle = handle;
    target.entity = cam_entity;
    state.active = true;
}

pub fn track_primary_camera_state(
    target: Res<PrimaryCaptureTarget>,
    mut state: ResMut<PrimaryCameraState>,
    mut buffer: ResMut<PrimaryCameraFrameBuffer>,
    cameras: Query<&GlobalTransform, With<PrimaryCaptureCamera>>,
    time: Res<Time>,
) {
    let Ok(transform) = cameras.get(target.entity) else {
        return;
    };
    state.last_transform = Some(*transform);
    state.frame_counter = state.frame_counter.wrapping_add(1);
    buffer.latest = Some(PrimaryCameraFrame {
        id: state.frame_counter,
        transform: *transform,
        captured_at: time.elapsed_secs_f64(),
    });
}

pub fn capture_primary_camera_frame(
    mode: Res<SimRunMode>,
    mut commands: Commands,
    target: Res<PrimaryCaptureTarget>,
) {
    if !matches!(*mode, SimRunMode::Datagen | SimRunMode::Inference) {
        return;
    }
    commands
        .entity(target.entity)
        .insert(Readback::texture(target.handle.clone()));
}

pub fn on_primary_capture_readback(
    ev: On<ReadbackComplete>,
    target: Res<PrimaryCaptureTarget>,
    mut readback: ResMut<PrimaryCaptureReadback>,
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
        app.insert_resource(PrimaryCaptureTarget {
            handle: Handle::default(),
            size: UVec2::ZERO,
            entity: Entity::PLACEHOLDER,
        })
        .init_resource::<PrimaryCaptureReadback>()
        .init_resource::<PrimaryCameraState>()
        .init_resource::<PrimaryCameraFrameBuffer>()
        .add_systems(Startup, setup_primary_capture)
        .add_systems(Update, track_primary_camera_state.in_set(ModeSet::Common))
        .add_systems(Update, capture_primary_camera_frame.in_set(ModeSet::Common))
        .add_observer(on_primary_capture_readback);
    }
}

// Inference ---------------------------------------------------------------

pub fn schedule_burn_inference(
    mode: Res<SimRunMode>,
    time: Res<Time>,
    mut jobs: ResMut<AsyncInferenceState>,
    mut buffer: ResMut<PrimaryCameraFrameBuffer>,
    handle: Option<ResMut<DetectorHandle>>,
    target: Res<PrimaryCaptureTarget>,
    mut readback: ResMut<PrimaryCaptureReadback>,
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
    thresh: Option<ResMut<InferenceThresholdsResource>>,
    handle: Option<ResMut<DetectorHandle>>,
    burn_loaded: Option<ResMut<ModelLoadState>>,
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
        thresh.objectness_threshold = (thresh.objectness_threshold - 0.05).clamp(0.0, 1.0);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Equal) {
        thresh.objectness_threshold = (thresh.objectness_threshold + 0.05).clamp(0.0, 1.0);
        changed = true;
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        thresh.iou_threshold = (thresh.iou_threshold - 0.05).clamp(0.1, 0.95);
        changed = true;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        thresh.iou_threshold = (thresh.iou_threshold + 0.05).clamp(0.1, 0.95);
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
            thresh.objectness_threshold, thresh.iou_threshold
        );
    }
}

/// Bevy plugin managing runtime inference coordination.
///
/// Handles async inference scheduling, model state tracking, detection overlays,
/// and threshold adjustment hotkeys. This is the runtime/visualization layer;
/// for core inference logic, see the `inference` crate.
pub struct InferenceRuntimePlugin;

impl Plugin for InferenceRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsyncInferenceState>()
            .init_resource::<ModelLoadState>()
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
        AsyncInferenceState, CapturePlugin, DetectionOverlayState, DetectorHandle, DetectorKind,
        InferenceRuntimePlugin, InferenceThresholdsResource, ModelLoadState, PrimaryCameraFrame,
        PrimaryCameraFrameBuffer, PrimaryCameraState, RuntimeDetectionResult,
    };
}
pub fn poll_inference_task(
    mut jobs: ResMut<AsyncInferenceState>,
    mut overlay: ResMut<DetectionOverlayState>,
    handle: Option<ResMut<DetectorHandle>>,
    mut burn_detector: ResMut<ModelLoadState>,
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
        jobs.last_result = Some(RuntimeDetectionResult {
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
