# NOTE: cargo-make has issues with workspaces

validate: build fmt lint test
ci: build fmt-ci lint-ci test

build:
	cargo build

fmt:
	cargo fmt --all

fmt-ci:
	cargo fmt --all --check

lint:
	cargo cranky --all --tests --fix --allow-dirty --allow-staged

lint-ci:
	cargo cranky --all --tests

test:
	cargo nextest run --lib --bins
