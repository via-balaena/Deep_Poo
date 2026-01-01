use bevy::prelude::{Component, Entity, Handle, Image, Resource, UVec2};

#[derive(Resource, Default, Clone)]
pub struct CaptureLimit {
    pub max_frames: Option<u32>,
}

#[derive(Component)]
pub struct FrontCamera;

#[derive(Component)]
pub struct FrontCaptureCamera;

#[derive(Resource)]
pub struct FrontCaptureTarget {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub entity: Entity,
}

#[derive(Resource, Default, Clone)]
pub struct FrontCaptureReadback {
    pub latest: Option<Vec<u8>>,
}
