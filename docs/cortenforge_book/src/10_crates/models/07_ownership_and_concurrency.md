# Ownership & Concurrency (models)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Models own their layers and configuration; no shared/global state.
- Inputs/outputs are Burn tensors; ownership follows Burn semantics (reference-counted buffers under the hood).

## Concurrency
- No threading/async in this crate. Concurrency depends on backend (e.g., WGPU handles GPU async internally).
- Models are immutable once constructed; `forward` takes `&self`, enabling shared references across threads if the backend supports it.

## Borrowing boundaries
- No retained borrows; forward methods consume input tensors by value (owned).

## Async boundaries
- None at the crate level; backend may execute lazily/asynchronously but API is synchronous.

## Risks / notes
- Sharing a model across threads is backend-dependent; Burn modules are typically `Send + Sync` for CPU/WGPU backends, but verify if using custom backends.

## Links
- Source: `crates/models/src/lib.rs`
