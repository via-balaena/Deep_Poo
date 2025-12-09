use bevy::prelude::*;

use crate::probe::ProbeHead;

#[derive(Resource)]
pub struct BalloonControl {
    pub offset: f32,
    pub max_offset: f32,
    pub move_speed: f32,
    pub inflated: bool,
}

impl Default for BalloonControl {
    fn default() -> Self {
        Self {
            offset: 2.0,
            max_offset: 8.0,
            move_speed: 3.0,
            inflated: false,
        }
    }
}

/// Simple input for a virtual balloon separate from the probe tip.
/// B toggles inflate/deflate; when deflated, V moves it forward and C moves it back (along probe forward).
pub fn balloon_control_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut balloon: ResMut<BalloonControl>,
    tip_q: Query<&GlobalTransform, With<ProbeHead>>,
) {
    let Ok(tip_tf) = tip_q.single() else {
        return;
    };

    if keys.just_pressed(KeyCode::KeyB) {
        balloon.inflated = !balloon.inflated;
    }

    if balloon.inflated {
        return;
    }

    let step = balloon.move_speed * time.delta_secs();
    if keys.pressed(KeyCode::KeyV) {
        balloon.offset = (balloon.offset + step).min(balloon.max_offset);
    }
    if keys.pressed(KeyCode::KeyC) {
        balloon.offset = (balloon.offset - step).max(0.0);
    }

    // Keep balloon offset finite even if tip orientation changes abruptly.
    let forward = (tip_tf.compute_transform().rotation * Vec3::Z).normalize_or_zero();
    if forward.length_squared() == 0.0 {
        balloon.offset = balloon.offset.clamp(0.0, balloon.max_offset);
    }
}
