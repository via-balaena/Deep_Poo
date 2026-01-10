# Design Review (training)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Straightforward dataset loading/collation with clear expectations (same image size per batch).
- Backend alias keeps code portable across CPU/GPU.
- Uses shared data contracts for validation, avoiding divergence from runtime schemas.

## Risks / gaps
- Uses anyhow for all errors; limited ability to discriminate error kinds programmatically.
- Collation assumes uniform dimensions; no auto-resize or padding path for mixed data.
- No built-in logging/metrics around skips/failures during load/collate.

## Refactor ideas
- Introduce typed errors for data issues (missing files, shape mismatch) to allow better handling/reporting.
- Add optional resize/pad strategy in collate for mixed-size datasets.
- Add basic instrumentation (counts, timing) around collation to spot bottlenecks.

## Links
- Source: `crates/training/src/dataset.rs`
