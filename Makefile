.PHONY: test

test:
	cargo check && \
	cargo test 2>&1
