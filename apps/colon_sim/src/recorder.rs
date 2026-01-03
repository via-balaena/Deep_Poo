use bevy::prelude::*;

use crate::probe::ProbeHead;
use crate::tunnel::CecumState;
use sim_core::recorder_meta::RecorderWorldState;
use sim_core::SimRunMode;

/// Update RecorderWorldState with probe head_z and stop flag from app state.
pub fn update_recorder_world_state(
    mode: Res<SimRunMode>,
    mut world_state: ResMut<RecorderWorldState>,
    heads: Query<&GlobalTransform, With<ProbeHead>>,
    cecum: Res<CecumState>,
) {
    if !matches!(*mode, SimRunMode::Sim | SimRunMode::Datagen) {
        return;
    }
    world_state.head_z = heads.single().ok().map(|tf| tf.translation().z);
    world_state.stop_flag = cecum.reached;
}
