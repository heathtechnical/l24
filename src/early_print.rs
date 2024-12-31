use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref EARLY_PRINT_SERIAL: Mutex<EarlyPrintSerial> = {
        Mutex::new(EarlyPrintSerial {
            port: unsafe {
                let mut port = SerialPort::new(0x3F8);
                port.init();
                port
            },
        })
    };
}

pub struct EarlyPrintSerial {
    port: SerialPort,
}

impl EarlyPrintSerial {
    pub fn print(&mut self, s: &str) {
        self.port.write_str(s).expect("Failed");
    }
}

impl fmt::Write for EarlyPrintSerial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    EARLY_PRINT_SERIAL.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! early_print {
    ($($arg:tt)*) => ($crate::early_print::_print(format_args!($($arg)*)));
}
