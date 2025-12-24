# TUI integration plan

Keep the TUI as a thin, separate crate/binary that talks to the core through a narrow API. Avoid threading TUI types through core code.

## Structure
- New crate/bin (e.g., `crates/cli-tui` or `src/bin/tui.rs`) that depends on `colon_sim` but not the other way around.
- Expose a small service layer from core: list runs, kick off datagen/train, stream logs/metrics via simple structs/traits. No UI concepts in core.
- TUI crate uses a terminal UI lib (e.g., `ratatui` + `crossterm`). Separate concerns: `app.rs` (state machine), `ui.rs` (render), `handlers.rs` (input/events).
- Render from immutable state; handlers mutate state and call service functions. Keep an event loop (input, tick, backend events).
- Provide graceful exit and a “dry” mode so it can run without touching files.

## Steps
1) Add a new crate/bin for the TUI; depend on `colon_sim` (lib) only. Do not add TUI deps to core.
2) Add a minimal service API in core (or a small facade module) for the TUI to call: list runs, start datagen, start training, read metrics/logs.
3) Scaffold the TUI: basic layout, event loop, and dummy data. Wire state/render/handlers separately.
4) Hook service calls into handlers (e.g., start/stop datagen, show run status/logs/metrics).
5) Add graceful shutdown and error reporting; keep the TUI optional (built via its bin/crate).

## Libraries
- Recommended: `ratatui` + `crossterm` (common, stable). Alternative: other tui crates if preferred.

## Notes
- Keep dependencies light; no changes to core logic required. The TUI should be an optional front-end.
