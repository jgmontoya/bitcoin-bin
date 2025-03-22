build:
	cargo build

test:
	cargo test

lint:
	cargo clippy

format:
	cargo fmt

coverage:
	rustup run nightly cargo llvm-cov --html
	open target/llvm-cov/html/index.html

.PHONY: coverage build test lint format
