Step 4 of 9
Progress: [####-----]

# We add eyes (vision_runtime + vision_core)

**Why**
`vision_core` defines detections; `vision_runtime` wires detectors into the sim loop.

**How it fits**
- Frames go in, detections come out.
- This is how the ship "sees".

**Try it**
- `cargo check -p cortenforge-vision-core -p cortenforge-vision-runtime`

**Learn more**
- Crate page: [vision_core](../10_crates/vision_core/README.md)
- Crate page: [vision_runtime](../10_crates/vision_runtime/README.md)
- docs.rs: https://docs.rs/cortenforge-vision-core and https://docs.rs/cortenforge-vision-runtime

**Unlocked**
The ship can now see the field.
