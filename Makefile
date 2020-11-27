
toolinstall:
	@echo "tool install start"
	@hash rustup || curl https://sh.rustup.rs -sSf | sh
	@rustup --version
	@rustc --version
	@cargo version
	@echo "tool install end"

test:
	@echo "test code start"
	@cargo test
	@echo "test code end"

coverage:
	@echo "test code coverage start"
	@cargo install cargo-tarpaulin
	@cargo tarpaulin --ignore-tests
	@echo "test code coverage end"

linting:
	@echo "test code linting start"
	@rustup component add clippy
	@cargo clippy -- -D warnings
	@echo "test code linting end"

fmt:
	@echo "test code format  start"
	@rustup component add rustfmt
	@cargo fmt ./...
	@echo "test code format  end"

fmtcheck:
	@echo "test code format check start"
	@rustup component add rustfmt
	@cargo fmt -- --check
	@echo "test code format check end"

audit:
	@echo "test code audit start"
	@cargo install cargo-audit
	@cargo audit
	@echo "test code audit end"

ci: toolinstall test coverage linting fmtcheck audit
	@echo "ci start"
	@echo "ci end"

.PHONY : ci test coverage linting fmt fmtcheck audit


