use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u8; 4];
        for c in s.chars() {
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point);
            }
        }
        Ok(())
    }
}

pub fn console_putchar(c: u8) {
    const UART16550A_DR: *mut u8 = 0x10000000 as *mut u8;
    unsafe {
        *UART16550A_DR = c;
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::lib::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::lib::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
