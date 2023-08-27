#!/usr/bin/env bash

STATUS="$(cmp --silent README.md playdate-rs/README.md; echo $?)"  # "$?" gives exit status for each comparison

if [[ $STATUS -ne 0 ]]; then  # if status isn't equal to 0, then execute code
    echo "ERROR: README.md and playdate-rs/README.md are not identical!"
    exit -1
fi

set -ex

cargo publish -p playdate-rs-sys $@
cargo publish -p playdate-rs-macros $@
cargo publish -p playdate-rs $@
cargo publish -p playdate-cli $@

