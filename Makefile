all: lint build

%:
	@echo "Running $@"
	@cargo $@

lint:
	@echo "Running $@"
	@cargo fmt
	@cargo clippy --allow-dirty --fix

.PHONY: all

#build_release:
#	cargo build --release

#install_debug: build_debug
#	cp target/debug/$(EXEC) $(PREFIX)/bin
#
#install: build_release
#	cp target/release/$(EXEC) $(PREFIX)/bin

