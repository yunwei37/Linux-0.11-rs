use crate::lib::*;

extern "C" {
    static _end: usize;
    static init_stack_top: usize;
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

pub static mut KERNEL_PTE: usize = 0;
pub static mut MEMORY_END: usize = 0;

pub const KERNEL_STARTPA: usize = 0x80000000;
pub const KERNEL_STARTVA: usize = 0xffffffe000000000;
pub const UART0: usize = 0x10000000;

fn ppage_alloc() -> usize {
    unsafe {
        assert!(MEMORY_END % PGSIZE == 0);
        let last_end = MEMORY_END;
        MEMORY_END += PGSIZE;
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
        MEMORY_END = &_end as *const usize as usize;
    }
    let kernel_pg = ppage_alloc();
    unsafe {
        KERNEL_PTE = kernel_pg;
    }
    create_mapping(
        kernel_pg,
        KERNEL_STARTPA,
        KERNEL_STARTPA,
        1024 * 16 * PGSIZE,
        PTE_R | PTE_W | PTE_X,
    );
    create_mapping(
        kernel_pg,
        KERNEL_STARTVA,
        KERNEL_STARTPA,
        1024 * 16 * PGSIZE,
        PTE_R | PTE_W | PTE_X,
    );
    create_mapping(kernel_pg, UART0, UART0, PGSIZE, PTE_R | PTE_W);
    w_satp((SATP_SV39 | ((kernel_pg) >> 12)) as u64);
    sfence_vma();
    unsafe {
        asm!("ld ra, 40(sp)");
        asm!("add sp, sp, a0");
        asm!("li a0, 0xffffffdf80000000");
        asm!("la sp, init_stack_top");
        asm!("add sp, sp, a0");
        asm!("add ra, ra, a0");
        asm!("ret");
    };
}
