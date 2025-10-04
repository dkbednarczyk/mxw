all: lint build

build:
	cargo build --release

clean:
	cargo clean

lint:
	cargo fmt
	cargo clippy --all -- -D warnings
