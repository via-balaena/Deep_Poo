# Ownership & Concurrency (cli_support)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Pure data structs (opts/args/seeds); no shared mutable state.
- `SeedState` holds a `u64` by value; ownership is straightforward.

## Concurrency
- No threading/async; types are `Send + Sync` by default due to primitive fields. Safe to share if callers choose.

## Borrowing boundaries
- All options are owned. Conversion `From<&CaptureOutputArgs> for CaptureOutputOpts` clones paths as needed.

## Async boundaries
- None; callers can use in async contexts without restriction.

## Risks / notes
- None; concurrency concerns are entirely caller-managed.

## Links
- Source: `crates/cli_support/src/common.rs`
- Source: `crates/cli_support/src/seed.rs`
