Step 6 of 9
Progress: [######---]

# We turn captures into datasets (burn_dataset)

**Why**
Training needs batches. `burn_dataset` turns captures into tensors the model can learn from.

**How it fits**
- Captures become indexed samples.
- Samples become batches for training.

**Try it**
- `cargo check -p cortenforge-burn-dataset`

**Learn more**
- Crate page: [burn_dataset](../10_crates/burn_dataset/README.md)
- docs.rs: https://docs.rs/cortenforge-burn-dataset

**Unlocked**
We can now feed the ship real data.
