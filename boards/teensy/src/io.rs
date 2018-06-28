use core::fmt::*;
use kernel::hil::uart::{self, UART};
use kernel::hil::led;
use kernel::debug;
use mk66::{self, gpio};

pub struct Writer {
    initialized: bool,
}

pub static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let uart = unsafe { &mut mk66::uart::UART0 };
        if !self.initialized {
            self.initialized = true;
            uart.init(uart::UARTParams {
                baud_rate: 115200,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
            });
            uart.enable_tx();
        }

        for c in s.bytes() {
            uart.send_byte(c);
        }
        while !uart.tx_ready() {}

        Ok(())
    }
}

#[cfg(not(test))]
#[no_mangle]
#[allow(unused_variables)]
#[lang="panic_fmt"]
pub unsafe extern "C" fn panic_fmt(args: Arguments, file: &'static str, line: u32) -> ! {
    let writer = &mut WRITER;

    // blink the panic signal
    gpio::PC05.release_claim();
    let led = &mut led::LedLow::new(gpio::PC05.claim_as_gpio());

    debug::panic(led, writer, args, file, line)
}

#[macro_export]
macro_rules! print {
        ($($arg:tt)*) => (
            {
                use core::fmt::write;
                let writer = unsafe { &mut $crate::io::WRITER };
                let _ = write(writer, format_args!($($arg)*));
            }
        );
}

#[macro_export]
macro_rules! println {
        ($fmt:expr) => (print!(concat!($fmt, "\n")));
            ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
