#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "1. Running Formatter Check..."
# Ensure code adheres to the style guide
cargo fmt --all -- --check

echo "2. Running Strict Linting (Clippy)..."
# This is the "Gatekeeper" step
# -D warnings ensures that any lint violation results in a build failure
cargo clippy --all-targets --all-features -- \
  -D clippy::all \
  -D clippy::pedantic \
  -D clippy::nursery \
  -D clippy::cargo \
  -D warnings

echo "3. Running Unit Tests..."
# Ensure logic is correct before final build
cargo test --all-targets --all-features

echo "4. Final Compilation..."
# Only reached if all previous steps pass
cargo build --all-targets --all-features

echo "Done! Project is clean and built successfully."

# cargo run --example <name>
# cargo bench
# cargo test
# cargo build
