# Ownership & Concurrency (data_contracts)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Pure data structs (`RunManifest`, `CaptureMetadata`, etc.) with owned fields (`PathBuf`, `Vec`); no shared ownership.
- Validation methods borrow `&self` briefly; no stored borrows.

## Concurrency
- No threading/async concerns; types are `Send`/`Sync` by virtue of fields, but the crate does not enforce cross-thread use.

## Borrowing boundaries
- All data is owned; callers choose whether to share/clone. Validation does not mutate.

## Async boundaries
- None; crate is synchronous and side-effect free (aside from validation checks).

## Risks / notes
- Safe to share across threads if caller needs (serde types are Send/Sync); concurrency behavior is entirely caller-controlled.

## Links
- Source: `data_contracts/src/capture.rs`
- Source: `data_contracts/src/manifest.rs`
