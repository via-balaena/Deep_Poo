use bevy::prelude::*;

use crate::autopilot_types::{AutoDrive, DataRun, DatagenInit};
use crate::recorder::{
    AutoRecordTimer, Config as RecorderConfig, Motion as RecorderMotion, State as RecorderState,
};
use crate::ModeSet;

/// Registers common runtime resources and ensures mode sets exist.
pub struct SimRuntimePlugin;

impl Plugin for SimRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference),
        )
        .insert_resource(AutoDrive::default())
        .insert_resource(DataRun::default())
        .insert_resource(DatagenInit::default())
        .insert_resource(RecorderConfig::default())
        .insert_resource(RecorderState::default())
        .insert_resource(RecorderMotion::default())
        .insert_resource(AutoRecordTimer::default());
    }
}

/// Convenience to register runtime defaults without adding the plugin type directly.
pub fn register_runtime_systems(app: &mut App) {
    SimRuntimePlugin.build(app);
}
