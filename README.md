# Deep Poo

This is a simplified, non-commercial demonstration inspired by the locomotion principles described in United States Patent Application
[Pub. No.: US 2007/024.9906 A1](https://patentimages.storage.googleapis.com/6b/ce/a2/051225b0d2ff0a/US20070249906A1.pdf)

The implementation shown here does **not** replicate the full patented system.
Instead, it demonstrates the core inchworm-style anchoring and extension concept in a reduced, abstracted form suitable for simulation and education.

On top of this abstracted mechanism, an **original automated supervisory control layer** has been added. This control layer is not described in the patent and is introduced solely for the purposes of:
- enforcing safety interlocks,
- coordinating motion phases,
- and enabling higher-level autonomous or semi-autonomous operation in simulation.

No attempt is made to reproduce proprietary hardware, clinical configurations, or commercial embodiments described in the patent.

**Video of auto probe in action**
https://github.com/user-attachments/assets/cbf42edf-c61e-476c-b1e8-549b5f5b7580


## Controls
- `P` begin automated process
- `C` toggle camera between free-fly and probe POV


## Running
```bash
cargo run --release
```

## Debug collider view
- Set `RAPIER_DEBUG_WIREFRAMES` in `src/lib.rs` to `true` to show collider wireframes (orange), or `false` to hide them. Rebuild/run after changing.
