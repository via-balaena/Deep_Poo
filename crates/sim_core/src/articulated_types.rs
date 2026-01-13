use bevy::prelude::*;

/// Marker component for articulated instrument physics segments.
#[derive(Component)]
pub struct ArticulatedSegment;

/// Spring settings for an articulated segment.
#[derive(Component)]
pub struct SegmentSpring {
    pub base_rest: f32,
}
