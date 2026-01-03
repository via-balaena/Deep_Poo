use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoStage {
    AnchorTail,
    Extend,
    AnchorHead,
    ReleaseTail,
    Contract,
    ReleaseHead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoDir {
    Forward,
    Reverse,
}

#[derive(Resource)]
pub struct AutoDrive {
    pub enabled: bool,
    pub stage: AutoStage,
    pub timer: f32,
    pub extend: bool,
    pub retract: bool,
    pub dir: AutoDir,
    pub last_head_z: f32,
    pub stuck_time: f32,
    pub primed_reverse: bool,
}

impl Default for AutoDrive {
    fn default() -> Self {
        Self {
            enabled: false,
            stage: AutoStage::AnchorTail,
            timer: 0.0,
            extend: false,
            retract: false,
            dir: AutoDir::Forward,
            last_head_z: 0.0,
            stuck_time: 0.0,
            primed_reverse: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct DataRun {
    pub active: bool,
}

#[derive(Resource, Default)]
pub struct DatagenInit {
    pub started: bool,
    pub elapsed: f32,
}
