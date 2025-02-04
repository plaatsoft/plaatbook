#!/bin/bash

PROFILE=${PROFILE:-default}
set -e

function clean() {
    cargo clean
    find . -name "*.db*" -delete
}

function check_copyright() {
    exit=0
    for file in $(find src -name "*.rs"); do
        if ! grep -E -q "Copyright \(c\) 20[0-9]{2}(-20[0-9]{2})? PlaatSoft" "$file"; then
            echo "Bad copyright header in: $file"
            exit=1
        fi
    done
    if [ "$exit" -ne 0 ]; then
        exit 1
    fi
}

function check() {
    # Format
    check_copyright
    cargo +nightly fmt -- --check
    # Lint
    cargo clippy --locked --all-targets --all-features -- -D warnings
    cargo deny check --hide-inclusion-graph
    # Test
    cargo nextest run --all-features --locked --config-file nextest.toml --profile "$PROFILE"
}

function coverage() {
    cargo llvm-cov nextest --all-features --locked --config-file nextest.toml --profile "$PROFILE"
}

function start() {
    cargo watch -x run
}

function release() {
    targets=("x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl")
    for target in "${targets[@]}"; do
        cargo zigbuild --release --target "$target"
    done
    rm .intentionally-empty-file.o
}

case "${1:-check}" in
    clean)
        clean
        ;;
    check)
        check
        ;;
    coverage)
        coverage
        ;;
    start)
        start
        ;;
    release)
        release
        ;;
    *)
        echo "Usage: $0 {clean|check|coverage|start|release}"
        exit 1
        ;;
esac
