use bevy::prelude::{Component, Entity, Handle, Image, Resource, UVec2};

#[derive(Resource, Default, Clone)]
pub struct CaptureLimit {
    pub max_frames: Option<u32>,
}

/// Marker component for the primary capture camera used for recording and inference.
#[derive(Component)]
pub struct PrimaryCaptureCamera;

/// Resource tracking the primary capture render target.
#[derive(Resource)]
pub struct PrimaryCaptureTarget {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub entity: Entity,
}

/// Resource holding the latest GPU readback from the primary capture camera.
#[derive(Resource, Default, Clone)]
pub struct PrimaryCaptureReadback {
    pub latest: Option<Vec<u8>>,
}
