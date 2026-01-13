use bevy::prelude::Resource;

/// Control parameters for articulated instrument actuation.
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
