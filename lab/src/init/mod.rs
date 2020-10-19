mod interrupt;
mod register;

use crate::lib::*;

#[no_mangle]
pub extern "C" fn start_kernel() {
    interrupt::trap_init();
    put_chars("Hello RISC-V!\n");
    loop {}
    //shut_down();
}
