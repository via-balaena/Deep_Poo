use bevy::prelude::*;
use bevy_rapier3d::prelude::PhysicsSet;
use sim_core::camera::{camera_controller, pov_toggle_system, setup_camera};
use sim_core::ModeSet;

use crate::balloon_control::{
    balloon_body_update, balloon_control_input, spawn_balloon_body, spawn_balloon_marker,
};
use crate::hud::{spawn_controls_ui, spawn_detection_overlay, update_controls_ui};
use crate::polyp::{
    apply_detection_votes, polyp_detection_system, polyp_removal_system, spawn_polyps,
};
use crate::probe::{distributed_thrust, peristaltic_drive, spawn_probe};
use crate::tunnel::{cecum_detection, setup_tunnel, start_detection, tunnel_expansion_system};

/// App-specific systems plugin (domain/world/UI). Recorder wiring stays in the root orchestrator.
pub struct AppSystemsPlugin;

impl Plugin for AppSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference),
        )
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_tunnel,
                spawn_probe,
                spawn_balloon_body,
                spawn_balloon_marker,
                spawn_polyps,
                spawn_controls_ui,
                spawn_detection_overlay,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                balloon_control_input,
                balloon_body_update,
                camera_controller,
                pov_toggle_system,
                apply_detection_votes
                    .after(polyp_detection_system)
                    .after(vision_runtime::schedule_burn_inference),
            )
                .in_set(ModeSet::Common),
        )
        .add_systems(
            Update,
            (
                update_controls_ui,
                cecum_detection,
                start_detection,
                tunnel_expansion_system,
                polyp_detection_system,
                polyp_removal_system.after(polyp_detection_system),
            )
                .in_set(ModeSet::SimDatagen),
        )
        .add_systems(
            FixedUpdate,
            (
                peristaltic_drive,
                distributed_thrust.before(PhysicsSet::SyncBackend),
            ),
        );
    }
}
