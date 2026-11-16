use uart_16550::SerialPort;
use spin::Mutex;
use x86_64::instructions::port::Port;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

pub fn init() {
    SERIAL1.lock().init();
}

pub fn try_read_byte() -> Option<u8> {
    let mut lsr = Port::<u8>::new(0x3FD);
    let mut data = Port::<u8>::new(0x3F8);
    unsafe {
        if lsr.read() & 0x01 != 0 {
            Some(data.read())
        } else {
            None
        }
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => {
        $crate::serial_print!("{}\n", format_args!($($arg)*));
    };
}