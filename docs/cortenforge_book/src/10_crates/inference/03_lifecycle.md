# inference: Lifecycle
Quick read: How data flows through this crate in practice.

## Typical usage
1) Construct factory and thresholds:
   ```rust,ignore
   let factory = InferenceFactory;
   let thresholds = InferenceThresholds { obj_thresh, iou_thresh };
   let detector = factory.build(thresholds, weights.as_deref());
   ```
2) Provide detector/thresholds to runtime/tools (e.g., insert into Bevy resources for vision_runtime, or use directly in single-image inference).

## Execution flow
- Factory loads model checkpoint (TinyDet/BigDet) via models; picks backend based on features (`backend-ndarray` default, `backend-wgpu` opt-in).
- If load fails/no weights, returns heuristic detector.
- Runtime/plugins (vision_runtime) schedule detector on captured frames; tools may invoke detector on images directly.

## Notes
- Stateless beyond factory/detector instances; lifecycle managed by caller.

## Links
- Source: `inference/src/factory.rs`
- Source: `inference/src/plugin.rs`
