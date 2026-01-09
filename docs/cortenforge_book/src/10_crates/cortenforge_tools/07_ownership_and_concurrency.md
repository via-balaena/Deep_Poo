# Ownership & Concurrency (cortenforge-tools)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Services operate on owned paths/commands; no shared state.
- Overlay/recorder modules re-export functions that operate on owned buffers/paths.
- Warehouse command builder constructs strings; stateless.
- Some bins may share `ServiceCommand`/options, but the crate itself holds no global mutable data.

## Concurrency
- Crate code is synchronous; concurrency is driven by calling binaries (e.g., spawning processes).
- `is_process_running`/`read_status` are read-only operations; no shared mutable state.

## Borrowing boundaries
- All functions take owned/borrowed params for the duration of the call; no references are stored.

## Async boundaries
- None; any async/process orchestration is managed by callers/binaries.

## Risks / notes
- Concurrent invocations that touch the same run directories/logs should be coordinated by the caller; this crate does not guard against races.
