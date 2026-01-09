# controls (sim_core)

## Responsibility
Holds control parameters for probe actuation.

## Key types
- `ControlParams` (Resource): tension, stiffness, damping, thrust, target_speed, linear_damping, friction.

## Invariants / Gotchas
- No defaults provided; callers should initialize reasonable values.

## Cross-module deps
- Consumed by app/control systems; not wired directly in sim_core.
