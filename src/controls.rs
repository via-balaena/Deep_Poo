use bevy::prelude::*;
use bevy::ui::{PositionType, Val};
use bevy_rapier3d::prelude::*;

use crate::probe::{ProbeSegment, SegmentSpring};

#[derive(Resource, Clone)]
pub struct ControlParams {
    pub tension: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub thrust: f32,
    pub target_speed: f32,
    pub linear_damping: f32,
    pub friction: f32,
}

#[derive(Component)]
pub struct ControlText;

pub fn spawn_controls_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Controls"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            ..default()
        },
        ControlText,
        children![
            (
                TextSpan::from("Tension:"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("0.50 [ [ ] ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("Stiff: 500 [ ; ' ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("Damp: 20 [ , . ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("Thrust: 40 [ 1 2 ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("Speed: 1.20 [ 3 4 ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("Lin Damp: 0.20 [ 5 6 ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                TextSpan::from("Friction: 1.20 [ 7 8 ]"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            )
        ],
    ));
}

pub fn update_controls_ui(
    control: Res<ControlParams>,
    ui: Single<Entity, (With<ControlText>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if control.is_changed() {
        *writer.text(*ui, 1) = format!("{:.2} [ [ ] ]", control.tension);
        *writer.text(*ui, 2) = format!("Stiff: {:.0} [ ; ' ]", control.stiffness);
        *writer.text(*ui, 3) = format!("Damp: {:.1} [ , . ]", control.damping);
        *writer.text(*ui, 4) = format!("Thrust: {:.1} [ 1 2 ]", control.thrust);
        *writer.text(*ui, 5) = format!("Speed: {:.2} [ 3 4 ]", control.target_speed);
        *writer.text(*ui, 6) = format!("Lin Damp: {:.2} [ 5 6 ]", control.linear_damping);
        *writer.text(*ui, 7) = format!("Friction: {:.2} [ 7 8 ]", control.friction);
    }
}

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
