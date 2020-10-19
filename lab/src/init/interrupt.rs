use super::register::*;
use super::start_kernel;
use crate::lib::*;

global_asm!(include_str!("../arch/riscv/kernel/head.S"));

extern "C" {
    fn timervec();
    fn kernelvec();
}

#[no_mangle]
pub extern "C" fn mmod_init() {
    let mut x = r_mstatus();
    x = x & (!MSTATUS_MPP_MASK);
    x |= MSTATUS_MPP_S;
    w_mstatus(x);

    w_mepc(start_kernel as u64);

    w_satp(0);

    w_medeleg(0xffff);
    w_mideleg(0xffff);
    w_sie(r_sie() | SIE_SEIE | SIE_SSIE);

    timer_init();

    unsafe {
        llvm_asm!("mret");
    }
}

global_asm!(include_str!("../arch/riscv/kernel/entry.S"));

const CLINT_MTIME: u64 = 0x200bff8;
const CLINT_MTIMECMP: u64 = 0x2004000;

const INTERVAL: u64 = 10000000;

pub static mut TICKS: usize = 0;

fn timer_init() {
    set_next_timeout();

    w_mtvec(timervec as u64);

    w_mstatus(r_mstatus() | MSTATUS_MIE);
    w_mie(r_mie() | MIE_MTIE);
}

#[no_mangle]
pub extern "C" fn set_next_timeout() {
    unsafe {
        *(CLINT_MTIMECMP as *mut u64) = *(CLINT_MTIME as *const u64) + INTERVAL;
    }
}

#[no_mangle]
pub extern "C" fn kerneltrap() {
    let scause = r_scause();
    if scause == ((1 << 63) | 5) {
        unsafe {
            TICKS += 1;
            put_chars("trap\n");
            if TICKS >= 5 {
                shut_down();
            }
        }
    }
}

pub fn trap_init() {
    w_stvec(kernelvec as u64);
    w_sstatus(r_sstatus() | SSTATUS_SIE);
}
