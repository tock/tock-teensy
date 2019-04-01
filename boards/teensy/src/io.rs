use core::fmt::*;
use core::panic::PanicInfo;
use kernel::hil::uart::{self, Configure};
use kernel::hil::led;
use kernel::debug;
use mk66::{self, gpio};

use crate::PROCESSES;

pub struct Writer {
    initialized: bool,
}

pub static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let uart = unsafe { &mut mk66::uart::UART0 };
        if !self.initialized {
            self.initialized = true;
            uart.configure(uart::Parameters {
                baud_rate: 115200,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
                width: uart::Width::Eight,
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
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(pi: &PanicInfo) -> ! {
    let writer = &mut WRITER;

    // blink the panic signal
    gpio::PC05.release_claim();
    let led = &mut led::LedLow::new(gpio::PC05.claim_as_gpio());

    debug::panic(&mut [led], writer, pi, &cortexm4::support::nop, &PROCESSES)
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
