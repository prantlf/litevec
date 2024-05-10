ifeq (1,${MACOS_ARM})
	TARGET:=--target=aarch64-apple-darwin
endif
ifeq (1,${LINUX_ARM})
	TARGET:=--target=aarch64-unknown-linux-gnu
endif

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

check: lint format outdated

upgrade:
	cargo update
	cargo upgrade --incompatible

build:
	cargo build --release $(TARGET)

start:
	target/release/litevec &

ping:
	curl -s -w "%{http_code}" http://localhost:8000/ping

stop:
	curl -X POST -s -w "%{http_code}" http://localhost:8000/shutdown

clean:
	rm -rf target

docker: docker-lint docker-build

docker-lint:
	docker run --rm -i \
		-v ${PWD}/.hadolint.yaml:/bin/hadolint.yaml \
		-e XDG_CONFIG_HOME=/bin hadolint/hadolint \
		< Dockerfile

docker-build:
	docker build -t litevec .

docker-start:
	docker run --rm -dt -p 8000:8000 -v ${PWD}/storage:/storage \
		--name litevec litevec

docker-enter:
	docker run --rm -it -p 8000:8000 -v ${PWD}/storage:/storage \
		--entrypoint sh litevec

docker-kill:
	docker container kill litevec

docker-log:
	docker logs litevec

docker-up:
	IMAGE_HUB= docker compose -f docker-compose.yml up -d

docker-down:
	IMAGE_HUB= docker compose -f docker-compose.yml down

docker-log1:
	docker logs litevec-litevec-1
