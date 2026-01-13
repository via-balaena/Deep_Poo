//! Recorder subsystem types for data capture and metadata management.
//!
//! This module consolidates all recorder-related types previously split across
//! `recorder_types` and `recorder_meta` modules for clearer organization.

use bevy::math::UVec2;
use bevy::prelude::{Resource, Timer, TimerMode};
use std::path::PathBuf;
use vision_core::prelude::Recorder;

// Configuration and State ------------------------------------------------

/// Configuration resource for the recorder subsystem.
#[derive(Resource)]
pub struct Config {
    pub output_root: PathBuf,
    pub capture_interval: Timer,
    pub resolution: UVec2,
    pub prune_empty: bool,
    pub prune_output_root: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_root: PathBuf::from("assets/datasets/captures"),
            capture_interval: Timer::from_seconds(0.33, TimerMode::Repeating),
            resolution: UVec2::new(640, 360),
            prune_empty: false,
            prune_output_root: None,
        }
    }
}

/// Runtime state resource for the recorder subsystem.
#[derive(Resource)]
pub struct State {
    pub enabled: bool,
    pub session_dir: PathBuf,
    pub frame_idx: u64,
    pub last_toggle: f64,
    pub last_image_ok: bool,
    pub paused: bool,
    pub overlays_done: bool,
    pub prune_done: bool,
    pub initialized: bool,
    pub manifest_written: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            enabled: false,
            session_dir: PathBuf::from("assets/datasets/captures/unsynced"),
            frame_idx: 0,
            last_toggle: 0.0,
            last_image_ok: false,
            paused: false,
            overlays_done: false,
            prune_done: false,
            initialized: false,
            manifest_written: false,
        }
    }
}

/// Motion tracking resource for recorder triggers.
#[derive(Resource, Default)]
pub struct Motion {
    pub last_head_z: Option<f32>,
    pub cumulative_forward: f32,
    pub started: bool,
}

/// Auto-record timer resource.
#[derive(Resource)]
pub struct AutoRecordTimer {
    pub timer: Timer,
}

impl Default for AutoRecordTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(30.0, TimerMode::Once),
        }
    }
}

// Metadata Management ----------------------------------------------------

/// Trait for providing metadata to the recorder (e.g., label seeds).
pub trait MetadataProvider: Send + Sync + 'static {
    fn label_seed(&self) -> u64;
}

/// Resource holding a boxed metadata provider.
#[derive(Resource)]
pub struct MetaProvider {
    pub provider: Box<dyn MetadataProvider>,
}

/// Basic metadata provider implementation with a configurable seed.
#[derive(Default)]
pub struct BasicMeta {
    pub seed: u64,
}

impl MetadataProvider for BasicMeta {
    fn label_seed(&self) -> u64 {
        self.seed
    }
}

// Integration Types ------------------------------------------------------

/// Resource holding the active recorder sink (trait object).
#[derive(Resource, Default)]
pub struct Sink {
    pub writer: Option<Box<dyn Recorder + Send + Sync>>,
}

/// App-provided world state for recorder triggers (head position, stop flag).
#[derive(Resource, Default)]
pub struct WorldState {
    pub head_z: Option<f32>,
    pub stop_flag: bool,
}
