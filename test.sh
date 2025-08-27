#!/bin/bash

# Quick test script to verify the library compiles and tests pass

echo "Testing with default features (enabled)..."
cargo test

echo -e "\nTesting with no default features (disabled)..."
cargo test --no-default-features

echo -e "\nChecking code with clippy..."
cargo clippy --all-features -- -D warnings

echo -e "\nChecking documentation..."
cargo doc --no-deps --all-features

echo -e "\nDone!"
