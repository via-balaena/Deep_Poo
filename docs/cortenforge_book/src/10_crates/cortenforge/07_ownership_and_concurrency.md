# Ownership & Concurrency (cortenforge)
Quick read: Ownership, threading, and async expectations.

## Ownership model
- Pure facade; re-exports member crates. No owned state.

## Concurrency
- Inherits behavior from re-exported crates; this crate introduces no threading or sharing concerns.

## Borrowing boundaries / Async
- None; no APIs beyond re-exports.

## Risks / notes
- Users should consult member crates for concurrency details. Facade use does not change ownership semantics.

## Links
- Source: `crates/cortenforge/src/lib.rs`
