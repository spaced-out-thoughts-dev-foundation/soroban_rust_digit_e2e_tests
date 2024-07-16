default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../contract_a || break; 
	cargo build --release --target wasm32-unknown-unknown

fmt:
	cargo fmt --all

clean:
	cargo clean