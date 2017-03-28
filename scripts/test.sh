#!/bin/bash

set -e

CRATES_WITH_TESTS="spatial_hash behaviour search perlin grid"

pushd crates

for crate in $CRATES_WITH_TESTS; do
    pushd $crate
    cargo test
    popd
done

popd
