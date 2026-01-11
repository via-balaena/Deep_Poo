# Examples (cortenforge)
Quick read: Minimal examples you can adapt safely.

## 1) Use facade to pull in sim_core + vision_runtime
```rust,ignore
use cortenforge::{sim_core, vision_runtime};

fn main() {
    // Build a Bevy app using re-exported crates
    let mut app = bevy::prelude::App::new();
    app.add_plugins((vision_runtime::CapturePlugin, vision_runtime::InferencePlugin));
    // sim_core types are available through the facade
    app.insert_resource(sim_core::recorder_meta::RecorderWorldState::default());
}
```

## 2) Compile with selected features
```toml
[dependencies]
cortenforge = { version = "0.3.0", features = ["sim-core", "vision-core", "vision-runtime", "data-contracts"] }
```

```rust,ignore
// Now you can reference re-exported crates without separate deps.
use cortenforge::vision_core::interfaces::Frame;
```

## Links
- Source: `src/lib.rs`
