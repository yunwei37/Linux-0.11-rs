use super::sched::*;
use crate::lib::*;

global_asm!(include_str!("../arch/riscv/kernel/head.S"));

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub regs: thread_struct,
    pub status: u64,
    pub epc: u64,
}

extern "C" {
    fn trap_m();
    fn trap_s();
}

global_asm!(include_str!("../arch/riscv/kernel/entry.S"));

pub static mut TICKS: usize = 0;

pub fn timer_init() {
    set_next_timeout();
    w_mtvec(trap_m as u64);
    w_mstatus(r_mstatus() | MSTATUS_MIE);
}

fn set_next_timeout() {
    const CLINT_MTIME: u64 = 0x200bff8;
    const CLINT_MTIMECMP: u64 = 0x2004000;
    const INTERVAL: u64 = 10000000;
    unsafe {
        *(CLINT_MTIMECMP as *mut u64) = *(CLINT_MTIME as *const u64) + INTERVAL;
    }
}

#[no_mangle]
pub extern "C" fn machine_trap_handler(context: &mut Context) {
    let mcause = r_mcause();
    //println!("machine_trap_handler {} epc {:x}", mcause, context.epc);
    const M_TIMER_INTERRUPT: u64 = (1 << 63) | 7;
    const S_ECALL: u64 = 9;
    if mcause == M_TIMER_INTERRUPT {
        w_mip(r_mip() | MIP_STIP);
        w_sie(r_sie() | SIE_STIE);
        w_mie(r_mie() & !MIE_MTIE);
    } else if mcause == S_ECALL {
        set_next_timeout();
        w_mie(r_mie() | MIE_MTIE);
        context.epc += 4;
    } else {
        panic!("unknown machine trap: mcause {}", mcause);
    }
}

#[no_mangle]
pub extern "C" fn supervisor_trap_handler(context: &mut Context) {
    let scause = r_scause();
    // println!("supervisor_trap_handler {} epc {:x}", scause, context.epc);
    const S_TIMER_INTERRUPT: u64 = (1 << 63) | 5;
    if scause == S_TIMER_INTERRUPT {
        unsafe {
            TICKS += 1;
            if TICKS >= 20 {
                shut_down();
            }
            SCHED.do_timer(context);
            w_sie(r_sie() & !SIE_STIE);
            llvm_asm!("ecall");
        }
    } else {
        panic!("unknown supervisor trap: scause {}", scause);
    }
}

pub fn trap_init() {
    w_stvec(trap_s as u64);
    w_sstatus(r_sstatus() | SSTATUS_SIE);
}
