.PHONY: default
default: test

.PHONY: build_debug
build_debug:
	cargo build

.PHONY: test_unit
test_unit:
	cargo test

.PHONY: test
test: build_debug test_unit
	scripts/run-repl tests/lish/repl.lish
