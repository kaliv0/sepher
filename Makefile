all: lint test

lint:
	cargo fmt
	cargo clippy

test:
	cargo test

#check:
#	cargo check

build:
	cargo build

#build_release:
#	cargo build --verbose --release

#install_debug: build_debug
#	cp target/debug/$(EXEC) $(PREFIX)/bin

#install: build_release
#	cp target/release/$(EXEC) $(PREFIX)/bin

run:
	cargo run

clean:
	cargo clean