use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::gpu_readback::{Readback, ReadbackComplete};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use image::ImageFormat;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::polyp::PolypDetectionVotes;

#[derive(Component)]
pub struct FrontCamera;

#[derive(Component)]
pub struct FrontCaptureCamera;

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

#[derive(Clone)]
pub struct BurnDetectionResult {
    pub frame_id: u64,
    pub positive: bool,
    pub confidence: f32,
}

#[derive(Resource)]
pub struct BurnInferenceState {
    pub pending: Option<Task<BurnDetectionResult>>,
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
pub struct RecorderConfig {
    pub output_root: PathBuf,
    pub capture_interval: Timer,
    pub resolution: UVec2,
}

const MAX_LABEL_DEPTH: f32 = 8.0;

impl Default for RecorderConfig {
    fn default() -> Self {
        Self {
            output_root: PathBuf::from("assets/datasets/captures"),
            capture_interval: Timer::from_seconds(0.33, TimerMode::Repeating),
            resolution: UVec2::new(640, 360),
        }
    }
}

#[derive(Resource)]
pub struct RecorderState {
    pub enabled: bool,
    pub session_dir: PathBuf,
    pub frame_idx: u64,
    pub last_toggle: f64,
    pub last_image_ok: bool,
}

impl Default for RecorderState {
    fn default() -> Self {
        Self {
            enabled: false,
            session_dir: PathBuf::from("assets/datasets/captures/unsynced"),
            frame_idx: 0,
            last_toggle: 0.0,
            last_image_ok: false,
        }
    }
}

#[derive(Serialize)]
struct PolypLabel {
    center_world: [f32; 3],
    bbox_px: Option<[f32; 4]>,
    bbox_norm: Option<[f32; 4]>,
}

#[derive(Serialize)]
struct CaptureMetadata {
    frame_id: u64,
    sim_time: f64,
    unix_time: f64,
    image: String,
    image_present: bool,
    camera_active: bool,
    polyp_labels: Vec<PolypLabel>,
}

pub fn track_front_camera_state(
    mut state: ResMut<FrontCameraState>,
    mut votes: ResMut<PolypDetectionVotes>,
    cams: Query<(&Camera, &GlobalTransform), With<FrontCamera>>,
) {
    let mut active = false;
    let mut transform = None;
    for (cam, tf) in &cams {
        if cam.is_active {
            active = true;
            transform = Some(*tf);
            break;
        }
    }
    state.active = active;
    state.last_transform = transform;

    if !state.active {
        votes.vision = false;
    }
}

#[derive(Resource)]
pub struct FrontCaptureTarget {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub entity: Entity,
}

#[derive(Resource, Default, Clone)]
pub struct FrontCaptureReadback {
    pub latest: Option<Vec<u8>>,
}

pub fn setup_front_capture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    config: Res<RecorderConfig>,
) {
    let size = config.resolution;
    let extent = Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new_fill(
        extent,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC;
    let handle = images.add(image);
    let entity = commands
        .spawn((Name::new("FrontCaptureTarget"), Readback::texture(handle.clone())))
        .id();
    commands.insert_resource(FrontCaptureTarget {
        handle: handle.clone(),
        size,
        entity,
    });
}

pub fn capture_front_camera_frame(
    time: Res<Time>,
    mut state: ResMut<FrontCameraState>,
    mut buffer: ResMut<FrontCameraFrameBuffer>,
) {
    if !state.active {
        buffer.latest = None;
        return;
    }
    let Some(transform) = state.last_transform else {
        return;
    };
    state.frame_counter = state.frame_counter.wrapping_add(1);
    buffer.latest = Some(FrontCameraFrame {
        id: state.frame_counter,
        transform,
        captured_at: time.elapsed_secs_f64(),
    });
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

pub fn schedule_burn_inference(
    time: Res<Time>,
    mut detector: ResMut<BurnDetector>,
    mut jobs: ResMut<BurnInferenceState>,
    mut buffer: ResMut<FrontCameraFrameBuffer>,
) {
    jobs.debounce.tick(time.delta());
    if jobs.pending.is_some() || !jobs.debounce.is_finished() {
        return;
    }
    let Some(frame) = buffer.latest.take() else {
        return;
    };

    // Placeholder inference off the main thread; replace with real burn model.
    let task = AsyncComputeTaskPool::get().spawn(async move {
        let confidence = 0.8;
        let positive = true;
        BurnDetectionResult {
            frame_id: frame.id,
            positive,
            confidence,
        }
    });
    detector.model_loaded = true;
    jobs.pending = Some(task);
}

pub fn poll_burn_inference(
    mut jobs: ResMut<BurnInferenceState>,
    mut votes: ResMut<PolypDetectionVotes>,
) {
    if let Some(task) = jobs.pending.as_mut() {
        if let Some(result) = future::block_on(future::poll_once(task)) {
            votes.vision = result.positive;
            jobs.last_result = Some(result);
            jobs.pending = None;
        }
    } else if let Some(result) = jobs.last_result.as_ref() {
        votes.vision = result.positive;
    }
}

pub fn recorder_toggle_hotkey(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    config: ResMut<RecorderConfig>,
    mut state: ResMut<RecorderState>,
) {
    if !keys.just_pressed(KeyCode::KeyL) {
        return;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    state.enabled = !state.enabled;
    state.last_toggle = time.elapsed_secs_f64();
    if state.enabled {
        let session = format!("run_{}", now as u64);
        let dir = config.output_root.join(session);
        state.session_dir = dir;
        state.frame_idx = 0;
        let _ = fs::create_dir_all(&state.session_dir);
    }
}

pub fn record_front_camera_metadata(
    time: Res<Time>,
    mut config: ResMut<RecorderConfig>,
    mut state: ResMut<RecorderState>,
    buffer: Res<FrontCameraFrameBuffer>,
    front_state: Res<FrontCameraState>,
    cams: Query<(&Camera, &GlobalTransform), With<FrontCamera>>,
    capture_cams: Query<(&Camera, &GlobalTransform), With<FrontCaptureCamera>>,
    capture: Res<FrontCaptureTarget>,
    readback: Res<FrontCaptureReadback>,
    polyps: Query<&GlobalTransform, With<crate::polyp::Polyp>>,
) {
    if !state.enabled {
        return;
    }
    {
        let interval = &mut config.capture_interval;
        interval.tick(time.delta());
        if !interval.just_finished() {
            return;
        }
    }
    let Some(frame) = buffer.latest.as_ref() else {
        return;
    };
    // Prefer the capture camera (renders the PNGs) for projection to keep boxes aligned.
    let (cam, cam_tf, viewport) = if let Ok((cap_cam, cap_tf)) = capture_cams.single() {
        (cap_cam, cap_tf, Vec2::new(capture.size.x as f32, capture.size.y as f32))
    } else if let Ok((cam, tf)) = cams.single() {
        let Some(vp) = cam.logical_viewport_size() else {
            return;
        };
        (cam, tf, vp)
    } else {
        return;
    };

    let right = cam_tf.right();
    let up = cam_tf.up();
    let bbox_radius = 0.28;

    let mut labels = Vec::new();
    for tf in polyps.iter() {
        let center = tf.translation();
        let to_polyp = center - cam_tf.translation();
        let forward = (cam_tf.rotation() * -Vec3::Z).normalize_or_zero();
        let depth = forward.dot(to_polyp);
        if depth <= 0.0 || depth > MAX_LABEL_DEPTH {
            continue;
        }
        let offsets = [
            Vec3::ZERO,
            right * bbox_radius,
            -right * bbox_radius,
            up * bbox_radius,
            -up * bbox_radius,
        ];
        let mut min = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        let mut all_projected = true;
        for off in offsets {
            let world = center + off;
            if let Ok(p) = cam.world_to_viewport(cam_tf, world) {
                min = min.min(p);
                max = max.max(p);
            } else {
                all_projected = false;
                break;
            }
        }
        let bbox_px = if all_projected {
            Some([min.x, min.y, max.x, max.y])
        } else {
            None
        };
        let bbox_norm = bbox_px.map(|b| {
            [
                b[0] / viewport.x,
                b[1] / viewport.y,
                b[2] / viewport.x,
                b[3] / viewport.y,
            ]
        });
        labels.push(PolypLabel {
            center_world: [center.x, center.y, center.z],
            bbox_px,
            bbox_norm,
        });
    }

    let image_name = format!("frame_{:05}.png", state.frame_idx);
    let mut image_present = false;
    let image_path = state.session_dir.join(&image_name);
    if let Some(data) = readback.latest.as_ref() {
        let expected_len = (capture.size.x * capture.size.y * 4) as usize;
        if data.len() == expected_len
            && image::save_buffer_with_format(
                &image_path,
                data,
                capture.size.x,
                capture.size.y,
                image::ColorType::Rgba8,
                ImageFormat::Png,
            )
            .is_ok()
        {
            image_present = true;
            state.last_image_ok = true;
        } else {
            state.last_image_ok = false;
        }
    }

    let meta = CaptureMetadata {
        frame_id: frame.id,
        sim_time: frame.captured_at,
        unix_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0),
        image: image_name.clone(),
        image_present,
        camera_active: front_state.active,
        polyp_labels: labels,
    };

    let out_dir = state.session_dir.join("labels");
    let _ = fs::create_dir_all(&out_dir);
    let meta_path = out_dir.join(format!("frame_{:05}.json", state.frame_idx));
    if let Ok(serialized) = serde_json::to_string_pretty(&meta) {
        let _ = fs::write(meta_path, serialized);
    }
    state.frame_idx += 1;
}
