# Workspace Metadata
**Why**: A quick snapshot of workspace-wide config and publish posture.
**How it fits**: Useful when changing dependencies or release settings.
**Learn more**: See [Reproducibility](reproducibility.md).

## Workspace metadata (root [Cargo.toml](https://github.com/via-balaena/CortenForge/blob/main/Cargo.toml))
Quick reference for workspace-wide configuration and publish posture.
| Item | Details |
| --- | --- |
| resolver | `2` |
| Patch overrides | <ul><li>local paths for all cortenforge crates (workspace dev convenience).</li></ul> |
| Publish status | <ul><li>most crates publishable</li><li>`cortenforge-tools` is published and app-agnostic.</li></ul> |

## Notes
- resolver = 2 
    - opts into Cargo’s 2021+ feature resolver, 
        - which prevents unwanted feature unification across the workspace.

            - Under the old resolver, if one crate enabled a feature on a shared dependency, every other crate using that dependency inherited it automatically. That often resulted in unexpected behavior, larger binaries, and optional dependencies being pulled in without intent. With resolver 2, each crate resolves features independently, so only the features a crate explicitly opts into are activated.

- Burn-core is fixed in 0.19.1; no vendored patch is required.

- `cortenforge-tools` is published and stays app-agnostic; add it directly instead of via the umbrella crate.
