all: check build

lint:
	cargo clippy -- -D warnings

audit:
	cargo audit
	cargo pants
	# cargo test --all-features

test: lint
	cargo test --all-features

check: lint audit test

build:
	cargo build --release

run:
	target/release/litevec &

stop:
	curl -v http://localhost:8000/shutdown -X POST
