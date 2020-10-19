use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    put_chars(info.message().unwrap().as_str().unwrap());
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

pub fn put_chars(msg: &str) {
    const UART16550A_DR: *mut u8 = 0x10000000 as *mut u8;
    for s in msg.as_bytes() {
        unsafe {
            *UART16550A_DR = s.clone();
        }
    }
}
