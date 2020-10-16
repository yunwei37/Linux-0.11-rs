    .section .text.entry
    .globl _start
_start:
    csrw mstatus, x0;
    la t0, trap_machine;
    csrw mtvec, t0;
    li t0, 0x800
    csrw mstatus, t0;
    la t0, next_mret;
    csrw mepc, t0
    mret;

next_mret:
    la t0, trap_super;
    csrw stvec, t0;
    la sp, stack_top
    jal start_kernel

trap_machine:
trap_super:
    ret