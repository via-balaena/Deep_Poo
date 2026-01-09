# seed (cli_support)

## Responsibility
- Provide a deterministic seed resolver for tooling and (optionally) Bevy resources.

## Key items
- `SeedState`: simple wrapper for a `u64` seed; can derive `Resource` under the `bevy-resource` feature.
- `resolve_seed(cli_seed)`: precedence-based resolver — CLI arg > `POLYP_SEED` env var > system time (nanos).

## Invariants / Gotchas
- Env var parsing is fallible; non-numeric values are ignored.
- Time-based fallback uses nanoseconds since epoch cast to `u64`; extremely unlikely but could collide if called rapidly.
- Only derives `Resource` when the feature is enabled to keep Bevy out of default dependency set.

## Cross-module deps
- Can be inserted into Bevy apps (when feature on) or used by CLI binaries needing reproducible runs.
