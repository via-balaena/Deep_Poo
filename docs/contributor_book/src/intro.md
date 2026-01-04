# Introduction

This book is for contributors to the CortenForge substrate (library-only). It explains how the crates fit, why the architecture looks the way it does, and how to extend and ship changes confidently.

What to expect:
- Architecture and data flow: substrate vs. apps, runtime wiring, capture/inference/ETL/training loops.
- Crate responsibilities and boundaries, including current feature flags.
- Extension points: sim hooks, recorder meta/world state, capture/inference hooks.
- Tooling: CLI utilities, what stays here vs. what moves to app repos.
- Testing/CI: what to run locally, how CI is configured.
- Release/publishing: versioning, burn-core patch note, publish order.
- Roadmap/migration: upcoming changes (burn-core fix, colon_sim_tools split, publish plan).

Housekeeping:
- Build this book: `mdbook build docs/contributor_book`
- User-facing flows live in the app repos (legacy user book retired).
- Current release target: crates at `0.1.1`; burn-core temporarily patched to a vendored 0.14.0 until upstream fixes the bincode publish break.
