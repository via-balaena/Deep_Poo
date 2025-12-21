use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use crate::polyp::PolypDetectionVotes;

#[derive(Component)]
pub struct FrontCamera;

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
