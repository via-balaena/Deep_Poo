# runtime (sim_core)

## Responsibility
Plugin to configure runtime resources and mode sets; convenience function to register runtime defaults.

## Key items
- `SimRuntimePlugin` (Plugin): configures Update sets (Common/SimDatagen/Inference) and inserts default resources (AutoDrive, DataRun, DatagenInit, RecorderConfig/State/Motion, AutoRecordTimer).
- `register_runtime_systems` (fn): convenience to invoke `SimRuntimePlugin::build` without adding the plugin type.

## Invariants / Gotchas
- Assumes ModeSet variants exist; configure sets before adding systems.
- Resources are inserted with defaults; apps may override after plugin setup.

## Cross-module deps
- Depends on autopilot_types and recorder_types for resources; used by app runtime wiring along with SimPlugin from lib.rs.
