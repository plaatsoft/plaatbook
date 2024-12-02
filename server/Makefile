export DATABASE_URL=sqlite://database.db

.PHONY: all
all: ci

.PHONY: start
start:
	cargo watch -x run

.PHONY: ci
ci: init-database
	./check_copyright.sh
	cargo +nightly fmt --all -- --check
	cargo clippy --locked --all --all-targets -- -D warnings
	cargo build --locked --release
	CARGO_TARGET_DIR=target/udeps cargo +nightly udeps --locked --all-targets
	cargo deny check --hide-inclusion-graph

.PHONY: init-database
init-database:
	sqlx database drop -y
	sqlx database create
	sqlx migrate run
