mod interrupt;

use crate::lib::*;
use interrupt::timer_init;

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
        llvm_asm!("mret");
    }
}

#[no_mangle]
pub extern "C" fn start_kernel() {
    interrupt::trap_init();
    println!("ZJU OS LAB 2             GROUP-01");
    loop {}
}
