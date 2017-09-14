#![allow(unused)]

use mk66::{gpio, clock, pit, spi};
use kernel::hil::time::{self, Alarm};
use kernel::hil::spi::*;
use kernel::ReturnCode;
use tests::blink;

static mut INTERVAL: u32 = 18_000_000;

static mut WBUF: [u8; 8] = ['H' as u8,
                            'i' as u8,
                            ' ' as u8,
                            't' as u8,
                            'h' as u8,
                            'e' as u8,
                            'r' as u8,
                            'e' as u8];

struct AlarmClient;
impl time::Client for AlarmClient {
    fn fired(&self) {
        unsafe {
            pit::PIT.set_alarm(INTERVAL);
            debug!("Alarm client");
            spi::SPI1.read_write_bytes(&mut WBUF, None, 8);
        }
    }
}

struct SpiClient;
impl SpiMasterClient for SpiClient {
    fn read_write_done(&self, write_buf: &'static mut [u8], 
                              read_buf: Option<&'static mut [u8]>, 
                              len: usize) {
        debug!("Spi client");
        blink::led_toggle();
    }
}

static ALARM: AlarmClient = AlarmClient;
static SPI: SpiClient = SpiClient;

pub fn spi_test() {
    unsafe {
        spi::SPI1.set_client(&SPI);

        let rate = spi::SPI1.set_rate(24_000_000);
        debug!("Baud rate: {}", rate);
        debug!("Bus clock: {}", clock::bus_clock_hz());

        pit::PIT.set_client(&ALARM);
        pit::PIT.set_alarm(INTERVAL);
    }
}
