#!/usr/bin/env sh
# Regenerate the committed JSON Schemas (CI fails if they drift).
set -e
cd "$(dirname "$0")/.."
for a in setup system edit pc-map; do
  cargo run -q -p ir200 -- schema "$a" > "schemas/ir200-$a.schema.json"
done
