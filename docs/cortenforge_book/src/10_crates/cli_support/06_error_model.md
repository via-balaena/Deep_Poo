# Error Model (cli_support)
Quick read: How errors are surfaced and handled.

## Errors defined
- None; structs are pure data holders. No functions return `Result`.

## Patterns
- Error handling is expected in consumers (CLI binaries) when converting/validating options.
- `resolve_seed` falls back silently if env parsing fails; no errors surfaced.

## Recoverability
- Fully up to callers; this crate never fails.

## Ergonomics
- Simplicity suits CLI args; add validation in callers if stricter invariants are needed (e.g., threshold ranges).

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
