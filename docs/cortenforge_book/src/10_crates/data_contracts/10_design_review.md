# Design Review (data_contracts)
Quick read: Strengths, risks, and refactor ideas.

## What’s solid
- Small, serde-friendly schemas with explicit validation helpers.
- Clear separation between capture metadata and run manifest; easy to version.

## Risks / gaps
- `RunManifest::validate` returns `String` errors; less structured than `ValidationError`.
- Schema evolution not yet exercised; adding versions may introduce branching logic and potential duplication.
- No borrowed/zero-copy variants; large data would require cloning.

## Refactor ideas
- Introduce a typed manifest error enum for consistency with capture validation.
- Plan versioning strategy (e.g., enum variants per version + conversion helpers) before adding new schema versions.
- If performance becomes an issue, consider borrowed deserialization (serde `Cow`) to avoid cloning strings/paths.

## Links
- Source: `data_contracts/src/capture.rs`
- Source: `data_contracts/src/manifest.rs`
