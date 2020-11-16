mod interrupt;
mod sched;

use crate::lib::*;
use interrupt::*;
use sched::*;

extern "C" {
    static bss_start: usize;
    static bss_end: usize;
}

#[no_mangle]
pub extern "C" fn mmod_init() {
    let mut x = r_mstatus();
    x = x & (!MSTATUS_MPP_MASK);
    x |= MSTATUS_MPP_S;
    w_mstatus(x);

    w_mepc(start_kernel as u64);

    w_satp(0);

    w_medeleg(0x00f0);
    w_mideleg(0x00f0);
    w_sie(r_sie() | SIE_SEIE | SIE_SSIE);

    w_mie(r_mie() | MIE_MTIE);

    timer_init();

    unsafe {
        for bit in bss_start..bss_end {
            *(bit as *mut u8) = 0;
        }
        llvm_asm!("mret");
    }
}

#[no_mangle]
pub extern "C" fn start_kernel() {
    println!("ZJU OS LAB 3             GROUP-01");
    interrupt::trap_init();
    SchedTest::task_init();
    loop {}
}
