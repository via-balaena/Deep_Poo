use bevy::prelude::*;

/// Hook trait for registering concrete controls in the app crate.
pub trait ControlsHook: Send + Sync + 'static {
    fn register(&self, app: &mut App);
}

/// Hook trait for registering concrete autopilot systems in the app crate.
pub trait AutopilotHook: Send + Sync + 'static {
    fn register(&self, app: &mut App);
}

/// Optional hooks the app can provide; left empty by default.
#[derive(Default, bevy::prelude::Resource)]
pub struct SimHooks {
    pub controls: Option<Box<dyn ControlsHook>>,
    pub autopilot: Option<Box<dyn AutopilotHook>>,
}

impl SimHooks {
    pub fn apply(&self, app: &mut App) {
        if let Some(h) = &self.controls {
            h.register(app);
        }
        if let Some(h) = &self.autopilot {
            h.register(app);
        }
    }
}
