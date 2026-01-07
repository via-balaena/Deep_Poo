# sim_core: Module Map

- `autopilot_types`: Autopilot-related types (AutoStage/AutoDir/AutoDrive/DataRun/DatagenInit).
- `camera`: Camera resources/systems (Flycam, ProbePovCamera, UiOverlayCamera), setup and controller systems.
- `controls`: Control parameter structs and related utilities.
- `hooks`: Hook traits (ControlsHook, AutopilotHook) and SimHooks container.
- `probe_types`: Probe segment types (ProbeSegment, SegmentSpring).
- `recorder_meta`: Recorder metadata provider trait and wrappers (RecorderMetadataProvider, RecorderMetaProvider, BasicRecorderMeta, RecorderSink, RecorderWorldState).
- `recorder_types`: Recorder config/state/motion types (RecorderConfig, RecorderState, AutoRecordTimer, RecorderMotion).
- `runtime`: Runtime plugin and system registration (SimRuntimePlugin, register_runtime_systems).
- `prelude`: Convenience re-exports for downstream users.

Cross-module dependencies: hooks/config feed runtime; recorder_meta/types used by runtime and apps; camera systems integrate with runtime; autopilot/control types used by hooks/apps.

## Mermaid maps

### Module dependency graph (high level)
```mermaid
flowchart TB
  autopilot_types --> hooks
  controls --> hooks
  hooks --> runtime
  recorder_meta --> runtime
  recorder_types --> runtime
  camera --> runtime
  probe_types --> runtime

  hooks --> prelude
  recorder_meta --> prelude
  recorder_types --> prelude
  autopilot_types --> prelude
  controls --> prelude
  camera --> prelude
  probe_types --> prelude
  runtime --> prelude
```

### Public surface grouping
```mermaid
flowchart LR
  subgraph sim_core
    subgraph modules
      autopilot_types
      camera
      controls
      hooks
      probe_types
      recorder_meta
      recorder_types
      runtime
    end
    prelude
  end

  prelude --> modules
```
