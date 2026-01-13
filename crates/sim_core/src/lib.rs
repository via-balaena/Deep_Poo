use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use std::path::PathBuf;

/// High-level run mode for the sim runtime (detector-free).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimRunMode {
    #[default]
    Sim,
    Datagen,
    Inference,
}

/// Common configuration for the sim runtime.
#[derive(Resource, Debug, Clone)]
pub struct SimConfig {
    pub mode: SimRunMode,
    pub headless: bool,
    pub capture_output_root: PathBuf,
    pub prune_empty: bool,
    pub prune_output_root: Option<PathBuf>,
    pub max_frames: Option<u32>,
    pub capture_interval_secs: Option<f32>,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            mode: SimRunMode::Sim,
            headless: false,
            capture_output_root: PathBuf::from("assets/datasets/captures"),
            prune_empty: false,
            prune_output_root: None,
            max_frames: None,
            capture_interval_secs: None,
        }
    }
}

/// System sets for the core sim scheduling.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModeSet {
    Common,
    SimDatagen,
    Inference,
}

/// Core sim plugin: registers mode-based system sets and injects default config.
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimConfig::default()).configure_sets(
            Update,
            (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference),
        );
    }
}

pub mod prelude {
    pub use super::{ModeSet, SimConfig, SimPlugin, SimRunMode};
    pub use crate::autopilot_types::{AutoDir, AutoDrive, AutoStage, DataRun, DatagenInit};
    pub use crate::camera::{
        camera_controller, pov_toggle_system, setup_camera, Flycam, InstrumentPovCamera, PovState,
        UiOverlayCamera,
    };
    pub use crate::controls::ControlParams;
    pub use crate::hooks::{AutopilotHook, ControlsHook, SimHooks};
    pub use crate::articulated_types::{ArticulatedSegment, SegmentSpring};
    pub use crate::recorder_meta::{
        BasicRecorderMeta, RecorderMetaProvider, RecorderMetadataProvider, RecorderSink,
        RecorderWorldState,
    };
    pub use crate::recorder_types::{
        AutoRecordTimer, RecorderConfig, RecorderMotion, RecorderState,
    };
    pub use crate::runtime::{register_runtime_systems, SimRuntimePlugin};
}

/// Build a base Bevy `App` with sim mode sets and config. Detector wiring is intentionally omitted.
pub fn build_app(sim_config: SimConfig) -> App {
    let mut app = App::new();
    let headless = sim_config.headless;
    app.insert_resource(sim_config)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: !headless,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .configure_sets(
            Update,
            (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference),
        );
    app
}

pub mod articulated_types;
pub mod autopilot_types;
pub mod camera;
pub mod controls;
pub mod hooks;
pub mod recorder_meta;
pub mod recorder_types;
pub mod runtime;
