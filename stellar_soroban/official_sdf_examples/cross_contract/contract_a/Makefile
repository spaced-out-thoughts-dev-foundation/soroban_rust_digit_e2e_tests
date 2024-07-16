default: build

all: test

test: build
	cargo test

build:
	cargo build --release --target wasm32-unknown-unknown

fmt:
	cargo fmt --all

clean:
	cargo clean