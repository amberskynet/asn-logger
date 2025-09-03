# Makefile for asn-logger project

# Variables
CARGO = cargo
RUSTC = rustc
TARGET_DIR = target
EXAMPLES_DIR = examples
WEB_EXAMPLE_DIR = $(EXAMPLES_DIR)/ex_web

# Default target
.PHONY: all
all: build

# Build the main project
.PHONY: build
build:
	$(CARGO) build

# Build the main project in release mode
.PHONY: release
release:
	$(CARGO) build --release

# Run tests
.PHONY: test
test:
	$(CARGO) test

# Run example 1
.PHONY: run-example-1
run-example-1:
	$(CARGO) run --example example_1 --features test_messages

# Run example 2
.PHONY: run-example-2
run-example-2:
	$(CARGO) run --example example_2

# Run example 3
.PHONY: run-example-3
run-example-3:
	$(CARGO) run --example example_3

# Run example 4
.PHONY: run-example-4
run-example-4:
	$(CARGO) run --example example_4

# Build web example
.PHONY: build-web
build-web:
	cd $(WEB_EXAMPLE_DIR) && wasm-pack build --target web

# Run web example
.PHONY: run-web
run-web:
	cd $(WEB_EXAMPLE_DIR) && ./run.sh

# Generate documentation
.PHONY: doc
doc:
	$(CARGO) doc --open

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf $(WEB_EXAMPLE_DIR)/pkg
	rm -rf $(WEB_EXAMPLE_DIR)/target

# Install wasm-pack if not present
.PHONY: install-wasm-pack
install-wasm-pack:
	@if ! command -v wasm-pack &> /dev/null; then \
		echo "Installing wasm-pack..."; \
		$(CARGO) install wasm-pack; \
	else \
		echo "wasm-pack is already installed"; \
	fi

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all              - Build the main project (default)"
	@echo "  build            - Build the main project"
	@echo "  release          - Build the main project in release mode"
	@echo "  test             - Run tests"
	@echo "  run-example-1    - Run example 1 (with test_messages feature)"
	@echo "  run-example-2    - Run example 2"
	@echo "  run-example-3    - Run example 3"
	@echo "  run-example-4    - Run example 4"
	@echo "  build-web        - Build web example"
	@echo "  run-web          - Run web example"
	@echo "  install-wasm-pack - Install wasm-pack if not present"
	@echo "  doc              - Generate and open documentation"
	@echo "  clean            - Clean build artifacts"
	@echo "  help             - Show this help message"