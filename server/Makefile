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
	cargo +nightly fmt -- --check
# 	Lint
	cargo clippy --locked --all-targets -- -D warnings
	cargo deny check --hide-inclusion-graph
# 	Test
	cargo nextest run --locked

.PHONY: coverage
coverage:
	cargo llvm-cov nextest

.PHONY: release
release:
	cargo +nightly build --release -Z build-std=std,panic_abort \
		-Z build-std-features=optimize_for_size \
		-Z build-std-features=panic_immediate_abort
