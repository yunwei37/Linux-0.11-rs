    .section .text.init
    .globl _start
_start:
    la sp, stack_top
    jal mmod_init