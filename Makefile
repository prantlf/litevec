all: check build

lint:
	cargo clippy -- -D warnings

format:
	cargo fmt --all

audit:
	cargo audit
	cargo pants

check: lint format audit

build:
	cargo build --release

start:
	target/release/litevec &

stop:
	curl -v http://localhost:8000/shutdown -X POST
