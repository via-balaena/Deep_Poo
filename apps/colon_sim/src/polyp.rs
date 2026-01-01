use bevy::math::primitives::{Capsule3d, Sphere};
use bevy::prelude::*;
use bevy_mesh::primitives::Meshable;
use bevy_rapier3d::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::probe::{ProbeHead, PROBE_BASE_LENGTH, PROBE_START_TAIL_Z};
use crate::tunnel::{
    advance_centerline, tunnel_tangent_rotation, wall_base_color, TUNNEL_BASE_RADIUS,
    TUNNEL_LENGTH, TUNNEL_START_Z,
};

#[derive(Component)]
pub struct Polyp {
    pub removed: bool,
    pub base_color: Color,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct PolypDetectionVotes {
    pub classic: bool,
    pub vision: bool,
}

impl PolypDetectionVotes {
    pub fn consensus(&self) -> bool {
        self.classic && self.vision
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct PolypTelemetry {
    pub total: usize,
    pub remaining: usize,
    pub nearest_distance: Option<f32>,
    pub nearest_entity: Option<Entity>,
    pub detected: bool,
    pub classic_detected: bool,
    pub vision_detected: bool,
    pub consensus_ready: bool,
    pub removing: bool,
    pub remove_progress: f32,
}

#[derive(Resource, Default)]
pub struct PolypRemoval {
    pub target: Option<Entity>,
    pub timer: Timer,
    pub in_progress: bool,
}

#[derive(Resource)]
pub struct PolypRandom {
    seed: u64,
    rng: StdRng,
}

impl PolypRandom {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }
    pub fn seed_from_env_or_time() -> u64 {
        std::env::var("POLYP_SEED")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or_else(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(1)
            })
    }
    pub fn from_env_or_time() -> Self {
        let seed = Self::seed_from_env_or_time();
        Self::new(seed)
    }
    pub fn seed(&self) -> u64 {
        self.seed
    }
    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }
}

#[derive(Resource, Clone, Copy)]
pub struct PolypSpawnMeta {
    pub seed: u64,
}

fn finite_vec3(v: Vec3) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}

pub fn spawn_polyps(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polyp_rng: ResMut<PolypRandom>,
    mut spawn_meta: ResMut<PolypSpawnMeta>,
) {
    spawn_meta.seed = polyp_rng.seed();
    // Keep all polyps ahead of the initial probe head.
    let (_, _, head_start_z) = advance_centerline(PROBE_START_TAIL_Z, PROBE_BASE_LENGTH);
    let rng = polyp_rng.rng();
    let count = rng.gen_range(12..=20);
    let margin = rng.gen_range(4.5..7.5);
    let usable_length = TUNNEL_LENGTH - margin * 2.0;
    let spacing = usable_length / (count as f32 + rng.gen_range(-2.0..2.0));
    #[derive(Clone)]
    enum Variant {
        UvSphere {
            mesh: Handle<Mesh>,
            bump: Option<Handle<Mesh>>,
        },
        IcoSphere {
            mesh: Handle<Mesh>,
        },
        Capsule {
            mesh: Handle<Mesh>,
        },
        SquashSphere {
            mesh: Handle<Mesh>,
        },
        HeadStalk {
            stalk: Handle<Mesh>,
            head: Handle<Mesh>,
        },
    }

    impl Variant {
        fn base_radius(&self) -> f32 {
            match self {
                Variant::UvSphere { .. } => 0.22,
                Variant::IcoSphere { .. } => 0.22,
                Variant::Capsule { .. } => 0.28,
                Variant::SquashSphere { .. } => 0.24,
                Variant::HeadStalk { .. } => 0.26,
            }
        }
    }

    let mesh_variants: Vec<Variant> = vec![
        Variant::UvSphere {
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.2 })),
            bump: Some(meshes.add(Mesh::from(Sphere { radius: 0.12 }))),
        },
        Variant::IcoSphere {
            mesh: meshes.add(Sphere { radius: 0.2 }.mesh().ico(3).unwrap()),
        },
        Variant::Capsule {
            mesh: meshes.add(Mesh::from(Capsule3d {
                radius: 0.14,
                half_length: 0.14,
            })),
        },
        Variant::SquashSphere {
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.22 })),
        },
        Variant::HeadStalk {
            stalk: meshes.add(Mesh::from(Capsule3d {
                radius: 0.08,
                half_length: 0.18,
            })),
            head: meshes.add(Mesh::from(Sphere { radius: 0.16 })),
        },
    ];

    let mut total = 0;

    for i in 0..count {
        let z_offset = margin + spacing * i as f32 + rng.gen_range(-0.4..0.4);
        let (center, tangent, _) = advance_centerline(TUNNEL_START_Z, z_offset);

        // Skip any position that would spawn behind the initial head.
        if center.z < head_start_z {
            continue;
        }
        let basis = tunnel_tangent_rotation(tangent);
        let right = basis * Vec3::X;
        let up = basis * Vec3::Y;

        // Radial placement around the tunnel wall.
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radial_dir = {
            let dir = (right * angle.cos() + up * angle.sin()).normalize_or_zero();
            if dir.length_squared() < 1e-6 {
                Vec3::X
            } else {
                dir
            }
        };

        let size_roll: f32 = rng.r#gen::<f32>();
        let size_r: f32 = rng.r#gen::<f32>();
        let (scale_min, scale_max) = if size_roll < 0.55 {
            (0.6, 1.05)
        } else if size_roll < 0.85 {
            (0.95, 1.8)
        } else {
            (1.6, 2.8)
        };
        let t = size_r * size_r;
        let mut scale = scale_min + (scale_max - scale_min) * t;
        if !scale.is_finite() {
            continue;
        }
        scale = scale.clamp(0.5, 3.0);

        let shape_roll: f32 = rng.r#gen::<f32>();
        let shape_idx = (shape_roll * mesh_variants.len() as f32)
            .floor()
            .clamp(0.0, (mesh_variants.len() - 1) as f32) as usize;
        let variant = &mesh_variants[shape_idx];
        let base_radius_scaled = variant.base_radius() * scale;
        // Seat the polyp near the wall and protrude inward; keep it inside the lumen and avoid tunneling outside.
        let radial_offset = (TUNNEL_BASE_RADIUS - base_radius_scaled * 0.25)
            .clamp(0.5, TUNNEL_BASE_RADIUS - base_radius_scaled * 0.2);
        let pos = center + radial_dir * radial_offset;

        let wall_color = wall_base_color().to_srgba();
        let jitter = |base: f32, rng: &mut StdRng| -> f32 {
            (base + rng.gen_range(-0.08..0.08)).clamp(0.0, 1.0)
        };
        let r = jitter(wall_color.red, rng);
        let g = jitter(wall_color.green, rng);
        let b = jitter(wall_color.blue, rng);
        let base_color = Color::srgba(r, g, b, 1.0);
        let bc = base_color.to_srgba();
        let emissive_base = Color::srgba(bc.red * 0.6, bc.green * 0.35, bc.blue * 0.6, 1.0);
        let material = materials.add(StandardMaterial {
            base_color,
            emissive: emissive_base.into(),
            perceptual_roughness: 0.4,
            metallic: 0.0,
            ..default()
        });

        let twist = Quat::from_axis_angle(
            radial_dir,
            rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI),
        );
        let align = Quat::from_rotation_arc(Vec3::Y, radial_dir);
        let mut root_transform = Transform {
            translation: pos,
            rotation: align * twist,
            ..default()
        };
        let mut children: Vec<(Handle<Mesh>, Transform)> = Vec::new();
        let root_mesh = match variant {
            Variant::UvSphere { mesh, bump } => {
                root_transform.scale = Vec3::splat(scale);
                if let Some(bump_mesh) = bump {
                    children.push((
                        bump_mesh.clone(),
                        Transform {
                            translation: Vec3::new(0.06, 0.16, 0.02),
                            scale: Vec3::splat(0.7 * scale),
                            ..default()
                        },
                    ));
                }
                mesh.clone()
            }
            Variant::IcoSphere { mesh } => {
                root_transform.scale = Vec3::splat(scale);
                mesh.clone()
            }
            Variant::Capsule { mesh } => {
                root_transform.scale = Vec3::splat(scale);
                mesh.clone()
            }
            Variant::SquashSphere { mesh } => {
                root_transform.scale = Vec3::new(scale, scale * 0.6, scale);
                mesh.clone()
            }
            Variant::HeadStalk { stalk, head } => {
                root_transform.scale = Vec3::new(scale * 0.8, scale * 1.1, scale * 0.8);
                children.push((
                    head.clone(),
                    Transform {
                        translation: Vec3::Y * 0.22,
                        scale: Vec3::splat(scale),
                        ..default()
                    },
                ));
                stalk.clone()
            }
        };

        if !finite_vec3(pos) {
            continue;
        }

        let collider_radius = (base_radius_scaled * 0.9).max(0.05);
        let mut entity = commands.spawn((
            Polyp {
                removed: false,
                base_color,
            },
            Collider::ball(collider_radius),
            Sensor,
            CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2),
            Mesh3d(root_mesh),
            MeshMaterial3d(material.clone()),
            root_transform,
            GlobalTransform::default(),
            Visibility::default(),
        ));

        if !children.is_empty() {
            entity.with_children(|child_builder| {
                for (mesh, transform) in children.iter() {
                    child_builder.spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        transform.clone(),
                        GlobalTransform::default(),
                        Visibility::default(),
                    ));
                }
            });
        }

        // Optional halo to help visual identification.
        total += 1;
    }

    commands.insert_resource(PolypTelemetry {
        total,
        remaining: total,
        nearest_distance: None,
        nearest_entity: None,
        detected: false,
        classic_detected: false,
        vision_detected: false,
        consensus_ready: false,
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
    mut votes: ResMut<PolypDetectionVotes>,
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
        let in_range = dist <= 4.0;
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
                mat.emissive =
                    Color::srgba(bc.red * 0.7, bc.green * 0.25, bc.blue * 0.7, bc.alpha).into();
            }
        }
    }

    telemetry.remaining = remaining;
    telemetry.classic_detected = nearest.is_some();
    votes.classic = telemetry.classic_detected;
    telemetry.vision_detected = votes.vision;
    telemetry.detected = telemetry.classic_detected || telemetry.vision_detected;
    telemetry.consensus_ready = votes.consensus();
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
    votes: Res<PolypDetectionVotes>,
    mut polyps: Query<(
        Entity,
        &GlobalTransform,
        &mut Polyp,
        Option<&mut Collider>,
        Option<&mut Visibility>,
    )>,
    cam_q: Query<&GlobalTransform, With<sim_core::camera::ProbePovCamera>>,
    head_q: Query<&GlobalTransform, With<crate::probe::ProbeHead>>,
) {
    let removal_center = if let Ok(cam_tf) = cam_q.single() {
        let forward = (cam_tf.rotation() * -Vec3::Z).normalize_or_zero();
        cam_tf.translation() + forward * 0.6
    } else if let Ok(head_tf) = head_q.single() {
        let forward = (head_tf.rotation() * Vec3::Z).normalize_or_zero();
        head_tf.translation() + forward * 0.6
    } else {
        Vec3::ZERO
    };
    let removal_radius = 1.2;

    // Start removal when a polyp is in reach and we are idle.
    if !removal.in_progress && votes.consensus() {
        if let Some(target) = telemetry.nearest_entity {
            if let Ok((_, tf, _, _, _)) = polyps.get_mut(target) {
                let dist = tf.translation().distance(removal_center);
                let head_reach = telemetry.nearest_distance.map_or(false, |d| d <= 1.3);
                if dist <= removal_radius || head_reach {
                    removal.target = Some(target);
                    removal.timer = Timer::from_seconds(1.5, TimerMode::Once);
                    removal.in_progress = true;
                    telemetry.removing = true;
                    telemetry.remove_progress = 0.0;
                }
            }
        }
    }

    if removal.in_progress {
        removal.timer.tick(time.delta());
        telemetry.remove_progress = removal.timer.fraction();
        telemetry.removing = true;

        if removal.timer.is_finished() {
            if let Some(target) = removal.target.take() {
                if let Ok((_, _, mut polyp, collider, vis)) = polyps.get_mut(target) {
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

pub fn apply_detection_votes(
    votes: Res<PolypDetectionVotes>,
    mut telemetry: ResMut<PolypTelemetry>,
) {
    let union = votes.classic || votes.vision;
    let consensus = votes.consensus();
    if telemetry.detected != union
        || telemetry.consensus_ready != consensus
        || telemetry.classic_detected != votes.classic
        || telemetry.vision_detected != votes.vision
    {
        telemetry.detected = union;
        telemetry.classic_detected = votes.classic;
        telemetry.vision_detected = votes.vision;
        telemetry.consensus_ready = consensus;
    }
}
