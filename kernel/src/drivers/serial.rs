use core::fmt::*;
use uart_16550::SerialPort;

lazy_static::lazy_static! {
    pub static ref SERIAL: SerialPort = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        serial_port
    };
}

pub fn _serial_print(args: Arguments) -> Result {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        unsafe { (&mut *(&*SERIAL as *const SerialPort as *mut SerialPort)).write_fmt(args).unwrap() }
    });

    Ok(())
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::drivers::serial::_serial_print(format_args!($($arg)*)).unwrap();
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
