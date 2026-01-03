use bevy::prelude::*;
use sim_core::ModeSet;
use vision_core::interfaces::DetectionResult;

#[derive(Resource, Default)]
pub struct InferenceState {
    pub last: Option<DetectionResult>,
}

fn inference_stub(mut state: ResMut<InferenceState>) {
    // Placeholder: real inference scheduling/polling to be implemented.
    if state.last.is_none() {
        state.last = Some(DetectionResult {
            frame_id: 0,
            positive: false,
            confidence: 0.0,
            boxes: Vec::new(),
            scores: Vec::new(),
        });
    }
}

/// Placeholder plugin for inference systems; to be wired when Burn detector is ready.
pub struct InferencePlugin;

impl Plugin for InferencePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InferenceState::default())
            .configure_sets(Update, (ModeSet::Inference,))
            .add_systems(Update, inference_stub.in_set(ModeSet::Inference));
    }
}
