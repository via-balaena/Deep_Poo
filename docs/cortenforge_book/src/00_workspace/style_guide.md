# Book Style Guide
**Why**: Keep the book consistent and easy to skim.
**How it fits**: Use this before adding or refactoring pages.
**Learn more**: See [Linking Style](linking_style.md) and [docs.rs Alignment](docsrs_alignment.md).

## Chapters & structure
How the book is organized and what belongs where.
| Area | Scope |
| --- | --- |
| Top-level | workspace overview, feature flags, build/dev workflow. |
| Per-crate | `10_crates/<crate>`: overview, public API, lifecycle/data flow, module map, error model, performance notes, examples, design review. |
| Cross-crate | canonical flows, integration contracts, docs.rs alignment, linking style, quality gates, changelog, reader’s guide. |

## Naming conventions
Consistent naming and placement rules for new pages.
| Convention | Guidance |
| --- | --- |
| Paths | `10_crates/<crate>/0X_*.md` for crate pages; `00_workspace/*.md` for workspace-wide topics. |
| Headings | Title Case; keep consistent order per crate. |
| Templates | reuse `_templates/crate_template.md` when creating new crate pages. |

### Crate Naming Strategy

The CortenForge workspace uses descriptive crate names that signal architectural layers:

**Core suffix**: `_core` indicates framework-agnostic interfaces and abstractions.

- `vision_core`: Detector, Recorder, and Frame interfaces with no framework dependencies.
- Can be used in CLI tools, web services, or any Rust application.

**Runtime suffix**: `_runtime` indicates framework-specific integration layers.

- `vision_runtime`: Bevy-specific plugins, resources, and systems wrapping `vision_core` interfaces.
- Bridges framework-agnostic abstractions to Bevy ECS.

**Other patterns**:

- `_support`: Utility/helper crates (e.g., `cli_support` for shared CLI args).
- `_utils`: Concrete implementations (e.g., `capture_utils` for filesystem recorders).
- No suffix: Domain-focused crates (e.g., `models`, `inference`, `training`).

This naming convention clarifies architectural boundaries and dependency direction:

```text
vision_core (interfaces) ← inference (detector impl) ← vision_runtime (Bevy integration)
```

### vision_core vs vision_runtime Split

A key architectural pattern in CortenForge:

**vision_core**: Framework-agnostic vision interfaces.

- Defines `Detector`, `Recorder`, `Frame`, `Label` traits and types.
- Zero Bevy dependencies; pure Rust abstractions.
- Enables detector implementations in non-Bevy contexts (CLI tools, servers, etc.).

**vision_runtime**: Bevy-integrated runtime layer.

- Provides `CapturePlugin`, `InferenceRuntimePlugin` for Bevy apps.
- Wraps `vision_core` interfaces as Bevy resources.
- Handles async inference scheduling, GPU readback, detection overlays.

**Why the split**:

- Keeps core detection logic portable and testable without framework overhead.
- Allows reuse of detector implementations across Bevy apps, CLI tools, and services.
- Follows dependency inversion: high-level Bevy code depends on stable abstractions.

### Function Verb Usage

Consistent function naming conventions across the workspace:

**Action verbs** (imperative):

- `build_*`: Construct complex objects from inputs (e.g., `build_app`, `build_train_val_iters`).
- `setup_*`: Initialize resources or Bevy entities once (e.g., `setup_camera`).
- `register_*`: Add systems/plugins to an app (e.g., `register_runtime_systems`).
- `load_*`: Read from filesystem or deserialize (e.g., `load_run_dataset`, `load_linear_classifier_from_checkpoint`).
- `resolve_*`: Apply precedence rules or compute from multiple sources (e.g., `resolve_seed`).
- `validate_*`: Check invariants and return errors (e.g., `validate_summary`, `validate_backend_choice`).

**Query verbs** (return information):

- `index_*`: Scan filesystem and build index structures (e.g., `index_runs`).
- `summarize_*`: Aggregate stats from datasets (e.g., `summarize_runs`, `summarize_with_thresholds`).
- `count_*`: Return count metrics (e.g., `count_boxes`).

**Mutation verbs** (modify state):

- `draw_*`: Render visuals onto images (e.g., `draw_rect`).
- `split_*`: Partition data (e.g., `split_runs`, `split_runs_stratified`).

**Bevy system naming**:

- Systems that modify state: descriptive nouns with `_system` suffix (e.g., `pov_toggle_system`, `camera_controller`).
- Systems that setup/initialize: `setup_*` prefix (e.g., `setup_camera`).

**Consistency rules**:

- Use `build` for complex construction with configuration; `new` for simple struct constructors.
- Use `load` for I/O operations; `from_*` for pure conversions.
- Avoid generic verbs like `process` or `handle`; be specific about what the function does.

### Feature Flag Organization

CortenForge uses feature flags to enable optional functionality and select backends/models:

**Backend selection** (mutually exclusive):

- `backend-ndarray`: CPU-based Burn backend (default for inference/training).
- `backend-wgpu`: GPU-accelerated WGPU backend (opt-in).

**Model selection** (mutually exclusive):

- `linear_detector`: Simple LinearClassifier model (default).
- `convolutional_detector`: MultiboxModel for multi-box detection (opt-in).

**Runtime integration**:

- `burn-runtime`: Enables Burn-specific batch iteration in `burn_dataset` (opt-in).
- `bevy-resource`: Derives `Resource` for types in `cli_support` to enable Bevy integration (opt-in).

**Naming conventions**:

- Use `backend-*` prefix for Burn backend selection.
- Use `*_detector` suffix for model architecture selection.
- Use descriptive names for integration features (`bevy-resource`, `burn-runtime`).

**Default features**:

- Most crates default to `backend-ndarray` + `linear_detector` for minimal dependencies.
- Feature flags are opt-in by default unless they provide baseline functionality.

**Cross-crate coordination**:

- `models` defines `linear_detector` and `convolutional_detector` as empty marker features.
- `inference` and `training` enable corresponding `models` features transitively.
- This ensures type aliases (`InferenceModel`, `InferenceModelConfig`) resolve consistently.

### Module Naming Conventions

Module file names use `snake_case` and follow these patterns:

**Domain modules** (singular nouns):

- Name describes primary concept: `capture`, `overlay`, `recorder`, `camera`, `runtime`.
- Contains related types and functions for that domain.

**Type collection modules** (plural nouns with `_types` suffix):

- Groups related type definitions: `articulated_types`, `autopilot_types`.
- Use when a module primarily exports structs/enums without much behavior.

**Utility modules** (descriptive names):

- `common`: Shared utilities used across the crate.
- `util`: Helper functions for the crate's primary purpose.
- `factory`: Builder/factory patterns for complex construction.
- `hooks`: Extension points or callback systems.

**Data modules**:

- `splits`: Data partitioning logic.
- `validation`: Validation logic for data structures.
- `batch`: Batching/collation for ML pipelines.
- `aug`: Augmentation pipelines.

**Integration modules** (external system name):

- `warehouse`: Shard-based storage integration.
- `preprocess`: Data preprocessing for external systems.

**Avoid**:

- Generic names like `core`, `base`, `main` (too vague).
- Redundant prefixes matching the crate name (e.g., don't name a module `sim_camera` in `sim_core`).
- Abbreviations unless domain-standard (e.g., `aug` for augmentation is acceptable in ML contexts).

### Trait Naming

Trait names use these patterns to signal their purpose:

**Capability traits** (verb/noun describing what implementors do):

- `Detector`: Types that detect objects in frames.
- `Recorder`: Types that persist frame records.
- `MetadataProvider`: Types that provide metadata.

**Source traits** (*Source suffix):

- `FrameSource`: Types that produce frames.
- Indicates producer/generator semantics.

**Hook traits** (*Hook suffix):

- `ControlsHook`: Extension point for control behavior.
- `AutopilotHook`: Extension point for autopilot behavior.
- Indicates callback/observer pattern.

**Factory traits** (*Factory suffix):

- `BurnDetectorFactory`: Types that construct detectors.
- Indicates builder/factory pattern.

**Store traits** (*Store suffix):

- `WarehouseShardStore`: Types that store shard data.
- Indicates storage/persistence layer.

**General rules**:

- Use noun or verb-noun phrases (not adjectives).
- Avoid `Trait` suffix (redundant; `DetectorTrait` is just `Detector`).
- Be specific: `Detector` is clearer than `Detectable` or `CanDetect`.
- Match suffix to architectural role (Hook, Source, Factory, Store, Provider).

### Type Alias Feature-Gating

CortenForge uses feature-gated type aliases to select backends and models at compile time:

**Backend selection**:

```rust,ignore
#[cfg(feature = "backend-wgpu")]
pub type InferenceBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type InferenceBackend = burn_ndarray::NdArray<f32>;
```

- Used in `inference` and `training` crates.
- Defaults to `NdArray` for CPU; opt-in to WGPU for GPU.

**Model selection**:

```rust,ignore
#[cfg(feature = "convolutional_detector")]
pub type InferenceModel<B> = models::MultiboxModel<B>;
#[cfg(not(feature = "convolutional_detector"))]
pub type InferenceModel<B> = models::LinearClassifier<B>;
```

- Used in `inference` crate to select model architecture.
- Paired with `InferenceModelConfig` alias for consistency.

**Benefits**:

- Consumers import `InferenceBackend` or `InferenceModel` without conditional compilation.
- Type aliases adapt based on enabled features.
- Ensures consistent naming across feature configurations.

**Naming pattern**:

- Use descriptive aliases: `InferenceBackend`, `TrainBackend`, `InferenceModel`.
- Avoid generic names like `Backend` or `Model` (too vague).
- Pair model aliases with their config aliases (`InferenceModel` + `InferenceModelConfig`).

**Result type aliases**:

- Simple result aliases are acceptable: `pub type DatasetResult<T> = Result<T, BurnDatasetError>`.
- Use when a crate has a dominant error type to reduce boilerplate.

### Error Types Naming

CortenForge uses consistent error type patterns across the workspace:

**Error naming convention**:

- All custom error types use `*Error` suffix: `ValidationError`, `BurnDatasetError`, `ImageStatsError`, `ServiceError`.
- Error enums derive `Debug` and `thiserror::Error` for automatic trait implementations.
- Error struct variants use structured fields for context (never tuple variants for complex errors).

**Error variant patterns**:

```rust,ignore
#[derive(Debug, Error)]
pub enum BurnDatasetError {
    // Context-rich variants with named fields
    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // Simple variants with descriptive messages
    #[error("image missing for label {path}")]
    MissingImage { path: PathBuf },

    // Catch-all for migration (avoid in new code)
    #[error("{0}")]
    Other(String),
}
```

**Error message formatting**:

- Use `#[error("...")]` with interpolation: `#[error("bbox_px invalid order or negative: {0:?}")]`.
- Include relevant context in the message (paths, values, expected vs actual).
- Keep messages lowercase and concise (error context is shown in stack traces).

**When to use custom errors vs anyhow**:

- **Custom error types** (`*Error` enums): Use in library crates with public APIs where callers need structured error handling.
  - Examples: `data_contracts`, `burn_dataset`, `tools/services`.
  - Provides type-safe error matching and clear API contracts.

- **anyhow::Result**: Use in binaries and private functions where errors propagate to top-level handlers.
  - Examples: `main()` functions, CLI tools, tests, internal helpers.
  - Simplifies error propagation with `.context()` for additional information.

**Result type aliases**:

- Define a type alias when a crate has a dominant error type: `pub type DatasetResult<T> = Result<T, BurnDatasetError>`.
- Place the alias near the error definition in the same module.
- Use consistently across the crate's public API.

**Error source chaining**:

- Use `#[source]` attribute on nested error fields to preserve error chains.
- Enables `.source()` method for error introspection and better debugging.

**Avoid**:

- Generic error names like `Error` (use `DatasetError`, `ValidationError` instead).
- Tuple variants for errors with context: `Io(PathBuf, std::io::Error)` is worse than structured fields.
- String-based errors in library code (use typed enums).
- Overly broad catch-all variants (`Other(String)`) in new code.

### Async Patterns

CortenForge uses minimal async code, primarily for offloading CPU-intensive work in Bevy systems:

**Bevy async compute tasks**:

The workspace uses Bevy's `AsyncComputeTaskPool` for non-blocking inference:

```rust,ignore
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future::{block_on, poll_once};

// Spawn inference on thread pool
let task = AsyncComputeTaskPool::get().spawn(async move {
    let result = detector.detect(&frame);
    (detector, result)
});

// Poll non-blockingly in a Bevy system
if let Some((detector, result)) = block_on(poll_once(&mut task)) {
    // Task completed, handle result
}
```

**Pattern rationale**:

- Bevy systems run on the main thread; blocking I/O or CPU work freezes the frame.
- `AsyncComputeTaskPool` offloads work to a thread pool without blocking.
- `poll_once` checks completion without blocking (returns `None` if not ready).
- Task state stored in a Bevy `Resource` for cross-frame polling.

**When to use async in CortenForge**:

- **Use Bevy async tasks**: For CPU-intensive work in Bevy systems (model inference, image processing).
- **Avoid async otherwise**: Most workspace code is synchronous.
  - CLI tools and binaries block directly (no need for async).
  - Library crates use synchronous APIs (simpler, no async trait issues).

**Task state management**:

- Store `Task<T>` in a Bevy `Resource` to persist across frames.
- Take ownership with `Option::take()` when polling to avoid double-polling.
- Replace resources (e.g., `std::mem::replace`) to move ownership into async closures.

**Naming conventions**:

- System names that spawn tasks: `schedule_*_task` or `spawn_*` (e.g., `schedule_inference_task`).
- System names that poll tasks: `poll_*_task` (e.g., `poll_inference_task`).
- Resource names for task state: `Async*State` or `*TaskState` (e.g., `AsyncInferenceState`).

**No async traits or channels**:

- CortenForge avoids `async fn` in trait definitions (unstable, complex).
- No async channels (`async_channel`, `tokio::mpsc`) - Bevy's task pool is sufficient.
- No tokio runtime - Bevy provides its own thread pool primitives.

### Resource Lifecycle

Bevy Resources and Components follow consistent patterns across the workspace:

**Resource initialization patterns**:

```rust,ignore
// Pattern 1: Derive Default for simple state
#[derive(Resource, Default)]
pub struct PrimaryCameraState {
    pub active: bool,
    pub last_transform: Option<GlobalTransform>,
    pub frame_counter: u64,
}

// Pattern 2: Manual Default for configuration
#[derive(Resource)]
pub struct Config {
    pub output_root: PathBuf,
    pub capture_interval: Timer,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_root: PathBuf::from("assets/datasets/captures"),
            capture_interval: Timer::from_seconds(0.33, TimerMode::Repeating),
        }
    }
}

// Pattern 3: No Default for required initialization
#[derive(Resource)]
pub struct AsyncInferenceState {
    pub pending: Option<Task<InferenceJobResult>>,
    pub debounce: Timer,
}
```

**When to use Default vs manual construction**:

- **Derive Default**: Use for simple state with sensible zero/empty defaults.
  - Examples: counters starting at 0, flags starting false, empty collections.
- **Manual Default impl**: Use for configuration with non-trivial defaults.
  - Examples: timer durations, filesystem paths, resolution settings.
- **No Default**: Use when Resources require construction parameters or external data.
  - Examples: trait object wrappers, loaded models, task state with timers.

**Resource wrapper pattern**:

For bridging framework-agnostic types into Bevy:

```rust,ignore
// Framework-agnostic type (no Bevy dependency)
pub struct InferenceThresholds {
    pub objectness_threshold: f32,
    pub iou_threshold: f32,
}

// Bevy wrapper Resource
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct InferenceThresholdsResource(pub InferenceThresholds);

impl std::ops::Deref for InferenceThresholdsResource {
    type Target = InferenceThresholds;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

**Benefits**: Core types remain portable; wrapper provides Bevy integration.

**Component patterns**:

Components mark entities with specific behavior or identity:

```rust,ignore
// Marker components (zero-sized types)
#[derive(Component)]
pub struct InstrumentPovCamera;

// Data components (carry state)
#[derive(Component)]
pub struct Flycam {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
}
```

**Component vs Resource**:

- **Component**: Per-entity data (camera settings, physics state, markers).
- **Resource**: Global/singleton data (game config, runtime state, subsystem state).

**Resource naming conventions**:

- Configuration: `*Config` or just `Config` in subsystem modules (e.g., `recorder::Config`).
- Runtime state: `*State` suffix (e.g., `PrimaryCameraState`, `AsyncInferenceState`).
- Flags/toggles: Descriptive names ending in `Flag` or `Mode` (e.g., `ModelLoadedFlag`, `ActiveCameraMode`).
- Buffers: `*Buffer` suffix (e.g., `PrimaryCameraFrameBuffer`).
- Trait object wrappers: Short names matching the trait (e.g., `Sink` wraps `Recorder`).

**Resource lifecycle management**:

- Initialize Resources in plugin `build()` methods via `app.init_resource::<T>()` or `app.insert_resource(T::new())`.
- Update Resources via `ResMut<T>` system parameters.
- Query Resources optionally via `Option<Res<T>>` or `Option<ResMut<T>>` when presence is conditional.
- No explicit cleanup needed - Bevy handles Drop when the app/world is torn down.

**System initialization pattern**:

```rust,ignore
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Flycam { yaw: 0.0, pitch: 0.0, speed: 5.0 },
    ));
}
```

- Systems named `setup_*` run once during app initialization.
- Use `Commands` to spawn entities with component bundles.

**State management**:

CortenForge currently does not use Bevy States (no `States` enum or `NextState` found). Runtime modes are handled via Resource enums:

```rust,ignore
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimRunMode {
    Datagen,
    Inference,
    Interactive,
}
```

**Avoid**:

- Deriving `Resource` on types with lifetimes (Bevy Resources must be `'static`).
- Storing non-`Send + Sync` types in Resources (Bevy is multi-threaded).
- Using `Default` for Resources that need external parameters (use explicit constructors).

### Test Organization

CortenForge uses integration tests exclusively (no unit test modules in source files):

**Test file structure**:

All tests live in `tests/` directories within crates:

```text
crates/
  capture_utils/
    tests/
      recorder_sink.rs
  burn_dataset/
    tests/
      integration_workflows.rs
  data_contracts/
    tests/
      capture_validation.rs
      manifest_schema_smoke.rs
```

**Test naming conventions**:

- Test function names describe the behavior under test in `snake_case`:
  - `invalid_bbox_norm_rejected` - tests that invalid input is rejected.
  - `valid_bbox_passes` - tests that valid input succeeds.
  - `workflow_capture_to_warehouse_etl` - tests end-to-end workflow.
- Use descriptive names that read like sentences when prefixed with "it should".
- No `test_` prefix (Rust's `#[test]` attribute makes it redundant).

**Test organization patterns**:

```rust,ignore
// Pattern 1: Simple validation tests
#[test]
fn invalid_bbox_norm_rejected() {
    let input = /* ... */;
    let err = validate(input).unwrap_err();
    matches!(err, ValidationError::InvalidBboxNorm(_));
}

// Pattern 2: Integration workflow tests with tempfile
#[test]
fn workflow_capture_to_warehouse_etl() -> anyhow::Result<()> {
    let tmp = tempfile::tempdir()?;
    let root = tmp.path();

    // Test workflow steps...
    Ok(())
}

// Pattern 3: Fixture helper functions
fn create_synthetic_run(
    root: &Path,
    run_name: &str,
    frame_count: usize,
) -> anyhow::Result<PathBuf> {
    // Build test fixture...
}
```

**Tempfile usage for filesystem tests**:

- Use `tempfile::tempdir()` for tests that write to disk.
- Store `TempDir` in a variable to ensure cleanup at scope end.
- Extract path with `.path()` for passing to functions under test.
- Helper functions return `PathBuf` to the created fixture location.

**Return type conventions**:

- Tests that can fail use `-> anyhow::Result<()>` for ergonomic error propagation.
- Simple assertion-based tests return `()` (no explicit return type).
- Use `?` operator freely in tests returning `Result` (no manual unwrapping).

**Integration test scope**:

CortenForge tests focus on:

- **API contracts**: Validate that public APIs work as documented (e.g., validation logic, recorder sinks).
- **Workflows**: Test end-to-end pipelines (capture → ETL → training iteration).
- **Smoke tests**: Verify compilation and basic functionality (e.g., model forward passes, checkpoint loading).

**No unit test modules**:

- CortenForge does not use `#[cfg(test)] mod tests` in source files.
- All tests are integration tests in `tests/` directories.
- This keeps source files clean and focuses tests on public API usage.

**Test helper patterns**:

- Helper functions are plain functions in test files (not in a `mod tests`).
- Fixtures are created via functions like `create_synthetic_run`, `create_synthetic_labels`.
- Reusable test data structures are defined inline or in helper functions.

**Feature-gated tests**:

Some tests require specific features:

```rust
#[cfg(feature = "burn-runtime")]
#[test]
fn workflow_warehouse_to_training_iteration() -> anyhow::Result<()> {
    // Test that requires burn-runtime feature...
}
```

**Test file naming**:

- Descriptive names matching what's being tested: `capture_validation.rs`, `recorder_sink.rs`.
- Workflow tests: `integration_workflows.rs`, `integration_checkpoint.rs`.
- Smoke tests: `smoke.rs`, `smoke_train.rs`, `smoke_bigdet.rs`.

## Cross-link style
Linking rules to keep references stable and readable.
| Pattern | Example |
| --- | --- |
| Relative links | `../<crate>/01_overview.md` for intra-book references. |
| Source links | `crate/path.rs:L123` format when pointing to GitHub code. |
| Cross-references | Mention related crates/flows inline; avoid duplicate content by linking to existing sections. |

## Auto-generated vs manual
Which sections are machine-derived versus hand-curated.
| Type | Items |
| --- | --- |
| Auto-generated | feature flags list, pub API tables, module maps, canonical flow diagrams. |
| Manual curation | lifecycle narratives, design reviews, performance notes, examples, error model nuances, integration contracts. |

## Examples & code blocks
Rules for concise, copyable examples.
1) Prefer small, focused snippets; mark non-runnable blocks with `ignore`/`no_run`.
2) Include inputs/outputs when helpful; tie examples to public API.

## Diagrams
When and how to use Mermaid.
- Use Mermaid for dependency graphs and flows; place inline in relevant pages.
    - Keep diagrams minimal and update when APIs/flows change.

## Quality
Baseline expectations for every page.
1) Each page should answer: what is this, how to use/extend it, boundaries, failure modes.
2) Keep prose concise; favor tables/diagrams over long text.
3) Note NdArray default and feature-gated GPU paths where relevant.
