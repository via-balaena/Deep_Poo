Step 8 of 9
Progress: [########-]

# We deploy the brain (inference)

**Why**
`inference` loads checkpoints and runs detection live during the sim.

**How it fits**
- Training produces checkpoints.
- Inference runs them inside the runtime loop.

**Try it**
- `cargo check -p cortenforge-inference`

**Learn more**
- Crate page: [inference](../10_crates/inference/README.md)
- Crate page: [vision_runtime](../10_crates/vision_runtime/README.md)
- docs.rs: https://docs.rs/cortenforge-inference

**Unlocked**
The ship can see and decide in real time.
