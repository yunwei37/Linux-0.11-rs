#![allow(dead_code)]

use crate::lib::*;

extern "C" {
    static _end: usize;
    static init_stack_top: usize;
    static text_start: usize;
    static text_end: usize;
    static rodata_start: usize;
    static rodata_end: usize;
    static data_start: usize;
    static data_end: usize;
    static bss_start: usize;
    static bss_end: usize;
}

pub const PGSIZE: usize = 4096; // bytes per page
pub const PGSHIFT: usize = 12;

pub const PTE_V: usize = 1 << 0;
pub const PTE_R: usize = 1 << 1;
pub const PTE_W: usize = 1 << 2;
pub const PTE_X: usize = 1 << 3;
pub const PTE_U: usize = 1 << 4;
pub const PXMASK: usize = 0x1FF;
pub const SATP_SV39: usize = 8 << 60;

macro_rules! PXSHIFT {
    ($level:expr) => {
        (PGSHIFT + (9 * ($level)))
    };
}

macro_rules! PX {
    ($level:expr, $va:expr) => {
        (((($va) as usize) >> PXSHIFT!($level)) & PXMASK)
    };
}

macro_rules! PTE2PA {
    ($pte:expr) => {
        ((($pte) >> 10) << 12)
    };
}

macro_rules! PA2PTE {
    ($pa:expr) => {
        ((($pa as usize) >> 12) << 10)
    };
}

macro_rules! PGROUNDUP {
    ($sz:expr) => {
        ((($sz) + PGSIZE - 1) & !(PGSIZE - 1))
    };
}

pub static mut KERNEL_PTE: usize = 0;
pub static mut KERNEL_END: usize = 0;
pub static mut INIT_STACK_TOP_ADDR: usize = 0;

pub const KERNEL_STARTPA: usize = 0x80000000;
pub const KERNEL_STARTVA: usize = 0xffffffe000000000;
pub const KERNEL_PVDIFF: usize = 0xffffffe000000000 - 0x80000000;
pub const UART0: usize = 0x10000000;

fn ppage_alloc() -> usize {
    unsafe {
        assert!(KERNEL_END % PGSIZE == 0);
        let last_end = KERNEL_END;
        KERNEL_END += PGSIZE;
        memset(last_end, 0, PGSIZE);
        last_end
    }
}

fn walk(pgtble: usize, va: usize, alloc: bool) -> usize {
    let mut pgtble = pgtble;
    for level in (1..3).rev() {
        let pte = pgtble + (PX!(level, va) * 8);
        let entry = unsafe { *(pte as *mut usize) };
        assert!(entry & PTE_W == 0);
        assert!(entry & PTE_R == 0);
        assert!(entry & PTE_X == 0);
        if entry & PTE_V != 0 {
            pgtble = PTE2PA!(entry);
        } else {
            if alloc {
                pgtble = ppage_alloc();
                memset(pgtble, 0, PGSIZE);
                unsafe { *(pte as *mut usize) = PA2PTE!(pgtble) | PTE_V }
            } else {
                return 0;
            }
        }
    }
    return pgtble + PX!(0, va) * 8;
}

fn create_mapping(pgtble: usize, va: usize, pa: usize, size: usize, perm: usize) {
    assert!(va % PGSIZE == 0);
    assert!(pa % PGSIZE == 0);
    assert!(size % PGSIZE == 0);

    for i in 0..size / PGSIZE {
        let pte = walk(pgtble, va + i * PGSIZE, true);
        assert!(pte != 0);
        unsafe {
            let pte: *mut usize = pte as *mut usize;
            assert!(*pte & PTE_V == 0);
            *pte = PA2PTE!(pa + i * PGSIZE) | perm | PTE_V;
        }
    }
}

pub fn paging_init() {
    unsafe {
        INIT_STACK_TOP_ADDR = &init_stack_top as *const usize as usize;
        KERNEL_END = &_end as *const usize as usize;
        let text_startp = &text_start as *const usize as usize;
        let text_endp = &text_end as *const usize as usize;
        let rodata_startp = &rodata_start as *const usize as usize;
        let rodata_endp = &rodata_end as *const usize as usize;
        let data_startp = &data_start as *const usize as usize;
        let kernel_pg = ppage_alloc();
        KERNEL_PTE = kernel_pg;
        create_mapping(
            kernel_pg,
            KERNEL_STARTPA,
            KERNEL_STARTPA,
            1024 * 16 * PGSIZE,
            PTE_R | PTE_W | PTE_X,
        );
        create_mapping(
            kernel_pg,
            text_startp + KERNEL_PVDIFF,
            text_startp,
            PGROUNDUP!(text_endp - text_startp),
            PTE_R | PTE_X,
        );
        create_mapping(
            kernel_pg,
            rodata_startp + KERNEL_PVDIFF,
            rodata_startp,
            PGROUNDUP!(rodata_endp - rodata_startp),
            PTE_R,
        );
        create_mapping(
            kernel_pg,
            data_startp + KERNEL_PVDIFF,
            data_startp,
            1024 * 16 * PGSIZE + KERNEL_STARTPA - data_startp,
            PTE_R | PTE_W,
        );
        /*
        create_mapping(
            kernel_pg,
            KERNEL_STARTVA,
            KERNEL_STARTPA,
            1024 * 16 * PGSIZE,
            PTE_R | PTE_W | PTE_X,
        );
        */
        create_mapping(kernel_pg, UART0, UART0, PGSIZE, PTE_R | PTE_W);
        w_satp((SATP_SV39 | ((kernel_pg) >> 12)) as u64);
        sfence_vma();
        asm!("ld ra, 168(sp)");
        asm!("add sp, sp, a0");
        asm!("li a0, 0xffffffdf80000000");
        asm!("la sp, init_stack_top");
        asm!("add sp, sp, a0");
        asm!("add ra, ra, a0");
        asm!("ret");
    };
}
