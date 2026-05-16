# RustDesk FFI Examples Makefile

CC = gcc
CXX = g++
PYTHON = python3

# Paths
RUSTDESK_ROOT = ../..
RUSTDESK_LIB_DIR = $(RUSTDESK_ROOT)/target/release
RUSTDESK_INC_DIR = $(RUSTDESK_ROOT)

# Compiler flags
CXXFLAGS = -std=c++17 -I$(RUSTDESK_INC_DIR) -Wall -Wextra -O2
LDFLAGS = -L$(RUSTDESK_LIB_DIR) -llibrustdesk -lpthread -ldl -lm

# For macOS
ifeq ($(shell uname),Darwin)
    LDFLAGS += -framework CoreFoundation -framework Security
endif

# Targets
C_EXAMPLES = client memory_test minimal_v1_validate
PYTHON_EXAMPLES = client.py

.PHONY: all clean c-examples python-examples run-memory-test

all: c-examples

c-examples: $(C_EXAMPLES)

client: client.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< $(LDFLAGS)

memory_test: memory_test.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< $(LDFLAGS)

minimal_v1_validate: minimal_v1_validate.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< $(LDFLAGS)

python-examples:
	@echo "Python examples don't need compilation. Run with: python3 client.py"

run-memory-test: memory_test
	./memory_test

clean:
	rm -f $(C_EXAMPLES)

# Help
help:
	@echo "RustDesk FFI Examples Build System"
	@echo "=================================="
	@echo ""
	@echo "Targets:"
	@echo "  all              - Build all C examples"
	@echo "  c-examples       - Build C examples only"
	@echo "  python-examples  - Python examples (no build needed)"
	@echo "  run-memory-test  - Run memory safety tests"
	@echo "  clean            - Remove built executables"
	@echo ""
	@echo "Examples:"
	@echo "  make             - Build all"
	@echo "  make client      - Build C client"
	@echo "  make run-memory-test - Run memory tests"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - RustDesk library: cargo build --release"
	@echo "  - VCPKG installed: VCPKG_ROOT set"