# Test Coverage Baseline (H2a)

Generated: 2026-01-13
Branch: hacky-crates-publishing-fix
Commit: 4c003a8 (refactor: Remove unused imports and fix clippy warnings in burn_dataset)

## Overall Coverage

**Total Workspace Coverage: 27.12%**

- Lines: 1,801 / 6,641 covered (27.12%)
- Functions: 107 / 353 covered (30.31%)
- Regions: 1,225 / 4,158 covered (29.46%)

## Coverage by Crate

### Critical Crates (Core Functionality)

| Crate | Line Coverage | Priority |
|-------|--------------|----------|
| data_contracts | 54.66% | HIGH - Schema validation |
| burn_dataset | 46.73% | HIGH - Data pipeline |
| inference | 37.50% | HIGH - Model inference |
| training | 37.32% | HIGH - Model training |
| models | 82.69% | MEDIUM - Model architectures tested |

### Support Crates

| Crate | Line Coverage | Priority |
|-------|--------------|----------|
| capture_utils | 41.53% | MEDIUM |
| cli_support | 50.89% | MEDIUM |
| sim_core | 29.27% | LOW - Sim-specific |
| vision_core | 63.64% | MEDIUM |
| vision_runtime | 62.84% | MEDIUM |

### Tools (Binaries)

| Crate | Line Coverage | Priority |
|-------|--------------|----------|
| tools | 22.76% | LOW - CLI binaries |

Note: Binary coverage is expected to be lower as many require end-to-end integration testing.

## Files with Critical Coverage Gaps (<20%)

### data_contracts (54.66% overall)
- ❌ `capture.rs`: 0.00% - **CRITICAL** (schema validation logic)
- ❌ `manifest.rs`: 0.00% - **CRITICAL** (manifest schema)
- ⚠️ `preprocess.rs`: 6.00% - Data preprocessing

### burn_dataset (46.73% overall)
- ⚠️ `capture.rs`: 19.00% - **CRITICAL** (dataset indexing)
- ⚠️ `types.rs`: 9.00% - Type definitions
- ❌ `validation.rs`: 1.00% - **CRITICAL** (dataset validation)
- ❌ `splits.rs`: 3.00% - **CRITICAL** (train/val splitting)

### inference (37.50% overall)
- ❌ `lib.rs`: 0.00% - **CRITICAL** (core inference logic)
- ❌ `factory.rs`: 2.00% - Model factory
- ⚠️ `plugin.rs`: 2.00% - Bevy plugin stub

### training (37.32% overall)
- ❌ `util.rs`: 20.22% - **CRITICAL** (checkpoint I/O)

### sim_core (29.27% overall)
- ❌ `autopilot_types.rs`: 0.00%
- ❌ `lib.rs`: 1.00%
- ❌ `hooks.rs`: 1.00%
- ❌ `recorder_meta.rs`: 1.00%

### cli_support (50.89% overall)
- ⚠️ `common.rs`: 10.00%
- ❌ `seed.rs`: 2.00%

### capture_utils (41.53% overall)
- ⚠️ `lib.rs`: 13.00%

## Top 10 Critical Untested Functions/Modules

Based on the coverage analysis, these are the highest-priority areas needing tests:

1. **data_contracts::capture.rs** (0% coverage)
   - `CaptureMetadata::validate()` - Schema validation
   - `PolypLabel` deserialization edge cases
   - Bbox normalization validation

2. **data_contracts::manifest.rs** (0% coverage)
   - `RunManifest` deserialization
   - Schema version handling

3. **burn_dataset::validation.rs** (1% coverage)
   - `summarize_with_thresholds()` - Dataset quality checks
   - Validation threshold enforcement

4. **burn_dataset::splits.rs** (3% coverage)
   - `split_runs()` - Random splitting
   - `split_runs_stratified()` - Stratified splitting

5. **burn_dataset::capture.rs** (19% coverage)
   - `index_runs()` - Dataset indexing
   - `index_run()` - Single run indexing
   - Missing file handling

6. **inference::lib.rs** (0% coverage)
   - `frame_to_tensor()` - Image preprocessing
   - Detector threshold updates

7. **inference::factory.rs** (2% coverage)
   - `build_detector()` - Model instantiation
   - Checkpoint loading fallback

8. **training::util.rs** (20% coverage)
   - Checkpoint save/load
   - Training state persistence

9. **burn_dataset::aug.rs** (34% coverage)
   - Transform pipeline edge cases
   - Zero-sized images
   - Invalid augmentation parameters

10. **burn_dataset::types.rs** (9% coverage)
    - Error type handling
    - Type conversions

## Well-Tested Areas

These areas already have good coverage and can serve as examples:

- ✅ `models/src/lib.rs`: 82.69% - Model forward passes well-tested
- ✅ `vision_core/src/overlay.rs`: 94.00% - Overlay rendering
- ✅ `tools/warehouse_commands/builder.rs`: 97.79% - Command building
- ✅ `burn_dataset/src/parquet.rs`: 84.00% - Parquet I/O
- ✅ `burn_dataset/src/warehouse.rs`: 77.00% - Warehouse format

## Test Distribution

Current test count by type:
- Unit tests: 33 total across workspace
- Integration tests: 3 in burn_dataset (capture→warehouse, warehouse→training, validation→splits)
- Binary tests: Limited (mostly smoke tests)

## Recommendations for H2b-H2f

### High Priority (H2b-H2e targets)

1. **H2b: data_contracts Unit Tests** (Target: 70%+)
   - Focus on `capture.rs` and `manifest.rs` (currently 0%)
   - Test all validation logic
   - Test schema version handling

2. **H2c: burn_dataset Unit Tests** (Target: 60%+)
   - Focus on `capture.rs`, `validation.rs`, `splits.rs` (all <20%)
   - Test error paths (missing files, invalid data)
   - Test edge cases in augmentation

3. **H2d: inference Unit Tests** (Target: 60%+)
   - Focus on `lib.rs` and `factory.rs` (currently 0-2%)
   - Test tensor conversion
   - Test detector instantiation

4. **H2e: training Unit Tests** (Target: 60%+)
   - Focus on `util.rs` checkpoint I/O (currently 20%)
   - Test save/load roundtrip
   - Test epoch tracking

### Coverage Improvement Opportunities

Based on this baseline:

- **Easy wins**: data_contracts validation (small surface area, clear test cases)
- **High impact**: burn_dataset validation and splits (critical for data quality)
- **Medium effort**: inference preprocessing and factory (some integration complexity)
- **Complex**: training checkpoint I/O (requires model state setup)

## Next Steps

1. Run H2b: Add unit tests for data_contracts (2-3 hours estimated)
2. Run H2c: Add unit tests for burn_dataset (3-4 hours estimated)
3. Run H2d: Add unit tests for inference (2-3 hours estimated)
4. Run H2e: Add unit tests for training (2-3 hours estimated)
5. Re-run coverage and compare to baseline in H2f

## Viewing the Report

To view the detailed HTML report:

```bash
open target/llvm-cov-html/html/index.html
```

To regenerate coverage:

```bash
cargo llvm-cov --workspace --html --output-dir target/llvm-cov-html
```

To get summary only:

```bash
cargo llvm-cov --workspace --summary-only
```
