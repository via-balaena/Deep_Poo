# probe_types (sim_core)

## Responsibility
Marker components for probe physics segments and spring settings.

## Key types
- `ProbeSegment` (Component): marks probe segment entities.
- `SegmentSpring` (Component): spring settings (base_rest) for segments.

## Invariants / Gotchas
- Stateless markers; actual physics/springs are defined in app/world systems.

## Cross-module deps
- Used by app physics systems; not wired in sim_core itself.
