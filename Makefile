all: check build

lint:
	cargo clippy -- -D warnings

format:
	cargo fmt --all

outdated:
	cargo outdated --exit-code 1

audit:
	cargo audit
	cargo pants

check: lint format outdated audit

upgrade:
	    cargo update
		cargo upgrade --incompatible

build:
	cargo build --release

start:
	target/release/litevec &

stop:
	curl -X POST -s -w "%{http_code}" http://localhost:8000/shutdown

clean:
	rm -rf target

build-docker:
	docker build -t litevec .

start-docker:
	docker run --rm -dt -p 8000:8000 -v $PWD/litevec-storage:/litevec/storage \
		--name litevec litevec

kill-docker:
	docker container kill litevec

logs-docker:
	docker logs litevec
