use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use sim_core::prelude::{ControlParams, ModeSet, ProbeSegment, SegmentSpring};

/// Register the colon-sim control systems into the sim/datagen schedule.
pub struct ControlsHookImpl;

impl sim_core::prelude::ControlsHook for ControlsHookImpl {
    fn register(&self, app: &mut App) {
        app.add_systems(Update, control_inputs_and_apply.in_set(ModeSet::SimDatagen));
    }
}

/// Handle keybinds and apply updated control parameters to the probe joints.
pub fn control_inputs_and_apply(
    keys: Res<ButtonInput<KeyCode>>,
    mut control: ResMut<ControlParams>,
    mut joints: Query<(&mut ImpulseJoint, &SegmentSpring)>,
    mut damping: Query<&mut Damping, With<ProbeSegment>>,
    mut frictions: Query<&mut Friction, With<ProbeSegment>>,
) {
    let mut changed = false;

    // Tension
    if keys.just_pressed(KeyCode::BracketLeft) {
        control.tension = (control.tension - 0.05).max(0.01);
        changed = true;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        control.tension = (control.tension + 0.05).min(5.0);
        changed = true;
    }
    // Stiffness
    if keys.just_pressed(KeyCode::Semicolon) {
        control.stiffness = (control.stiffness - 50.0).max(20.0);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Quote) {
        control.stiffness = (control.stiffness + 50.0).min(5000.0);
        changed = true;
    }
    // Damping
    if keys.just_pressed(KeyCode::Comma) {
        control.damping = (control.damping - 5.0).max(0.5);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Period) {
        control.damping = (control.damping + 5.0).min(200.0);
        changed = true;
    }
    // Thrust
    if keys.just_pressed(KeyCode::Digit1) {
        control.thrust = (control.thrust - 10.0).max(5.0);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        control.thrust = (control.thrust + 10.0).min(200.0);
        changed = true;
    }
    // Target speed
    if keys.just_pressed(KeyCode::Digit3) {
        control.target_speed = (control.target_speed - 0.1).max(0.05);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Digit4) {
        control.target_speed = (control.target_speed + 0.1).min(5.0);
        changed = true;
    }
    // Linear damping
    if keys.just_pressed(KeyCode::Digit5) {
        control.linear_damping = (control.linear_damping - 0.05).max(0.01);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Digit6) {
        control.linear_damping = (control.linear_damping + 0.05).min(2.0);
        changed = true;
    }
    // Friction
    if keys.just_pressed(KeyCode::Digit7) {
        control.friction = (control.friction - 0.1).max(0.1);
        changed = true;
    }
    if keys.just_pressed(KeyCode::Digit8) {
        control.friction = (control.friction + 0.1).min(3.0);
        changed = true;
    }

    if !changed {
        return;
    }

    // Apply to springs
    for (mut joint, rope) in &mut joints {
        if let TypedJoint::GenericJoint(ref mut spring) = joint.data {
            spring.set_motor_position(
                JointAxis::LinY,
                rope.base_rest * control.tension,
                control.stiffness,
                control.damping,
            );
            spring.set_motor_model(JointAxis::LinY, MotorModel::AccelerationBased);
            spring.set_motor_max_force(JointAxis::LinY, 10_000.0);
        }
    }

    // Apply damping/friction to segments
    for mut d in &mut damping {
        d.linear_damping = control.linear_damping;
    }
    for mut f in &mut frictions {
        f.coefficient = control.friction;
    }
}
