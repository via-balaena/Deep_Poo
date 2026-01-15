use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::ui::IsDefaultUiCamera;

#[derive(Component)]
pub struct Flycam {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub mouse_sensitivity: f32,
}

#[derive(Component)]
pub struct InstrumentPovCamera;

#[derive(Resource, Default)]
pub struct ActiveCameraMode {
    pub use_instrument: bool,
}

#[derive(Component)]
pub struct UiOverlayCamera;

pub fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(-6.0, 4.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    // 3D camera with flycam controls
    // IsDefaultUiCamera ensures UI renders here, not on the endoscope POV camera
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        IsDefaultUiCamera,
        transform,
        Flycam {
            yaw,
            pitch,
            speed: 5.0,
            mouse_sensitivity: 0.0025,
        },
    ));
}

pub fn camera_controller(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Flycam)>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut cam) in &mut query {
        if mouse_buttons.pressed(MouseButton::Right) {
            let mut delta = Vec2::ZERO;
            for ev in mouse_motion.read() {
                delta += ev.delta;
            }
            cam.yaw -= delta.x * cam.mouse_sensitivity;
            cam.pitch -= delta.y * cam.mouse_sensitivity;
            cam.pitch = cam.pitch.clamp(-1.54, 1.54);
        } else {
            // Clear any accumulated motion if mouse not held.
            mouse_motion.clear();
        }

        let yaw = cam.yaw;
        let pitch = cam.pitch;
        let rot = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        // Camera-relative axes (true free-fly; no world-up lock).
        // Use -Z as forward to align with Bevy's camera look direction.
        let forward = rot * -Vec3::Z;
        let right = rot * Vec3::X;
        let up = rot * Vec3::Y;

        let mut velocity = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            velocity += forward;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            velocity -= forward;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            velocity += right;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            velocity -= right;
        }
        if keys.pressed(KeyCode::Space) {
            velocity += up;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            velocity -= up;
        }

        if velocity.length_squared() > 0.0 {
            transform.translation += velocity.normalize() * cam.speed * dt;
        }

        transform.rotation = rot;
    }
}

pub fn pov_toggle_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ActiveCameraMode>,
    mut free_cams: Query<(Entity, &mut Camera), (With<Flycam>, Without<InstrumentPovCamera>)>,
    mut instrument_cams: Query<(Entity, &mut Camera), With<InstrumentPovCamera>>,
) {
    if !keys.just_pressed(KeyCode::KeyC) {
        return;
    }

    state.use_instrument = !state.use_instrument;
    let use_instrument = state.use_instrument;
    let use_free = !use_instrument;

    for (entity, mut cam) in &mut free_cams {
        cam.is_active = use_free;
        // Move IsDefaultUiCamera marker to active camera
        if use_free {
            commands.entity(entity).insert(IsDefaultUiCamera);
        } else {
            commands.entity(entity).remove::<IsDefaultUiCamera>();
        }
    }
    for (entity, mut cam) in &mut instrument_cams {
        cam.is_active = use_instrument;
        // Move IsDefaultUiCamera marker to active camera
        if use_instrument {
            commands.entity(entity).insert(IsDefaultUiCamera);
        } else {
            commands.entity(entity).remove::<IsDefaultUiCamera>();
        }
    }
}
