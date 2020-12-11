#[macro_use]
pub mod console;
pub mod register;

pub use console::*;
pub use register::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic: {}", info);
    shut_down();
    loop {}
}

pub fn shut_down() {
    const SIFIVE_TEST: u32 = 0x100000;
    const VIRT_TEST_FINISHER_PASS: u32 = 0x5555;
    unsafe {
        llvm_asm!("sh $0, 0($1)"::"r"(VIRT_TEST_FINISHER_PASS),"r"(SIFIVE_TEST));
    }
}

pub fn memset(start: usize, ch: u8, n: usize) {
    unsafe {
        for bit in start..start + n {
            *(bit as *mut u8) = ch;
        }
    }
}
