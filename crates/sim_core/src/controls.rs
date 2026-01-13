use bevy::prelude::Resource;

/// Control configuration for articulated instrument actuation.
#[derive(Resource, Clone)]
pub struct ControlConfig {
    pub tension: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub thrust: f32,
    pub target_speed: f32,
    pub linear_damping: f32,
    pub friction: f32,
}
