Step 9 of 9
Progress: [#########]

# We add mission control (cortenforge-tools + cli_support)

**Why**
Tooling helps run and observe the pipeline: scheduling, overlays, exports, and common CLI helpers.

**How it fits**
- `cli_support` keeps commands consistent.
- `cortenforge-tools` holds bins and shared helpers (published; add as a direct dependency).

**Try it**
- `cargo check -p cortenforge-cli-support -p cortenforge-tools`

**Learn more**
- Crate page: [cli_support](../10_crates/cli_support/README.md)
- Crate page: [cortenforge-tools](../10_crates/cortenforge_tools/README.md)
- docs.rs: https://docs.rs/cortenforge-cli-support and https://docs.rs/cortenforge-tools

**Unlocked**
You now have a full, runnable mission stack.
