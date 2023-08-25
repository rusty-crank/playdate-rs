#!/usr/bin/env bash

set -ex

cargo publish -p playdate-rs-sys
cargo publish -p playdate-rs-macros
cargo publish -p playdate-rs
cargo publish -p playdate-cli

