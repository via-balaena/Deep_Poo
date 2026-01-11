# sim_core

## Overview
Bevy runtime scaffold for modes, hooks, and recorder types.

## Usage
Start with `sim_core::build_app` plus `SimPlugin`/`SimRuntimePlugin`; docs.rs: https://docs.rs/cortenforge-sim-core; source: https://github.com/via-balaena/CortenForge/tree/main/crates/sim_core.

## Pitfalls
ModeSet gating and recorder wiring are easy to misconfigure; follow the lifecycle and ownership pages.
