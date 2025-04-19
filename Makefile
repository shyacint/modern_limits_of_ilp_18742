CC = riscv64-linux-gnu-gcc        # RISC-V C compiler
QEMU = qemu-riscv64               # QEMU emulator for running RISC-V programs
QEMU_FLAGS = -d in_asm,cpu		      # QEMU flags to generate instruction trace and execution trace
QEMU_CPU = rv64                   # options = {max, rv64, shakti-c, sifive-e51, sifive-u54, thead-c906, veyron-v1, x-rv128}
CFLAGS = -O2 -static              # Compile flags: -O2 for optimization, -static for static linking
LIBRARY_PATH = /usr/riscv64-linux-gnu # Path to RISC-V libraries needed by QEMU

# List your source code files (C programs)
SRC = matrix_multiply.c saxpy.c
PROGS = matrix_multiply saxpy

# Directory to store the trace logs
TRACE_DIR = qemu_dynamic_traces

# Default target: Make all programs and generate traces
all: $(PROGS)

# Rule to compile C source code files
matrix_multiply: matrix_multiply.c
	$(CC) $(CFLAGS) -o matrix_multiply.bin matrix_multiply.c

saxpy: saxpy.c
	$(CC) $(CFLAGS) -o saxpy.bin saxpy.c

# generate the QEMU dynamic traces in RISC-V ISA
run_matrix_multiply: matrix_multiply
	@echo "Running matrix_multiply and generating trace log..."
	@mkdir -p $(TRACE_DIR)
	$(QEMU) -cpu $(QEMU_CPU) $(QEMU_FLAGS) -L $(LIBRARY_PATH) ./matrix_multiply.bin 2> $(TRACE_DIR)/matrix_multiply_trace

run_saxpy: saxpy
	@echo "Running saxpy and generating trace log..."
	@mkdir -p $(TRACE_DIR)
	$(QEMU) -cpu $(QEMU_CPU) $(QEMU_FLAGS) -L $(LIBRARY_PATH) ./saxpy.bin 2> $(TRACE_DIR)/saxpy_trace

# Rule to run all programs and generate trace logs
run: $(PROGS)
	@echo "Running all programs and generating trace logs..."
	@mkdir -p $(TRACE_DIR)
	for prog in $(PROGS); do \
		$(QEMU) -cpu $(QEMU_CPU) $(QEMU_FLAGS) -L $(LIBRARY_PATH) ./$$prog.bin 2> $(TRACE_DIR)/$$prog"_trace"; \
	done


clean_bin:
	rm -f *.bin

clean: clean_bin clean_traces
