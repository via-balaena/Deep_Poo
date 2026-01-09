# hooks (sim_core)

## Responsibility
Define hook traits for controls/autopilot and container to apply them.

## Key types
- `ControlsHook` (trait): app registers control systems into Bevy `App`.
- `AutopilotHook` (trait): app registers autopilot systems into `App`.
- `SimHooks` (Resource): holds optional hook implementations and can apply them.

## Important functions
- `SimHooks::apply`: invokes hook `register` methods if present.

## Invariants / Gotchas
- Hooks are optional; default is empty. App must insert `SimHooks` with boxed trait objects.
- Hooks must be `Send + Sync + 'static` to be Bevy-compatible.

## Cross-module deps
- Applied during app setup (sim_core runtime build); interacts with ModeSet/system registration in runtime/app code.
