use bevy::math::primitives::Cylinder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::plugin::ReadRapierContext;
use std::f32::consts::FRAC_PI_2;

use crate::controls::ControlParams;
use crate::balloon_control::BalloonControl;
use crate::autopilot::AutoDrive;
use crate::polyp::{PolypRemoval, PolypTelemetry};
use crate::tunnel::{advance_centerline, tunnel_centerline, tunnel_tangent_rotation};

pub const MIN_STRETCH: f32 = 1.0;
// Allow stretching up to +68% of the deflated length.
pub const MAX_STRETCH: f32 = 1.68;
const STRETCH_RATE: f32 = 0.2; // slowed to ~1/3 speed
const RETRACT_RATE: f32 = 0.3;
pub const PROBE_BASE_LENGTH: f32 = 25.0;
pub const PROBE_START_TAIL_Z: f32 = -10.0;

#[derive(Resource, Default)]
pub struct StretchState {
    pub factor: f32,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct TipSense {
    pub pressure_world: Vec3,
    pub pressure_local: Vec3,
    pub steer_dir: Vec3,
    pub steer_strength: f32,
}

#[derive(Component)]
pub struct CapsuleProbe;

#[derive(Component)]
pub struct ProbeTip;

#[derive(Component)]
pub struct ProbeHead;

#[derive(Component)]
pub struct ProbeSegment;

#[derive(Component)]
pub struct ProbeParam {
    pub tail_z: f32,
}

#[derive(Component)]
pub struct SegmentSpring {
    pub base_rest: f32,
}

#[derive(Component)]
pub struct SegmentIndex(pub usize);

#[derive(Component)]
pub struct ProbeBody {
    pub base_radius: f32,
    pub base_length: f32,
    pub ring_count: usize,
}

#[derive(Component)]
pub struct ProbeRing {
    pub index: usize,
}

#[derive(Component)]
pub struct ProbeVisualSegment {
    pub index: usize,
}

fn ring_shell_collider(radius: f32, half_height: f32) -> Collider {
    let wall_thickness = 0.08;
    let segments = 16;
    let angle_step = std::f32::consts::TAU / segments as f32;
    let wall_half = wall_thickness * 0.5;
    let tangent_half = radius * (angle_step * 0.5).tan() + wall_half;

    let mut shapes = Vec::with_capacity(segments);
    for i in 0..segments {
        let angle = i as f32 * angle_step;
        let dir = Vec2::new(angle.cos(), angle.sin());
        let center = Vec3::new(dir.x * radius, dir.y * radius, 0.0);
        let rot = Quat::from_rotation_z(angle);
        shapes.push((center, rot, Collider::cuboid(wall_half, tangent_half, half_height)));
    }

    Collider::compound(shapes)
}

pub fn spawn_probe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    control: Res<ControlParams>,
) {
    // Elastic probe tube built from ring colliders (like the tunnel) driven by stretch.
    let base_radius = 0.8;
    let base_length = PROBE_BASE_LENGTH;
    let ring_count = 24usize;
    let ring_spacing = base_length / ring_count as f32;
    let ring_half_height = ring_spacing * 0.45;
    // Keep similar placement to previous chain: tail back near -16, tip ahead in the straight.
    let tail_z = PROBE_START_TAIL_Z;
    let (tail_center, tail_tangent) = tunnel_centerline(tail_z);

    let visual_mesh = meshes.add(Mesh::from(Cylinder {
        radius: base_radius * 0.9,
        half_height: base_length * 0.5,
    }));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.6, 1.0),
        emissive: Color::srgb(0.05, 0.3, 0.8).into(),
        perceptual_roughness: 0.15,
        metallic: 0.1,
        unlit: true,
        ..default()
    });

    let mut root = commands.spawn((
        ProbeBody {
            base_radius,
            base_length,
            ring_count,
        },
        CapsuleProbe,
        ProbeParam { tail_z },
        RigidBody::KinematicPositionBased,
        Collider::ball(base_radius),
        Friction {
            coefficient: control.friction,
            combine_rule: CoefficientCombineRule::Average,
            ..default()
        },
        CollisionGroups::new(Group::GROUP_1, Group::ALL),
        Transform {
            translation: tail_center,
            rotation: tunnel_tangent_rotation(tail_tangent),
            ..default()
        },
        GlobalTransform::default(),
        Visibility::default(),
    ));

    root.with_children(|child| {
        // Head marker and collider.
        let (head_center, head_tangent, _) = advance_centerline(tail_z, base_length);
        child.spawn((
            ProbeHead,
            ProbeTip,
            Collider::ball(base_radius * 0.9),
            Friction {
                coefficient: control.friction,
                combine_rule: CoefficientCombineRule::Average,
                ..default()
            },
            CollisionGroups::new(Group::GROUP_1, Group::ALL),
            Transform {
                translation: head_center - tail_center,
                rotation: tunnel_tangent_rotation(head_tangent),
                ..default()
            },
            GlobalTransform::default(),
        ));

        // Body rings for collision hull.
        for i in 0..=ring_count {
            let (ring_center, ring_tangent, _) = advance_centerline(tail_z, i as f32 * ring_spacing);
            child.spawn((
                ProbeRing { index: i },
                ring_shell_collider(base_radius, ring_half_height),
                Friction {
                    coefficient: control.friction,
                    combine_rule: CoefficientCombineRule::Average,
                    ..default()
                },
                CollisionGroups::new(Group::GROUP_1, Group::ALL),
                Transform {
                    translation: ring_center - tail_center,
                    rotation: tunnel_tangent_rotation(ring_tangent),
                    ..default()
                },
                GlobalTransform::default(),
            ));
        }

        // Visual skin segments along the curve.
        for i in 0..ring_count {
            let arc = (i as f32 + 0.5) * ring_spacing;
            let (seg_center, seg_tangent, _) = advance_centerline(tail_z, arc);
            let length_scale = ring_spacing / base_length;
            child.spawn((
                ProbeVisualSegment { index: i },
                Mesh3d(visual_mesh.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform {
                    translation: seg_center - tail_center,
                    rotation: tunnel_tangent_rotation(seg_tangent) * Quat::from_rotation_x(FRAC_PI_2),
                    scale: Vec3::new(1.0, length_scale, 1.0),
                },
                GlobalTransform::default(),
                Visibility::default(),
            ));
        }
    });
}

pub fn peristaltic_drive(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    control: Res<ControlParams>,
    balloon: Res<BalloonControl>,
    rapier: ReadRapierContext<'_, '_>,
    removal: Res<PolypRemoval>,
    polyp: Res<PolypTelemetry>,
    auto: Res<AutoDrive>,
    mut sense: ResMut<TipSense>,
    mut stretch: ResMut<StretchState>,
    mut tail_body: Query<(&ProbeBody, Entity, &mut RigidBody, &mut ProbeParam), With<CapsuleProbe>>,
    front_rings: Query<(Entity, &ProbeRing, &GlobalTransform)>,
    mut transforms: ParamSet<(
        Query<&mut Transform, (With<ProbeHead>, Without<ProbeVisualSegment>)>,
        Query<(&ProbeVisualSegment, &mut Transform)>,
        Query<(&ProbeRing, &mut Transform, &mut Collider, &mut Friction)>,
        Query<&mut Friction, With<CapsuleProbe>>,
        Query<&mut Transform, With<CapsuleProbe>>,
    )>,
) {
    let Ok((body, tail_entity, mut body_rb, mut params)) = tail_body.single_mut() else {
        return;
    };

    let mut tail_tf_query = transforms.p4();
    let Ok(mut body_tf) = tail_tf_query.get_mut(tail_entity) else {
        return;
    };

    // Capture current head world position before we change length so head-anchored deflate pulls the tail forward.
    let current_length = body.base_length * stretch.factor.max(MIN_STRETCH);
    let head_anchor_pos = if balloon.head_inflated {
        Some(advance_centerline(params.tail_z, current_length).0)
    } else {
        None
    };

    // Manual extension/retraction:
    // - Extend: tail balloon on, head balloon off, hold Up/I.
    // - Retract/deflate: Down/K always shrinks length, regardless of anchor state.
    // - Head balloon on locks length from auto change, but manual deflate still works.
    let interlocked = balloon.tail_inflated && balloon.head_inflated;
    let manual_extend_forward = !interlocked
        && balloon.tail_inflated
        && !balloon.head_inflated
        && (keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyI));
    // Allow reverse-style extension when head is anchored and tail free (front clamp engaged).
    let manual_extend_reverse = !interlocked
        && balloon.head_inflated
        && !balloon.tail_inflated
        && (keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyI));
    let extend_command = manual_extend_forward || manual_extend_reverse;
    let retract_command = !interlocked
        && (keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyK));

    let dt = time.delta_secs();
    // Pause manual length changes while removing a polyp.
    let autopause = removal.in_progress;

    // Slow extend as we approach a detected polyp or the cecum to avoid overshooting.
    let slow_factor = if auto.enabled && polyp.detected {
        polyp
            .nearest_distance
            .map(|d| (d / 3.5).clamp(0.2, 1.0))
            .unwrap_or(0.5)
    } else {
        1.0
    };


    if (extend_command || auto.enabled && auto.extend) && !autopause {
        stretch.factor = (stretch.factor + STRETCH_RATE * dt * slow_factor).min(MAX_STRETCH);
    } else if (retract_command || auto.enabled && auto.retract) && !autopause {
        stretch.factor = (stretch.factor - RETRACT_RATE * dt).max(MIN_STRETCH);
    }

    // Anchor tail rigidly when tail balloon is on; release when off.
    *body_rb = if balloon.tail_inflated {
        RigidBody::KinematicPositionBased
    } else {
        RigidBody::Dynamic
    };

    let length = body.base_length * stretch.factor.max(MIN_STRETCH);
    let (head_center, head_tangent, _) = advance_centerline(params.tail_z, length);

    // If the head is anchored, slide the tail forward/back along the curve to keep head world position fixed.
    if let Some(anchor_pos) = head_anchor_pos {
        let max_z = crate::tunnel::TUNNEL_START_Z + crate::tunnel::TUNNEL_LENGTH;
        let mut low = (anchor_pos.z - length - 5.0)
            .clamp(crate::tunnel::TUNNEL_START_Z, max_z);
        let mut high = (anchor_pos.z + 5.0).clamp(crate::tunnel::TUNNEL_START_Z, max_z);
        let head_at = |tail_z: f32| advance_centerline(tail_z, length).0;
        for _ in 0..18 {
            let m1 = low + (high - low) / 3.0;
            let m2 = high - (high - low) / 3.0;
            let d1 = head_at(m1).distance_squared(anchor_pos);
            let d2 = head_at(m2).distance_squared(anchor_pos);
            if d1 < d2 {
                high = m2;
            } else {
                low = m1;
            }
        }
        params.tail_z = (low + high) * 0.5;
    }

    let (tail_center, tail_tangent) = tunnel_centerline(params.tail_z);
    body_tf.translation = tail_center;
    body_tf.rotation = tunnel_tangent_rotation(tail_tangent);

    let spacing = length / body.ring_count as f32;
    let ring_half_height = spacing * 0.45;
    let feel_length = 3.0;
    let front_start = (length - feel_length).max(0.0);

    // Sense wall pressure on the front few rings and nudge the head away from it.
    let mut pressure_raw = Vec3::ZERO;
    let Ok(ctx) = rapier.single() else {
        return;
    };
    let front_start_index = body
        .ring_count
        .saturating_sub((feel_length / spacing).ceil() as usize + 1);
    for (entity, ring, _) in front_rings.iter() {
        if ring.index < front_start_index {
            continue;
        }
        for pair in ctx.contact_pairs_with(entity) {
            for manifold in pair.manifolds() {
                let normal: Vec3 = if pair.collider1() == Some(entity) {
                    manifold.normal().into()
                } else {
                    (-manifold.normal()).into()
                };

                for contact in manifold.points() {
                    // Small bias so light touches register; also include solver impulse so resting contact shows up.
                    let penetration = (-contact.dist() + 0.01).max(0.0);
                    let impulse = contact.impulse().max(0.0);
                    let weight = penetration + impulse * 0.5;
                    if weight > 0.0 {
                        pressure_raw += normal * weight;
                    }
                }
            }
        }
    }

    // Apply deadband + low-pass filtering so readings and steering don't flicker.
    let deadband = 0.02;
    let target_pressure = if pressure_raw.length() < deadband {
        Vec3::ZERO
    } else {
        pressure_raw
    };

    // Use previous sensed pressure for filtering.
    let prev_pressure = sense.pressure_world;
    let alpha_rise = 1.0 - f32::exp(-8.0 * dt);
    let alpha_fall = 1.0 - f32::exp(-5.0 * dt);
    let alpha = if target_pressure.length_squared() >= prev_pressure.length_squared() {
        alpha_rise
    } else {
        alpha_fall
    };
    let pressure = prev_pressure.lerp(target_pressure, alpha);

    let steering_dir = if pressure.length_squared() > 1e-6 {
        (-pressure).normalize_or_zero()
    } else {
        Vec3::ZERO
    };

    // Softer response: lower gain, plus slew limit to avoid snap.
    let target_blend = (pressure.length() * 0.8).clamp(0.0, 0.35);
    let prev_blend = sense.steer_strength;
    let max_delta = 1.5 * dt;
    let blend_delta = (target_blend - prev_blend).clamp(-max_delta, max_delta);
    let smoothed_blend = (prev_blend + blend_delta).clamp(0.0, 0.35);
    let steering_blend = if balloon.head_inflated { 0.0 } else { smoothed_blend };
    let steered_head = if steering_blend > 0.0 {
        head_tangent
            .lerp(steering_dir, steering_blend)
            .normalize_or_zero()
    } else {
        head_tangent
    };
    let head_rot = tunnel_tangent_rotation(head_tangent);
    sense.pressure_world = pressure;
    sense.pressure_local = head_rot.inverse() * pressure;
    sense.steer_dir = steered_head;
    sense.steer_strength = steering_blend;

    // Update head transform to new tip position.
    if let Ok(mut head_tf) = transforms.p0().single_mut() {
        head_tf.translation = head_center - tail_center;
        head_tf.rotation = tunnel_tangent_rotation(steered_head);
    }

    // Update visual skin segments along the curved centerline.
    for (seg, mut vis_tf) in transforms.p1().iter_mut() {
        let arc = (seg.index as f32 + 0.5) * spacing;
        let (center, tangent, _) = advance_centerline(params.tail_z, arc);
        let steer_blend = ((arc - front_start) / feel_length).clamp(0.0, 1.0) * steering_blend;
        let forward = if steer_blend > 0.0 {
            tangent.lerp(steered_head, steer_blend).normalize_or_zero()
        } else {
            tangent
        };
        vis_tf.translation = center - tail_center;
        vis_tf.rotation =
            tunnel_tangent_rotation(forward) * Quat::from_rotation_x(FRAC_PI_2);
        vis_tf.scale = Vec3::new(1.0, spacing / body.base_length, 1.0);
    }

    // Tail friction spikes when anchored; head friction spikes when head balloon is on.
    if let Ok(mut tail_fric) = transforms.p3().single_mut() {
        let tail_anchor = if balloon.tail_inflated { 8.0 } else { 1.0 };
        tail_fric.coefficient = control.friction * tail_anchor;
    }

    // Update rings to cover the stretched length and adjust friction gradient.
    for (ring, mut tf, mut collider, mut fric) in transforms.p2().iter_mut() {
        let arc = ring.index as f32 * spacing;
        let (ring_center, ring_tangent, _) = advance_centerline(params.tail_z, arc);
        let steer_blend = ((arc - front_start) / feel_length).clamp(0.0, 1.0) * steering_blend;
        let forward = if steer_blend > 0.0 {
            ring_tangent
                .lerp(steered_head, steer_blend)
                .normalize_or_zero()
        } else {
            ring_tangent
        };
        tf.translation = ring_center - tail_center;
        tf.rotation = tunnel_tangent_rotation(forward);
        *collider = ring_shell_collider(body.base_radius, ring_half_height);

        let t = ring.index as f32 / body.ring_count as f32;
        let anchor_scale = match (balloon.tail_inflated, balloon.head_inflated) {
            (true, false) => {
                // Tail anchored, head free.
                (1.0 - t) * 8.0 + t * 0.25
            }
            (false, true) => {
                // Head anchored, tail free (for retraction).
                t * 8.0 + (1.0 - t) * 0.25
            }
            (true, true) => {
                // Both anchored; clamp both ends.
                ((1.0 - t) * 8.0 + t * 8.0).max(1.0)
            }
            _ => 1.0,
        };
        fric.coefficient = control.friction * anchor_scale;
    }
}

pub fn distributed_thrust(
    balloon: Res<BalloonControl>,
    mut query: Query<
        (
            &SegmentIndex,
            &mut ExternalForce,
            &mut ExternalImpulse,
            &mut Velocity,
        ),
        With<ProbeSegment>,
    >,
) {
    // Pneumatic extension replaces thrust; keep external forces cleared.
    let active = balloon.tail_inflated && !balloon.head_inflated;
    for (_, mut force, mut impulse, mut velocity) in &mut query {
        force.force = Vec3::ZERO;
        impulse.impulse = Vec3::ZERO;
        if !active {
            velocity.linvel = Vec3::ZERO;
        }
    }
}
