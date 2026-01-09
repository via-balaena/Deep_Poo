# Design Review (cli_support)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Concise, concrete option structs; minimal deps (clap optional).
- Clear separation between CLI-facing `Args` and internal `Opts`.
- Feature-gated Bevy derive keeps the default lightweight.

## Risks / gaps
- No validation (e.g., threshold ranges, paths existing); relies entirely on callers.
- Env hint struct is inert; consumers must remember to apply it.

## Refactor ideas
- Add light validation helpers (threshold clamping, path existence) to reduce duplication in consumers.
- Provide convenience functions to apply `WgpuEnvHints` to env/logging to avoid repeat boilerplate.

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
