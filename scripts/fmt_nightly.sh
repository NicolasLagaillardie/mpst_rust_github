#!/bin/sh

# Format according to the rustfmt.toml file provided

mv rustfmt_0.toml rustfmt.toml
cargo +nightly fmt --all
mv rustfmt.toml rustfmt_2.toml
