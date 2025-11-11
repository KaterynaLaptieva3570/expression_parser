# Makefile for expression_parser project

# Default target: run the program normally
run:
	cargo run

	Enter expression: result = (a + b) * (c - d / 2)
	Enter variables: a=3 b=5 c=10 d=4

	result = 68

# Run with an example formula file
example:
	cargo run -- parse formula.txt

# Display built-in help and credits
help-cli:
	cargo run -- --help

credits:
	cargo run -- credits

# Run all unit tests
test:
	cargo test

# Format source code
fmt:
	cargo fmt

# Lint code with Clippy (treat warnings as errors)
clippy:
	cargo clippy --all-targets --all-features -- -D warnings


# Clean build artifacts
clean:
	cargo clean

# Fetch all dependencies
deps:
	cargo fetch

# Build the project
build:
	cargo build

# Info about usage
help:
	@echo "Available commands:"
	@echo "  make run         – run program (same as 'cargo run')"
	@echo "  make example     – run program with formula.txt"
	@echo "  make help-cli    – show CLI help ('cargo run -- --help')"
	@echo "  make credits     – show project credits ('cargo run -- credits')"
	@echo "  make test        – run tests"
	@echo "  make fmt         – format code"
	@echo "  make clippy      – lint code"
	@echo "  make check       – run fmt + clippy + test before commit"
	@echo "  make clean       – remove build artifacts"
