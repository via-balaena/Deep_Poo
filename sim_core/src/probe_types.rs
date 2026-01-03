use bevy::prelude::*;

/// Marker component for probe physics segments.
#[derive(Component)]
pub struct ProbeSegment;

/// Spring settings for a probe segment.
#[derive(Component)]
pub struct SegmentSpring {
    pub base_rest: f32,
}
