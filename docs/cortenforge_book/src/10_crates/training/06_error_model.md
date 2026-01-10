# Error Model (training)
Quick read: How errors are surfaced and handled.

## Errors defined
- Uses `anyhow::Result` for dataset loading/collation and training helpers; no custom error enums.

## Patterns
- `DatasetConfig::load` and `collate` propagate IO/image/serde errors via anyhow with context.
- Collation bails on mixed image sizes or empty batches.
- Training runner (`run_train`) surfaces errors from dataset/model setup (not shown here) via anyhow.

## Recoverability
- Errors are caller-visible and typically fatal for the current run (invalid data, mismatched shapes).
- No retry logic; callers should fix data or adjust configs and rerun.

## Ergonomics
- anyhow keeps signatures simple for binaries; for library use, a typed error might improve matchability.
- Errors include context (file paths, reasons) in message form; sufficient for CLIs.

## Links
- Source: `crates/training/src/dataset.rs`
- Source: `crates/training/src/util.rs`
