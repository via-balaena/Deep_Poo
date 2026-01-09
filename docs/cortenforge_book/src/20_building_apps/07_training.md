Step 7 of 9
Progress: [#######--]

# We teach the ship (models + training)

**Why**
`models` defines TinyDet/BigDet. `training` runs the loop that turns data into checkpoints.

**How it fits**
- Dataset batches go in.
- Checkpoints come out.

**Try it**
- `cargo check -p cortenforge-models -p cortenforge-training`

**Learn more**
- Crate page: [models](../10_crates/models/README.md)
- Crate page: [training](../10_crates/training/README.md)
- docs.rs: https://docs.rs/cortenforge-models and https://docs.rs/cortenforge-training

**Unlocked**
The ship has a trained brain.
