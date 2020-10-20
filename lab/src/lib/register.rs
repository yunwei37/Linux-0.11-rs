#![allow(dead_code)]
pub const MSTATUS_MPP_MASK: u64 = 6144;
pub const MSTATUS_MPP_M: u64 = 6144;
pub const MSTATUS_MPP_S: u64 = 2048;
pub const MSTATUS_MPP_U: u64 = 0;
pub const MSTATUS_MIE: u64 = 8;

pub const SSTATUS_SIE: u64 = 2;

#[inline]
pub fn r_sstatus() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, sstatus" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_sstatus(x: u64) {
    unsafe {
        llvm_asm!("csrw sstatus, $0" : : "r" (x));
    }
}

#[inline]
pub fn r_mstatus() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, mstatus" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_mstatus(x: u64) {
    unsafe {
        llvm_asm!("csrw mstatus, $0" : : "r" (x));
    }
}

#[inline]
pub fn w_satp(x: u64) {
    unsafe {
        llvm_asm!("csrw satp, $0" : : "r" (x));
    }
}

#[inline]
pub fn r_medeleg() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, medeleg" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_medeleg(x: u64) {
    unsafe {
        llvm_asm!("csrw medeleg, $0" : : "r" (x));
    }
}

#[inline]
pub fn r_mepc() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, mepc" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_mepc(x: u64) {
    unsafe {
        llvm_asm!("csrw mepc, $0" : : "r" (x));
    }
}

#[inline]
pub fn r_mideleg() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, mideleg" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_mideleg(x: u64) {
    unsafe {
        llvm_asm!("csrw mideleg, $0" : : "r" (x));
    }
}

pub const SIE_SEIE: u64 = 512;
pub const SIE_STIE: u64 = 32;
pub const SIE_SSIE: u64 = 2;

#[inline]
pub fn r_sie() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, sie" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_sie(x: u64) {
    unsafe {
        llvm_asm!("csrw sie, $0" : : "r" (x));
    }
}

pub const MIP_STIP: u64 = 32;

#[inline]
pub fn r_mip() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, mip" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_mip(x: u64) {
    unsafe {
        llvm_asm!("csrw mip, $0" : : "r" (x));
    }
}

pub const MIE_MEIE: u64 = 2048;
pub const MIE_MTIE: u64 = 128;
pub const MIE_MSIE: u64 = 8;

#[inline]
pub fn r_mie() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, mie" : "=r" (x));
    }
    x
}

#[inline]
pub fn w_mie(x: u64) {
    unsafe {
        llvm_asm!("csrw mie, $0" : : "r" (x));
    }
}

#[inline]
pub fn w_mtvec(x: u64) {
    unsafe {
        llvm_asm!("csrw mtvec, $0" : : "r" (x));
    }
}

#[inline]
pub fn w_stvec(x: u64) {
    unsafe {
        llvm_asm!("csrw stvec, $0" : : "r" (x));
    }
}

#[inline]
pub fn w_mscratch(x: u64) {
    unsafe {
        llvm_asm!("csrw mscratch, $0" : : "r" (x));
    }
}

#[inline]
pub fn r_scause() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, scause" : "=r" (x));
    }
    x
}

#[inline]
pub fn r_mcause() -> u64 {
    let mut x: u64;
    unsafe {
        llvm_asm!("csrr $0, mcause" : "=r" (x));
    }
    x
}
