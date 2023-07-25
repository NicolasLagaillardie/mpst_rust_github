#!/bin/sh

# Check Rust code before pushing

set -e

{
    set -e
    
    # Replace toml
    cat Cargo.toml > scripts/toml/save_cargo.toml
    cat scripts/toml/full_cargo.toml > Cargo.toml
    
    # Clean everything
    ./scripts/clean_all.sh

    # cargo check each feature
    cargo check --all-targets --verbose --workspace --no-default-features
    sed -n '/^################################### Features$/,/^################################### Doc handling for all-features$/p' Cargo.toml | \
    grep -iE '[a-z|_]+ =' | \
    grep -iEwo '[a-z|_]+ ' | \
    while read -r line ; do
        echo "Processing $line"
        cargo check --all-targets --verbose --workspace --features="$line"
    done
    cargo check --all-targets --verbose --workspace --all-features

    # cargo test each feature
    cargo test --all-targets --verbose --workspace --no-default-features
    sed -n '/^################################### Features$/,/^################################### Doc handling for all-features$/p' Cargo.toml | \
    grep -iE '[a-z|_]+ =' | \
    grep -iEwo '[a-z|_]+ ' | \
    while read -r line ; do
        echo "Processing $line"
        cargo test --all-targets --verbose --workspace --features="$line"
    done
    cargo test --all-targets --verbose --workspace --all-features

    # Reverse toml
    cat scripts/toml/save_cargo.toml > Cargo.toml

    echo "done"
} || {
    # Reverse toml
    cat scripts/toml/save_cargo.toml > Cargo.toml

    echo "error"

    exit 2
}
