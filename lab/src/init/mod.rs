mod interrupt;
mod page;
mod sched;

use crate::lib::*;
use interrupt::*;
use page::*;
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

    w_medeleg(0xf0ff);
    w_mideleg(0xf0ff);
    w_sie(r_sie() | SIE_SEIE | SIE_SSIE);

    w_mie(r_mie() | MIE_MTIE);

    timer_init();

    unsafe {
        let bss_starta = &bss_start as *const usize as usize;
        let bss_enda = &bss_end as *const usize as usize;
        memset(bss_starta, 0, bss_enda - bss_starta);
        llvm_asm!("mret");
    }
}

#[no_mangle]
pub extern "C" fn start_kernel() {
    paging_init();
    println!("ZJU OS LAB 4             GROUP-01");
    trap_init();
    SchedTest::task_init();
    //unsafe{asm!("ld a0, 123(x0)");}
    loop {}
}
