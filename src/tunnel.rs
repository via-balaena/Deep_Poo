use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

#[derive(Resource)]
#[allow(dead_code)]
pub struct TunnelRoot(pub Entity);

pub fn spawn_tunnel(mut commands: Commands) {
    // Cylindrical-ish tunnel built from ring segments to avoid the "+" look.
    let inner_radius = 0.9; // slightly wider clearance over probe collider radius (~0.8)
    let wall_thickness = 0.05;
    let base_length = 12.0;
    let length_a = base_length * 5.0;
    let trans_len = base_length * 0.5; // smoother elbow
    let length_b = base_length;
    let segments = 16;

    let wall_half = wall_thickness * 0.5;
    let ring_radius = inner_radius + wall_half;
    let angle_step = TAU / segments as f32;
    let tangent_half = inner_radius * (angle_step * 0.5).tan() + wall_half;

    // Spread the elbow over multiple smaller bends (3 x ~10 degrees) for a sharper curve.
    let elbow_step = 10.0_f32.to_radians();

    let tunnel_id = commands
        .spawn((
            Name::new("Tunnel"),
            RigidBody::Fixed,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .with_children(|child| {
            let sections = [
                ("Tunnel A", Quat::IDENTITY, length_a),
                ("Elbow 1", Quat::from_rotation_x(elbow_step), trans_len),
                ("Elbow 2", Quat::from_rotation_x(elbow_step), trans_len),
                ("Tunnel B", Quat::from_rotation_x(elbow_step), length_b),
            ];

            let mut origin = Vec3::ZERO;
            let mut rot = Quat::IDENTITY;

            for (name, delta_rot, len) in sections {
                let dir = rot * Vec3::Z;
                let half_len = len * 0.5;
                let center = origin + dir * half_len;

                child
                    .spawn((
                        Name::new(name),
                        Transform {
                            translation: center,
                            rotation: rot,
                            ..default()
                        },
                        GlobalTransform::default(),
                    ))
                    .with_children(|section| {
                        for i in 0..segments {
                            let angle = i as f32 * angle_step;
                            let dir2 = Vec2::new(angle.cos(), angle.sin());
                            let center2 =
                                Vec3::new(dir2.x * ring_radius, dir2.y * ring_radius, 0.0);
                            let rot2 = Quat::from_rotation_z(angle);

                            section.spawn((
                                Name::new(format!("{name} Segment {i}")),
                                Collider::cuboid(wall_half, tangent_half, half_len),
                                Friction {
                                    coefficient: 1.2,
                                    combine_rule: CoefficientCombineRule::Average,
                                    ..default()
                                },
                                Transform::from_translation(center2).with_rotation(rot2),
                                GlobalTransform::default(),
                            ));
                        }
                    });

                origin += dir * len;
                rot = rot * delta_rot;
            }
        })
        .id();

    commands.insert_resource(TunnelRoot(tunnel_id));
}
