.section .data
value: 
    .word 0              # Initial value

.section .text
.global _start

_start:
    la   t0, value       # Load address of 'value'
    li   t1, 10          # Loop counter = 10

loop_mem:
    lw   t2, 0(t0)       # Load word from memory
    addi t2, t2, 1       # Increment value
    sw   t2, 0(t0)       # Store back to memory
    addi t1, t1, -1      # Decrement loop counter
    bnez t1, loop_mem    # Repeat if t1 != 0

    li   a7, 93          # Exit syscall
    ecall
