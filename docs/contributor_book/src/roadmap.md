# Roadmap

Forward-looking areas. Keep this short and actionable; link to issues/PRs when concrete.

## Near-term
- Upstream `burn-core` fix: drop the vendored patch; update deps; republish v0.1.1 crates.
- Publish flow: finish release checklist (fmt/clippy/tests/hakari/publish order), tag/push.
- Docs: finish contributor book deep dives and add diagrams/examples; retire any lingering user-doc references.
- Tools: clarify colon_sim_tools split plan (shared vs app-specific) and trim app-specific bins.

## Medium-term
- Crate ergonomics: add “does/doesn’t” tables and quickstarts per crate; consider a crate dependency graph in docs.
- Testing/CI: optional GPU lane for WGPU smokes; keep default NdArray lane fast.
- Models: document checkpoints/layout and add guidance for new variants/export.
- GPU upgrade/testing: define a WGPU/GPU validation plan (features to enable, minimal smokes to run, CI opt-in job).

## Longer-term
- Pluggable sinks: DB/object store options with schema compatibility guarantees.
- Templates for new apps/domains using the substrate.
- Telemetry/observability hooks for runtime and tools.
