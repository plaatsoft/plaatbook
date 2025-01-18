PROFILE ?= default

.PHONY: all
all: check

.PHONY: clean
clean:
	cargo clean
	find . -name "*.db*" -delete

.PHONY: start
start:
	cargo watch -x run

.PHONY: check
check:
#	Format
	./check_copyright.sh
	cargo +nightly fmt -- --check
# 	Lint
	cargo clippy --locked --all-targets -- -D warnings
	cargo deny check --hide-inclusion-graph
# 	Test
	cargo nextest run --locked --config-file nextest.toml --profile $(PROFILE)

.PHONY: coverage
coverage:
	cargo llvm-cov nextest --config-file nextest.toml --profile $(PROFILE)

.PHONY: release
release:
	cargo +nightly build --release -Z build-std=std,panic_abort \
		-Z build-std-features=optimize_for_size \
		-Z build-std-features=panic_immediate_abort
