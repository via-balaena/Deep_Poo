Step 3 of 9
Progress: [###------]

# We add the ship body (sim_core)

**Why**
`sim_core` gives us the simulation scaffold: modes, plugins, and the base runtime loop.

**How it fits**
- It is the "world" our ship lives in.
- Everything else plugs into this loop.

**Try it**
- `cargo check -p cortenforge-sim-core`

**Learn more**
- Crate page: [sim_core](../10_crates/sim_core/README.md)
- docs.rs: https://docs.rs/cortenforge-sim-core

**Unlocked**
The ship has a body to fly.
