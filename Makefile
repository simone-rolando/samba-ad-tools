# Makefile for building and installing Rust project

BUILD_DIR = target/release
BINARY_PATH = $(BUILD_DIR)/get-login
INSTALL_PATH = /usr/bin/get-login

.PHONY: all clean build install

all: build

build:
	cargo build --release

install:
	@if [ ! -f "$(BINARY_PATH)" ]; then \
		echo "Binary not found, building as regular user..."; \
		su "$$SUDO_USER" -c "cargo build --release"; \
	fi
	cp $(BINARY_PATH) $(INSTALL_PATH)

clean:
	cargo clean

