PROJECT_NAME := yars_cli
RUST_LOG ?= INFO
RUST_DEBUG_LOG ?= DEBUG
# Phoenix CI Variables
SERVICE := yars_cli
VERSION := 0.0.0

all: build test

get_dependencies: .get_build_dependencies.ts .get_drill.ts .get_llvm_cov.ts

clean:
	cargo clean

distclean:
	rm -rvf .*.ts
	cargo clean

build: get_dependencies
	cargo build
	@echo "Built into target/debug/${PROJECT_NAME}"

run:
	RUST_LOG=${RUST_LOG} cargo run

dev:
	RUST_LOG=${RUST_DEBUG_LOG} cargo watch -x run

test:
	cargo test

coverage:
	cargo llvm-cov --html

ci-build:
	cargo build --target=x86_64-unknown-linux-gnu --release

ci-test: .get_ci_dependencies.ts
	mkdir -p tests
	cargo test
	cargo junit --name output.xml

## Hidden targets ##

.get_build_dependencies.ts:
	@if [ ! `which rustup` ]; then \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/install_rust.sh; \
		sh /tmp/install_rust.sh -y; \
	else \
		echo 'Rust is already installed'; \
	fi
	@touch $@

.get_ci_dependencies.ts: .get_cargo_junit.ts
	@touch $@

.get_drill.ts:
	@if [ ! `which drill` ]; then \
		echo 'Installing Drill (for load testing)'; \
		cargo install drill; \
	else \
		echo 'Drill is already installed'; \
	fi

.get_cargo_junit.ts:
	@if [ ! `which cargo-junit` ]; then \
		echo 'Installing cargo-junit (for test output compatibility)'; \
		cargo install cargo-junit; \
	else \
		echo 'cargo-junit' is already installed; \
	fi

.get_llvm_cov.ts:
	@if [ ! `which cargo-llvm-cov` ]; then \
		echo 'Installing cargo-llvm-cov (for coverage)'; \
		cargo install cargo-llvm-cov; \
	else \
		echo 'cargo-llvm-cov' is already installed; \
	fi
	@touch $@