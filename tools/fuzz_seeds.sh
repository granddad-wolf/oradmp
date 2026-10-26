#!/usr/bin/env bash
# Copies the .dmp files of the three exp fixture folders into target/fuzz-seeds/ (read only from the fixtures),
# then writes SYNTH_SEEDS (default 200) synthetic exp files there as synth__NNNN.dmp (T-17: the generator
# crates/oradmp-core/tests/common/synth.rs through the ignored test `write_fuzz_seeds` of t17_synth.rs), and prints
# the count. libFuzzer writes new inputs only into its first corpus folder, so the seeds stay separate from
# fuzz/corpus/<target>/. Usage: tools/fuzz_seeds.sh   (T-14, ADR-62 item 5)
# The script changes to the repository root first: rustup then takes the root rust-toolchain.toml from any caller
# directory (T-17b, review 06b N-8), and the script prints the version of the cargo it uses once.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dest="$root/target/fuzz-seeds"
cd "$root"
rm -rf "$dest"
mkdir -p "$dest"
for folder in 18c 11g 11g-cp1251; do
    for f in "$root/corpus/fixtures/$folder"/*.dmp; do
        # the folder name is part of the seed name: the three folders hold files of the same names
        cp -- "$f" "$dest/${folder}__$(basename "$f")"
    done
done
# synthetic seeds (T-17, ADR-80 (g)); the folder is shared by the three targets (tools/fuzz_run.sh passes it to each)
cargo --version >&2
ORADMP_SEED_DIR="$dest" ORADMP_SEED_COUNT="${SYNTH_SEEDS:-200}" \
    cargo test --quiet -p oradmp-core --test t17_synth -- --ignored write_fuzz_seeds >&2
find "$dest" -type f -name '*.dmp' | wc -l
