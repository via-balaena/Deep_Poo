use bevy::prelude::Resource;
use vision_core::prelude::Recorder;

pub trait RecorderMetadataProvider: Send + Sync + 'static {
    fn label_seed(&self) -> u64;
}

#[derive(Resource)]
pub struct RecorderMetaProvider {
    pub provider: Box<dyn RecorderMetadataProvider>,
}

#[derive(Default)]
pub struct BasicRecorderMeta {
    pub seed: u64,
}

impl RecorderMetadataProvider for BasicRecorderMeta {
    fn label_seed(&self) -> u64 {
        self.seed
    }
}

#[derive(Resource, Default)]
pub struct RecorderSink {
    pub writer: Option<Box<dyn Recorder + Send + Sync>>,
}

/// App-provided world state for recorder triggers (head position, stop flag).
#[derive(Resource, Default)]
pub struct RecorderWorldState {
    pub head_z: Option<f32>,
    pub stop_flag: bool,
}
