Step 5 of 9
Progress: [#####----]

# We add a recorder (capture_utils + data_contracts)

**Why**
Captures turn live runs into training data. Schemas keep those captures consistent.

**How it fits**
- Recorder saves frames + labels.
- `data_contracts` defines the shape of that data.

**Try it**
- `cargo check -p cortenforge-capture-utils -p cortenforge-data-contracts`

**Learn more**
- Crate page: [capture_utils](../10_crates/capture_utils/README.md)
- Crate page: [data_contracts](../10_crates/data_contracts/README.md)
- docs.rs: https://docs.rs/cortenforge-capture-utils and https://docs.rs/cortenforge-data-contracts

**Unlocked**
The ship can now remember what it saw.
