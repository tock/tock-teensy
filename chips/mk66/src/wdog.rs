//! Implementation of the MK66 hardware watchdog timer

use core::mem;
use kernel::common::VolatileCell;
use kernel::hil;

#[repr(C, packed)]
pub struct Registers {
    stctrlh: VolatileCell<u16>,
    stctrll: VolatileCell<u16>,
    tovalh: VolatileCell<u16>,
    tovall: VolatileCell<u16>,
    winh: VolatileCell<u16>,
    winl: VolatileCell<u16>,
    refresh: VolatileCell<u16>,
    unlock: VolatileCell<u16>,
    tmrouth: VolatileCell<u16>,
    tmroutl: VolatileCell<u16>,
    rstcnt: VolatileCell<u16>,
    presc: VolatileCell<u16>,
}

pub struct Wdog {
    registers: *mut Registers,
}

const BASE_ADDRESS: *mut Registers = 0x40052000 as *mut Registers;

pub static mut WDOG: Wdog = Wdog::new();

impl Wdog {
    pub const fn new() -> Wdog {
        Wdog {
            registers: BASE_ADDRESS
        }
    }

    fn unlock(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        const WDOG_UNLOCK_SEQ1: u16 = 0xc520;
        const WDOG_UNLOCK_SEQ2: u16 = 0xd928;

        regs.unlock.set(WDOG_UNLOCK_SEQ1);
        regs.unlock.set(WDOG_UNLOCK_SEQ2);
        unsafe {
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
        }
    }

    #[allow(unused_variables)]
    pub fn start(&self, period: usize) {
        // TODO: implement
    }

    pub fn stop(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.registers) };

        // Must write the correct unlock sequence to the WDOG unlock register before reconfiguring
        // the module.
        self.unlock();

        // WDOG disabled in all power modes
        // Allow future updates to the watchdog configuration
        // Clock source is the 1kHz LPO
        // No testing functionality
        regs.stctrlh.set(1 << 4);
    }

    pub fn tickle(&self) {
        // TODO: implement
    }
}

impl hil::watchdog::Watchdog for Wdog {
    fn start(&self, period: usize) {
        self.start(period);
    }

    fn stop(&self) {
        self.stop();
    }

    fn tickle(&self) {
        self.tickle();
    }
}
