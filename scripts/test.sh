#!/usr/bin/env bash
# Run all tests

set -e

echo "Running Jagannath tests..."

# Unit tests
echo "Running unit tests..."
cargo test --workspace

# Integration tests
echo "Running integration tests..."
cargo test --workspace --test '*'

# Benchmark tests (optional)
if [ "$1" == "--bench" ]; then
    echo "Running benchmarks..."
    cargo bench --workspace
fi

echo "All tests passed!"
