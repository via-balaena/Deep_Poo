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
