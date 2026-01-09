# autopilot_types (sim_core)

## Responsibility
Defines autopilot state and staging for simulated movement/datagen.

## Key types
- `AutoStage` (enum): Autopilot stages (AnchorTail, Extend, AnchorHead, ReleaseTail, Contract, ReleaseHead).
- `AutoDir` (enum): Direction (Forward/Reverse).
- `AutoDrive` (Resource): Autopilot state (enabled, stage, timers, extend/retract flags, direction, last_head_z, stuck_time, primed_reverse).
- `DataRun` (Resource): Datagen run active flag.
- `DatagenInit` (Resource): Datagen init state (started, elapsed).

## Invariants / Gotchas
- `AutoDrive` defaults to disabled with initial stage AnchorTail; consumers must drive transitions.
- `DataRun`/`DatagenInit` are simple flags; ensure they’re kept in sync with actual run state.

## Cross-module deps
- Resources consumed by runtime systems/hooks (app-provided autopilot logic). Inserted by `SimRuntimePlugin`.
