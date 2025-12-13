pub mod camera;
pub mod balloon_control;
pub mod controls;
pub mod hud;
pub mod autopilot;
pub mod polyp;
pub mod probe;
pub mod tunnel;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use balloon_control::{
    balloon_body_update, balloon_control_input, balloon_marker_update, spawn_balloon_body,
    spawn_balloon_marker, BalloonControl,
};
use autopilot::{auto_inchworm, auto_toggle, AutoDrive};
use camera::{camera_controller, setup_camera};
use controls::{control_inputs_and_apply, ControlParams};
use hud::{spawn_controls_ui, update_controls_ui};
use polyp::{polyp_detection_system, polyp_removal_system, spawn_polyps, PolypRemoval, PolypTelemetry};
use probe::{distributed_thrust, peristaltic_drive, spawn_probe, StretchState, TipSense};
use tunnel::{setup_tunnel, tunnel_expansion_system, cecum_detection, start_detection, CecumState};

pub fn run_app() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            brightness: 0.4,
            affects_lightmapped_meshes: true,
        })
        .insert_resource(BalloonControl::default())
        .insert_resource(StretchState::default())
        .insert_resource(TipSense::default())
        .insert_resource(PolypTelemetry::default())
        .insert_resource(PolypRemoval::default())
        .insert_resource(AutoDrive::default())
        .insert_resource(CecumState::default())
        .insert_resource(ControlParams {
            tension: 0.5,
            stiffness: 500.0,
            damping: 20.0,
            thrust: 40.0,
            target_speed: 1.2,
            linear_damping: 0.2,
            friction: 1.2,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                setup_camera,
                spawn_environment,
                disable_gravity,
                setup_tunnel,
                spawn_probe,
                spawn_balloon_body,
                spawn_balloon_marker,
                spawn_polyps,
                spawn_controls_ui,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                balloon_control_input,
                balloon_body_update,
                auto_toggle,
                auto_inchworm,
                balloon_marker_update,
                camera_controller,
                control_inputs_and_apply,
                update_controls_ui,
                cecum_detection,
                start_detection,
                tunnel_expansion_system,
                polyp_detection_system,
                polyp_removal_system.after(polyp_detection_system),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                peristaltic_drive,
                distributed_thrust.before(PhysicsSet::SyncBackend),
            ),
        )
        .run();
}

fn spawn_environment(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 15_000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn disable_gravity(mut configs: Query<&mut RapierConfiguration, With<DefaultRapierContext>>) {
    for mut config in &mut configs {
        config.gravity = Vec3::new(0.0, -0.5, 0.0);
    }
}
