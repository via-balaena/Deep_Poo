# Glossary & references

Key terms and pointers to design docs/repos.

## Terms
- **CortenForge**: the substrate (shared crates) for simulation, capture, ETL, training, inference, and tooling.
- **App repo**: where binaries live (e.g., `colon_sim` at https://github.com/via-balaena/Deep-Poo); domain/world logic belongs here.
- **SimRunMode / ModeSet**: run-mode gating for systems (Common/SimDatagen/Inference).
- **Recorder**: capture pipeline (meta + world state + sinks). Defaults to JSON sink in `capture_utils`.
- **Warehouse**: tensor artifacts produced by ETL (`warehouse_etl`), consumed by training.
- **TinyDet/BigDet**: model variants defined in `models`.
- **Burn patch**: temporary vendored `burn-core 0.14.0` due to bincode API change; remove when upstream ships a fix.

## References
- Crate sources: see each crate’s `src/` directory.
- Schemas: `data_contracts/src/`
- Models: `models/src/`
- Training: `training/src/bin/train.rs`, `training/src/bin/eval.rs`
- Inference factory: `inference/src/lib.rs`
- Tools: `tools/src/bin/`, `tools/src/services.rs`
- Migration notes: `MIGRATION.md`, `docs/contributor_book/src/migration.md`
- Release process: `RELEASE.md`, `docs/contributor_book/src/publishing.md`
