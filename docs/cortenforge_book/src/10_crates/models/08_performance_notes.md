# Performance Notes (models)
Quick read: Hot paths, tradeoffs, and perf boundaries.

## Hot paths
- Forward passes of `TinyDet` / `BigDet`; cost scales with hidden size, depth, and max_boxes.
- `forward_multibox` does additional tensor math to reorder/clamp boxes.

## Allocation patterns
- Layers are allocated at construction; forwards reuse layer weights.
- Intermediate tensors allocated per forward (per Burn backend semantics); rely on backend caching/arena.

## Trait objects
- None; generic over backend with static dispatch.

## Assumptions
- Models are small; intended for lightweight inference. `BigDet` `max_boxes` multiplies output tensor size.

## Improvements
- If using GPU, ensure backend feature is enabled to offload compute.
- For CPU, consider reducing hidden/depth/max_boxes for faster demos, or fuse operations if profiling shows bottlenecks.

## Links
- Source: `models/src/lib.rs`
