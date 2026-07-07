CARGO ?= cargo

.PHONY: build test lint

build:
	$(CARGO) build

test:
	$(CARGO) check && \
	$(CARGO) test 2>&1

lint:
	$(CARGO) clippy -- -D warnings
