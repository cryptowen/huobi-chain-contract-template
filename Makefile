TARGET := riscv64imac-unknown-none-elf
DOCKER_IMAGE := jjy0/ckb-capsule-recipe-rust:2020-5-9

default: fmt test

fmt:
	cargo fmt
	cd contract && cargo fmt

test: build-contract-debug
	cargo test -- --nocapture

build-contract-debug:
	docker run --rm -v `pwd`:/code -v `pwd`/contract/target/.cargo/git:/root/.cargo/git -v `pwd`/contract/target/.cargo/registry:/root/.cargo/registry ${DOCKER_IMAGE} bash -c 'cd /code/contract && make debug'

build-contract-release:
	docker run --rm -v `pwd`:/code -v `pwd`/contract/target/.cargo/git:/root/.cargo/git -v `pwd`/contract/target/.cargo/registry:/root/.cargo/registry ${DOCKER_IMAGE} bash -c 'cd /code/contract && make release'

.PHONY: test