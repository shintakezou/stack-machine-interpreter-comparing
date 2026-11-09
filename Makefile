# Makefile for stack-machine-interpreter-comparing
#
# Usage:
#   make                 build everything into $(BUILD)/ with the default optimization (-O2)
#   make OPT='-O0'       build with a different optimization level
#   make OPT='-Ofast -march=native -fshort-enums'
#   make benchmark       run the full benchmark sweep (benchmark.sh)
#   make clean           remove the build directory
#   make distclean       also remove the generated transpiled sources
#
# Note: i_TTC_dispatch_function_table_shared dlopen()s "./vm_shared.so"
# relative to the current directory, so run it from inside $(BUILD)/.

# use gcc (as the original scripts do) unless CC is set explicitly
ifeq ($(origin CC),default)
CC = gcc
endif
OPT ?= -O2
LDLIBS = -lm

BUILD ?= build

# stack-language program fed to the transpiler
PROGRAM ?= test3_function_call.f

INTERPRETERS = \
	$(BUILD)/i_TTC_dispatch_computed_goto \
	$(BUILD)/i_TTC_dispatch_switch \
	$(BUILD)/i_TTC_dispatch_if \
	$(BUILD)/i_TTC_dispatch_function_table \
	$(BUILD)/i_DCT_dispatch_function_call

NATIVES = \
	$(BUILD)/native_function_init \
	$(BUILD)/native_function_call \
	$(BUILD)/native_baseline

SHARED_BIN = $(BUILD)/i_TTC_dispatch_function_table_shared
SHARED_LIB = $(BUILD)/vm_shared.so

TRANSPILED_BIN = $(BUILD)/native_transpiled_run400k_O2 $(BUILD)/native_transpiled_run400k_Ofast
TRANSPILED_SRC = transpiled_main.c transpiled_words_used.c

ALL_BIN = $(INTERPRETERS) $(NATIVES) $(SHARED_BIN) $(BUILD)/transpiler $(TRANSPILED_BIN)

.PHONY: all benchmark clean distclean

all: $(ALL_BIN) $(SHARED_LIB)

$(BUILD):
	mkdir -p $(BUILD)

# --- interpreters and native versions (one .c each) -------------------------

$(INTERPRETERS) $(NATIVES): $(BUILD)/%: %.c common.h | $(BUILD)
	$(CC) $(OPT) $< -o $@ $(LDLIBS)

$(BUILD)/i_TTC_dispatch_computed_goto \
$(BUILD)/i_TTC_dispatch_switch \
$(BUILD)/i_TTC_dispatch_if \
$(BUILD)/i_TTC_dispatch_function_table: Opcode_enum.c test3.c

$(BUILD)/i_DCT_dispatch_function_call: test3.c

$(BUILD)/native_function_init: Opcode_enum.c test3_function_init.c
$(BUILD)/native_function_call: Opcode_enum.c test3_function_call.c

# --- shared-library variant (hot reload) ------------------------------------

$(SHARED_LIB): vm_shared.c vm.h Opcode_enum.c | $(BUILD)
	$(CC) -std=c99 $(OPT) -fPIC -fvisibility=hidden -shared $< -o $@ $(LDLIBS)

# at runtime it dlopen()s ./vm_shared.so, so build that too
$(SHARED_BIN): $(BUILD)/%: %.c vm.h Opcode_enum.c test3.c $(SHARED_LIB) | $(BUILD)
	$(CC) $(OPT) $< -o $@ $(LDLIBS) -ldl

# --- transpiler --------------------------------------------------------------

$(BUILD)/transpiler: transpiler.c load_primitives.c tokenizer.c \
            parser_include_words.c parser_token_digest.c | $(BUILD)
	$(CC) $(OPT) $< -o $@

# the transpiler reads words/*.c at runtime and writes the transpiled
# sources in the current directory, so it must run from the project root
transpiled_main.c: $(BUILD)/transpiler $(PROGRAM) $(wildcard words/*.c)
	$(BUILD)/transpiler $(PROGRAM)

transpiled_words_used.c: transpiled_main.c

# the transpiled program is benchmarked at two fixed optimization levels
$(BUILD)/native_transpiled_run400k_O2: native_transpiled_run400k.c $(TRANSPILED_SRC) vm.h | $(BUILD)
	$(CC) -I. -O2 $< -o $@ $(LDLIBS)

$(BUILD)/native_transpiled_run400k_Ofast: native_transpiled_run400k.c $(TRANSPILED_SRC) vm.h | $(BUILD)
	$(CC) -I. -Ofast $< -o $@ $(LDLIBS)

# --- misc ---------------------------------------------------------------------

benchmark:
	./benchmark.sh

clean:
	rm -rf $(BUILD)

distclean: clean
	rm -f $(TRANSPILED_SRC)
