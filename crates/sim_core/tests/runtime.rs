use sim_core::prelude::*;
use sim_core::runtime;

#[test]
fn runtime_plugin_inserts_defaults() {
    let mut app = bevy::prelude::App::new();
    app.add_plugins(SimPlugin)
        .add_plugins(runtime::SimRuntimePlugin);

    app.update();

    assert!(app.world().contains_resource::<SimConfig>());
    assert!(app.world().contains_resource::<AutoDrive>());
    assert!(app.world().contains_resource::<RecorderConfig>());
    assert!(app.world().contains_resource::<RecorderState>());
}
