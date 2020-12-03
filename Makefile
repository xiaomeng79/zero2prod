
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
	@cargo fmt
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

initdb:
	@echo "launch postgres"
	@chmod +x scripts/init_db.sh && scripts/init_db.sh

migrationdb:
	@echo "launch migration db"
	@chmod +x scripts/init_db.sh && SKIP_DOCKER=true scripts/init_db.sh

ci: toolinstall test coverage linting fmtcheck audit


.PHONY : ci test coverage linting fmt fmtcheck audit


