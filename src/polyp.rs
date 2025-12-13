use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::tunnel::{advance_centerline, tunnel_tangent_rotation, TUNNEL_LENGTH, TUNNEL_START_Z};
use crate::probe::{ProbeHead, PROBE_BASE_LENGTH, PROBE_START_TAIL_Z};

#[derive(Component)]
pub struct Polyp {
    pub removed: bool,
    pub base_color: Color,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct PolypTelemetry {
    pub total: usize,
    pub remaining: usize,
    pub nearest_distance: Option<f32>,
    pub nearest_entity: Option<Entity>,
    pub detected: bool,
    pub removing: bool,
    pub remove_progress: f32,
}

#[derive(Resource, Default)]
pub struct PolypRemoval {
    pub target: Option<Entity>,
    pub timer: Timer,
    pub in_progress: bool,
}

fn hash01(i: u32) -> f32 {
    // Simple deterministic hash to pseudo-random [0,1).
    let x = (i as f32 * 12.9898 + 78.233).sin() * 43758.5453;
    x.fract()
}

pub fn spawn_polyps(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Keep all polyps ahead of the initial probe head.
    let (_, _, head_start_z) = advance_centerline(PROBE_START_TAIL_Z, PROBE_BASE_LENGTH);
    let count = 14;
    let margin = 6.0;
    let usable_length = TUNNEL_LENGTH - margin * 2.0;
    let spacing = usable_length / (count as f32);
    let mesh = meshes.add(Mesh::from(bevy::math::primitives::Sphere { radius: 0.2 }));

    let mut total = 0;

    for i in 0..count {
        let z_offset = margin + spacing * i as f32;
        let (center, tangent, _) = advance_centerline(TUNNEL_START_Z, z_offset);

        // Skip any position that would spawn behind the initial head.
        if center.z < head_start_z {
            continue;
        }
        let basis = tunnel_tangent_rotation(tangent);
        let right = basis * Vec3::X;
        let up = basis * Vec3::Y;

        // Radial placement around the tunnel wall.
        let angle = hash01(i as u32) * std::f32::consts::TAU;
        let radial_dir = (right * angle.cos() + up * angle.sin()).normalize_or_zero();
        let radial_offset = 0.85 + 0.2 * hash01((i * 17) as u32);
        let pos = center + radial_dir * radial_offset;

        let base_color = Color::srgba(0.9, 0.1, 0.85, 1.0);
        let bc = base_color.to_srgba();
        let emissive_base = Color::srgba(bc.red * 0.7, bc.green * 0.25, bc.blue * 0.7, bc.alpha);
        let material = materials.add(StandardMaterial {
            base_color,
            emissive: emissive_base.into(),
            perceptual_roughness: 0.4,
            metallic: 0.0,
            ..default()
        });

        commands.spawn((
            Polyp {
                removed: false,
                base_color,
            },
            Collider::ball(0.2),
            Sensor,
            CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform {
                translation: pos,
                ..default()
            },
            GlobalTransform::default(),
            Visibility::default(),
        ));

        // Optional halo to help visual identification.
        total += 1;
    }

    commands.insert_resource(PolypTelemetry {
        total,
        remaining: total,
        nearest_distance: None,
        nearest_entity: None,
        detected: false,
        removing: false,
        remove_progress: 0.0,
    });
}

pub fn polyp_detection_system(
    head_q: Query<&GlobalTransform, With<ProbeHead>>,
    mut polyps: Query<(
        Entity,
        &Polyp,
        &GlobalTransform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut telemetry: ResMut<PolypTelemetry>,
) {
    let Ok(head_tf) = head_q.single() else {
        return;
    };
    let head_pos = head_tf.translation();
    // Use +Z basis to align with tunnel tangent (Bevy's forward() uses -Z).
    let (_, head_rot, _) = head_tf.to_scale_rotation_translation();
    let head_forward = (head_rot * Vec3::Z).normalize_or_zero();

    let mut remaining = 0usize;
    let mut nearest: Option<(f32, Entity)> = None;

    for (entity, polyp, tf, mut mat_handle) in polyps.iter_mut() {
        if polyp.removed {
            continue;
        }
        remaining += 1;

        let to_polyp = tf.translation() - head_pos;
        let dist = to_polyp.length();
        let direction_ok = if dist > 0.001 {
            // Allow close-range hits regardless of angle; wider cone otherwise.
            head_forward.dot(to_polyp / dist) > 0.2 || dist < 1.2
        } else {
            false
        };
        let in_range = dist <= 3.5;
        let detected = direction_ok && in_range;

        if (detected || dist < 1.2) && nearest.map_or(true, |(d, _)| dist < d) {
            nearest = Some((dist, entity));
        }

        if let Some(mat) = materials.get_mut(&mut mat_handle.0) {
            if detected {
                mat.base_color = Color::srgba(1.0, 0.35, 0.95, 1.0);
                mat.emissive = Color::srgba(1.0 * 0.7, 0.35 * 0.6, 0.95 * 0.7, 1.0).into();
            } else {
                mat.base_color = polyp.base_color;
                let bc = polyp.base_color.to_srgba();
                mat.emissive = Color::srgba(bc.red * 0.7, bc.green * 0.25, bc.blue * 0.7, bc.alpha).into();
            }
        }
    }

    telemetry.remaining = remaining;
    telemetry.detected = nearest.is_some();
    telemetry.nearest_distance = nearest.map(|(d, _)| d);
    telemetry.nearest_entity = nearest.map(|(_, e)| e);
    if !telemetry.removing {
        telemetry.remove_progress = 0.0;
    }
}

pub fn polyp_removal_system(
    time: Res<Time>,
    mut telemetry: ResMut<PolypTelemetry>,
    mut removal: ResMut<PolypRemoval>,
    mut polyps: Query<(Entity, &mut Polyp, Option<&mut Collider>, Option<&mut Visibility>)>,
) {
    // Start removal when a polyp is in reach and we are idle.
    if !removal.in_progress {
        if let Some(target) = telemetry
            .nearest_entity
            .filter(|_| telemetry.nearest_distance.map_or(false, |d| d <= 1.2))
        {
            removal.target = Some(target);
            removal.timer = Timer::from_seconds(1.5, TimerMode::Once);
            removal.in_progress = true;
            telemetry.removing = true;
            telemetry.remove_progress = 0.0;
        }
    }

    if removal.in_progress {
        removal.timer.tick(time.delta());
        telemetry.remove_progress = removal.timer.fraction();
        telemetry.removing = true;

        if removal.timer.is_finished() {
            if let Some(target) = removal.target.take() {
                if let Ok((_, mut polyp, collider, vis)) = polyps.get_mut(target) {
                    polyp.removed = true;
                    if let Some(mut c) = collider {
                        *c = Collider::ball(0.001);
                    }
                    if let Some(mut v) = vis {
                        *v = Visibility::Hidden;
                    }
                    telemetry.remaining = telemetry.remaining.saturating_sub(1);
                }
            }
            removal.in_progress = false;
            telemetry.removing = false;
            telemetry.remove_progress = 0.0;
            telemetry.nearest_entity = None;
            telemetry.nearest_distance = None;
            telemetry.detected = false;
        }
    }
}
