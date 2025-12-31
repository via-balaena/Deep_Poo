# Happy-path commands (defaults only)

Use these minimal invocations to exercise the full pipeline with defaults. Swap paths/versions as needed.

## Capture (interactive or headless)
- Interactive sim/datagen (writes under `assets/datasets/captures/run_<timestamp>`):
  ```bash
  cargo run --bin sim_view
  ```
- Headless datagen wrapper (defaults to headless=true, same output root):
  ```bash
  cargo run -p colon_sim_tools --bin datagen
  ```

## ETL (warehouse_etl)
Build a warehouse from filtered captures with defaults:
```bash
cargo run -p colon_sim_tools --bin warehouse_etl
```

## Training (train CLI)
Train from a manifest with defaults (NdArray backend unless you enable burn_wgpu):
```bash
cargo run -p training --features burn_runtime --bin train -- \
  --manifest artifacts/tensor_warehouse/v<version>/manifest.json
```

## Inference
- Real-time view (inference mode):
  ```bash
  cargo run --bin inference_view
  ```
- Single-image inference with default thresholds:
  ```bash
  cargo run -p colon_sim_tools --bin single_infer -- --image path/to/image.png
  ```

# Common commands (ETL + training)

## ETL (warehouse_etl)
```bash
cargo run -p colon_sim_tools --bin warehouse_etl -- \
  --input-root assets/datasets/captures_filtered \
  --output-root artifacts/tensor_warehouse \
  --target-size 384x384 \
  --resize-mode letterbox \
  --max-boxes 16 \
  --shard-samples 1024

# Optional: export manifest summary to Parquet (defaults manifest to <output-root>/manifest.json)
cargo run -p colon_sim_tools --bin warehouse_export -- \
  --output-root artifacts/tensor_warehouse \
  --out warehouse_summary.parquet
```

## Training (train_hp)
```bash
cargo train_hp -- \
  --tensor-warehouse artifacts/tensor_warehouse/v<version>/manifest.json \
  --batch-size 64 \
  --epochs 20 \
  --status-file logs/train_status.json
```

See `reference/cli_api.md` for full flag details and `reference/wgpu_envs.md` for WGPU env vars.
