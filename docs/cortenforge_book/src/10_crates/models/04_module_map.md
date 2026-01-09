# models: Module Map
Quick read: What each module owns and why it exists.

- `lib.rs`: Defines TinyDet/BigDet configs and models; includes a `prelude` for re-exports.
- `prelude`: Convenience re-export of core model types.

Cross-module dependencies:
- self-contained definitions.
- consumed by training and inference.

## Links
- Source: `models/src/lib.rs`
