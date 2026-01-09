# Ownership & Concurrency (training)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Dataset samples (`RunSample`) and tensors are owned per batch; no global shared state.
- `CollatedBatch` owns tensors; caller controls their lifecycle.
- `TrainBackend` selection is compile-time; no runtime sharing across backends.

## Concurrency
- No explicit threading in this crate; relies on Burn/backend for parallelism.
- Functions consume slices/Vecs and allocate new buffers; thread safety is caller-dependent if processing batches in parallel.

## Borrowing boundaries
- `collate` borrows a slice of samples for the duration of collation, then produces owned tensors.
- No retained borrows beyond the function call.

## Async boundaries
- None; all operations are synchronous CPU-side, though WGPU backend may execute kernels asynchronously under the hood.

## Risks / notes
- Concurrent collation on shared inputs would require callers to avoid mutating the same samples concurrently.

## Links
- Source: `training/src/dataset.rs`
