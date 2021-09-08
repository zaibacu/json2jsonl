CARGO=cargo

build:
	$(CARGO) build

run: build
	RUST_LOG=DEBUG $(CARGO) run examples/huge.json huge.jsonl