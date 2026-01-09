# Ownership & Concurrency (sim_core)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Resources (`SimHooks`, `RecorderMetaProvider`, `RecorderSink`, `RecorderWorldState`) are stored as Bevy resources with owned boxed trait objects.
- Hooks are applied by borrowing `&mut App`; no shared references persist beyond setup.

## Concurrency
- No explicit threading or async. Trait objects are bounded by `Send + Sync + 'static` to be Bevy-resource safe if Bevy uses multi-threaded schedules.
- Recorder sink and metadata provider are boxed trait objects; implementations must be thread-safe due to bounds.

## Borrowing boundaries
- Hook registration occurs during setup; lifetimes are `'static` via boxed trait objects. No non-'static borrows are stored.

## Async boundaries
- None in this crate; async behavior lives in higher layers (vision_runtime).

## Risks / notes
- Minimal; ensure custom hook/recorder implementations honor `Send + Sync` and avoid interior mutability pitfalls beyond what Bevy expects.

## Mermaid maps

### Ownership flow (resources)
```mermaid
flowchart TB
  App["Bevy App"] --> Resources["Resources owned by App"]
  Resources --> SimHooks
  Resources --> RecorderMetaProvider
  Resources --> RecorderSink
  Resources --> RecorderWorldState

  SimHooks --> ControlsHook
  SimHooks --> AutopilotHook
  RecorderMetaProvider --> RecorderMetadataProvider
  RecorderSink --> RecorderTrait["vision_core::Recorder"]
```

### Concurrency boundaries
```mermaid
flowchart LR
  Hooks["Hook trait objects"] --> Bounds["Send + Sync + 'static"]
  Meta["Metadata provider"] --> Bounds
  Sink["Recorder sink"] --> Bounds
  Bounds --> Scheduler["Bevy multi-threaded schedule"]
```

### Setup-time borrowing
```mermaid
sequenceDiagram
  participant App as Bevy App
  participant RunP as SimRuntimePlugin
  participant Hooks as SimHooks
  participant C as ControlsHook
  participant A as AutopilotHook
  participant Rec as Recorder resources

  App->>RunP: add plugin and systems
  App->>Rec: insert recorder resources
  App->>Hooks: apply(&mut App)
  Hooks->>C: register(&mut App)
  Hooks->>A: register(&mut App)
  App-->>Hooks: release mutable borrow
```

## Links
- Source: `sim_core/src/hooks.rs`
- Module: `sim_core/src/runtime.rs`
