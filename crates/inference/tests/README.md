Smoke tests:
- `inference::tests::inference_factory_falls_back_without_weights` (unit) ensures a detector is produced even when no weights are provided (heuristic fallback).

When a checkpoint is available, add an integration test that points `InferenceFactory` at the checkpoint and asserts the detector returns non-empty scores.
