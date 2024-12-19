export DATABASE_URL=sqlite://database.db

.PHONY: all
all: ci

.PHONY: clean
clean:
	cargo clean

.PHONY: start
start:
	cargo watch -x run

.PHONY: ci
ci:
#	Format
	./check_copyright.sh
	cargo +nightly fmt --all -- --check
# 	Lint
	cargo clippy --locked --all --all-targets -- -D warnings
	CARGO_TARGET_DIR=target/udeps cargo +nightly udeps --locked --all-targets
	cargo deny check --hide-inclusion-graph

.PHONY: release
release:
	cargo +nightly build --release -Z build-std=std,panic_abort \
		-Z build-std-features=optimize_for_size \
		-Z build-std-features=panic_immediate_abort
