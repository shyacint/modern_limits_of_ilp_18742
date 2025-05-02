.section .data
value:
    .word 0

.section .text
.global _start

_start:
    la   t0, value
    li   t1, 10

// while (counter--) {
loop_mem:
    lw   t2, 0(t0)
    addi t2, t2, 1
    // value = value + 1
    sw   t2, 0(t0)
    addi t1, t1, -1
    bnez t1, loop_mem
// }

    li   a7, 93
    ecall
