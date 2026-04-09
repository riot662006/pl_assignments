# Makefile for Boa Compiler
# Use -f macho64 instead of elf64 on macOS

.SECONDARY: # Don't delete intermediate .s files until we're done with them

# -----------------------------
# Config: choose test directory
# -----------------------------
# Default folder is "test". Override like:
#   make transcript DIR=test/ex_010
DIR ?= test

# Top-level only in DIR (no recursion)
SNEK_FILES := $(wildcard $(DIR)/*.snek)
S_FILES    := $(patsubst %.snek,%.s,$(SNEK_FILES))
RUN_FILES  := $(patsubst %.snek,%.run,$(SNEK_FILES))

# ---------------------------------------------
# Compilation rules (works for any subfolder under test/)
# ---------------------------------------------

# .snek → .s
test/%.s: test/%.snek src/main.rs
	cargo run -- $< $@

# .s → .run
test/%.run: test/%.s runtime/start.rs
	nasm -f elf64 $< -o runtime/our_code.o
	ar rcs runtime/libour_code.a runtime/our_code.o
	rustc -L runtime/ runtime/start.rs -o $@

# ---------------------------------------------
# Run all tests in DIR normally
# ---------------------------------------------
.PHONY: test
test: $(RUN_FILES)
	@echo "Running tests in $(DIR)..."
	@for f in $(RUN_FILES); do ./$$f; done

# ---------------------------------------------
# Unified transcript for all tests in DIR
# (keeps .s until after we cat it, then deletes .s and .run)
# ---------------------------------------------
.PHONY: transcript
transcript:
	@set -e; \
	if [ -z "$(SNEK_FILES)" ]; then \
	  echo "No .snek files found in '$(DIR)'"; \
	  exit 1; \
	fi; \
	for f in $(SNEK_FILES); do \
	  echo "> cat $$f"; \
	  cat "$$f"; \
	  echo; \
	  run="$${f%.snek}.run"; \
	  asm="$${f%.snek}.s"; \
	  echo "> make $$run"; \
	  $(MAKE) "$$run"; \
	  echo; \
	  echo "> cat $$asm"; \
	  cat "$$asm"; \
	  echo; \
	  echo "> ./$$run"; \
	  set +e; \
	  ./$$run; \
	  status=$$?; \
	  set -e; \
	  if [ $$status -ne 0 ]; then \
	    echo "[exit $$status]"; \
	  fi; \
	  rm -f "$$asm" "$$run"; \
	  echo; echo; \
	done

# ---------------------------------------------
# Clean
# ---------------------------------------------
.PHONY: clean
clean:
	cargo clean
	rm -f test/*.s test/*.run test/*/*.s test/*/*.run runtime/*.o runtime/*.a
