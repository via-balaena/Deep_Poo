# burn_dataset

## Overview
Burn dataset loading, validation, splitting, and batching utilities.

## Usage
Point at capture runs and build `BatchIter`/`collate` for training; docs.rs: https://docs.rs/cortenforge-burn-dataset; source: https://github.com/via-balaena/CortenForge/tree/main/crates/burn_dataset.

## Pitfalls
Batches require consistent image sizes and `max_boxes` alignment.
