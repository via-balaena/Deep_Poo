# Workspace overview

# Dependency Graph

```mermaid
flowchart LR
    CLI[cli_support]
    BD[burn_dataset]
    TOOLS[colon_sim_tools]
    CU[capture_utils]
    SC[sim_core]
    VC[vision_core]
    VR[vision_runtime]
    DC[data_contracts]
    TR[training]
    INF[inference]
    M[models]
    CF[cortenforge]

    TOOLS --> CLI
    TOOLS --> BD
    TOOLS --> CU
    TOOLS --> DC
    TOOLS --> INF
    TOOLS --> M

    BD --> DC
    CU --> DC
    VR --> SC
    VR --> VC
    VR --> INF
    INF --> SC
    INF --> VC
    INF --> M
    TR --> DC
    TR --> M
    CF --> SC
    CF --> VC
    CF --> VR
    CF --> DC
    CF --> CU
    CF --> M
    CF --> TR
    CF --> INF
    CF --> CLI
    CF --> BD
    CF --> TOOLS
```

<details>
<summary>Dependency Graph Interpretation</summary>

## Interpretation
| Area | Details |
| --- | --- |
| Core runtime path | `sim_core` + `vision_core` + `vision_runtime` form the runtime/capture/inference stack; `inference` wires detectors; `models` provides TinyDet/BigDet. |
| Data path | `data_contracts` defines schemas; `capture_utils` and tools use them; `burn_dataset` consumes schemas for Burn loaders. |
| Training path | `training` depends on `models` and `data_contracts` to produce checkpoints; `inference` consumes them. |
| Tooling | `colon_sim_tools` wraps CLI helpers (`cli_support`), recorder/capture (`capture_utils`), schemas (`data_contracts`), dataset (`burn_dataset`), and inference/models; **planned to be split into app-agnostic vs. app-specific pieces in the future.** |
| Umbrella | `cortenforge` re-exports the stack with feature wiring. |
</details>

<br> 

## Core crates (high centrality)
| Crate | Version | Path | Type | Edition | Notes |
| ----- | ------- | ---- | ---- | ------- | ----- |
| **cortenforge-sim-core** | 0.1.1 | sim_core | lib | 2021 | Bevy runtime scaffolding, hooks, recorder types |
| **cortenforge-vision-core** | 0.1.1 | vision_core | lib | 2021 | Vision interfaces, overlay math |
| **cortenforge-data-contracts** | 0.1.1 | data_contracts | lib | 2021 | Schemas/validation for captures/warehouse |
| **cortenforge-models** | 0.1.1 | models | lib | 2021 | TinyDet/BigDet definitions |

<br>
<details>
<summary><strong>Rationale</strong></summary>

Core crates sit on the critical path of runtime (sim_core/vision_core) and data contracts/models that feed training/inference.

</details>


## Mid-layer

| Crate | Version | Path | Type | Edition | Notes |
| ----- | ------- | ---- | ---- | ------- | ----- |
| **cortenforge-inference** | 0.1.1 | inference | lib | 2021 | Detector factory (Burn-backed/heuristic) |
| **cortenforge-training** | 0.1.1 | training | lib + bins | 2021 | Burn training/eval CLI (train/eval bins) |
| **cortenforge-capture-utils** | 0.1.1 | capture_utils | lib | 2021 | Recorder sinks and capture helpers |
| **cortenforge-burn-dataset** | 0.1.1 | crates/burn_dataset | lib | 2021 | Burn dataset loading/splitting |
| **cortenforge-cli-support** | 0.1.1 | crates/cli_support | lib | 2021 | Shared CLI args/helpers; optional Bevy feature |

<br>
<details>
<summary><strong>Rationale</strong></summary>

Mid-layer crates adapt core capabilities to specific tasks (detector factory, training, recorder sinks, CLI parsing).

</details>

## Leaf/runtime tooling

| Crate | Version | Path | Type | Edition | Notes |
| ----- | ------- | ---- | ---- | ------- | ----- |
| **cortenforge-vision-runtime** | 0.1.1 | vision_runtime | lib | 2021 | Capture/inference plugins for Bevy |
| **colon_sim_tools** | 0.1.1 | tools | lib + bins | 2021 | Tooling crate; bins include overlay/prune/etl/export/cmd/single_infer; app-facing bins gated by features |

<br>
<details>
<summary><strong>Rationale</strong></summary>

Leaf/runtime tooling crates are consumers or runtime glue with fewer inward dependencies.

</details>

## Umbrella

| Crate | Version | Path | Type | Edition | Notes |
| ----- | ------- | ---- | ---- | ------- | ----- |
| **cortenforge** | 0.1.1 | crates/cortenforge | lib | 2024 | Umbrella re-export; feature wiring |

<br>
<details>
<summary><strong>Rationale</strong></summary>

The umbrella crate is a facade that re-exports the stack with feature wiring.

</details>

<br>

