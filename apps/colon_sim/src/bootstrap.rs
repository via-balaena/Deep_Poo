use bevy::prelude::*;

use sim_core::prelude::ControlParams;

use crate::balloon_control::BalloonControl;
use crate::polyp::{
    PolypDetectionVotes, PolypRandom, PolypRemoval, PolypSpawnMeta, PolypTelemetry,
};
use crate::probe::{StretchState, TipSense};
use crate::tunnel::CecumState;

/// Insert app-specific resources (domain state, control params) with the provided seed.
pub fn insert_domain_resources(app: &mut App, seed: u64) {
    app.insert_resource(BalloonControl::default())
        .insert_resource(StretchState::default())
        .insert_resource(TipSense::default())
        .insert_resource(PolypSpawnMeta { seed })
        .insert_resource(PolypRandom::new(seed))
        .insert_resource(PolypTelemetry::default())
        .insert_resource(PolypDetectionVotes::default())
        .insert_resource(PolypRemoval::default())
        .insert_resource(CecumState::default())
        .insert_resource(ControlParams {
            tension: 0.5,
            stiffness: 500.0,
            damping: 20.0,
            thrust: 40.0,
            target_speed: 1.2,
            linear_damping: 0.2,
            friction: 1.2,
        });
}
