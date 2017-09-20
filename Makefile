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
	scripts/run-repl tests/lish/simple_shell.lish
	scripts/run-repl tests/lish/simple_lisp.lish
	scripts/run-repl tests/lish/aliases.lish
	scripts/run-repl tests/lish/cd.lish
	scripts/run-repl tests/lish/environment.lish
