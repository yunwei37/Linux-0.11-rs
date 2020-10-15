#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

global_asm!(include_str!("head.S"));

#[no_mangle]
pub extern "C" fn start_kernel() -> ! {
    put_char("Hello RISC-V!\n");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn put_char(msg: &str){
    let addr = 0x10000000 as *mut u8;
    for s in msg.as_bytes() {
        unsafe {
            *addr = s.clone();
        }
    }
}
