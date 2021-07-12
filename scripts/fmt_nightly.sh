#!/bin/sh

# Format according to the rustfmt.toml file provided

mv rustfmt_2.toml rustfmt.toml
cargo +nightly fmt
mv rustfmt.toml rustfmt_2.toml