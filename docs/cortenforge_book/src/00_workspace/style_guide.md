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
