.section .text
.global _start

_start:
    li   t0, 0           # Initialize t0 = 0
    li   t1, 10          # Loop counter = 10

loop_add:
    addi t0, t0, 1       # t0 += 1
    addi t1, t1, -1      # Decrement loop counter
    bnez t1, loop_add    # Repeat if t1 != 0

    li   a7, 93          # Exit syscall
    ecall
