PACKAGE = cti-api
E2E_TEST = cti-api-e2e

.PHONY: build dev-build run test test/e2e

build:
	cargo build --release -p $(PACKAGE)

dev-build:
	cargo build -p $(PACKAGE)

run:
	cargo run -p $(PACKAGE)

test:
	cargo test -p $(PACKAGE)

test/e2e:
	cargo test --test $(E2E_TEST) -p $(PACKAGE)
