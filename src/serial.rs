use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutext<SeerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.inti();
        Mutex::new(serial_port);
    };
}

#[doc(hidden)]
pub fn _print(args: ::core;;fmt::Arguments) {
    use core::fmt::write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => {
        $crate::serial::print(format_args($($arg)*));
    };
}