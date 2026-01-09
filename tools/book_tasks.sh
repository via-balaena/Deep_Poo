#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd -- "$(dirname -- "$0")"/.. && pwd)
cd "$ROOT"

case "${1:-}" in
  extract)
    exec tools/extract_for_book.sh
    ;;
  build)
    exec mdbook build docs/cortenforge_book
    ;;
  serve)
    exec mdbook serve docs/cortenforge_book --open
    ;;
  doc)
    exec cargo doc --no-deps --workspace
    ;;
  *)
    echo "Usage: $0 {extract|build|serve|doc}" >&2
    exit 1
    ;;
 esac
